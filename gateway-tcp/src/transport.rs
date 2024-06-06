use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use bincode::{Decode, Encode};

pub type Res<T> = Result<T, String>;

#[derive(Encode, Decode)]
pub struct StoreTelemetryData {
    pub principal: Vec<u8>,
    pub telemetry: Vec<u8>,
    pub signature: Vec<u8>,
}

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: SocketAddr) -> Res<Self> {
        let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10)).map_err(|e| e.to_string())?;
        Ok(Self { stream })
    }

    pub fn store_telemetry(&mut self, data: StoreTelemetryData) -> Res<()> {
        let buf = bincode::encode_to_vec(data, bincode::config::standard()).map_err(|e| e.to_string())?;
        self.stream.write_all(&buf).map_err(|e| e.to_string())?;
        Ok(())
    }
}
