use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Manager;
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

        let hubble_uuid = uuid::Uuid::parse_str("0000fca6-0000-1000-8000-00805f9b34fb")?; // 16-bit UUID expanded

        loop {
            let peripherals = adapter.peripherals().await?;
            
            for peripheral in peripherals {
                let properties = peripheral.properties().await?;
                if let Some(props) = properties {
                    // Check for Hubble Service Data
                    if let Some(data) = props.service_data.get(&hubble_uuid) {
                        if data.len() == 4 {
                            let coherence_bytes: [u8; 4] = data[..4].try_into()?;
                            let coherence = f32::from_le_bytes(coherence_bytes);
                            info!("🛰️  HUBBLE PACKET (from {:?}): Coherence = {:.4}", props.local_name, coherence);
                        }
                    } 
                    // Fallback to name check for debugging
                    else if let Some(local_name) = &props.local_name {
                        if local_name.contains("D16") || local_name.contains("Hazard") {
                            // info!("Found D16 Device: {}", local_name);
                        }
                    }
                }
            }
            time::sleep(Duration::from_millis(100)).await; // Faster scan loop
        }
    }
}
