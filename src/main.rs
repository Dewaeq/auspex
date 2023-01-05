use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use auspex::api::{station::add_station, station::get_station};
use auspex::{config::Config, repository::db::DBRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = Config::new().await;

    HttpServer::new(move || {
        let logger = Logger::default();
        let db_repo = DBRepository::new(config.clone());
        let db_data = Data::new(db_repo);

        App::new()
            .wrap(logger)
            .app_data(db_data)
            .service(add_station)
            .service(get_station)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
