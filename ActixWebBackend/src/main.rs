use std::sync::Mutex;
mod bubble;
use bubble::bubble_sort;
use std::time::Instant;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

const PORT: u16 = 4000;
struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/bubble")]
async fn bubble_it() -> String {
    let before = Instant::now();

    for _i in 0..1 {
        bubble_sort();
    }
    let time = format!("{:.2?} ms", before.elapsed().as_millis());
    println!("{time}");
    time
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    println!("{counter}");
    format!("{app_name} Request number: {counter}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let global_data = web::Data::new(AppState {
        app_name: String::from("ActixWeb"),
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(global_data.clone())
            .service(hello)
            .service(echo)
            .service(bubble_it)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
