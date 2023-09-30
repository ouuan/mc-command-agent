mod auth;
mod config;
mod routes;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use actix_web_httpauth::{extractors::basic, middleware::HttpAuthentication};
use anyhow::Result;
use auth::CommandsData;
use config::Config;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info"))
        .format_timestamp_micros()
        .init();

    let config = Arc::new(Config::read()?);
    let server_host = config.server_host();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(HttpAuthentication::basic(auth::auth))
            .app_data(basic::Config::default().realm("mc-command-agent"))
            .app_data(CommandsData::default())
            .app_data(Data::new(config.clone()))
            .service(routes::index)
            .service(routes::list)
            .service(routes::run)
    })
    .bind(server_host)?
    .run()
    .await?;

    Ok(())
}
