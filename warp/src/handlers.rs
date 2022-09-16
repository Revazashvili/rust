use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::db::Db;
use crate::models::Customer;

/// Returns a list of customers as JSON
#[utoipa::path(
    get,
    path = "/customers",
    tag = "customers",
    responses(
        (status = 200, description = "List customers successfully", body = [Customer])
    )
)]
pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await;
    let customers = customers.clone();
    Ok(warp::reply::json(&customers))
}

/// Gets a single customer from the data store
///
/// Returns a JSON object of an existing customer. If the customer
/// is not found, it returns a NOT FOUND status code.
#[utoipa::path(
    get,
    path = "/customers/{guid}",
    tag = "customers",
    params (
        ("guid" = String, Path, description = "Customer's unique id")
    ),
    responses(
        (status = 200, description = "returns customer"),
        (status = 404, description = "customer does not exists")
    )
)]
pub async fn get_customer(guid: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let customers = db.lock().await;
    for customer in customers.iter() {
        if customer.guid == guid {
            return Ok(Box::new(warp::reply::json(&customer)));
        }
    }
    Ok(Box::new(StatusCode::NOT_FOUND))
}

/// Creates a new customer
///
/// Adds a new customer to the data store if the customer
/// doesn't already exist.
#[utoipa::path(
    post,
    path = "/customers",
    tag = "customers",
    request_body = Customer,
    responses(
        (status = 200, description = "customer created successfully"),
        (status = 400, description = "customer already exists"),
    )
)]
pub async fn create_customer(
    new_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;
    for customer in customers.iter() {
        if customer.guid == new_customer.guid {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }
    customers.push(new_customer);
    Ok(StatusCode::CREATED)
}

/// Updates customers
///
/// Overwrites an existing customer in the data store and returns
/// an OK status code. If the customer is not found, a NOT FOUND status
/// code is returned.
#[utoipa::path(
    put,
    path = "/customers",
    tag = "customers",
    request_body = Customer,
    responses(
        (status = 200, description = "customer updated successfully"),
        (status = 404, description = "customer does not exists"),
    )
)]
pub async fn update_customer(
    guid: String,
    updated_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;
    for customer in customers.iter_mut() {
        if customer.guid == guid {
            *customer = updated_customer;
            return Ok(StatusCode::OK);
        }
    }
    Ok(StatusCode::NOT_FOUND)
}

/// Deletes a customer from the data store
///
/// If the customer exists in the data store, the customer is removed
/// and a NO CONTENT status code is returned. If the customer does not exist,
/// a NOT FOUND status code is returned.
#[utoipa::path(
    delete,
    path = "/customers/{guid}",
    tag = "customers",
    params (
        ("guid" = String, Path, description = "Customer's unique id")
    ),
    responses(
    (status = 204, description = "customer deleted successfully"),
    (status = 404, description = "customer does not exists"),
    )
)]
pub async fn delete_customer(guid: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;
    let customer_count = customers.len();
    customers.retain(|customer| customer.guid != guid);

    let deleted = customers.len() != customer_count;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
