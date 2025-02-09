use actix_web::{web, HttpResponse, Responder };
use serde::{Deserialize, Serialize};

// CreateNS struct represents the create_namespace POST body
#[derive(Deserialize, Serialize)]
pub struct CreateNS {
    id: i32,
    exec_cmd: String,
    //field3: bool,
}
pub async fn create_namespace(web::Json(received_body): web::Json<CreateNS>) -> impl Responder {
    println!("Hello from create_namespace()");

    let formatted_string = format!("Received ID: {}, field2: {}", received_body.id, received_body.exec_cmd);
    println!("{formatted_string}");

    HttpResponse::Created().finish()
}


