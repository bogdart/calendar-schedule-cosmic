// SPDX-License-Identifier: GPL-3.0-only

use crate::calendar::{CalendarInfo, Meeting};
use crate::calendar_view::{calendar_grid, create_datetime};
use crate::config::{Config, EventStatusFilter, WorldClockCity};
use crate::fl;
use crate::formatting::parse_hex_color;
use crate::weather::{self, GeocodingResult, WeatherData};
use crate::widgets::{
    calendar_color_dot, secondary_text_style, settings_nav_row_with_icon, settings_page_header,
    spacing,
};
use chrono::{NaiveDate, Timelike};
use cosmic::applet::token::subscription::{
    TokenRequest, TokenUpdate, activation_token_subscription,
};
use cosmic::cctk::sctk::reexports::calloop;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Length, Limits, Rectangle, Subscription, clipboard, window::Id};
use cosmic::iced_futures::stream;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::widget::{self, rectangle_tracker::*};
use futures_util::SinkExt;
use icu::datetime::{
    DateTimeFormatter, DateTimeFormatterPreferences, fieldsets, options::TimePrecision,
};
use icu::locale::Locale;
use std::collections::HashSet;
use tokio::sync::watch;
use tokio::time;

/// Time applet config ID (shared with cosmic-applet-time)
const TIME_CONFIG_ID: &str = "com.system76.CosmicAppletTime";

/// Time display settings (read from system time applet config)
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    cosmic::cosmic_config::cosmic_config_derive::CosmicConfigEntry,
)]
#[version = 1]
pub struct TimeConfig {
    pub military_time: bool,
    pub show_seconds: bool,
    pub first_day_of_week: u8,
    pub show_date_in_top_panel: bool,
    pub show_weekday: bool,
    #[serde(default, skip_serializing_if = "str::is_empty")]
    pub format_strftime: String,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            military_time: false,
            show_seconds: false,
            first_day_of_week: 6, // Sunday
            show_date_in_top_panel: true,
            show_weekday: false,
            format_strftime: String::new(),
        }
    }
}

/// Get the system locale from environment variables
fn get_system_locale() -> Locale {
    for var in ["LC_TIME", "LC_ALL", "LANG"] {
        if let Ok(locale_str) = std::env::var(var) {
            let cleaned_locale = locale_str
                .split('.')
                .next()
                .unwrap_or(&locale_str)
                .replace('_', "-");

            if let Ok(locale) = Locale::try_from_str(&cleaned_locale) {
                return locale;
            }

            if let Some(lang) = cleaned_locale.split('-').next()
                && let Ok(locale) = Locale::try_from_str(lang)
            {
                return locale;
            }
        }
    }
    Locale::try_from_str("en-US").expect("Failed to parse fallback locale")
}

/// The application model stores app-specific state.
pub struct AppModel {
    /// Application state managed by COSMIC runtime.
    core: cosmic::Core,
    /// The popup id.
    popup: Option<Id>,
    /// Configuration data (meeting settings).
    config: Config,
    /// Config context for saving changes.
    config_context: Option<cosmic_config::Config>,
    /// Time display settings (from system time applet).
    time_config: TimeConfig,

    // Time state
    now: chrono::DateTime<chrono::FixedOffset>,
    timezone: Option<chrono_tz::Tz>,
    date_today: NaiveDate,
    date_selected: NaiveDate,
    locale: Locale,
    show_seconds_tx: watch::Sender<bool>,
    token_tx: Option<calloop::channel::Sender<TokenRequest>>,

    // Meeting state
    upcoming_meetings: Vec<Meeting>,
    available_calendars: Vec<CalendarInfo>,
    current_page: PopupPage,
    is_refreshing: bool,
    has_loaded_meetings: bool,

    // Rectangle tracker for popup positioning
    rectangle_tracker: Option<RectangleTracker<u32>>,
    rectangle: Rectangle,

    // Weather state
    weather_data: Option<WeatherData>,
    weather_search_query: String,
    weather_search_results: Vec<GeocodingResult>,
    weather_is_searching: bool,

    // World clock state
    world_clock_search_query: String,
    world_clock_search_results: Vec<GeocodingResult>,
    world_clock_is_searching: bool,
}

impl Default for AppModel {
    fn default() -> Self {
        let now = chrono::Local::now().fixed_offset();
        let today = NaiveDate::from(now.naive_local());
        let time_config = cosmic_config::Config::new(TIME_CONFIG_ID, TimeConfig::VERSION)
            .ok()
            .map(|ctx| TimeConfig::get_entry(&ctx).unwrap_or_else(|(_e, c)| c))
            .unwrap_or_default();
        let (show_seconds_tx, _) = watch::channel(time_config.show_seconds);

        Self {
            core: cosmic::Core::default(),
            popup: None,
            config: Config::default(),
            config_context: None,
            time_config,
            now,
            timezone: None,
            date_today: today,
            date_selected: today,
            locale: get_system_locale(),
            show_seconds_tx,
            token_tx: None,
            upcoming_meetings: Vec::new(),
            available_calendars: Vec::new(),
            current_page: PopupPage::default(),
            is_refreshing: false,
            has_loaded_meetings: false,
            rectangle_tracker: None,
            rectangle: Rectangle::default(),
            weather_data: None,
            weather_search_query: String::new(),
            weather_search_results: Vec::new(),
            weather_is_searching: false,
            world_clock_search_query: String::new(),
            world_clock_search_results: Vec::new(),
            world_clock_is_searching: false,
        }
    }
}

/// Navigation state for popup pages
#[derive(Debug, Default, Clone, PartialEq)]
pub enum PopupPage {
    #[default]
    Main,
    Settings,
    Calendars,
    RefreshSettings,
    EventsToShowSettings,
    WeatherSettings,
    WorldClockSettings,
    About,
}

impl AppModel {
    /// Format panel date/time display using ICU
    fn format_panel_datetime(&self) -> String {
        // Use custom strftime format if set
        if !self.time_config.format_strftime.is_empty() {
            let mut s = String::new();
            if self
                .now
                .format(&self.time_config.format_strftime)
                .write_to(&mut s)
                .is_ok()
            {
                return s;
            }
        }

        let datetime = create_datetime(
            &self.now.naive_local(),
            self.now.hour() as u8,
            self.now.minute() as u8,
            self.now.second() as u8,
        );
        // Use system time settings for hour cycle (24h/12h)
        let mut prefs = DateTimeFormatterPreferences::from(self.locale.clone());
        if self.time_config.military_time {
            prefs.hour_cycle =
                Some(icu::locale::preferences::extensions::unicode::keywords::HourCycle::H23);
        } else {
            prefs.hour_cycle =
                Some(icu::locale::preferences::extensions::unicode::keywords::HourCycle::H12);
        }

        // Build format based on our panel settings
        let show_weekday = self.config.panel_show_weekday;
        let show_year = self.config.panel_show_year;
        let show_seconds = self.time_config.show_seconds;

        match (show_weekday, show_year) {
            (true, true) => {
                // Year + Month + Day + Weekday + Time (e.g., "Thu, Feb 5, 2026, 2:30 PM")
                let mut fs = fieldsets::YMDET::medium();
                if !show_seconds {
                    fs = fs.with_time_precision(TimePrecision::Minute);
                }
                DateTimeFormatter::try_new(prefs, fs)
                    .unwrap()
                    .format(&datetime)
                    .to_string()
            }
            (true, false) => {
                // Month + Day + Weekday + Time (e.g., "Thu, Feb 5, 2:30 PM")
                let mut fs = fieldsets::MDET::medium();
                if !show_seconds {
                    fs = fs.with_time_precision(TimePrecision::Minute);
                }
                DateTimeFormatter::try_new(prefs, fs)
                    .unwrap()
                    .format(&datetime)
                    .to_string()
            }
            (false, true) => {
                // Year + Month + Day + Time (e.g., "Feb 5, 2026, 2:30 PM")
                let mut fs = fieldsets::YMDT::medium();
                if !show_seconds {
                    fs = fs.with_time_precision(TimePrecision::Minute);
                }
                DateTimeFormatter::try_new(prefs, fs)
                    .unwrap()
                    .format(&datetime)
                    .to_string()
            }
            (false, false) => {
                // Month + Day + Time (e.g., "Feb 5, 2:30 PM")
                let mut fs = fieldsets::MDT::medium();
                if !show_seconds {
                    fs = fs.with_time_precision(TimePrecision::Minute);
                }
                DateTimeFormatter::try_new(prefs, fs)
                    .unwrap()
                    .format(&datetime)
                    .to_string()
            }
        }
    }

    /// Get UIDs of enabled meeting source calendars
    fn enabled_meeting_source_uids(&self) -> Vec<String> {
        if self.available_calendars.is_empty() {
            return self.config.enabled_calendar_uids.clone();
        }

        if self.config.enabled_calendar_uids.is_empty() {
            self.available_calendars
                .iter()
                .filter(|c| c.is_meeting_source())
                .map(|c| c.uid.clone())
                .collect()
        } else {
            self.config
                .enabled_calendar_uids
                .iter()
                .filter(|uid| {
                    self.available_calendars
                        .iter()
                        .find(|c| &c.uid == *uid)
                        .is_some_and(CalendarInfo::is_meeting_source)
                })
                .cloned()
                .collect()
        }
    }

    /// Get meetings for a specific day
    fn meetings_for_day(&self, date: NaiveDate) -> Vec<&Meeting> {
        self.upcoming_meetings
            .iter()
            .filter(|m| m.start.date_naive() == date)
            .collect()
    }

    /// Get set of dates that have meetings
    fn days_with_events(&self) -> HashSet<NaiveDate> {
        self.upcoming_meetings
            .iter()
            .map(|m| m.start.date_naive())
            .collect()
    }

    /// Main popup page with calendar and schedule side-by-side
    #[allow(clippy::too_many_lines)]
    fn view_main_page(&self) -> Element<'_, Message> {
        let space = spacing();
        let prefs = DateTimeFormatterPreferences::from(self.locale.clone());

        // LEFT SIDE: Calendar
        let datetime = create_datetime(
            &self.date_selected,
            self.now.hour() as u8,
            self.now.minute() as u8,
            0,
        );

        let month_year = DateTimeFormatter::try_new(prefs, fieldsets::YM::long())
            .unwrap()
            .format(&datetime)
            .to_string();

        let calendar_header = widget::row()
            .push(
                widget::container(widget::text::heading(month_year)).padding([
                    0,
                    0,
                    0,
                    space.space_s,
                ]),
            )
            .push(widget::horizontal_space())
            .push(
                widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                    .padding(4)
                    .on_press(Message::PreviousMonth),
            )
            .push(
                widget::button::icon(widget::icon::from_name("go-next-symbolic"))
                    .padding(4)
                    .on_press(Message::NextMonth),
            )
            .spacing(space.space_xxxs)
            .align_y(cosmic::iced::Alignment::Center)
            .width(Length::Fill);

        let first_day_of_week = chrono::Weekday::try_from(self.time_config.first_day_of_week)
            .unwrap_or(chrono::Weekday::Sun);
        let calendar = calendar_grid(
            self.date_selected,
            self.date_today,
            first_day_of_week,
            &self.locale,
            &self.days_with_events(),
        );

        let mut calendar_section = widget::column()
            .push(calendar_header)
            .push(widget::vertical_space().height(space.space_xxs))
            .push(widget::container(calendar).padding([0, 0, 0, space.space_s]))
            .width(Length::Fixed(290.0))
            .padding([space.space_xs, 0]);

        // Weather section (below calendar)
        if self.weather_data.is_some() {
            calendar_section = calendar_section
                .push(
                    cosmic::applet::padded_control(widget::divider::horizontal::default())
                        .padding([space.space_xxs, space.space_s]),
                )
                .push(widget::container(self.view_weather()).padding([
                    0,
                    space.space_xs,
                    0,
                    space.space_s,
                ]));
        }

        // World clock section (below weather)
        if !self.config.world_clock_cities.is_empty() {
            calendar_section = calendar_section
                .push(
                    cosmic::applet::padded_control(widget::divider::horizontal::default())
                        .padding([space.space_xxs, space.space_s]),
                )
                .push(widget::container(self.view_world_clock()).padding([
                    0,
                    space.space_xs,
                    0,
                    space.space_s,
                ]));
        }

        // RIGHT SIDE: Schedule for selected day
        let selected_datetime = create_datetime(&self.date_selected, 12, 0, 0);
        let selected_date_str = DateTimeFormatter::try_new(prefs, fieldsets::MDE::long())
            .unwrap()
            .format(&selected_datetime)
            .to_string();

        let day_meetings = self.meetings_for_day(self.date_selected);

        let schedule_content: Element<'_, Message> = if day_meetings.is_empty() {
            widget::text::body(fl!("no-meetings-day"))
                .apply(widget::container)
                .padding(space.space_s)
                .into()
        } else {
            let secondary_text = cosmic::theme::Text::Custom(secondary_text_style);
            let mut list = widget::column().spacing(space.space_xxs);

            for meeting in day_meetings {
                let time_str = meeting.start.format("%H:%M").to_string();
                let uid = meeting.uid.clone();

                let mut row = widget::row()
                    .spacing(space.space_xs)
                    .align_y(cosmic::iced::Alignment::Center);

                if self.config.popup_calendar_indicator
                    && let Some(dot) = calendar_color_dot::<Message>(
                        &meeting.calendar_uid,
                        &self.available_calendars,
                        8.0,
                        None,
                    )
                {
                    row = row.push(dot);
                }

                row = row
                    .push(widget::text::body(time_str).class(secondary_text))
                    .push(
                        widget::container(widget::text::body(&meeting.title)).width(Length::Fill),
                    );

                list =
                    list.push(cosmic::applet::menu_button(row).on_press(Message::OpenEvent(uid)));
            }

            list.into()
        };

        let schedule_section = widget::column()
            .push(widget::text::heading(selected_date_str))
            .push(widget::vertical_space().height(space.space_xs))
            .push(schedule_content)
            .width(Length::Fill)
            .padding([
                space.space_xs,
                space.space_xs,
                space.space_xs,
                space.space_xxs,
            ]);

        // Combined layout
        let main_content = widget::row()
            .push(calendar_section)
            .push(widget::divider::vertical::default())
            .push(schedule_section)
            .spacing(space.space_xxs);

        // Bottom actions
        let bottom_actions = widget::column()
            .push(
                cosmic::applet::padded_control(widget::divider::horizontal::default())
                    .padding([space.space_xxs, space.space_s]),
            )
            .push(
                cosmic::applet::menu_button(
                    widget::row()
                        .push(
                            widget::icon::from_name("office-calendar-symbolic").size(space.space_m),
                        )
                        .push(widget::text::body(fl!("open-calendar")))
                        .push(widget::horizontal_space())
                        .spacing(space.space_xs)
                        .align_y(cosmic::iced::Alignment::Center)
                        .width(Length::Fill),
                )
                .on_press(Message::OpenCalendar),
            )
            .push(
                cosmic::applet::menu_button(
                    widget::row()
                        .push(
                            widget::icon::from_name("preferences-system-time-symbolic")
                                .size(space.space_m),
                        )
                        .push(widget::text::body(fl!("datetime-settings")))
                        .push(widget::horizontal_space())
                        .spacing(space.space_xs)
                        .align_y(cosmic::iced::Alignment::Center)
                        .width(Length::Fill),
                )
                .on_press(Message::OpenDateTimeSettings),
            )
            .push(
                cosmic::applet::menu_button(
                    widget::row()
                        .push(
                            widget::icon::from_name("preferences-system-symbolic")
                                .size(space.space_m),
                        )
                        .push(widget::text::body(fl!("settings")))
                        .push(widget::horizontal_space())
                        .spacing(space.space_xs)
                        .align_y(cosmic::iced::Alignment::Center)
                        .width(Length::Fill),
                )
                .on_press(Message::Navigate(PopupPage::Settings)),
            );

        widget::column()
            .push(main_content)
            .push(bottom_actions)
            .padding([space.space_xxs, space.space_none])
            .into()
    }

    /// Settings page
    #[allow(clippy::too_many_lines)]
    fn view_settings_page(&self) -> Element<'_, Message> {
        let space = spacing();
        let mut content = widget::column::with_capacity(8)
            .padding(space.space_xs)
            .spacing(space.space_xs)
            .width(Length::Fill);

        content = content.push(settings_page_header(
            fl!("back"),
            fl!("settings"),
            Message::Navigate(PopupPage::Main),
        ));

        let calendars_summary = if self.available_calendars.is_empty() {
            fl!("calendars-none")
        } else {
            let total = self.available_calendars.len();
            let meeting_sources: Vec<_> = self
                .available_calendars
                .iter()
                .filter(|c| c.is_meeting_source())
                .collect();
            let enabled = if self.config.enabled_calendar_uids.is_empty() {
                meeting_sources.len()
            } else {
                meeting_sources
                    .iter()
                    .filter(|c| self.config.enabled_calendar_uids.contains(&c.uid))
                    .count()
            };
            fl!("calendars-enabled", enabled = enabled, total = total)
        };

        let filter_summary = {
            let all_day = !self.config.show_all_day_events;
            let status = match self.config.event_status_filter {
                EventStatusFilter::All => None,
                EventStatusFilter::Accepted => Some(fl!("filter-summary-accepted")),
                EventStatusFilter::AcceptedOrTentative => Some(fl!("filter-summary-tentative")),
            };
            match (all_day, status) {
                (false, None) => fl!("filter-summary-all"),
                (true, None) => fl!("filter-summary-no-all-day"),
                (false, Some(s)) => s,
                (true, Some(s)) => fl!(
                    "filter-summary-combo",
                    allday = fl!("filter-summary-no-all-day"),
                    status = s
                ),
            }
        };

        let refresh_summary = if self.config.auto_refresh_enabled {
            fl!(
                "refresh-summary-on",
                interval = self.config.auto_refresh_interval_minutes
            )
        } else {
            fl!("refresh-summary-off")
        };

        let calendars_section = widget::list_column()
            .list_item_padding([space.space_xxs, space.space_xs])
            .add(settings_nav_row_with_icon(
                "x-office-calendar-symbolic",
                fl!("calendars-section"),
                calendars_summary,
                Message::Navigate(PopupPage::Calendars),
            ))
            .add(settings_nav_row_with_icon(
                "view-filter-symbolic",
                fl!("filter-events-section"),
                filter_summary,
                Message::Navigate(PopupPage::EventsToShowSettings),
            ))
            .add(settings_nav_row_with_icon(
                "emblem-synchronizing-symbolic",
                fl!("refresh-section"),
                refresh_summary,
                Message::Navigate(PopupPage::RefreshSettings),
            ));

        content = content.push(calendars_section);
        content = content.push(widget::vertical_space().height(space.space_xs));

        // Weather & World Clock
        let weather_summary = self
            .config
            .weather_city
            .as_ref()
            .map_or_else(|| fl!("weather-not-configured"), |c| c.name.clone());

        let clock_summary = if self.config.world_clock_cities.is_empty() {
            fl!("world-clock-not-configured")
        } else {
            fl!(
                "world-clock-cities-count",
                count = self.config.world_clock_cities.len()
            )
        };

        let features_section = widget::list_column()
            .list_item_padding([space.space_xxs, space.space_xs])
            .add(settings_nav_row_with_icon(
                "weather-clear-symbolic",
                fl!("weather-section"),
                weather_summary,
                Message::Navigate(PopupPage::WeatherSettings),
            ))
            .add(settings_nav_row_with_icon(
                "preferences-system-time-symbolic",
                fl!("world-clock-section"),
                clock_summary,
                Message::Navigate(PopupPage::WorldClockSettings),
            ));

        content = content.push(features_section);
        content = content.push(widget::vertical_space().height(space.space_xs));

        // Panel display settings
        let panel_section = widget::list_column()
            .list_item_padding([space.space_xxs, space.space_xs])
            .add(
                widget::row()
                    .push(widget::text::body(fl!("panel-show-weekday")))
                    .push(widget::horizontal_space())
                    .push(
                        widget::toggler(self.config.panel_show_weekday)
                            .on_toggle(Message::SetPanelShowWeekday),
                    )
                    .align_y(cosmic::iced::Alignment::Center)
                    .width(Length::Fill),
            )
            .add(
                widget::row()
                    .push(widget::text::body(fl!("panel-show-year")))
                    .push(widget::horizontal_space())
                    .push(
                        widget::toggler(self.config.panel_show_year)
                            .on_toggle(Message::SetPanelShowYear),
                    )
                    .align_y(cosmic::iced::Alignment::Center)
                    .width(Length::Fill),
            );

        content = content.push(panel_section);
        content = content.push(widget::vertical_space().height(space.space_xs));

        let about_section = widget::list_column()
            .list_item_padding([space.space_xxs, space.space_xs])
            .add(settings_nav_row_with_icon(
                "help-about-symbolic",
                fl!("about"),
                String::new(),
                Message::Navigate(PopupPage::About),
            ));

        content = content.push(about_section);
        content.into()
    }

    /// Calendars page
    fn view_calendars_page(&self) -> Element<'_, Message> {
        let space = spacing();
        let mut content = widget::column::with_capacity(2 + self.available_calendars.len())
            .padding(space.space_xs)
            .spacing(space.space_xs)
            .width(Length::Fill);

        content = content.push(
            widget::column()
                .push(
                    widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                        .extra_small()
                        .padding(space.space_none)
                        .label(fl!("settings"))
                        .spacing(space.space_xxxs)
                        .class(cosmic::theme::Button::Link)
                        .on_press(Message::Navigate(PopupPage::Settings)),
                )
                .push(widget::text::title4(fl!("calendars-section")))
                .spacing(space.space_xxs),
        );

        let mut calendars_list =
            widget::list_column().list_item_padding([space.space_xxs, space.space_xs]);

        for calendar in &self.available_calendars {
            let is_meeting_source = calendar.is_meeting_source();
            let is_enabled = is_meeting_source
                && (self.config.enabled_calendar_uids.is_empty()
                    || self.config.enabled_calendar_uids.contains(&calendar.uid));

            let uid = calendar.uid.clone();

            let mut row = widget::row()
                .spacing(space.space_xs)
                .align_y(cosmic::iced::Alignment::Center);

            if let Some(color) = &calendar.color
                && let Some(parsed_color) = parse_hex_color(color)
            {
                row = row.push(
                    widget::container(widget::Space::new(0, 0))
                        .width(Length::Fixed(12.0))
                        .height(Length::Fixed(12.0))
                        .class(cosmic::theme::Container::custom(move |_theme| {
                            cosmic::iced_widget::container::Style {
                                background: Some(cosmic::iced::Background::Color(parsed_color)),
                                border: cosmic::iced::Border {
                                    radius: 6.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        })),
                );
            }

            let name_col = widget::column()
                .push(widget::text::body(&calendar.display_name))
                .spacing(space.space_xxxs);

            let toggler = if is_meeting_source {
                widget::toggler(is_enabled).on_toggle(move |_| Message::ToggleCalendar(uid.clone()))
            } else {
                widget::toggler(false)
            };

            row = row
                .push(name_col)
                .push(widget::horizontal_space())
                .push(toggler);

            calendars_list = calendars_list.add(row);
        }

        if self.available_calendars.is_empty() {
            let secondary_text = cosmic::theme::Text::Custom(secondary_text_style);
            content = content
                .push(widget::text::body(fl!("no-calendars-description")).class(secondary_text));
        } else {
            content = content.push(calendars_list);
        }

        content.into()
    }

    /// Refresh settings page
    fn view_refresh_settings_page(&self) -> Element<'_, Message> {
        let space = spacing();
        let mut content = widget::column::with_capacity(6)
            .padding(space.space_xs)
            .spacing(space.space_xs)
            .width(Length::Fill);

        content = content.push(settings_page_header(
            fl!("settings"),
            fl!("refresh-section"),
            Message::Navigate(PopupPage::Settings),
        ));

        let refresh_settings = widget::list_column()
            .list_item_padding([space.space_xxs, space.space_xs])
            .add(
                widget::row()
                    .push(widget::text::body(fl!("auto-refresh")))
                    .push(widget::horizontal_space())
                    .push(
                        widget::toggler(self.config.auto_refresh_enabled)
                            .on_toggle(Message::SetAutoRefresh),
                    )
                    .align_y(cosmic::iced::Alignment::Center)
                    .width(Length::Fill),
            );

        content = content.push(refresh_settings);
        content = content.push(widget::vertical_space().height(space.space_xs));

        let sync_button = widget::button::standard(if self.is_refreshing {
            fl!("refreshing")
        } else {
            fl!("refresh-now")
        });
        let sync_button = if self.is_refreshing {
            sync_button
        } else {
            sync_button.on_press(Message::RefreshCalendars)
        };

        content = content.push(
            widget::container(sync_button)
                .width(Length::Fill)
                .align_x(cosmic::iced::alignment::Horizontal::Center),
        );

        content.into()
    }

    /// Events filter settings page
    fn view_events_to_show_settings_page(&self) -> Element<'_, Message> {
        let space = spacing();
        let mut content = widget::column::with_capacity(6)
            .padding(space.space_xs)
            .spacing(space.space_xs)
            .width(Length::Fill);

        content = content.push(settings_page_header(
            fl!("settings"),
            fl!("filter-events-section"),
            Message::Navigate(PopupPage::Settings),
        ));

        let status_options = vec![
            fl!("status-filter-all"),
            fl!("status-filter-accepted"),
            fl!("status-filter-accepted-tentative"),
        ];
        let status_idx = match self.config.event_status_filter {
            EventStatusFilter::All => Some(0),
            EventStatusFilter::Accepted => Some(1),
            EventStatusFilter::AcceptedOrTentative => Some(2),
        };

        let filter_settings = widget::list_column()
            .list_item_padding([space.space_xxs, space.space_xs])
            .add(
                widget::row()
                    .push(widget::text::body(fl!("show-all-day-events")))
                    .push(widget::horizontal_space())
                    .push(
                        widget::toggler(self.config.show_all_day_events)
                            .on_toggle(Message::SetShowAllDayEvents),
                    )
                    .align_y(cosmic::iced::Alignment::Center)
                    .width(Length::Fill),
            )
            .add(
                widget::row()
                    .push(widget::text::body(fl!("status-filter-section")))
                    .push(widget::horizontal_space())
                    .push(widget::dropdown(
                        status_options,
                        status_idx,
                        Message::SetEventStatusFilter,
                    ))
                    .align_y(cosmic::iced::Alignment::Center)
                    .width(Length::Fill),
            );

        content = content.push(filter_settings);
        content.into()
    }

    /// About page
    fn view_about_page(&self) -> Element<'_, Message> {
        let space = spacing();
        let secondary_text = cosmic::theme::Text::Custom(secondary_text_style);

        let mut content = widget::column::with_capacity(6)
            .padding(space.space_xs)
            .spacing(space.space_xs)
            .align_x(cosmic::iced::Alignment::Center)
            .width(Length::Fill);

        content = content.push(
            widget::column()
                .push(
                    widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                        .extra_small()
                        .padding(space.space_none)
                        .label(fl!("settings"))
                        .spacing(space.space_xxxs)
                        .class(cosmic::theme::Button::Link)
                        .on_press(Message::Navigate(PopupPage::Settings)),
                )
                .width(Length::Fill),
        );

        content = content.push(widget::vertical_space().height(space.space_m));
        content = content.push(widget::icon::from_name("x-office-calendar-symbolic").size(64));
        content = content.push(widget::text::title3(fl!("app-title")));

        let version = env!("CARGO_PKG_VERSION");
        let git_hash = env!("GIT_HASH");
        let version_str = format!("{version} ({git_hash})");
        content = content
            .push(widget::text::body(fl!("version", version = version_str)).class(secondary_text));

        content = content.push(widget::vertical_space().height(space.space_l));
        content.into()
    }

    /// Weather widget for the main page left column.
    fn view_weather(&self) -> Element<'_, Message> {
        let space = spacing();
        let secondary_text = cosmic::theme::Text::Custom(secondary_text_style);

        let Some(ref data) = self.weather_data else {
            return widget::Space::new(0, 0).into();
        };

        let mut col = widget::column().spacing(space.space_xxs);

        // Current weather row: icon + temperature + city name
        let city_name = self
            .config
            .weather_city
            .as_ref()
            .map_or("", |c| c.name.as_str());

        let current_row = widget::row()
            .push(
                widget::icon::from_name(weather::weather_icon_name(data.current.weather_code))
                    .size(20)
                    .symbolic(true),
            )
            .push(widget::text::body(format!(
                "{:.0}°C",
                data.current.temperature
            )))
            .push(widget::text::body(city_name).class(secondary_text))
            .spacing(space.space_xs)
            .align_y(cosmic::iced::Alignment::Center);

        col = col.push(current_row);

        // Forecast row: 4 days
        let mut forecast_row = widget::row().spacing(space.space_s);
        for day in &data.daily {
            let day_name = day.date.format("%a").to_string();
            let day_col = widget::column()
                .push(widget::text::caption(day_name).class(secondary_text))
                .push(
                    widget::icon::from_name(weather::weather_icon_name(day.weather_code))
                        .size(16)
                        .symbolic(true),
                )
                .push(widget::text::caption(format!("{:.0}°", day.temp_max)))
                .spacing(space.space_xxxs)
                .align_x(cosmic::iced::Alignment::Center);
            forecast_row = forecast_row.push(day_col);
        }

        col = col.push(forecast_row);

        col.into()
    }

    /// World clock widget for the main page left column.
    fn view_world_clock(&self) -> Element<'_, Message> {
        let space = spacing();
        let secondary_text = cosmic::theme::Text::Custom(secondary_text_style);

        let mut col = widget::column().spacing(space.space_xxxs);

        for city in &self.config.world_clock_cities {
            let tz: Result<chrono_tz::Tz, _> = city.timezone.parse();
            let time_str = if let Ok(tz) = tz {
                let city_now = self.now.with_timezone(&tz);
                if self.time_config.military_time {
                    city_now.format("%H:%M").to_string()
                } else {
                    city_now.format("%I:%M %p").to_string()
                }
            } else {
                "—".to_string()
            };

            let row = widget::row()
                .push(widget::text::body(&city.name))
                .push(widget::horizontal_space())
                .push(widget::text::body(time_str).class(secondary_text))
                .align_y(cosmic::iced::Alignment::Center)
                .width(Length::Fill);

            col = col.push(row);
        }

        col.into()
    }

    /// Weather settings page.
    fn view_weather_settings_page(&self) -> Element<'_, Message> {
        let space = spacing();
        let secondary_text = cosmic::theme::Text::Custom(secondary_text_style);

        let mut content = widget::column::with_capacity(8)
            .padding(space.space_xs)
            .spacing(space.space_xs)
            .width(Length::Fill);

        content = content.push(settings_page_header(
            fl!("settings"),
            fl!("weather-section"),
            Message::Navigate(PopupPage::Settings),
        ));

        // Current city
        if let Some(ref city) = self.config.weather_city {
            let city_row = widget::row()
                .push(
                    widget::icon::from_name("weather-clear-symbolic")
                        .size(space.space_m)
                        .symbolic(true),
                )
                .push(widget::text::body(format!(
                    "{}, {}",
                    city.name, city.country
                )))
                .push(widget::horizontal_space())
                .push(
                    widget::button::destructive(fl!("weather-clear"))
                        .on_press(Message::ClearWeatherCity),
                )
                .spacing(space.space_xs)
                .align_y(cosmic::iced::Alignment::Center)
                .width(Length::Fill);

            let city_section = widget::list_column()
                .list_item_padding([space.space_xxs, space.space_xs])
                .add(city_row);

            content = content.push(city_section);
            content = content.push(widget::vertical_space().height(space.space_xs));
        }

        // Search input
        content = content.push(
            widget::text_input(
                fl!("weather-search-placeholder"),
                &self.weather_search_query,
            )
            .on_input(Message::WeatherSearchInput)
            .width(Length::Fill),
        );

        // Search results
        if self.weather_is_searching {
            content = content.push(widget::text::body("...").class(secondary_text));
        } else if !self.weather_search_results.is_empty() {
            let mut results =
                widget::list_column().list_item_padding([space.space_xxs, space.space_xs]);

            for result in &self.weather_search_results {
                let label = if let Some(ref admin) = result.admin1 {
                    format!("{}, {}, {}", result.name, admin, result.country)
                } else {
                    format!("{}, {}", result.name, result.country)
                };
                let r = result.clone();
                results = results.add(
                    cosmic::applet::menu_button(widget::text::body(label))
                        .on_press(Message::SelectWeatherCity(r)),
                );
            }
            content = content.push(results);
        }

        content.into()
    }

    /// World clock settings page.
    fn view_world_clock_settings_page(&self) -> Element<'_, Message> {
        let space = spacing();
        let secondary_text = cosmic::theme::Text::Custom(secondary_text_style);

        let mut content = widget::column::with_capacity(8)
            .padding(space.space_xs)
            .spacing(space.space_xs)
            .width(Length::Fill);

        content = content.push(settings_page_header(
            fl!("settings"),
            fl!("world-clock-section"),
            Message::Navigate(PopupPage::Settings),
        ));

        // Current cities
        if !self.config.world_clock_cities.is_empty() {
            let mut list =
                widget::list_column().list_item_padding([space.space_xxs, space.space_xs]);

            for (idx, city) in self.config.world_clock_cities.iter().enumerate() {
                let row = widget::row()
                    .push(
                        widget::icon::from_name("preferences-system-time-symbolic")
                            .size(space.space_s)
                            .symbolic(true),
                    )
                    .push(widget::text::body(&city.name))
                    .push(widget::horizontal_space())
                    .push(widget::text::body(&city.timezone).class(secondary_text))
                    .push(
                        widget::button::icon(widget::icon::from_name("edit-delete-symbolic"))
                            .extra_small()
                            .on_press(Message::RemoveWorldClockCity(idx)),
                    )
                    .spacing(space.space_xs)
                    .align_y(cosmic::iced::Alignment::Center)
                    .width(Length::Fill);
                list = list.add(row);
            }
            content = content.push(list);
            content = content.push(widget::vertical_space().height(space.space_xs));
        }

        // Add city (only if < 3)
        if self.config.world_clock_cities.len() < 3 {
            content = content.push(
                widget::text_input(
                    fl!("world-clock-search-placeholder"),
                    &self.world_clock_search_query,
                )
                .on_input(Message::WorldClockSearchInput)
                .width(Length::Fill),
            );

            if self.world_clock_is_searching {
                content = content.push(widget::text::body("...").class(secondary_text));
            } else if !self.world_clock_search_results.is_empty() {
                let mut results =
                    widget::list_column().list_item_padding([space.space_xxs, space.space_xs]);

                for result in &self.world_clock_search_results {
                    let label = if let Some(ref admin) = result.admin1 {
                        format!("{}, {}, {}", result.name, admin, result.country)
                    } else {
                        format!("{}, {}", result.name, result.country)
                    };
                    let r = result.clone();
                    results = results.add(
                        cosmic::applet::menu_button(widget::text::body(label))
                            .on_press(Message::SelectWorldClockCity(r)),
                    );
                }
                content = content.push(results);
            }
        }

        content.into()
    }
}

/// Open a URL in the default browser.
pub fn open_url(url: &str) -> bool {
    std::process::Command::new("xdg-open")
        .arg(url)
        .spawn()
        .is_ok()
}

/// Open an event in the calendar application.
fn open_event_in_calendar(event_uid: &str) {
    let desktop_file = std::process::Command::new("xdg-mime")
        .args(["query", "default", "text/calendar"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    if desktop_file.is_empty() {
        return;
    }

    let lowercase = desktop_file.to_lowercase();
    if lowercase.contains("gnome-calendar") || lowercase.contains("gnome.calendar") {
        let _ = std::process::Command::new("gnome-calendar")
            .arg("--uuid")
            .arg(event_uid)
            .spawn();
    } else {
        let gtk_result = std::process::Command::new("gtk-launch")
            .arg(&desktop_file)
            .spawn();

        if gtk_result.is_err() {
            let xdg_dirs = xdg::BaseDirectories::new();
            if let Some(path) = xdg_dirs.find_data_file(format!("applications/{desktop_file}")) {
                let path_str = path.to_string_lossy();
                let _ = std::process::Command::new("gio")
                    .args(["launch", path_str.as_ref()])
                    .spawn();
            }
        }
    }
}

/// Messages emitted by the application.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    Tick,
    TimezoneUpdate(String),
    SelectDay(NaiveDate),
    PreviousMonth,
    NextMonth,
    Navigate(PopupPage),
    UpdateConfig(Config),
    UpdateTimeConfig(TimeConfig),
    MeetingsUpdated(Vec<Meeting>),
    CalendarsLoaded(Vec<CalendarInfo>),
    ToggleCalendar(String),
    SetPanelShowWeekday(bool),
    SetPanelShowYear(bool),
    SetShowAllDayEvents(bool),
    SetEventStatusFilter(usize),
    SetAutoRefresh(bool),
    RefreshCalendars,
    RefreshCompleted,
    CalendarChanged,
    SystemResumed,
    OpenCalendar,
    OpenEvent(String),
    OpenUrl(String),
    OpenDateTimeSettings,
    CopyToClipboard(String),
    Token(TokenUpdate),
    Rectangle(RectangleUpdate<u32>),
    // Weather
    WeatherUpdated(Option<WeatherData>),
    WeatherSearchInput(String),
    WeatherSearchResults(Vec<GeocodingResult>),
    SelectWeatherCity(GeocodingResult),
    ClearWeatherCity,
    // World Clock
    WorldClockSearchInput(String),
    WorldClockSearchResults(Vec<GeocodingResult>),
    SelectWorldClockCity(GeocodingResult),
    RemoveWorldClockCity(usize),
    Noop,
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.bogdart.CalendarSchedule";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let config_context = cosmic_config::Config::new(Self::APP_ID, Config::VERSION).ok();
        let config = config_context
            .as_ref()
            .map(|ctx| Config::get_entry(ctx).unwrap_or_else(|(_e, c)| c))
            .unwrap_or_default();

        // Load time config from system time applet
        let time_config = cosmic_config::Config::new(TIME_CONFIG_ID, TimeConfig::VERSION)
            .ok()
            .map(|ctx| TimeConfig::get_entry(&ctx).unwrap_or_else(|(_e, c)| c))
            .unwrap_or_default();

        let now = chrono::Local::now().fixed_offset();
        let today = NaiveDate::from(now.naive_local());
        let (show_seconds_tx, _) = watch::channel(time_config.show_seconds);

        let enabled_uids = config.enabled_calendar_uids.clone();
        let upcoming_count = config.upcoming_events_count as usize;
        let additional_emails = config.additional_emails.clone();

        let app = AppModel {
            core,
            config,
            config_context,
            time_config,
            now,
            date_today: today,
            date_selected: today,
            locale: get_system_locale(),
            show_seconds_tx,
            ..Default::default()
        };

        let calendars_task = Task::perform(
            async { crate::calendar::get_available_calendars().await },
            |calendars| Message::CalendarsLoaded(calendars).into(),
        );

        let meetings_task = Task::perform(
            async move {
                crate::calendar::get_upcoming_meetings(
                    &enabled_uids,
                    upcoming_count + 1,
                    &additional_emails,
                )
                .await
            },
            |meetings| Message::MeetingsUpdated(meetings).into(),
        );

        (app, Task::batch([calendars_task, meetings_task]))
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let time_str = self.format_panel_datetime();

        let button = widget::button::custom(self.core.applet.text(time_str))
            .class(cosmic::theme::Button::AppletIcon)
            .on_press_down(Message::TogglePopup);

        let content = if let Some(tracker) = self.rectangle_tracker.as_ref() {
            Element::from(tracker.container(0, button).ignore_bounds(true))
        } else {
            button.into()
        };

        self.core.applet.autosize_window(content).into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let content: Element<'_, Self::Message> = match self.current_page {
            PopupPage::Main => self.view_main_page(),
            PopupPage::Settings => self.view_settings_page(),
            PopupPage::Calendars => self.view_calendars_page(),
            PopupPage::RefreshSettings => self.view_refresh_settings_page(),
            PopupPage::EventsToShowSettings => self.view_events_to_show_settings_page(),
            PopupPage::WeatherSettings => self.view_weather_settings_page(),
            PopupPage::WorldClockSettings => self.view_world_clock_settings_page(),
            PopupPage::About => self.view_about_page(),
        };

        let limits = Limits::NONE
            .max_width(580.0)
            .min_width(550.0)
            .min_height(300.0)
            .max_height(800.0);

        self.core
            .applet
            .popup_container(content)
            .limits(limits)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        use std::hash::{Hash, Hasher};

        let enabled_uids = self.config.enabled_calendar_uids.clone();
        let upcoming_count = self.config.upcoming_events_count as usize;
        let additional_emails = self.config.additional_emails.clone();
        let auto_refresh_enabled = self.config.auto_refresh_enabled;
        let auto_refresh_interval = self.config.auto_refresh_interval_minutes;

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        enabled_uids.hash(&mut hasher);
        upcoming_count.hash(&mut hasher);
        additional_emails.hash(&mut hasher);
        let config_hash = hasher.finish();

        fn time_subscription(mut show_seconds: watch::Receiver<bool>) -> Subscription<Message> {
            Subscription::run_with_id(
                "time-sub",
                stream::channel(1, |mut output| async move {
                    show_seconds.mark_changed();
                    let mut period = 60u64;
                    let mut timer = time::interval(time::Duration::from_secs(period));
                    timer.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

                    loop {
                        tokio::select! {
                            _ = timer.tick() => {
                                let _ = output.send(Message::Tick).await;
                                let current = chrono::Local::now().second() as u64 % period;
                                if current != 0 {
                                    timer.reset_after(time::Duration::from_secs(period - current));
                                }
                            },
                            Ok(()) = show_seconds.changed() => {
                                let seconds = *show_seconds.borrow_and_update();
                                period = if seconds { 1 } else { 60 };
                                let delta = if seconds {
                                    time::Duration::from_secs(1)
                                } else {
                                    time::Duration::from_secs(60 - chrono::Utc::now().second() as u64 % 60)
                                };
                                timer = time::interval_at(time::Instant::now() + delta, time::Duration::from_secs(period));
                                timer.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
                            }
                        }
                    }
                }),
            )
        }

        let show_seconds_rx = self.show_seconds_tx.subscribe();
        let mut subscriptions = vec![
            rectangle_tracker_subscription(0).map(|e| Message::Rectangle(e.1)),
            time_subscription(show_seconds_rx),
            activation_token_subscription(0).map(Message::Token),
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
            // Watch the system time applet config for changes
            self.core()
                .watch_config::<TimeConfig>(TIME_CONFIG_ID)
                .map(|update| Message::UpdateTimeConfig(update.config)),
            Subscription::run_with_id(
                config_hash,
                stream::channel(4, move |mut channel| {
                    let enabled_uids = enabled_uids.clone();
                    let additional_emails = additional_emails.clone();
                    async move {
                        let mut interval =
                            tokio::time::interval(std::time::Duration::from_secs(60));
                        loop {
                            interval.tick().await;
                            let calendars = crate::calendar::get_available_calendars().await;
                            let _ = channel.send(Message::CalendarsLoaded(calendars)).await;
                            let meetings = crate::calendar::get_upcoming_meetings(
                                &enabled_uids,
                                upcoming_count + 1,
                                &additional_emails,
                            )
                            .await;
                            let _ = channel.send(Message::MeetingsUpdated(meetings)).await;
                        }
                    }
                }),
            ),
        ];

        // Weather refresh subscription (every 30 minutes)
        if let Some(ref city) = self.config.weather_city {
            let city = city.clone();
            subscriptions.push(Subscription::run_with_id(
                ("weather", city.name.clone()),
                stream::channel(1, move |mut output| async move {
                    loop {
                        if let Some(data) = weather::fetch_weather(&city).await {
                            let _ = output.send(Message::WeatherUpdated(Some(data))).await;
                        }
                        tokio::time::sleep(std::time::Duration::from_secs(30 * 60)).await;
                    }
                }),
            ));
        }

        if auto_refresh_enabled {
            let refresh_uids = self.config.enabled_calendar_uids.clone();
            subscriptions.push(Subscription::run_with_id(
                ("auto-refresh", auto_refresh_interval),
                stream::channel(2, move |mut channel| async move {
                    let interval_secs = u64::from(auto_refresh_interval) * 60;
                    let mut interval =
                        tokio::time::interval(std::time::Duration::from_secs(interval_secs));
                    interval.tick().await;
                    loop {
                        interval.tick().await;
                        crate::calendar::refresh_calendars(&refresh_uids).await;
                        let _ = channel.send(Message::RefreshCalendars).await;
                    }
                }),
            ));
        }

        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::TogglePopup => {
                if let Some(p) = self.popup.take() {
                    return destroy_popup(p);
                }
                self.date_today = NaiveDate::from(self.now.naive_local());
                self.date_selected = self.date_today;
                self.current_page = PopupPage::Main;

                let new_id = Id::unique();
                self.popup = Some(new_id);

                let mut popup_settings = self.core.applet.get_popup_settings(
                    self.core.main_window_id().unwrap(),
                    new_id,
                    None,
                    None,
                    None,
                );

                let Rectangle {
                    x,
                    y,
                    width,
                    height,
                } = self.rectangle;
                popup_settings.positioner.anchor_rect = Rectangle::<i32> {
                    x: x.max(1.) as i32,
                    y: y.max(1.) as i32,
                    width: width.max(1.) as i32,
                    height: height.max(1.) as i32,
                };
                popup_settings.positioner.size = None;

                return get_popup(popup_settings);
            }
            Message::Rectangle(u) => match u {
                RectangleUpdate::Rectangle(r) => {
                    self.rectangle = r.1;
                }
                RectangleUpdate::Init(tracker) => {
                    self.rectangle_tracker = Some(tracker);
                }
            },
            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
            }
            Message::Tick => {
                self.now = self.timezone.map_or_else(
                    || chrono::Local::now().into(),
                    |tz| chrono::Local::now().with_timezone(&tz).fixed_offset(),
                );
            }
            Message::TimezoneUpdate(timezone) => {
                if let Ok(tz) = timezone.parse::<chrono_tz::Tz>() {
                    self.now = chrono::Local::now().with_timezone(&tz).fixed_offset();
                    self.date_today = NaiveDate::from(self.now.naive_local());
                    self.timezone = Some(tz);
                }
            }
            Message::SelectDay(date) => {
                self.date_selected = date;
            }
            Message::PreviousMonth => {
                if let Some(date) = self
                    .date_selected
                    .checked_sub_months(chrono::Months::new(1))
                {
                    self.date_selected = date;
                }
            }
            Message::NextMonth => {
                if let Some(date) = self
                    .date_selected
                    .checked_add_months(chrono::Months::new(1))
                {
                    self.date_selected = date;
                }
            }
            Message::Navigate(page) => {
                self.current_page = page;
            }
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::UpdateTimeConfig(time_config) => {
                // Update show_seconds channel for time subscription
                let _ = self.show_seconds_tx.send(time_config.show_seconds);
                self.time_config = time_config;
            }
            Message::MeetingsUpdated(meetings) => {
                self.upcoming_meetings = meetings;
                self.has_loaded_meetings = true;
            }
            Message::CalendarsLoaded(calendars) => {
                self.available_calendars = calendars;
            }
            Message::ToggleCalendar(uid) => {
                if self.config.enabled_calendar_uids.is_empty() {
                    self.config.enabled_calendar_uids = self
                        .available_calendars
                        .iter()
                        .filter(|c| c.is_meeting_source())
                        .map(|c| c.uid.clone())
                        .collect();
                }

                if self.config.enabled_calendar_uids.contains(&uid) {
                    self.config.enabled_calendar_uids.retain(|u| u != &uid);
                } else {
                    self.config.enabled_calendar_uids.push(uid);
                }

                if let Some(ref ctx) = self.config_context {
                    let _ = self.config.write_entry(ctx);
                }

                let enabled_uids = self.enabled_meeting_source_uids();
                let upcoming_count = self.config.upcoming_events_count as usize;
                let additional_emails = self.config.additional_emails.clone();
                return Task::perform(
                    async move {
                        crate::calendar::get_upcoming_meetings(
                            &enabled_uids,
                            upcoming_count + 1,
                            &additional_emails,
                        )
                        .await
                    },
                    |meetings| Message::MeetingsUpdated(meetings).into(),
                );
            }
            Message::SetPanelShowWeekday(enabled) => {
                self.config.panel_show_weekday = enabled;
                if let Some(ref ctx) = self.config_context {
                    let _ = self.config.write_entry(ctx);
                }
            }
            Message::SetPanelShowYear(enabled) => {
                self.config.panel_show_year = enabled;
                if let Some(ref ctx) = self.config_context {
                    let _ = self.config.write_entry(ctx);
                }
            }
            Message::SetShowAllDayEvents(enabled) => {
                self.config.show_all_day_events = enabled;
                if let Some(ref ctx) = self.config_context {
                    let _ = self.config.write_entry(ctx);
                }
            }
            Message::SetEventStatusFilter(idx) => {
                self.config.event_status_filter = match idx {
                    1 => EventStatusFilter::Accepted,
                    2 => EventStatusFilter::AcceptedOrTentative,
                    _ => EventStatusFilter::All,
                };
                if let Some(ref ctx) = self.config_context {
                    let _ = self.config.write_entry(ctx);
                }
            }
            Message::SetAutoRefresh(enabled) => {
                self.config.auto_refresh_enabled = enabled;
                if let Some(ref ctx) = self.config_context {
                    let _ = self.config.write_entry(ctx);
                }
            }
            Message::RefreshCalendars => {
                self.is_refreshing = true;
                let uids = self.enabled_meeting_source_uids();
                return Task::perform(
                    async move {
                        crate::calendar::refresh_calendars(&uids).await;
                    },
                    |()| Message::RefreshCompleted.into(),
                );
            }
            Message::RefreshCompleted => {
                self.is_refreshing = false;
            }
            Message::CalendarChanged | Message::SystemResumed => {
                let enabled_uids = self.config.enabled_calendar_uids.clone();
                let upcoming_count = self.config.upcoming_events_count as usize;
                let additional_emails = self.config.additional_emails.clone();
                return Task::perform(
                    async move {
                        crate::calendar::get_upcoming_meetings(
                            &enabled_uids,
                            upcoming_count + 1,
                            &additional_emails,
                        )
                        .await
                    },
                    |meetings| Message::MeetingsUpdated(meetings).into(),
                );
            }
            Message::OpenCalendar => {
                if let Ok(output) = std::process::Command::new("xdg-mime")
                    .args(["query", "default", "text/calendar"])
                    .output()
                {
                    let desktop_file = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !desktop_file.is_empty() {
                        let gtk_result = std::process::Command::new("gtk-launch")
                            .arg(&desktop_file)
                            .spawn();

                        if gtk_result.is_err() {
                            let xdg_dirs = xdg::BaseDirectories::new();
                            if let Some(path) =
                                xdg_dirs.find_data_file(format!("applications/{desktop_file}"))
                            {
                                let path_str = path.to_string_lossy();
                                let _ = std::process::Command::new("gio")
                                    .args(["launch", path_str.as_ref()])
                                    .spawn();
                            }
                        }
                    }
                }
            }
            Message::OpenEvent(uid) => {
                open_event_in_calendar(&uid);
            }
            Message::OpenUrl(url) => {
                open_url(&url);
            }
            Message::OpenDateTimeSettings => {
                let exec = "cosmic-settings time".to_string();
                if let Some(tx) = self.token_tx.as_ref() {
                    let _ = tx.send(TokenRequest {
                        app_id: Self::APP_ID.to_string(),
                        exec,
                    });
                } else {
                    let mut cmd = std::process::Command::new("cosmic-settings");
                    cmd.arg("time");
                    tokio::spawn(cosmic::process::spawn(cmd));
                }
            }
            Message::CopyToClipboard(text) => {
                return clipboard::write(text);
            }
            Message::Token(u) => match u {
                TokenUpdate::Init(tx) => {
                    self.token_tx = Some(tx);
                }
                TokenUpdate::Finished => {
                    self.token_tx = None;
                }
                TokenUpdate::ActivationToken { token, .. } => {
                    let mut cmd = std::process::Command::new("cosmic-settings");
                    cmd.arg("time");
                    if let Some(token) = token {
                        cmd.env("XDG_ACTIVATION_TOKEN", &token);
                        cmd.env("DESKTOP_STARTUP_ID", &token);
                    }
                    tokio::spawn(cosmic::process::spawn(cmd));
                }
            },
            // Weather messages
            Message::WeatherUpdated(data) => {
                self.weather_data = data;
            }
            Message::WeatherSearchInput(query) => {
                self.weather_search_query.clone_from(&query);
                if query.len() >= 2 {
                    self.weather_is_searching = true;
                    return Task::perform(
                        async move { weather::search_cities(&query).await },
                        |results| Message::WeatherSearchResults(results).into(),
                    );
                }
                self.weather_search_results.clear();
                self.weather_is_searching = false;
            }
            Message::WeatherSearchResults(results) => {
                self.weather_search_results = results;
                self.weather_is_searching = false;
            }
            Message::SelectWeatherCity(result) => {
                let city = crate::weather::WeatherCity {
                    name: result.name,
                    latitude: result.latitude,
                    longitude: result.longitude,
                    country: result.country,
                    timezone: result.timezone,
                };
                self.config.weather_city = Some(city.clone());
                self.weather_search_query.clear();
                self.weather_search_results.clear();
                if let Some(ref ctx) = self.config_context {
                    let _ = self.config.write_entry(ctx);
                }
                return Task::perform(async move { weather::fetch_weather(&city).await }, |data| {
                    Message::WeatherUpdated(data).into()
                });
            }
            Message::ClearWeatherCity => {
                self.config.weather_city = None;
                self.weather_data = None;
                if let Some(ref ctx) = self.config_context {
                    let _ = self.config.write_entry(ctx);
                }
            }
            // World Clock messages
            Message::WorldClockSearchInput(query) => {
                self.world_clock_search_query.clone_from(&query);
                if query.len() >= 2 {
                    self.world_clock_is_searching = true;
                    return Task::perform(
                        async move { weather::search_cities(&query).await },
                        |results| Message::WorldClockSearchResults(results).into(),
                    );
                }
                self.world_clock_search_results.clear();
                self.world_clock_is_searching = false;
            }
            Message::WorldClockSearchResults(results) => {
                self.world_clock_search_results = results;
                self.world_clock_is_searching = false;
            }
            Message::SelectWorldClockCity(result) => {
                if self.config.world_clock_cities.len() < 3 {
                    self.config.world_clock_cities.push(WorldClockCity {
                        name: result.name,
                        timezone: result.timezone,
                    });
                    self.world_clock_search_query.clear();
                    self.world_clock_search_results.clear();
                    if let Some(ref ctx) = self.config_context {
                        let _ = self.config.write_entry(ctx);
                    }
                }
            }
            Message::RemoveWorldClockCity(idx) => {
                if idx < self.config.world_clock_cities.len() {
                    self.config.world_clock_cities.remove(idx);
                    if let Some(ref ctx) = self.config_context {
                        let _ = self.config.write_entry(ctx);
                    }
                }
            }
            Message::Noop => {}
        }
        Task::none()
    }
}
