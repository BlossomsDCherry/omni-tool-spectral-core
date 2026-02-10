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
                    if let Some(data) = props.service_data.values().next() {
                         // Removed UUID check for now to just print ANY service data
                         info!("📡  BLE SERVICE DATA (from {:?}): {:?}", props.local_name, data);
                    }
                    
                    
                    // Always check for Mesh Beacons (regardless of name)
                    for (uuid, data) in &props.service_data {
                         if uuid.to_string().contains("fdf7") {
                             let mac = peripheral.address();
                             info!("⚡ MESH NODE [{}] RSSI: {}", mac, props.rssi.unwrap_or(0));
                             info!("   Payload: {:?}", data);
                             
                             // Check for "Three of the same number" pattern (e.g. repeated bytes)
                             // or just count the nodes.
                             // We rely on the log stream for now to count unique MACs.
                         }
                    }

                    if let Some(local_name) = &props.local_name {
                        // Priority Check
                        if local_name.contains("Bluefruit") || local_name.contains("Hubble") || local_name.contains("Pico") {
                             info!("✅ TARGET ACQUIRED: {} [{}] RSSI: {}", local_name, peripheral.address(), props.rssi.unwrap_or(0));
                             // Optional: dump other services for named targets
                             if !props.services.is_empty() {
                                 info!("   Services: {:?}", props.services);
                             }
                        }
                    } 
                    
                    // Look for specific Manufacturer IDs if needed (e.g. Nordic)
                    /*
                    for (id, data) in &props.manufacturer_data {
                        if *id == 0x0059 { // Nordic Semiconductor
                             info!("🔎 Nordic Device: [{}] {:?}", peripheral.address(), data);
                        }
                    }
                    */
                }
            }
            time::sleep(Duration::from_millis(100)).await;
        }
    }
}
