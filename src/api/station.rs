use crate::{
    repository::db::DBRepository,
    services::station_service::StationService, models::station::Station,
};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[get("/station/{station_token}")]
pub async fn get_station(db: Data<DBRepository>, station_token: Path<String>) -> HttpResponse {
    let service = StationService::new(db);
    let token = station_token.into_inner();
    let station = service.get_station(token).await;

    if station.is_ok() {
        HttpResponse::Ok().json(station.unwrap())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/station/all/active")]
pub async fn get_active_stations(db: Data<DBRepository>) -> HttpResponse {
    let service = StationService::new(db);
    let result = service.get_active_stations().await;

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::NoContent().finish()
    }
}

#[put("/station/{station_token}/register")]
pub async fn add_station(db: Data<DBRepository>, body: Json<AddStationRequest>) -> HttpResponse {
    let service = StationService::new(db);
    let request = body.into_inner();
    let station = Station::from(request);
    let id = service.put_station(station).await;

    if id.is_ok() {
        HttpResponse::Ok().json(id.unwrap())
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
    let service = StationService::new(db);
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
    let service = StationService::new(db);
    let request = body.into_inner();
    let result = service.update_location(request).await;

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
