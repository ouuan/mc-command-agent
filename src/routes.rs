use crate::auth::CommandsData;
use crate::config::Config;
use actix_files::NamedFile;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

#[get("/list")]
async fn list(commands: CommandsData) -> HttpResponse {
    HttpResponse::Ok().json(commands)
}

#[derive(Deserialize)]
struct RunData {
    command: String,
}

#[post("/run")]
async fn run(
    data: web::Json<RunData>,
    commands: CommandsData,
    config: web::Data<Arc<Config>>,
) -> HttpResponse {
    if !commands.borrow().contains(&data.command) {
        return HttpResponse::Forbidden().body("You are not allowed to run this command");
    }

    let mut rcon = match config.rcon_connection().await {
        Ok(rcon) => rcon,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to connect to RCON"),
    };

    match rcon.cmd(&data.command).await {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(_) => HttpResponse::InternalServerError().body("Failed to run command"),
    }
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}
