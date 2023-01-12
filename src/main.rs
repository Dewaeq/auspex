use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use auspex::api::reading::{
    add_reading, get_latest_reading, get_past_hour_readings, get_past_minutes_readings,
    get_readings_between,
};
use auspex::api::station::{add_station, get_station, update_location, update_station};
use auspex::{config::Config, repository::db::DBRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = Config::new().await;

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let logger = Logger::default();
        let db_repo = DBRepository::new(config.clone());
        let db_data = Data::new(db_repo);

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(db_data)
            .service(add_station)
            .service(get_station)
            .service(update_station)
            .service(update_location)
            .service(get_latest_reading)
            .service(get_past_hour_readings)
            .service(get_past_minutes_readings)
            .service(get_readings_between)
            .service(add_reading)
    })
    .bind(("192.168.0.190", 80))?
    .run()
    .await
}
