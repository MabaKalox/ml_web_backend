use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::db::Db;
use crate::models::Customer;

pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await.clone();
    return Ok(warp::Reply::json(&customers));
}

pub async fn create_customer(new_customer: Customer, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await.clone();

    for customer in customers.iter() {
        if customer.guid == new_customer.guid {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    customers.push(new_customer);

    return Ok(StatusCode::CREATED);
}

pub async fn get_customer(guid: String, db: Db) -> Result<dyn warp::Reply, Infallible> {
    todo!()
}