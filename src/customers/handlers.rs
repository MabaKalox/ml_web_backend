use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::db::Db;
use crate::customers::models::Customer;

pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await.clone();
    return Ok(warp::reply::json(&customers));
}

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

    return Ok(StatusCode::CREATED);
}

pub async fn get_customer(guid: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let customers = db.lock().await;

    for customer in customers.iter() {
        if customer.guid.eq(&guid) {
            return Ok(Box::new(warp::reply::json(&customer)));
        }
    }

    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_customer(
    updated_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter_mut() {
        if customer.guid == updated_customer.guid {
            *customer = updated_customer;
            return Ok(StatusCode::OK);
        }
    }

    return Ok(StatusCode::NOT_FOUND);
}

pub async fn delete_customer(guid: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    let customers_counter = customers.len();

    customers.retain(|customer| customer.guid != guid);

    return match customers.len() == customers_counter {
        true => Ok(StatusCode::NOT_FOUND),
        false => Ok(StatusCode::OK),
    };
}
