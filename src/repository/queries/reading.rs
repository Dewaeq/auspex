use crate::{
    models::reading::{AverageReading, Reading},
    repository::query::Query,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};

pub struct PutReadingRequest {
    pub id: i32,
}

impl Query {
    pub async fn get_readings_between(
        &self,
        station_id: i32,
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
            station_id,
            start,
            end
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get_latest_reading(&self, station_id: i32) -> Result<Reading> {
        let rec = sqlx::query_as!(
            Reading,
            r#"
        SELECT * FROM readings
        WHERE station_id = $1
        ORDER BY date DESC
        LIMIT 1    
        "#,
            station_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get_average_reading(&self, station_id: i32) -> Result<AverageReading> {
        let hour_readings = self.get_past_hour_readings(1).await?;
        let day_readings = self.get_past_hour_readings(24).await?;
        let rec = AverageReading::new(hour_readings, day_readings);

        Ok(rec)
    }

    pub async fn get_past_hour_readings(&self, hours: i64) -> Result<Vec<Reading>> {
        let date = Utc::now() - Duration::hours(hours);
        let rec = sqlx::query_as!(
            Reading,
            r#"
        SELECT * FROM readings
        WHERE date >= $1
        "#,
            date
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get_past_minutes_readings(&self, minutes: i64) -> Result<Vec<Reading>> {
        let date = Utc::now() - Duration::minutes(minutes);
        let rec = sqlx::query_as!(
            Reading,
            r#"
        SELECT * FROM readings
        WHERE (date, station_id) IN (
            SELECT MAX(date), station_id FROM readings
            GROUP BY station_id
        )    
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn put_reading(&self, reading: &Reading) -> Result<PutReadingRequest> {
        let rec = sqlx::query_as!(
            PutReadingRequest,
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

        Ok(rec)
    }
}
