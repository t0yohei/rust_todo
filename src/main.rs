# [macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate actix_web;
use actix_web::{server, App, http, Path, Responder};
use actix_web::fs::StaticFiles;
extern crate askama;
use askama::Template;
extern crate diesel;

// mod schema;

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate;

fn index(_: Path<()>) -> impl Responder {
    IndexTemplate
}

fn ping(_: Path<()>) -> impl Responder {
    "pong"
}

fn main() {
    env_logger::init();

    debug!("Lunching app...");

    server::new(||
         App::new()
         .handler("/static",
            StaticFiles::new("./static").unwrap())
         .route("/", http::Method::GET, index)
         .route("/ping", http::Method::GET, ping)).bind("[::]:8080").unwrap().run();

}
