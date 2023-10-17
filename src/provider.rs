use ethers::{middleware::Middleware, providers::Provider};
use rusnap::exports::handler;
use rusnap_ethers::MetamaskProvider;

#[handler]
async fn handle_chain_id() {
    let provider = MetamaskProvider::default();
    log::info!("{:?}", provider.get_chainid().await);
}

#[handler]
async fn handle_balance() {
    let provider = MetamaskProvider::default();
    log::info!(
        "{:?}",
        provider
            .get_balance("0xDAFEA492D9c6733ae3d56b7Ed1ADB60692c98Bc5", None)
            .await
    )
}
