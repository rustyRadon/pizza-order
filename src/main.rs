//src/main.rs
use actix_web::{
    App, HttpResponse, HttpServer, get, patch, post, web::{Data, Json, Path}
};
use uuid::Uuid;
use validator::Validate;

mod error;
mod models;
mod db;

use db::Database;
use db::PizzaDataTrait;
use crate::error::pizza_error::PizzaError;
use crate::models::pizza::{BuyPizzaRequest, Pizza, UpdatePizzaURL};

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = Database::get_all_pizzas(&db).await;
    match pizzas {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzaFound),
    }
} 

#[post("/buypizza")] 
async fn buy_pizza(
    body: Json<BuyPizzaRequest>,
    db: Data<Database>,
) -> HttpResponse {
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

    let new_pizza = Database::add_pizza(&db, Pizza::new(
            String::from(new_uuid),
            pizza_name.to_string(),
        ))
        .await;

    match new_pizza {
        Some(created) => HttpResponse::Ok().json(created), 
        None => HttpResponse::InternalServerError()
            .body("Pizza creation failed"),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(
    update_pizza_url: Path<UpdatePizzaURL>,
    db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let update_result = Database::update_pizza(&db, uuid).await;
    match update_result {
        Some(updated_pizza) => Ok(Json(updated_pizza)),
        None => Err(PizzaError::NoSuchPizza),
    }
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Error connecting to database");
    let db_data = Data::new(db);

    println!("Server running at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
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