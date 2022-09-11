use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::Customer;


fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> {
    warp::any().map(move || db.clone())
}