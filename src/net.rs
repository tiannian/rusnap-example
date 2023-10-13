use rusnap::exports::handler;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HBResponse {
    pub origin: String,
}

#[handler]
pub async fn handle_fetch() -> String {
    log::info!("1.");
    if let Err(e) = reqwest::get("https://httpbin.org/get").await {
        log::error!("{:?}", e);
    }
    // let res: HBResponse = res.json().await.unwrap();
    // log::info!("3.");

    // res.origin
    "hello".into()
}
