extern crate diesel;
extern crate web_server;

use self::diesel::prelude::*;
use self::models::*;
use self::web_server::*;
use actix_web::{error, post, get, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::StreamExt;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World Again!")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/api/plan")]
async fn plan() -> impl Responder {
    use web_server::schema::plans::dsl::*;

    let connection = establish_connection();
    let results = plans
        .load::<Plan>(&connection)
        .expect("Error loading plans");

    let xs = serde_json::to_string(&results).unwrap();

    HttpResponse::Ok().body(xs)
}

#[post("/api/login")]
async fn login(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<User>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            .service(plan)
            .service(login)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
