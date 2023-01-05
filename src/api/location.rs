use crate::{
    models::{location::Location, station::Station},
    repository::db::DBRepository,
};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/location/{station_token}")]
pub async fn get_station(db: Data<DBRepository>, location_id: Path<i32>) -> HttpResponse {
    let id = location_id.into_inner();
    let location = db.get_location(id).await;

    if location.is_ok() {
        HttpResponse::Ok().json(location.unwrap())
    } else {
        HttpResponse::NotFound().finish()
    }
}

/* #[post("/station/{station_token}")]
pub async fn add_station(db: Data<DBRepository>, location: Path<String>) -> HttpResponse {
    let token = location.into_inner();
    let location = Location::new(0.0, 0.0, "BE", "OV", "GE", "DP", "45");

    let id = db.put_location(location).await;

    if id.is_ok() {
        HttpResponse::Ok().json(id.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
} */
