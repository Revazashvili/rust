pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use models::*;
use schema::posts::dsl::*;
use uuid::Uuid;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| String::from("test.db"));
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", 
                                   database_url))
}

pub fn get_posts() -> Vec<Post> {
    match posts.load::<Post>(&establish_connection()) {
        Ok(ps) => ps,
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Vec::new()
        }
    }
}

pub fn create_post(t: &str,b: &str) -> String{
    let uuid = Uuid::new_v4().as_hyphenated().to_string();
    let new_post = NewPost { id:&uuid,title:t,body:b,published:false };

    let _ = diesel::insert_into(posts)
        .values(&new_post)
        .execute(&establish_connection());
    uuid
}

pub fn publish_post(key:String) -> bool {
    match diesel::update(posts.find(key))
        .set(published.eq(true))
        .execute(&establish_connection()){
        Ok(_) => true,
        Err(_) => false
    }
}