use rusnap::{
    api::{state_get, state_update},
    exports::handler,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}

#[handler]
pub async fn handle_update1() {
    let s = State {
        key1: Some("hello1".into()),
        key2: None,
    };

    state_update(s).await.unwrap()
}

#[handler]
pub async fn handle_update2() {
    let s = State {
        key2: Some("hello2".into()),
        key1: None,
    };

    state_update(s).await.unwrap()
}

#[handler]
pub async fn handle_get() {
    let s: State = state_get().await.unwrap();

    log::info!("{:?}", s);
}
