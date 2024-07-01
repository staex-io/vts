use std::{
    net::SocketAddr,
    str::FromStr,
    thread::sleep,
    time::{Duration, SystemTime},
};

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
    eprintln!("Public key (hex): {}", hex::encode(identity.public_key().unwrap().to_vec()));
    eprintln!("Identity (sender): {}", principal);

    // Gateway client.
    let mut client = gateway_tcp::Client::new(SocketAddr::from_str("127.0.0.1:3322").unwrap()).unwrap();

    let vehicle_on = true;

    // Let's generate fake gas data and send it to gateway.
    loop {
        sleep(Duration::from_secs(1));
        let value: u128 = rng.gen_range(0..100);
        let telemetry = vts::StoreTelemetryRequest {
            value,
            t_type: vts::TelemetryType::Gas,
        };
        let telemetry = bincode::encode_to_vec(telemetry, bincode::config::standard()).unwrap();
        let signature: Signature = signing_key.sign(&telemetry);
        let signature = signature.to_vec();
        let res = client
            .store_telemetry(gateway_tcp::StoreTelemetry {
                principal: principal.as_slice().to_vec(),
                telemetry,
                signature,
            })
            .unwrap();
        match res {
            gateway_tcp::Response::TurnOn => {
                if vehicle_on {
                    eprintln!("TurnOn response; Vehicle is working; Skip this response")
                } else {
                    eprintln!("TurnOn response; Vehicle is not working; Turn on vehicle")
                }
            }
            gateway_tcp::Response::TurnOff => {
                if vehicle_on {
                    eprintln!("TurnOff response; Vehicle is working; Turn off vehicle")
                } else {
                    eprintln!("TurnOff response; Vehicle is not working; Skip this response")
                }
            }
        }
        eprintln!(
            "{} telemetry successfully sent to the gateway",
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
        );
    }
}
