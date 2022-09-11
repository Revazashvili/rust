use crate::{db::init_db, routes::customer_routes};

mod db;
mod handlers;
mod models;
mod routes;
mod storage;

#[tokio::main]
async fn main() {
    let db = init_db();
    let customer_routes = customer_routes(db);

    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await
}
