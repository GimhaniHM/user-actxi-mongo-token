

use actix_web::{web, web::Data, App, HttpServer, get};

//route handler function
#[get("/")]
async fn index() -> String {
    "This is Health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server started successfully");

    let data = "hi";

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
