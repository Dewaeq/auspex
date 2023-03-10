use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::station::{AddStationRequest, UpdateStationRequest},
    repository::queries::station::StationRecord,
};

use super::location::Location;

#[derive(Serialize, Deserialize)]
pub struct Station {
    pub id: i32,
    pub uid: String,
    pub token: String,
    pub hw_version: i32,
    pub sw_version: i32,
    pub location_id: Option<i32>,
    pub location: Option<Location>,
    #[serde(with = "ts_milliseconds")]
    pub last_online: DateTime<Utc>,
}

impl Station {
    pub fn new(token: impl Into<String>, hw_version: i32, sw_version: i32) -> Self {
        let mut station = Station::default();
        station.token = token.into();
        station.hw_version = hw_version;
        station.sw_version = sw_version;

        station
    }

    pub fn apply_update(&mut self, update: UpdateStationRequest) {
        if let Some(hw_version) = update.hw_version {
            self.hw_version = hw_version;
        }
        if let Some(sw_version) = update.sw_version {
            self.hw_version = sw_version;
        }
        if let Some(last_online) = update.last_online {
            self.last_online = last_online;
        }
    }
}

impl Default for Station {
    fn default() -> Self {
        Self {
            id: 0,
            uid: Uuid::new_v4().to_string(),
            token: "".into(),
            hw_version: 1,
            sw_version: 1,
            location_id: None,
            location: None,
            last_online: Utc::now(),
        }
    }
}

impl From<AddStationRequest> for Station {
    fn from(request: AddStationRequest) -> Self {
        let mut station = Station::default();
        station.token = request.token.clone();
        station.hw_version = request.hw_version;
        station.sw_version = request.sw_version;

        if let Some(location) = request.location {
            station.location = Some(Location::from(location));
        }

        station
    }
}

impl From<&StationRecord> for Station {
    fn from(rec: &StationRecord) -> Self {
        Station {
            id: rec.id,
            uid: rec.uid.clone(),
            token: rec.token.clone(),
            hw_version: rec.hw_version,
            sw_version: rec.sw_version,
            location_id: rec.location_id,
            location: None,
            last_online: rec.last_online,
        }
    }
}
