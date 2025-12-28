//src/main.rs

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, patch, post, web::{Data, Json, Path}
};
use uuid::Uuid;
use validator::Validate;

mod models;
mod db;
use db::Database;
use uuid;

use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaURL, Pizza};

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizzas().await;
        match pizzas{
            Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
            None => HttpResponse::Ok().body("Error"),
        }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db:Data<Database> ) -> impl Responder {
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
    let mut buffer = Uuid::encode_buffer();
    let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);
    let new_pizza = db.add_pizza(Pizza::new(
        String::from(new_uuid), 
        pizza_name.to_string()
    )).await;

    match new_pizza{
        Some(created) => {
            HttpResponse::Ok().body(format!( "created new pizza: {:?}", created))
        },
        None =>HttpResponse::Ok().body("error buying pizza"),
    }

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