//src/models/pizza.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "pizza name required"))]
    pub pizza_name: String
}