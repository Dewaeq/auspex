use crate::{
    config::Config,
    models::{
        location::Location,
        reading::{AverageReading, Reading},
        station::Station,
    },
};
use anyhow::Result;
use chrono::{serde::ts_seconds, DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use super::query::Query;

pub struct DBRepository {
    pool: Pool<Postgres>,
    query: Query,
}

impl DBRepository {
    pub fn new(config: Config) -> Self {
        DBRepository {
            pool: config.pool.clone(),
            query: Query::new(config.pool),
        }
    }

    pub async fn get_station(&self, token: String, include_location: bool) -> Result<Station> {
        let rec = self.query.get_station(token).await?;

        let location = match rec.location_id {
            Some(id) if include_location => Some(self.get_location(id).await?),
            _ => None,
        };

        let mut station = Station::from(rec);
        station.location = location;

        Ok(station)
    }

    pub async fn put_station(&self, mut station: Station) -> Result<i32> {
        if let Some(location) = &station.location {
            let id = self.put_location(location).await?;
            station.location_id = Some(id);
        }

        let rec = self.query.put_station(station).await?;

        Ok(rec.id)
    }

    pub async fn update_station(&self, station: &Station) -> Result<()> {
        self.query.update_station(station).await?;

        Ok(())
    }

    pub async fn update_location_id(&self, location_id: i32, token: String) -> Result<()> {
        self.query.update_location_id(location_id, token).await?;

        Ok(())
    }

    pub async fn get_location(&self, location_id: i32) -> Result<Location> {
        let rec = self.query.get_location(location_id).await?;

        Ok(rec)
    }

    pub async fn put_location(&self, location: &Location) -> Result<i32> {
        let rec = self.query.put_location(location).await?;

        Ok(rec.id)
    }

    pub async fn get_readings_between(
        &self,
        station: Station,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Reading>> {
        let rec = self
            .query
            .get_readings_between(station.id, start, end)
            .await?;

        Ok(rec)
    }

    pub async fn get_latest_reading(&self, station: Station) -> Result<Reading> {
        let rec = self.query.get_latest_reading(station.id).await?;

        Ok(rec)
    }

    pub async fn get_latest_readings(
        &self,
        station: Station,
        count: i64,
    ) -> Result<Vec<Reading>> {
        let rec = self.query.get_latest_readings(station.id, count).await?;

        Ok(rec)
    }

    pub async fn get_average_reading(&self, station: Station) -> Result<AverageReading> {
        let rec = self.query.get_average_reading(station.id).await?;

        Ok(rec)
    }

    pub async fn get_past_hour_readings(&self, hours: impl Into<i64>) -> Result<Vec<Reading>> {
        let rec = self.query.get_all_past_hour_readings(hours.into()).await?;

        Ok(rec)
    }

    pub async fn get_past_minutes_readings(&self, minutes: impl Into<i64>) -> Result<Vec<Reading>> {
        let rec = self
            .query
            .get_all_past_minutes_readings(minutes.into())
            .await?;

        Ok(rec)
    }

    pub async fn put_reading(&self, reading: &Reading) -> Result<i32> {
        let rec = self.query.put_reading(reading).await?;

        Ok(rec.id)
    }
}
