use std::sync::{Arc, Mutex};
use z_rr::railgun::ZRailgun;

// Setup UniFFI without UDL
uniffi::setup_scaffolding!("d16_sdk");

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum D16Error {
    #[error("Network failure")]
    NetworkError,
    #[error("Configuration invalid")]
    ConfigError,
    #[error("Platform not supported")]
    PlatformError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum D16Platform {
    Android,
    Windows,
    Linux,
    Embedded,
}

#[derive(uniffi::Record)]
pub struct ConnectionConfig {
    pub udp_port: u16,
    pub hub_ip: String,
}

#[derive(uniffi::Object)]
pub struct D16Engine {
    config: ConnectionConfig,
    railgun: Arc<Mutex<Option<ZRailgun>>>,
}

#[uniffi::export]
impl D16Engine {
    #[uniffi::constructor]
    pub fn new(config: ConnectionConfig) -> Arc<Self> {
        Arc::new(Self {
            config,
            railgun: Arc::new(Mutex::new(None)),
        })
    }

    pub fn connect(&self) -> Result<(), D16Error> {
        let mut railgun_guard = self.railgun.lock().unwrap();
        if railgun_guard.is_some() {
            return Ok(());
        }

        println!("D16 SDK: Initializing Z-RR Railgun Transport...");
        // Initialize ZRailgun with a seed (e.g. port as seed for now)
        let railgun = ZRailgun::new(self.config.udp_port as u64);
        *railgun_guard = Some(railgun);

        println!("D16 SDK: Connected via UDP 8 Pipeline.");
        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), D16Error> {
        let mut railgun_guard = self.railgun.lock().unwrap();
        if railgun_guard.is_none() {
            return Ok(());
        }
        *railgun_guard = None;
        println!("D16 SDK: Disconnected Z-RR.");
        Ok(())
    }

    pub fn get_status(&self) -> String {
        let railgun_guard = self.railgun.lock().unwrap();
        if let Some(ref rg) = *railgun_guard {
            // We can even expose Z-RR state here
            format!(
                "Connected to {}. Entropy Seed: {}",
                self.config.hub_ip, rg.entropy_seed
            )
        } else {
            "Disconnected".to_string()
        }
    }
}
