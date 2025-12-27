use actix_web::{get, post, patch, web, App, HttpServer, HttpResponse, Responder,};
use actix_web::web::Json;
use crate::models::pizza::BuyPizzaRequest;
mod models;
use crate::models::BuyPizzaRequest;
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("pizza available")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest> ) -> impl Responder { 
    let is_valid =  body.validate();
    match is_valid {
         Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("izza entered is {pizza_name }"))
         }
         Err(_) => HttpResponse::Ok().body("pizza name required"),
    }
}

#[patch("/updatepizza/{uuid}")]
async  fn  update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Updating a pizza")
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    }) 
    .bind("127.0.0.1:8080")?
    .run()
    .await
}