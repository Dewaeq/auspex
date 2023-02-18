use crate::{
    api::station::{AddLocationRequest, UpdateStationRequest},
    models::{location::Location, station::Station},
    repository::db::DBRepository,
};
use actix_web::web::Data;
use anyhow::Result;

pub struct StationService<'a> {
    db: &'a Data<DBRepository>,
}

impl<'a> StationService<'a> {
    pub fn new(db: &'a Data<DBRepository>) -> Self {
        StationService { db }
    }

    pub async fn get_station(&self, token: String) -> Result<Station> {
        self.db.get_station(token, true).await
    }

    pub async fn get_active_stations(&self) -> Result<Vec<Station>> {
        self.db.get_active_stations().await
    }

    pub async fn put_station(&self, station: Station) -> Result<i32> {
        self.db.put_station(station).await
    }

    pub async fn update_station(
        &self,
        token: String,
        request: UpdateStationRequest,
    ) -> Result<()> {
        let mut station = self.db.get_station(token, false).await?;
        station.apply_update(request);

        self.db.update_station(&station).await
    }

    pub async fn update_location(&self, request: AddLocationRequest) -> Result<()> {
        let location = Location::from(request);
        let location_id = self.db.put_location(&location).await?;

        self.db
            .update_location_id(location_id, location.station_token)
            .await
    }
}
