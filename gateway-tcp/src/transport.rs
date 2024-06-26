use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use bincode::{Decode, Encode};

pub type Res<T> = Result<T, String>;

#[derive(Encode, Decode)]
pub enum Request {
    StoreTelemetry(StoreTelemetry),
}

#[derive(Encode, Decode)]
pub enum Response {
    TurnOn,
    TurnOff,
}

#[derive(Encode, Decode)]
pub struct StoreTelemetry {
    pub principal: Vec<u8>,
    pub telemetry: Vec<u8>,
    pub signature: Vec<u8>,
}

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: SocketAddr) -> Res<Self> {
        let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10)).map_err(map_err)?;
        Ok(Self { stream })
    }

    pub fn store_telemetry(&mut self, data: StoreTelemetry) -> Res<Response> {
        let req = Request::StoreTelemetry(data);
        let mut buf = bincode::encode_to_vec(req, bincode::config::standard()).map_err(map_err)?;
        buf.push(b'\n');
        self.stream.write_all(&buf).map_err(map_err)?;

        let mut buf = vec![0; 8];
        self.stream.read(&mut buf).map_err(map_err)?;
        let res: Response = bincode::decode_from_slice(&buf, bincode::config::standard()).map_err(map_err)?.0;
        Ok(res)
    }
}

fn map_err<E: ToString>(e: E) -> String {
    e.to_string()
}
