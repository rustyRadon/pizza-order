use actix_web::{
    http:: { header::ContentType, StatusCode},
    HttpResponse, ResponseError
};
 

 use derive_more::Display;
 #[derive(Debug, Display)]
 #[allow(dead_code)]
pub enum  PizzaError {
    NoPizzaFound,
    PizzaCreationFAiled,
    NoSuchPizza, 
} 

impl  ResponseError for PizzaError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match self {
            PizzaError::NoPizzaFound => StatusCode::NOT_FOUND,
            PizzaError::PizzaCreationFAiled => StatusCode::INTERNAL_SERVER_ERROR,
            PizzaError::NoSuchPizza => StatusCode::NOT_FOUND,
        }
    }
}