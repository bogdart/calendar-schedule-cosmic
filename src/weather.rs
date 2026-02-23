// SPDX-License-Identifier: GPL-3.0-only
//
// Open-Meteo weather API client and types.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A city for weather display, with geocoordinates.
/// `Eq` is manually implemented because `f64` doesn't derive `Eq`,
/// but coordinate equality is fine for config serialization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherCity {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: String,
    pub timezone: String,
}

impl Eq for WeatherCity {}

/// Current weather conditions.
#[derive(Debug, Clone)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub weather_code: u8,
}

/// Daily forecast for a single day.
#[derive(Debug, Clone)]
pub struct DailyForecast {
    pub date: NaiveDate,
    pub weather_code: u8,
    pub temp_max: f64,
}

/// Combined weather data (current + daily forecast).
#[derive(Debug, Clone)]
pub struct WeatherData {
    pub current: CurrentWeather,
    pub daily: Vec<DailyForecast>,
}

/// Result from the Open-Meteo geocoding API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeocodingResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub admin1: Option<String>,
    #[serde(default)]
    pub timezone: String,
}

/// Fetch weather data from the Open-Meteo forecast API.
pub async fn fetch_weather(city: &WeatherCity) -> Option<WeatherData> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
         ?latitude={}&longitude={}\
         &current=temperature_2m,weather_code\
         &daily=weather_code,temperature_2m_max,temperature_2m_min\
         &timezone=auto\
         &forecast_days=7",
        city.latitude, city.longitude,
    );

    let resp = reqwest::get(&url).await.ok()?;
    let json: serde_json::Value = resp.json().await.ok()?;

    let current_obj = json.get("current")?;
    #[allow(clippy::cast_possible_truncation)]
    let current = CurrentWeather {
        temperature: current_obj.get("temperature_2m")?.as_f64()?,
        weather_code: current_obj.get("weather_code")?.as_u64()? as u8,
    };

    let daily_obj = json.get("daily")?;
    let dates = daily_obj.get("time")?.as_array()?;
    let codes = daily_obj.get("weather_code")?.as_array()?;
    let maxes = daily_obj.get("temperature_2m_max")?.as_array()?;
    let mut daily = Vec::new();
    for i in 0..dates.len() {
        let date_str = dates[i].as_str()?;
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
        #[allow(clippy::cast_possible_truncation)]
        daily.push(DailyForecast {
            date,
            weather_code: codes[i].as_u64()? as u8,
            temp_max: maxes[i].as_f64()?,
        });
    }

    Some(WeatherData { current, daily })
}

/// Search cities via the Open-Meteo geocoding API.
pub async fn search_cities(query: &str) -> Vec<GeocodingResult> {
    let url =
        format!("https://geocoding-api.open-meteo.com/v1/search?name={query}&count=5&language=en",);

    let Ok(resp) = reqwest::get(&url).await else {
        return Vec::new();
    };
    let Ok(json) = resp.json::<serde_json::Value>().await else {
        return Vec::new();
    };

    let Some(results) = json.get("results").and_then(|r| r.as_array()) else {
        return Vec::new();
    };

    results
        .iter()
        .filter_map(|v| serde_json::from_value::<GeocodingResult>(v.clone()).ok())
        .collect()
}

/// Map a WMO weather code to a freedesktop icon name.
pub fn weather_icon_name(code: u8) -> &'static str {
    match code {
        0 => "weather-clear-symbolic",
        1..=2 => "weather-few-clouds-symbolic",
        3 => "weather-overcast-symbolic",
        45 | 48 => "weather-fog-symbolic",
        51..=67 => "weather-showers-symbolic",
        71..=77 | 85 | 86 => "weather-snow-symbolic",
        80..=82 => "weather-showers-scattered-symbolic",
        95..=99 => "weather-storm-symbolic",
        _ => "weather-severe-alert-symbolic",
    }
}
