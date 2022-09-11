use mongodb::{Client};
use serde::{Serialize, Deserialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    name: String,
}


#[tokio::main]
async fn main() -> io::Result<()> {
    const DATABASE_NAME: &str = "todos";
    const COLLECTION_NAME: &str = "todos";

    println!("Hello, world!");

    let client = Client::with_uri_str("mongodb://root:example@mongo:27017").await.unwrap();
    let collection = client.database(DATABASE_NAME).collection::<Todo>(COLLECTION_NAME);

    for i in 1..5{
        collection.insert_one(Todo{name:i.to_string()}, None);
    }

    Ok(())
}
