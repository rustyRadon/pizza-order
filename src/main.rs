//src/main.rs

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, patch, post, web::{Data, Json, Path}
};
use validator::Validate;

mod models;
use models::BuyPizzaRequest;
use models::UpdatePizzaURL;

mod db;
use db::Database;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizzas().await;
        match pizzas{
            Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
            None => HttpResponse::Ok().body("Error"),
        }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    if let Err(validation_error) = body.validate() {
        let error_message = validation_error
            .field_errors()
            .values()
            .flat_map(|errors| errors.iter())
            .filter_map(|error| error.message.as_ref())
            .map(|msg| msg.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        
        return HttpResponse::BadRequest()
            .body(format!("Validation error: {}", error_message));
    }
    
    let pizza_name = &body.pizza_name;
    HttpResponse::Ok().body(format!("Pizza ordered: {}", pizza_name))
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating pizza with ID {uuid}"))
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {

    let db = Database::init()
        .await
        .expect("rror connecting  db");
    let db_data = Data::new(db);

    println!("Server running at http://127.0.0.1:8080");
    
    HttpServer::new(move|| {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}