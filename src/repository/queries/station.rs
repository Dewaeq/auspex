use crate::{models::station::Station, repository::query::Query};
use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct StationRecord {
    pub id: i32,
    pub uid: String,
    pub token: String,
    pub hw_version: i32,
    pub sw_version: i32,
    pub location_id: Option<i32>,
    pub last_online: DateTime<Utc>,
}

pub struct PutStationRecord {
    pub id: i32,
}

impl Query {
    pub async fn get_station(&self, token: String) -> Result<StationRecord> {
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

        Ok(rec)
    }

    pub async fn get_active_stations(&self) -> Result<Vec<StationRecord>> {
        let rec = sqlx::query_as!(
            StationRecord,
            r#"
        SELECT * FROM stations
        WHERE last_online >= NOW() - INTERVAL '1 HOUR'
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn put_station(&self, station: Station) -> Result<PutStationRecord> {
        let rec = sqlx::query_as!(
            PutStationRecord,
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

        Ok(rec)
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
}
