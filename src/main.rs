use actix_web::{get, App, HttpServer, HttpResponse, Responder};

mod payment_methods;
use crate::payment_methods::PaymentMethod;

// Get all payment methods by iterating PaymentMethod enum
// Send the response as a JSON array
#[get("/payment")]
async fn get_card_types() -> impl Responder{
        HttpResponse::Ok()
            .json(PaymentMethod::enum_to_string())
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(get_card_types)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}