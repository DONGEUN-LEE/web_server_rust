extern crate diesel;
extern crate web_server;

use self::diesel::prelude::*;
use self::models::*;
use self::web_server::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

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

#[get("/plan")]
async fn plan() -> impl Responder {
    use web_server::schema::plans::dsl::*;

    let connection = establish_connection();
    let results = plans
        .load::<Plan>(&connection)
        .expect("Error loading plans");

    let xs = serde_json::to_string(&results).unwrap();

    HttpResponse::Ok().body(xs)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            .service(plan)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
