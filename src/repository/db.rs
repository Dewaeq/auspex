use crate::{
    config::Config,
    models::{location::Location, station::Station},
};
use anyhow::Result;
use sqlx::{Pool, Postgres};

pub struct DBRepository {
    pool: Pool<Postgres>,
}

pub struct DBError;

impl DBRepository {
    pub fn new(config: Config) -> Self {
        DBRepository { pool: config.pool }
    }

    pub async fn get_station(&self, token: String) -> Result<Station> {
        let rec = sqlx::query!(
            r#"
            SELECT * FROM stations 
            WHERE token = $1
        "#,
            token
        )
        .fetch_one(&self.pool)
        .await?;

        let location_id = rec.location_id;
        let location = match location_id {
            Some(id) => Some(self.get_location(id).await?),
            None => None,
        };

        let station = Station {
            id: rec.id,
            uid: rec.uid,
            token: rec.token,
            hw_version: rec.hw_version,
            sw_version: rec.sw_version,
            last_online: rec.last_online,
            location,
            location_id,
        };

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
        let rec = sqlx::query!(
            r#"
        SELECT * FROM locations
        WHERE id = $1
        "#,
            location_id
        )
        .fetch_one(&self.pool)
        .await?;

        let location = Location {
            id: rec.id,
            station_token: rec.station_token,
            latitude: rec.latitude,
            longitude: rec.longitude,
            country: rec.country,
            province: rec.province,
            city: rec.city,
            street: rec.street,
            number: rec.number,
        };

        Ok(location)
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
}
