use actix_web::web::Data;
use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    api::reading::AddReadingRequest,
    models::reading::{AverageReading, Reading},
    repository::db::DBRepository,
};

pub struct ReadingService {
    db: Data<DBRepository>,
}

impl ReadingService {
    pub fn new(db: Data<DBRepository>) -> Self {
        ReadingService { db }
    }

    pub async fn get_readings_between(
        &self,
        token: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Reading>> {
        let station = self.db.get_station(token, false).await?;
        self.db.get_readings_between(station, start, end).await
    }

    pub async fn get_latest_reading(&self, token: String) -> Result<Reading> {
        let station = self.db.get_station(token, false).await?;
        self.db.get_latest_reading(station).await
    }

    pub async fn get_latest_readings(&self, token: String, count: i64) -> Result<Vec<Reading>> {
        let station = self.db.get_station(token, false).await?;
        self.db.get_latest_readings(station, count).await
    }

    pub async fn get_average_reading(&self, token: String) -> Result<AverageReading> {
        let station = self.db.get_station(token, false).await?;
        self.db.get_average_reading(station).await
    }

    pub async fn get_past_hour_readings(&self) -> Result<Vec<Reading>> {
        self.db.get_past_hour_readings(1).await
    }

    /// Get every reading from the past 5 minutes, across all stations
    pub async fn get_past_minute_readings(&self) -> Result<Vec<Reading>> {
        self.db.get_past_minutes_readings(5).await
    }

    pub async fn put_reading(&self, request: AddReadingRequest) -> Result<i32> {
        let mut station = self
            .db
            .get_station(request.station_token.clone(), false)
            .await?;
        station.last_online = Utc::now();
        self.db.update_station(&station).await?;

        let mut reading = Reading::from(request);
        reading.station_id = station.id;
        reading.location_id = station.location_id;

        self.db.put_reading(&reading).await
    }
}
