use bluer::adv::Advertisement;
use std::time::Duration;
use std::collections::BTreeMap;
use tokio::time;
use log::info;
use uuid::Uuid;

pub async fn start_broadcasting() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    
    info!("📡 BROADCAST: Using adapter '{}'", adapter.name());
    adapter.set_powered(true).await?;

    info!("📡 BROADCAST: Starting Sovereign Uplink (UUID 0xFCA7)...");

    loop {
        // Create Advertisement
        let mut le_advertisement = Advertisement {
            advertisement_type: bluer::adv::Type::Peripheral,
            local_name: Some("Sovereign (Robin)".to_string()),
            ..Default::default()
        };

        // Service UUIDs
        // 0xFCA7 expanded to 128-bit
        let sovereign_uuid = Uuid::parse_str("0000fca7-0000-1000-8000-00805f9b34fb").unwrap(); 
        le_advertisement.service_uuids.insert(sovereign_uuid);

        // Service Data (Payload)
        // [CrewID (Robin=7), Load (0-100), Status, Reserved]
        let load = 42; // Placeholder for system load
        let payload = vec![0x07, load, 0x01, 0x00]; 
        
        let mut service_data = BTreeMap::new();
        service_data.insert(sovereign_uuid, payload);
        le_advertisement.service_data = service_data;

        // Register
        let handle = adapter.advertise(le_advertisement).await?;
        
        // info!("📡 BROADCAST: Beacon Active (Load: {}%)", load);
        
        // Keep active for 5 seconds then update (Simulating dynamic load)
        time::sleep(Duration::from_secs(10)).await;
        
        // Dropping 'handle' unregisters the advertisement
        drop(handle);
    }
}
