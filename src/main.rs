use std::collections::BTreeMap;
use env_logger::Env;
use actix_web::{App, HttpServer, HttpResponse, Responder, get};


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(BTreeMap::from([
            ("name", "API"),
            ("status", "Online"),
            ("version", "0.1.0"),
        ]))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Starting HTTP server on port 3000");
    log::info!("Running http://localhost:3000");

    HttpServer::new(|| App::new().service(index))
        .bind("localhost:3000")?
        .run()
        .await
}
