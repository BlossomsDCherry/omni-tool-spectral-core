/*
    AP-OS V1: Toral Sorter Core (BLE Edition)
    Engine: Adam Wood Metal (RPi 5 A76)
    Logic: 16-Bin Chemically Reactive Spectrum (Helium -> Xenon)
    Transport: Bluetooth Low Energy (BLE) Pure Signal
*/

use bluer::adv::Advertisement;
use crew_core::rainbow_railgun::RailgunCore;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

const CYCLE_MICRO: u64 = 100_000; // 100ms cycle for BLE stability
const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0x12345678_1234_5678_1234_56789abcdef0); // Custom Toral UUID

#[tokio::main]
async fn main() -> bluer::Result<()> {
    println!("🧪 [AP-OS] Toral Sorter (BLE) Initializing...");

    // Initialize BlueZ Session
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    println!("⚓ [METAL] BLE Adapter Active: {}", adapter.name());

    let mut railgun = RailgunCore::new();
    let mut current_knots = 0.002;
    let mut pulse_count = 0;

    // 1. Register Advertisement ONCE (Static Beacon for Stability)
    // We use a static payload for the prototype to avoid D-Bus churn.
    // Nami will detect the Service UUID and know the "Pipe" is open.
    let payload = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x01]; // Static "Lock" Signature
    let le_advertisement = Advertisement {
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        service_data: BTreeMap::from([(SERVICE_UUID, payload)]),
        discoverable: Some(true),
        local_name: Some("Toral_Pulse".to_string()),
        ..Default::default()
    };
    
    println!("📡 [BLE] Registering Static Beacon...");
    // We bind the handle to `_adv_handle` to keep it alive for the duration of the program
    let _adv_handle = match adapter.advertise(le_advertisement).await {
        Ok(h) => Some(h),
        Err(e) => {
            eprintln!("⚠️ [BLE] Failed to register beacon: {}", e);
            eprintln!("⚠️ [BLE] Continuing in INTERNAL SIMULATION mode.");
            None
        }
    };

    loop {
        let pulse_start = Instant::now();
        
        // 2. Calculate Velocity (Physics Simulation)
        let velocity = railgun.calculate_v_rr(current_knots);
        let mode = if velocity > 30370.0 - 5000.0 && velocity < 30370.0 + 5000.0 { 
            0x01 // LOCK
        } else { 
            0x00 // DRIFT
        };

        // 3. Log Physics State
        if pulse_count % 100 == 0 {
             println!("🚀 [INTERNAL] Vel: {:.2} | Mode: {}", velocity, if mode == 1 { "LOCK" } else { "DRIFT" });
        }

        pulse_count += 1;
        current_knots = (current_knots + 0.00001) % 0.01;

        let elapsed = pulse_start.elapsed();
        if elapsed.as_micros() < CYCLE_MICRO as u128 {
            tokio::time::sleep(Duration::from_micros(CYCLE_MICRO - elapsed.as_micros() as u64)).await;
        }
    }
}
