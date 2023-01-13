use std::iter::Sum;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::reading::AddReadingRequest;

#[derive(Serialize, Deserialize)]
pub struct Reading {
    pub id: i32,
    pub station_id: i32,
    pub location_id: Option<i32>,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub temperature: f32,
    pub humidity: f32,
    pub pm10: f32,
    pub pm25: f32,
    pub co2: f32,
    pub voc: f32,
}

#[derive(Serialize, Deserialize)]
pub struct AverageReadingValues {
    pub temperature: f32,
    pub humidity: f32,
    pub pm10: f32,
    pub pm25: f32,
    pub co2: f32,
    pub voc: f32,
}

#[derive(Serialize, Deserialize)]
pub struct AverageReading {
    pub hour: AverageReadingValues,
    pub day: AverageReadingValues,
}

impl Reading {
    pub fn new(
        station_id: i32,
        location_id: Option<i32>,
        date: DateTime<Utc>,
        temperature: f32,
        humidity: f32,
        pm10: f32,
        pm25: f32,
        co2: f32,
        voc: f32,
    ) -> Self {
        Reading {
            id: 0,
            station_id,
            location_id,
            date,
            temperature,
            humidity,
            pm10,
            pm25,
            co2,
            voc,
        }
    }
}

impl From<AddReadingRequest> for Reading {
    fn from(request: AddReadingRequest) -> Self {
        Reading {
            id: 0,
            station_id: 0,
            location_id: None,
            date: request.date.unwrap_or(Utc::now()),
            temperature: request.temperature,
            humidity: request.humidity,
            pm10: request.pm10,
            pm25: request.pm25,
            co2: request.co2,
            voc: request.voc,
        }
    }
}

impl AverageReading {
    pub fn new(hour_readings: Vec<Reading>, day_readings: Vec<Reading>) -> Self {
        let mut hour = AverageReadingValues::new(hour_readings);
        let mut day = AverageReadingValues::new(day_readings);

        AverageReading { hour, day }
    }
}

impl AverageReadingValues {
    pub fn new(values: Vec<Reading>) -> Self {
        let mut result = AverageReadingValues::default();

        for val in &values {
            result.temperature += val.temperature;
            result.humidity += val.humidity;
            result.pm10 += val.pm10;
            result.pm25 += val.pm25;
            result.co2 += val.co2;
            result.voc += val.co2;
        }

        if !values.is_empty() {
            let size = values.len() as f32;
            result.temperature /= size;
            result.humidity /= size;
            result.pm10 /= size;
            result.pm25 /= size;
            result.co2 /= size;
            result.voc /= size;
        }

        result
    }
}

impl Default for AverageReadingValues {
    fn default() -> Self {
        AverageReadingValues {
            temperature: 0.0,
            humidity: 0.0,
            pm10: 0.0,
            pm25: 0.0,
            co2: 0.0,
            voc: 0.0,
        }
    }
}
