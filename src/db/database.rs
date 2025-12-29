//src/db/database.rs
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::pizza::Pizza;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client.signin(Root {
            username: "root",
            password: "root"
        })
        .await?;
        client.use_ns("Surreal").use_db("pizzas").await?;
        Ok(Database {
            client,
        })
    }
    
    pub async fn get_all_pizzas(&self) -> Option<Vec<Pizza>> {
        let result = self.client.select("pizza").await;
        match result {
            Ok(all_pizzas) => Some(all_pizzas),
            Err(_) => None,
        }
    }

    pub async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza> {
        let created_pizza = self.client
            .create(("pizza", new_pizza.uuid.clone()))
            .content(new_pizza)
            .await;

        match created_pizza {
            Ok(created) => created,
            Err(_) => None,
        }
    }

    pub async fn update_pizza(&self, uuid: String) -> Option<Pizza> {
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