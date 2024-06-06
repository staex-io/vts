use std::{net::SocketAddr, str::FromStr, thread::sleep, time::Duration};

use ic_agent::{identity::Secp256k1Identity, Identity};
use k256::{
    ecdsa::{signature::SignerMut, Signature, SigningKey},
    SecretKey,
};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let secret_key = include_bytes!("../secret_key");
    let secret_key = SecretKey::from_bytes(secret_key.into()).unwrap();
    let mut signing_key = SigningKey::from(&secret_key);
    let identity = Secp256k1Identity::from_private_key(secret_key);
    let principal = identity.sender().unwrap();
    eprintln!("Identity (sender): {}", principal);

    // Gateway client.
    let mut client = gateway_tcp::Client::new(SocketAddr::from_str("127.0.0.1:3322").unwrap()).unwrap();

    // Let's generate fake gas data and send it to gateway.
    loop {
        sleep(Duration::from_secs(1));
        let value: u128 = rng.gen_range(0..100);
        let telemetry = vts::Telemetry {
            value,
            t_type: vts::TelemetryType::Gas,
        };
        let telemetry = bincode::encode_to_vec(telemetry, bincode::config::standard()).unwrap();
        let signature: Signature = signing_key.sign(&telemetry);
        let signature = signature.to_vec();
        client
            .store_telemetry(gateway_tcp::StoreTelemetry {
                principal: principal.as_slice().to_vec(),
                telemetry,
                signature,
            })
            .unwrap();
    }
}
