use crew_core::Talu64;
use std::fs;
use std::thread;
use std::time::Duration;

/// Sanji Agent: Heat & Simple Machine Axioms
/// Role: Thermal Monitoring and Simple State Management.
struct Sanji {
    thermal_zone: String,
}

impl Sanji {
    fn new() -> Self {
        println!("🔥 SANJI: Initializing Heat Management...");
        Self {
            thermal_zone: "/sys/class/thermal/thermal_zone0/temp".to_string(),
        }
    }

    fn read_temperature(&self) -> f64 {
        match fs::read_to_string(&self.thermal_zone) {
            Ok(temp_str) => temp_str.trim().parse::<f64>().unwrap_or(0.0) / 1000.0,
            Err(_) => {
                // Mock for non-Linux or missing file
                42.0 + (Talu64::PHI % 10.0)
            }
        }
    }

    fn apply_heat_check(&self) {
        let temp = self.read_temperature();
        let precision_temp = Talu64::truncate_8_sig_fig(temp);

        println!(
            "🍳 [DIABLE JAMBE] Core Temperature: {:.8}°C",
            precision_temp
        );

        if temp > 75.0 {
            println!("⚠️ [OVERHEAT] Fire is getting too hot! Throttling intent...");
        } else {
            println!("✅ [STABLE] Kitchen is at optimal temperature.");
        }
    }
}

fn main() {
    let cook = Sanji::new();
    loop {
        cook.apply_heat_check();
        thread::sleep(Duration::from_secs(5));
    }
}
