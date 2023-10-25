use std::time::Duration;

use reqwest::Client;

pub fn get_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build client")
}
