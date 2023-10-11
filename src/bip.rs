use rusnap::{
    handler,
    snap::{get_bip32_entropy, get_bip32_public_key, Curve},
};

#[handler]
pub async fn handle_bip32() {
    let res = get_bip32_entropy("m/44'/3'", Curve::Secp256k1)
        .await
        .unwrap();
    log::info!("Secret Key: {:?}", res);

    let res = get_bip32_public_key("m/44'/3'/0'/0/0", Curve::Secp256k1, false)
        .await
        .unwrap();
    log::info!("Public Key: {:?}", res);
}
