use std::{fmt::Debug, time::Duration};

use log::{debug, error, info, trace, LevelFilter};
use tokio::{
    select,
    sync::watch,
    time::{sleep, timeout},
};

type Res<T> = Result<T, Error>;

pub(crate) struct Error(String);

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: ToString> From<T> for Error {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

#[tokio::main]
async fn main() -> Res<()> {
    env_logger::builder()
        .filter(None, LevelFilter::Off)
        .filter_module("gateway", LevelFilter::Trace)
        .init();
    let (stop_s, stop_r) = watch::channel(());
    tokio::spawn(async move { wait_for_firmware_requests(stop_r).await });
    info!("gateway started; waiting for termination signal");
    tokio::signal::ctrl_c().await?;
    debug!("received termination signal");
    stop_s.send(())?;
    match timeout(Duration::from_secs(10), stop_s.closed()).await {
        Ok(_) => info!("everything was stopped successfully"),
        Err(e) => {
            error!("failed to stop everything: {}", e)
        }
    }
    Ok(())
}

async fn wait_for_firmware_requests(mut stop_r: watch::Receiver<()>) {
    loop {
        select! {
            _ = stop_r.changed() => {
                trace!("received stop signal, exit tcp server loop");
                return;
            }
            _ = sleep(Duration::from_secs(1)) => {
                if let Err(e) = check_firmware_requests().await {
                    error!("failed to check firmware requests: {:?}", e)
                }
            }
        }
    }
}

async fn check_firmware_requests() -> Res<()> {
    Ok(())
}
