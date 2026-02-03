use std::fs;
use std::path::Path;
use crew_core::Talu64;

/// Vision Packet (The Payload for Sight)
/// Represents a single frame of inference from the Hailo-8.
struct VisionPacket {
    frame_id: u64,
    detected_class: String, // e.g., "Person"
    confidence: f64,        // 0.0 - 1.0 (Truncated to 8 sig figs)
    bbox: [f64; 4],         // [x, y, w, h] normalized
}

struct HailoProbe {
    device_path: String,
    driver_version_path: String,
    connected: bool,
}

impl HailoProbe {
    fn new() -> Self {
        HailoProbe {
            device_path: "/dev/hailo0".to_string(),
            driver_version_path: "/sys/module/hailo_pci/version".to_string(),
            connected: false,
        }
    }

    fn probe_hardware(&mut self) {
        println!("--- Vision Probe Initiating ---");
        
        // 1. Check Driver Version
        match fs::read_to_string(&self.driver_version_path) {
            Ok(v) => {
                println!("   [Hailo-8] Driver Version: {}", v.trim());
            },
            Err(_) => {
                println!("   [Hailo-8] Driver Version: UNKNOWN (Module not loaded?)");
            }
        }

        // 2. Check Device Node
        let path = Path::new(&self.device_path);
        if path.exists() {
            println!("   [Hailo-8] Device Node: FOUND ({})", self.device_path);
            self.connected = true;
        } else {
            println!("   [Hailo-8] Device Node: MISSING");
            self.connected = false;
        }
    }

    fn open_eye(&self) {
        if self.connected {
            println!("   [Vision Probe] The Eye is OPEN. Directional Anchor established.");
            println!("   [Vision Probe] Ready to bind to the Spectral Trajectory.");
            
            // Simulate receiving a packet to prove data structure
            let packet = VisionPacket {
                frame_id: 1,
                detected_class: "User".to_string(),
                confidence: Talu64::truncate_8_sig_fig(0.99999999), 
                bbox: [0.5, 0.5, 0.2, 0.6]
            };
            
            println!("   >>> TRAJECTORY POINT [Frame {}]: Class='{}' Conf={:.8} Bbox=[{:.2}, {:.2}, {:.2}, {:.2}]", 
                packet.frame_id, packet.detected_class, packet.confidence, 
                packet.bbox[0], packet.bbox[1], packet.bbox[2], packet.bbox[3]
            );
        } else {
            println!("   [Vision Probe] The Eye is CLOSED. Directional anchor missing.");
        }
    }
}

fn main() {
    let mut probe = HailoProbe::new();
    probe.probe_hardware();
    probe.open_eye();
}
