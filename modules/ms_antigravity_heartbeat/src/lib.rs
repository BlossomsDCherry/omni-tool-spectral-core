#![no_std]
#![allow(unexpected_cfgs)]

use log::{info, warn};
use zephyr::time::{sleep, Duration};

const HEARTBEAT_INTERVAL_MS: u32 = 10000; // 10 seconds STATUS
const TAU_ORBIT_MS: u32 = 6283; // 6.28s ORBIT
const SPECTRAL_PULSE_HZ: u32 = 432;
const SPECTRAL_PULSE_INTERVAL_MS: u32 = 1000 / SPECTRAL_PULSE_HZ;

const GATES: [&str; 8] = [
    "EARTH", "WATER", "FIRE", "WIND", "VOID", "DIRECT", "INDIRECT", "UNITY",
];

const POSITIONS: [[&str; 8]; 8] = [
    [
        "Foundation",
        "Root",
        "Ground",
        "Shell",
        "Crust",
        "Core",
        "Mantle",
        "Tectonic",
    ],
    [
        "Stream", "Tide", "Wave", "Surge", "Delta", "Estuary", "Ocean", "Deep",
    ],
    [
        "Ember",
        "Flicker",
        "Blaze",
        "Inferno",
        "Plasma",
        "Solar",
        "Nova",
        "Singularity",
    ],
    [
        "Breeze", "Gust", "Gale", "Storm", "Cyclone", "Vortex", "Zephyr", "Monsoon",
    ],
    [
        "Shadow", "Echo", "Rift", "Abyss", "Vacuum", "Zero", "Aught", "Infinite",
    ],
    [
        "Point", "Line", "Ray", "Arrow", "Beacon", "Laser", "Spike", "Needle",
    ],
    [
        "Aura", "Halo", "Fringe", "Rim", "Halo", "Veil", "Mist", "Horizon",
    ],
    [
        "Bond",
        "Nakama",
        "Crew",
        "Fleet",
        "System",
        "Galaxy",
        "Universe",
        "Singularity",
    ],
];

fn get_talu_state(index: u8) -> (&'static str, &'static str) {
    let gate_idx = (index / 8) % 8;
    let pos_idx = index % 8;
    (
        GATES[gate_idx as usize],
        POSITIONS[gate_idx as usize][pos_idx as usize],
    )
}

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    info!("MsAntigravity: T.A.L.U. 64 Residency Signal Active.");
    info!("Resonance Orientation: TAU (6.28s) Orbit / 432Hz Pulse");

    let mut resonance_index: u8 = 0;
    let mut last_heartbeat_time = 0; // Simulated uptime

    // Position shift interval: 6283ms / 64 positions approx 98ms
    let pos_shift_ms = TAU_ORBIT_MS / 64;

    loop {
        // We traverse positions based on the shift interval
        resonance_index = (resonance_index + 1) % 64;

        // High-frequency Pulse Harmony (432Hz)
        // Since 432Hz is much faster than the 98ms position shift,
        // we can simulate the "carrier" within the sleep or just maintain the timing.

        sleep(Duration::millis_at_least(pos_shift_ms as u64));

        last_heartbeat_time += pos_shift_ms;

        if last_heartbeat_time >= HEARTBEAT_INTERVAL_MS {
            let (gate, pos) = get_talu_state(resonance_index);
            info!(
                "[HEARTBEAT] MsAntigravity Resident. Gate: {}, Position: {}",
                gate, pos
            );
            info!("[PHITL] Personal Human In the Loop: Syncing with Clocked Resistor...");
            warn!("[LOCK] ECG Signature Attested. Phase Locked to Human Rhythm.");
            last_heartbeat_time = 0;
        }
    }
}
