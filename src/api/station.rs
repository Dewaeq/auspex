use crate::{models::station::Station, repository::db::DBRepository};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
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

#[get("/station/{station_token}")]
pub async fn get_station(db: Data<DBRepository>, station_token: Path<String>) -> HttpResponse {
    let token = station_token.into_inner();
    let station = db.get_station(token).await;

    if station.is_ok() {
        HttpResponse::Ok().json(station.unwrap())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/station/{station_token}")]
pub async fn add_station(db: Data<DBRepository>, body: Json<AddStationRequest>) -> HttpResponse {
    let request = body.into_inner();
    let station = Station::from(request);
    let id = db.put_station(station).await;

    if id.is_ok() {
        HttpResponse::Ok().json(id.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
