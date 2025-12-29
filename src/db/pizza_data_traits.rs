//src/db/pizza_data_traits.rs
use crate::db::Database;
use crate::models::pizza::Pizza;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait PizzaDataTrait {
    async fn get_all_pizzas(&self) -> Option<Vec<Pizza>>;
    async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza>;
    async fn update_pizza(&self, uuid: String) -> Option<Pizza>;
}

#[async_trait]
impl PizzaDataTrait for Database {
    async fn get_all_pizzas(&self) -> Option<Vec<Pizza>> {
        let result = self.client.select("pizza").await;
        match result {
            Ok(all_pizzas) => Some(all_pizzas),
            Err(_) => None,
        }
    }

    async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza> {
        let created_pizza = self.client
            .create(("pizza", new_pizza.uuid.clone()))
            .content(new_pizza)
            .await;

        match created_pizza {
            Ok(created) => created,
            Err(_) => None,
        }
    }

    async fn update_pizza(&self, uuid: String) -> Option<Pizza> {
        // First check if pizza exists
        let find_pizza: Result<Option<Pizza>, Error> = 
            self.client.select(("pizza", &uuid)).await;

        match find_pizza { 
            Ok(found) => {
                match found {
                    Some(_found_pizza) => {
                        // Update the pizza
                        let updated_pizza: Result<Option<Pizza>, Error> = self
                            .client
                            .update(("pizza", &uuid))
                            .merge(Pizza {
                                uuid: uuid.clone(),
                                pizza_name: String::from("sold"),
                            })
                            .await;

                        match updated_pizza {
                            Ok(updated) => updated,
                            Err(_) => None,
                        }
                    }
                    None => None,
                }
            }
            Err(_) => None,
        }
    }
}