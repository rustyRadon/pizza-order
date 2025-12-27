use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "pizza name is required"))]
    pub pizza_name: String,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String,
}

#[allow(dead_code)]
impl Pizza {
    pub fn new(uuid: String, pizza_name: String) -> Pizza {
        Pizza { uuid, pizza_name }
    }
}