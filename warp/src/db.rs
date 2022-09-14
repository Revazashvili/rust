use std::{sync::Arc, fs::{File}};
use serde_json::from_reader;
use tokio::sync::Mutex;

use crate::models::Customer;

const PATH: &str = "customers.json";

/// Represents an in memory data store of customer data
pub type Db = Arc<Mutex<Vec<Customer>>>;


/// Initializes the data store
/// 
/// Returns a Db type that either contains customers or is empty.
pub fn init_db() -> Db {
    match File::open(PATH) {
        Ok(file) => Arc::new(Mutex::new(from_reader(file).unwrap())),
        Err(_) => Arc::new(Mutex::new(Vec::new()))
    }
}

#[cfg(test)]
mod tests{
    use super::{init_db,PATH};
    use std::{fs::{self}};
    use std::io::Write;

    #[tokio::test]
    async fn should_return_empty_type(){
        let db = init_db();
        assert_eq!(true,db.lock().await.is_empty());
    }

    #[tokio::test]
    async fn should_return_non_empty_type(){
        let mut file = fs::OpenOptions::new().write(true).create(true).open(PATH).unwrap();
        let customers_json = r#"
        [
            {
                "guid":"9d4819bd-86a9-4f66-9b8d-7428c414ab23",
                "first_name":"John",
                "last_name":"Doe",
                "email":"doe@example.com",
                "address":"address"
            }
        ]"#;
        match file.write(customers_json.as_bytes()){
            Err(e) => panic!("{}",e),
            Ok(_) => {
                let db = init_db();
                assert_ne!(true,db.lock().await.is_empty());
            }
        }
        fs::remove_file(PATH).unwrap();
    }
}
