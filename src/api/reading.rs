use actix_web::{
    get, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use chrono::{serde::ts_milliseconds, serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{repository::db::DBRepository, services::reading_service::ReadingService};

#[derive(Serialize, Deserialize)]
pub struct AddReadingRequest {
    pub station_token: String,
    #[serde(with = "ts_milliseconds_option")]
    pub date: Option<DateTime<Utc>>,
    pub temperature: f32,
    pub humidity: f32,
    pub pm10: f32,
    pub pm25: f32,
    pub co2: f32,
    pub voc: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ReadingsBetweenRequest {
    station_token: String,
    #[serde(with = "ts_milliseconds")]
    start: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    end: DateTime<Utc>,
}

#[get("/reading/{station_token}/latest")]
pub async fn get_latest_reading(
    db: Data<DBRepository>,
    station_token: Path<String>,
) -> HttpResponse {
    let service = ReadingService::new(db);
    let token = station_token.into_inner();
    let result = service.get_latest_reading(token).await;

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::NoContent().finish()
    }
}

#[get("/reading/{station_token}/latest/{count}")]
pub async fn get_latest_readings(db: Data<DBRepository>, params: Path<(String, i64)>) -> HttpResponse {
    let service = ReadingService::new(db);
    let (token, count) = params.into_inner();
    let result = service.get_latest_readings(token, count).await;

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::NoContent().finish()
    }
}

#[get("/reading/{station_token}/average")]
pub async fn get_average_reading(
    db: Data<DBRepository>,
    station_token: Path<String>,
) -> HttpResponse {
    let service = ReadingService::new(db);
    let token = station_token.into_inner();
    let result = service.get_average_reading(token).await;

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/reading/{station_token}/between/{start}/{end}")]
pub async fn get_readings_between(
    db: Data<DBRepository>,
    path: Path<ReadingsBetweenRequest>,
) -> HttpResponse {
    let service = ReadingService::new(db);
    let request = path.into_inner();
    let result = service
        .get_readings_between(request.station_token, request.start, request.end)
        .await;

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::NoContent().finish()
    }
}

#[get("/reading/all/past_hour")]
pub async fn get_past_hour_readings(db: Data<DBRepository>) -> HttpResponse {
    let service = ReadingService::new(db);
    let result = service.get_past_hour_readings().await;

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::NoContent().finish()
    }
}

#[get("/reading/all/past_minutes")]
pub async fn get_past_minutes_readings(db: Data<DBRepository>) -> HttpResponse {
    let service = ReadingService::new(db);
    let result = service.get_past_minute_readings().await;

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[put("/reading/{station_token}/new")]
pub async fn add_reading(db: Data<DBRepository>, body: Json<AddReadingRequest>) -> HttpResponse {
    let service = ReadingService::new(db);
    let request = body.into_inner();
    let id = service.put_reading(request).await;

    if id.is_ok() {
        HttpResponse::Ok().json(id.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
