use rusnap::handler;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HBResponse {
    pub origin: String,
}

#[handler]
pub async fn handle_fetch() -> String {
    log::info!("1.");
    let res = reqwest::get("https://httpbin.org/get").await.unwrap();
    log::info!("2.");
    let res: HBResponse = res.json().await.unwrap();
    log::info!("3.");

    res.origin
}
