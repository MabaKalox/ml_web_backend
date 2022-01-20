use serde_json::from_reader;
use std::convert::Infallible;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use crate::customers::models::Customer;

pub type Db = Arc<Mutex<Vec<Customer>>>;

pub fn init_db() -> Db {
    match File::open("../data/customers.json") {
        Ok(json) => Arc::new(Mutex::new(from_reader(json).unwrap())),
        Err(_) => Arc::new(Mutex::new(Vec::new())),
    }
}

pub fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
