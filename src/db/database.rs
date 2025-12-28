//src/db/database.rs
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::pizza::Pizza;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    //pub name_space: String,
    //pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client.signin(Root {
            username : "root",
            password : "Hassan@pizza123"
        })
        .await?;
        client.use_ns("Surreal").use_db("pizzas").await.unwrap();
        Ok(Database {
            client,
           // name_space: String::from("Surreal"),
           // db_name: String::from("pizzas")
        })
    }
    pub async fn get_all_pizzas(&self) -> Option<Vec<Pizza>> {
        let result: Result<Vec<Pizza>, Error> = self.client.select("pizza").await;
        match result {
             Ok(all_pizzas) => Some(all_pizzas),
             Err(_) => None,
        }
    }

pub async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza> {
    let created_pizza: Result<Option<Pizza>, Error> = self.client
        .create(("pizza", new_pizza.uuid.clone()))
        .content(new_pizza)
        .await;

    match created_pizza {
        Ok(created) => created,
        Err(_) => None,
    }
}
}
 
