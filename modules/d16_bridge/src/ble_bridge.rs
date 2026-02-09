use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use log::{info, error};

pub struct BleBridge;

impl BleBridge {
    pub async fn new() -> Self {
        Self
    }

    pub async fn scan_and_listen(&self) -> Result<(), Box<dyn Error>> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        
        if adapters.is_empty() {
            error!("No Bluetooth adapters found on Sovereign.");
            return Ok(());
        }

        let adapter = &adapters[0];
        info!("Starting BLE Scan on adapter: {:?}", adapter.adapter_info().await?);

        // Start scanning
        adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(2)).await;

        loop {
            let peripherals = adapter.peripherals().await?;
            
            if peripherals.is_empty() {
                 // info!("..Scanning..");
            } else {
                for peripheral in peripherals {
                    let properties = peripheral.properties().await?;
                    if let Some(props) = properties {
                        if let Some(local_name) = props.local_name {
                            if local_name.contains("Hubble") || local_name.contains("Giga") || local_name.contains("Arduino") {
                                info!("🛰️  HUBBLE CONTACT: Found '{}' [{:?}]", local_name, peripheral.id());
                                // TODO: Connect and Subscribe
                            }
                        }
                    }
                }
            }
            time::sleep(Duration::from_secs(5)).await;
        }
    }
}
