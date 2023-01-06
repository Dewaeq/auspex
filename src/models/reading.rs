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
