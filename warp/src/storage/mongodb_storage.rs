use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::Customer;
use crate::storage::Storage;

use mongodb::{
    sync::{Client, Collection},
};
use mongodb::bson::doc;

pub struct MongoDbStorage {
    customers: Collection<Customer>,
}

impl Storage for MongoDbStorage {
    fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("warp-db");
        let col: Collection<Customer> = db.collection("customers");

        MongoDbStorage { customers: col }
    }

    fn list(&self) -> Option<Vec<Customer>> {
        let cursor = self
            .customers
            .find(None, None)
            .ok()
            .expect("Error getting list of customers");
        let customers: Vec<Customer> = cursor.map(|customer| customer.unwrap()).collect();
        if customers.is_empty() {
            None
        } else {
            Some(customers)
        }
    }

    fn get(&self, guid: String) -> Option<Customer> {
        let filter =  doc! {"guid": guid};
        self.customers.find_one(filter,None).ok().unwrap()
    }

    fn create(&mut self, customer: Customer)  -> bool {
        self.customers
            .insert_one(customer, None)
            .is_ok()
    }

    fn update(&mut self, guid: String, customer: Customer) -> bool {
        let filter = doc! {"guid": guid};
        let new_customer = doc! {
                "$set":
                    {
                        "guid": customer.guid,
                        "guid": customer.first_name,
                        "last_name": customer.last_name,
                        "email": customer.email,
                        "address": customer.address,
                    },
            };
        self.customers
            .update_one(filter, new_customer, None)
            .is_ok()
    }

    fn delete(&mut self, guid: String) {
        let filter = doc! {"guid": guid};
        let _ = self
            .customers
            .delete_one(filter,None)
            .ok()
            .expect("Error deleting customer");
    }
}
