use rusnap::{
    api::{
        get_bip32_entropy, get_bip32_public_key, get_bip44_entropy, get_entropy_v1,
        get_entropy_v1_salt, Curve,
    },
    exports::handler,
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

#[handler]
pub async fn handle_bip44() {
    let res = get_bip44_entropy(3).await.unwrap();

    log::info!("Secret {:?}", res);
}

#[handler]
pub async fn handle_entropy_v1() {
    let res = get_entropy_v1().await.unwrap();

    log::info!("{:?}", res);
}

#[handler]
pub async fn handle_entropy_v1_salt() {
    let res = get_entropy_v1_salt("Hello").await.unwrap();

    log::info!("{:?}", res);
}
