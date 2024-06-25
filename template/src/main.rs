use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Hello World !???")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let port = 8081;
    println!("Starting server on port {port}");

    HttpServer::new(|| App::new().service(greet))
        .bind("0.0.0.0:8081")?
        .workers(2)
        .run()
        .await


}

