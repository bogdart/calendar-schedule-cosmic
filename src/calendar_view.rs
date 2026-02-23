// SPDX-License-Identifier: GPL-3.0-only
//
// Calendar grid view adapted from cosmic-applet-time

use chrono::{Datelike, Days, NaiveDate, Weekday};
use cosmic::Apply;
use cosmic::iced::Length;
use cosmic::widget::{self, Grid, button, container, grid, text};
use icu::datetime::{
    DateTimeFormatter, DateTimeFormatterPreferences, fieldsets,
    input::{Date, DateTime, Time},
};
use icu::locale::Locale;
use std::collections::HashSet;

use crate::app::Message;

/// Cell size for the calendar grid (width and height of each day button)
const CELL_SIZE: f32 = 36.0;

/// Gets the first date that will be visible on the calendar grid.
/// This is the date in the top-left corner, which may be from the previous month.
pub fn get_calendar_first(year: i32, month: u32, from_weekday: Weekday) -> NaiveDate {
    let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let num_days = (date.weekday() as u32 + 7 - from_weekday as u32) % 7;
    date.checked_sub_days(Days::new(num_days as u64)).unwrap()
}

/// Create an ICU DateTime from a chrono date and time components
pub fn create_datetime<D: Datelike>(
    date: &D,
    hour: u8,
    minute: u8,
    second: u8,
) -> DateTime<icu::calendar::Gregorian> {
    DateTime {
        date: Date::try_new_gregorian(date.year(), date.month() as u8, date.day() as u8).unwrap(),
        time: Time::try_new(hour, minute, second, 0).unwrap(),
    }
}

/// Build the calendar grid widget
pub fn calendar_grid(
    date_selected: NaiveDate,
    date_today: NaiveDate,
    first_day_of_week: Weekday,
    locale: &Locale,
    days_with_events: &HashSet<NaiveDate>,
) -> Grid<'static, Message> {
    let mut calendar: Grid<'_, Message> = grid().width(Length::Fill);

    let first_day = get_calendar_first(
        date_selected.year(),
        date_selected.month(),
        first_day_of_week,
    );

    let day_iter = first_day.iter_days();
    let prefs = DateTimeFormatterPreferences::from(locale.clone());
    let weekday_fmt = DateTimeFormatter::try_new(prefs, fieldsets::E::short()).unwrap();

    // Weekday headers (Mon, Tue, etc.)
    let mut current_weekday = first_day_of_week;
    for date in day_iter.take(7) {
        let datetime = create_datetime(&date, 12, 0, 0);
        calendar = calendar.push(
            text::caption(weekday_fmt.format(&datetime).to_string())
                .apply(container)
                .center_x(Length::Fixed(CELL_SIZE)),
        );
        current_weekday = current_weekday.succ();
    }
    calendar = calendar.insert_row();

    // Day buttons (6 weeks x 7 days = 42 cells)
    let mut day_iter = first_day.iter_days();
    for i in 0..42 {
        if i > 0 && i % 7 == 0 {
            calendar = calendar.insert_row();
        }

        let date = day_iter.next().unwrap();
        let is_month =
            date.month() == date_selected.month() && date.year_ce() == date_selected.year_ce();
        let is_day = date.day() == date_selected.day() && is_month;
        let is_today = date == date_today;
        let has_events = days_with_events.contains(&date);

        calendar = calendar.push(date_button(date, is_month, is_day, is_today, has_events));
    }

    calendar
}

/// Create a date button for the calendar grid
fn date_button(
    date: NaiveDate,
    is_month: bool,
    is_selected: bool,
    is_today: bool,
    has_events: bool,
) -> widget::Button<'static, Message> {
    let style = if is_selected {
        button::ButtonClass::Suggested
    } else if is_today {
        button::ButtonClass::Standard
    } else {
        button::ButtonClass::Text
    };

    let day_text = text::body(format!("{}", date.day()));

    // Use consistent layout for all dates - text centered with space for dot below
    let dot_element: cosmic::Element<'_, Message> = if has_events && !is_selected {
        container(widget::Space::new(0, 0))
            .width(Length::Fixed(4.0))
            .height(Length::Fixed(4.0))
            .class(cosmic::theme::Container::custom(|theme| {
                cosmic::iced_widget::container::Style {
                    background: Some(cosmic::iced::Background::Color(
                        theme.cosmic().accent_color().into(),
                    )),
                    border: cosmic::iced::Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }))
            .into()
    } else {
        // Invisible spacer with same size as dot
        widget::Space::new(4, 4).into()
    };

    let content: cosmic::Element<'_, Message> = widget::column()
        .push(
            day_text
                .apply(container)
                .center_x(Length::Fill)
                .height(Length::Fixed(20.0)),
        )
        .push(
            container(dot_element)
                .center_x(Length::Fill)
                .height(Length::Fixed(6.0)),
        )
        .align_x(cosmic::iced::Alignment::Center)
        .into();

    let button = button::custom(content)
        .class(style)
        .height(Length::Fixed(CELL_SIZE))
        .width(Length::Fixed(CELL_SIZE));

    if is_month {
        button.on_press(Message::SelectDay(date))
    } else {
        button
    }
}
