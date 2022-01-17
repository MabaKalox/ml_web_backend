use std::fs::File;
use std::sync::Arc;
use serde_json::from_reader;
use tokio::sync::Mutex;

use crate::models::Customer;

pub type Db = Arc<Mutex<Vec<Customer>>>;

pub fn init_db() -> Db {
    match File::open("./data/customers.json") {
        Ok(json) => Arc::new(Mutex::new(from_reader(json).unwrap())),
        Err(_) => Arc::new(Mutex::new(Vec::new()))
    }
}

