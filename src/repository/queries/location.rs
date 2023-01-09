use crate::{models::location::Location, repository::query::Query};
use anyhow::Result;

pub struct PutLocationRecord {
    pub id: i32,
}

impl Query {
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

    pub async fn put_location(&self, location: &Location) -> Result<PutLocationRecord> {
        let rec = sqlx::query_as!(
            PutLocationRecord,
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

        Ok(rec)
    }
}
