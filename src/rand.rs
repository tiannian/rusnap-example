use rand_core::{OsRng, RngCore};
use rusnap::handler;

#[handler]
pub async fn handle_rand() {
    let mut res = [0u8; 16];

    OsRng.fill_bytes(&mut res);

    log::info!("{:?}", res);
}
