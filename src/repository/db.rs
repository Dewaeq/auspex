use crate::{
    config::Config,
    models::{location::Location, reading::Reading, station::Station},
};
use anyhow::Result;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub struct DBRepository {
    pool: Pool<Postgres>,
}

pub struct StationRecord {
    pub id: i32,
    pub uid: String,
    pub token: String,
    pub hw_version: i32,
    pub sw_version: i32,
    pub location_id: Option<i32>,
    pub last_online: DateTime<Utc>,
}

impl DBRepository {
    pub fn new(config: Config) -> Self {
        DBRepository { pool: config.pool }
    }

    pub async fn get_station(&self, token: String) -> Result<Station> {
        let rec = sqlx::query_as!(
            StationRecord,
            r#"
            SELECT * FROM stations 
            WHERE token = $1
        "#,
            token
        )
        .fetch_one(&self.pool)
        .await?;

        let location = match rec.location_id {
            Some(id) => Some(self.get_location(id).await?),
            None => None,
        };

        let mut station = Station::from(rec);
        station.location = location;

        Ok(station)
    }

    pub async fn put_station(&self, mut station: Station) -> Result<i32> {
        if let Some(location) = station.location {
            let id = self.put_location(&location).await?;
            station.location_id = Some(id);
        }

        let rec = sqlx::query!(
            r#"
        INSERT INTO stations (uid, token, hw_version, sw_version, location_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
            station.uid,
            station.token,
            station.hw_version,
            station.sw_version,
            station.location_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn update_station(&self, station: &Station) -> Result<()> {
        sqlx::query!(
            r#"
        UPDATE stations
        SET hw_version = $1,
            sw_version = $2,
            location_id = $3,
            last_online = $4
        WHERE id = $5
        "#,
            station.hw_version,
            station.sw_version,
            station.location_id,
            station.last_online,
            station.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_location_id(&self, location_id: i32, token: String) -> Result<()> {
        sqlx::query!(
            r#"
        UPDATE stations
        SET location_id = $1
        WHERE token = $2
        "#,
            location_id,
            token,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_location(&self, location_id: i32) -> Result<Location> {
        let rec = sqlx::query_as!(
            Location,
            r#"
        SELECT * FROM locations
        WHERE id = $1
        "#,
            location_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn put_location(&self, location: &Location) -> Result<i32> {
        let rec = sqlx::query!(
            r#"
        INSERT INTO locations (station_token, latitude, longitude, country, province, city, street, number)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
            location.station_token,
            location.latitude,
            location.longitude,
            location.country,
            location.province,
            location.city,
            location.street,
            location.number
        ).fetch_one(&self.pool).await?;

        Ok(rec.id)
    }

    pub async fn get_readings_between(
        &self,
        station: Station,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Reading>> {
        let rec = sqlx::query_as!(
            Reading,
            r#"
        SELECT * FROM readings
        WHERE station_id = $1
        AND date BETWEEN $2 AND $3
        "#,
            station.id,
            start,
            end
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get_latest_reading(&self, station: Station) -> Result<Reading> {
        let rec = sqlx::query_as!(
            Reading,
            r#"
        SELECT * FROM readings
        WHERE station_id = $1
        ORDER BY date DESC
        LIMIT 1
        "#,
            station.id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn put_reading(&self, reading: Reading) -> Result<i32> {
        let rec = sqlx::query!(
            r#"
        INSERT INTO readings (station_id, location_id, date, temperature, humidity, pm10, pm25, co2, voc)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id
        "#,
            reading.station_id,
            reading.location_id,
            reading.date,
            reading.temperature,
            reading.humidity,
            reading.pm10,
            reading.pm25,
            reading.co2,
            reading.voc
        ).fetch_one(&self.pool).await?;

        Ok(rec.id)
    }
}
