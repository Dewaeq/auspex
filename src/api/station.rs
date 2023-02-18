use crate::{
    models::reading::Reading, models::station::Station, repository::db::DBRepository,
    services::reading_service::ReadingService, services::station_service::StationService,
};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetStationResponse {
    pub station: Station,
    pub last_reading: Option<Reading>,
}

#[derive(Serialize, Deserialize)]
pub struct AddStationRequest {
    pub token: String,
    pub hw_version: i32,
    pub sw_version: i32,
    pub location: Option<AddLocationRequest>,
}

#[derive(Serialize, Deserialize)]
pub struct AddLocationRequest {
    pub latitude: f32,
    pub longitude: f32,
    pub country: String,
    pub province: String,
    pub city: String,
    pub street: String,
    pub number: String,
    pub station_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStationRequest {
    pub hw_version: Option<i32>,
    pub sw_version: Option<i32>,
    pub last_online: Option<DateTime<Utc>>,
}

async fn create_station_response(db: &Data<DBRepository>, station: Station) -> GetStationResponse {
    let service = ReadingService::new(&db);
    let last_reading = service.get_latest_reading(station.token.clone()).await.ok();

    GetStationResponse {
        station,
        last_reading,
    }
}

async fn create_station_responses(
    db: Data<DBRepository>,
    stations: Vec<Station>,
) -> Vec<GetStationResponse> {
    let service = ReadingService::new(&db);
    let mut result = Vec::with_capacity(stations.len());

    for station in stations.into_iter() {
        let res = create_station_response(&db, station).await;
        result.push(res)
    }

    result
}

#[get("/station/{station_token}")]
pub async fn get_station(db: Data<DBRepository>, station_token: Path<String>) -> HttpResponse {
    let service = StationService::new(&db);
    let token = station_token.into_inner();
    let station = service.get_station(token).await;

    if let Ok(station) = station {
        let res = create_station_response(&db, station).await;
        HttpResponse::Ok().json(res)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/station/all/active")]
pub async fn get_active_stations(db: Data<DBRepository>) -> HttpResponse {
    let service = StationService::new(&db);
    let result = service.get_active_stations().await;

    if let Ok(stations) = result {
        let res = create_station_responses(db, stations).await;
        HttpResponse::Ok().json(res)
    } else {
        HttpResponse::NoContent().finish()
    }
}

#[put("/station/{station_token}/register")]
pub async fn add_station(db: Data<DBRepository>, body: Json<AddStationRequest>) -> HttpResponse {
    let service = StationService::new(&db);
    let request = body.into_inner();
    let station = Station::from(request);
    let id = service.put_station(station).await;

    if let Ok(id) = id {
        HttpResponse::Ok().json(id)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/station/{station_token}/update")]
pub async fn update_station(
    db: Data<DBRepository>,
    station_token: Path<String>,
    body: Json<UpdateStationRequest>,
) -> HttpResponse {
    let service = StationService::new(&db);
    let token = station_token.into_inner();
    let request = body.into_inner();
    let result = service.update_station(token, request).await;

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/station/{station_token}/location/update")]
pub async fn update_location(
    db: Data<DBRepository>,
    body: Json<AddLocationRequest>,
) -> HttpResponse {
    let service = StationService::new(&db);
    let request = body.into_inner();
    let result = service.update_location(request).await;

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
