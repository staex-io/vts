use ic_agent::{identity::Secp256k1Identity, Identity};
use k256::SecretKey;

fn main() {
    let secret_key = include_bytes!("../secret_key");
    let secret_key = SecretKey::from_bytes(secret_key.into()).unwrap();
    let identity = Secp256k1Identity::from_private_key(secret_key);
    eprintln!("Identity (sender): {}", identity.sender().unwrap());
}
