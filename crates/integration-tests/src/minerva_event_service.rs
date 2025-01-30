use std::net::{SocketAddr, TcpStream};
use std::process::Command;
use std::time::Duration;

use assert_cmd::prelude::*;
use log::{debug, error};
use minerva::error::{Error, RuntimeError};

pub struct MinervaEventServiceConfig {
    pub pg_host: String,
    pub pg_port: String,
    pub pg_sslmode: String,
    pub pg_database: String,
    pub service_address: String,
    pub service_port: u16,
}

pub struct MinervaEventService {
    conf: MinervaEventServiceConfig,
    pub proc_handle: std::process::Child,
}

impl MinervaEventService {
    pub fn start(conf: MinervaEventServiceConfig) -> Result<MinervaEventService, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("minerva-event-service")?;

        cmd.env("PGHOST", &conf.pg_host)
            .env("PGPORT", &conf.pg_port)
            .env("PGSSLMODE", &conf.pg_sslmode)
            .env("PGDATABASE", &conf.pg_database)
            .env("SERVICE_ADDRESS", &conf.service_address)
            .env("SERVICE_PORT", conf.service_port.to_string());

        let proc_handle = cmd.spawn()?;

        Ok(MinervaEventService { conf, proc_handle })
    }

    pub async fn wait_for(&mut self) -> Result<(), Error> {
        let service_address = format!("{}:{}", self.conf.service_address, self.conf.service_port);

        let timeout = Duration::from_millis(1000);

        let ipv4_addr: SocketAddr = service_address.parse().unwrap();

        loop {
            let result = TcpStream::connect_timeout(&ipv4_addr, timeout);

            debug!("Trying to connect to service at {}", ipv4_addr);

            match result {
                Ok(_) => return Ok(()),
                Err(_) => {
                    // Check if process is still running
                    let wait_result = self.proc_handle.try_wait().map_err(|e| {
                        RuntimeError::from_msg(format!("Could not wait for service exit: {e}"))
                    })?;

                    if let Some(status) = wait_result {
                        panic!("Service prematurely exited with code: {status}");
                    }

                    tokio::time::sleep(timeout).await
                }
            }
        }
    }

    pub fn base_url(&self) -> String {
        format!(
            "http://{}:{}",
            self.conf.service_address, self.conf.service_port
        )
    }
}

impl Drop for MinervaEventService {
    fn drop(&mut self) {
        match self.proc_handle.kill() {
            Err(e) => error!("Could not stop web service: {e}"),
            Ok(_) => debug!("Stopped web service"),
        }
    }
}
