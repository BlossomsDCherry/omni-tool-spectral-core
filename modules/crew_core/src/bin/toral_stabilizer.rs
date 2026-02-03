/*
    Toral Stabilizer: Zemon Receiver (BLE Edition)
    Role: Scans for Toral Pulse packets and harmonizes them via Zemons.
*/

use bluer::{AdapterEvent, Session};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, Seek, Write};

const FB_PATH: &str = "/dev/fb0";
const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0x12345678_1234_5678_1234_56789abcdef0);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ToralPulse {
    velocity: f64,
    #[allow(dead_code)]
    stability: String,
    burst: Option<(u8, u8, u8, f64)>,
}

/// The Theremin: Tunes to specific wave densities
struct Theremin {
    tuned_frequency: f64,
    bandwidth: f64,
}

impl Theremin {
    fn new(freq: f64, width: f64) -> Self {
        Self {
            tuned_frequency: freq,
            bandwidth: width,
        }
    }

    fn check_resonance(&self, signal_freq: f64) -> bool {
        (signal_freq - self.tuned_frequency).abs() < self.bandwidth
    }
}

/// The Zemon
struct Zemon {
    #[allow(dead_code)]
    identity: String,
    role: ZemonRole,
    hardware_link: Option<SenseHatFb>,
}

enum ZemonRole {
    SignalOwner,
    NoiseCleaner,
}

impl Zemon {
    fn new_signal(identity: &str) -> Result<Self, Box<dyn Error>> {
        let hardware_link = SenseHatFb::new().ok(); 
        Ok(Self {
            identity: identity.to_string(),
            role: ZemonRole::SignalOwner,
            hardware_link,
        })
    }

    fn new_noise(identity: &str) -> Self {
        Self {
            identity: identity.to_string(),
            role: ZemonRole::NoiseCleaner,
            hardware_link: None,
        }
    }

    /// Harmonizes the feed based on Theremin Resonance
    fn harmonize(&mut self, velocity: f64, mode: u8, theremin: &Theremin) -> io::Result<()> {
        let is_resonant = theremin.check_resonance(velocity);

        match self.role {
            ZemonRole::SignalOwner => {
                if is_resonant && mode == 0x01 {
                    // "Owns" the signal -> Pass to Hardware
                    if let Some(hat) = &mut self.hardware_link {
                        // Visualize Resonance: Green Burst
                        hat.set_all(0, 255, 100)?;
                    }
                } else if self.hardware_link.is_some() {
                     // Drift / No Signal
                     if let Some(hat) = &mut self.hardware_link {
                        hat.set_all(0, 0, 0)?;
                     }
                }
            }
            ZemonRole::NoiseCleaner => {
                // Silently cleans noise
            }
        }
        Ok(())
    }
}

struct SenseHatFb {
    file: fs::File,
}

impl SenseHatFb {
    fn new() -> io::Result<Self> {
        let file = fs::OpenOptions::new().write(true).open(FB_PATH)?;
        Ok(Self { file })
    }

    fn set_pixels(&mut self, pixels: &[(u8, u8, u8); 64]) -> io::Result<()> {
        let mut buffer = [0u8; 128]; // 64 pixels * 2 bytes (RGB565)
        for (i, &(r, g, b)) in pixels.iter().enumerate() {
            let r5 = (r >> 3) as u16;
            let g6 = (g >> 2) as u16;
            let b5 = (b >> 3) as u16;
            let rgb565 = (r5 << 11) | (g6 << 5) | b5;
            buffer[i * 2] = (rgb565 & 0xFF) as u8;
            buffer[i * 2 + 1] = (rgb565 >> 8) as u8;
        }
        self.file.seek(io::SeekFrom::Start(0))?;
        self.file.write_all(&buffer)?;
        Ok(())
    }

    fn set_all(&mut self, r: u8, g: u8, b: u8) -> io::Result<()> {
        self.set_pixels(&[(r, g, b); 64])
    }
}

#[tokio::main]
async fn main() -> bluer::Result<()> {
    println!("👾 [Zemon] Adaptive BLE Filter Initializing...");
    
    // The Theremin: Tuned to the Superconducting Velocity ~30370
    let theremin = Theremin::new(30370.0, 5000.0); 

    // The Agents
    let mut signal_agent = Zemon::new_signal("AM_Signal_Owner").unwrap_or_else(|_| Zemon::new_noise("Fallback"));
    let mut noise_agent = Zemon::new_noise("Static_Scrubber");
    
    // Initial Clear
    if let Some(hat) = &mut signal_agent.hardware_link {
        hat.set_all(0, 0, 0).ok();
    }

    let session = Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    
    println!("📡 [BLE] Scanning for Toral Pulse ({})...", SERVICE_UUID);
    
    let mut events = adapter.events().await?;
    
    while let Some(event) = events.next().await {
        match event {
             AdapterEvent::DeviceAdded(addr) => {
                 if let Ok(device) = adapter.device(addr) {
                     if let Ok(Some(services)) = device.service_data().await {
                         if let Some(payload) = services.get(&SERVICE_UUID) {
                             if payload.len() >= 5 {
                                 // Decode Payload: [Vel (4 bytes) | Mode (1 byte)]
                                 let mut vel_bytes = [0u8; 4];
                                 vel_bytes.copy_from_slice(&payload[0..4]);
                                 let velocity = u32::from_le_bytes(vel_bytes) as f64;
                                 let mode = payload[4];

                                 // Harmonize
                                 signal_agent.harmonize(velocity, mode, &theremin).ok();
                                 noise_agent.harmonize(velocity, mode, &theremin).ok();
                                 
                                 // println!("✨ [{}] Vel: {:.0} | Mode: {}", addr, velocity, mode);
                             }
                         }
                     }
                 }
             }
             _ => {}
        }
    }

    Ok(())
}
