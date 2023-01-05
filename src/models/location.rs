use serde::{Deserialize, Serialize};

use crate::api::station::AddLocationRequest;

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub id: i32,
    pub station_token: String,
    pub latitude: f32,
    pub longitude: f32,
    pub country: String,
    pub province: String,
    pub city: String,
    pub street: String,
    pub number: String,
}

impl Location {
    pub fn new(
        station_token: impl Into<String>,
        latitude: impl Into<f32>,
        longitude: impl Into<f32>,
        country: impl Into<String>,
        province: impl Into<String>,
        city: impl Into<String>,
        street: impl Into<String>,
        number: impl Into<String>,
    ) -> Self {
        Location {
            id: 0,
            station_token: station_token.into(),
            latitude: latitude.into(),
            longitude: longitude.into(),
            country: country.into(),
            province: province.into(),
            city: city.into(),
            street: street.into(),
            number: number.into(),
        }
    }
}

impl From<AddLocationRequest> for Location {
    fn from(request: AddLocationRequest) -> Self {
        Location::new(
            request.station_token,
            request.latitude,
            request.longitude,
            request.country,
            request.province,
            request.city,
            request.street,
            request.number,
        )
    }
}
