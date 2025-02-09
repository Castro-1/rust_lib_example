use rand::{prelude::*, rng};

mod database;
mod auth_utils;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = rng().random_range(100..500);
    println!("Timeout: {}", timeout);
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}