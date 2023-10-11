use log::Level;
use rusnap::{handler, types, Route};

mod bip;
mod dialog;
mod net;
mod rand;
mod state;

#[handler]
pub async fn handle_hello(params: types::Params<Vec<String>>) -> String {
    log::info!("Handle hello");

    format!("Hello info: {}", params.0[0])
}

async fn main() {
    console_log::init_with_level(Level::Debug).unwrap();

    Route::new(())
        .at("hello", handle_hello)
        .at("alert", dialog::handle_alert)
        .at("confirm", dialog::handle_confirm)
        .at("promat", dialog::handle_promat)
        .at("notify", dialog::handle_notify)
        .at("update1", state::handle_update1)
        .at("update2", state::handle_update2)
        .at("get", state::handle_get)
        .at("bip32", bip::handle_bip32)
        .at("bip44", bip::handle_bip44)
        .at("entropy", bip::handle_entropy_v1)
        .at("entropy_salt", bip::handle_entropy_v1_salt)
        .at("network", net::handle_fetch)
        .at("rand", rand::handle_rand)
        .serve();
}

rusnap::entry!(main);
