use actix_web::{web, App, HttpServer};
mod controller {
    pub mod controller;
}

const SERVER_ADDRESS: &str = "0.0.0.0:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello from ns-manager");

    // curl -X POST http://localhost:8080/namespace/create -H "Content-Type: application/json" -d '{"id": 1,  "exec_cmd" : "Command to execute in NS"}'

    HttpServer::new(|| {
        App::new()
            .route("/namespace/create", web::post().to(controller::controller::create_namespace))
    })
        .bind(SERVER_ADDRESS)?
        .run()
        .await
}
