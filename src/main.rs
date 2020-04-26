#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, Result, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct MyResponse {
    id: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
struct MyRequest {
    url: String
}

impl MyRequest {
    fn to_response(&self) -> MyResponse {
        MyResponse {
            id: self.url.to_string(),
            url: self.url.to_string(),
        }
    }
}

/// handler with path parameters like `/api/v1/encode`
#[post("/api/v1/encode")]
async fn welcome(_req: HttpRequest, json: web::Json<MyRequest>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(json.to_response()))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register simple route, handle all methods
            .service(welcome)
    }).bind("localhost:9090")?
        .run()
        .await
}
