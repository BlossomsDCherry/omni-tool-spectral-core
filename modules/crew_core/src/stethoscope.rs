use std::process::Command;
use std::time::Instant;

/// The Medical Tool: System Stethoscope 🩺
/// Monitors PMIC telemetry for high-fidelity biological intent.
pub struct Stethoscope {
    baseline_voltage: f64,
    sag_threshold: f64,
    last_pulse: Instant,
    remote_target: Option<String>,
    persistence: u32,
    persistence_threshold: u32,
    pulse_durations: Vec<f64>, // [NEW] Rolling buffer for frequency calculation
}

impl Stethoscope {
    pub fn new() -> Self {
        Self {
            baseline_voltage: 3.303, // Updated baseline from live trace
            sag_threshold: 0.005,    // Sensitive 5mV sag threshold
            last_pulse: Instant::now(),
            remote_target: Some("uv_d_antigravity@10.0.0.80".to_string()),
            persistence: 0,
            persistence_threshold: 2,
            pulse_durations: Vec::with_capacity(10),
        }
    }

    /// [NEW] Calculates the resonance frequency from captured pulse durations.
    pub fn resonance_hz(&self) -> f64 {
        if self.pulse_durations.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.pulse_durations.iter().sum();
        let avg = sum / self.pulse_durations.len() as f64;
        if avg > 0.0 {
            1.0 / avg
        } else {
            0.0
        }
    }

    /// [NEW] Records a completed pulse duration.
    pub fn record_pulse(&mut self, duration: f64) {
        if self.pulse_durations.len() >= 10 {
            self.pulse_durations.remove(0);
        }
        self.pulse_durations.push(duration);
    }

    pub fn listen(&mut self) -> Option<f64> {
        let output = if let Some(ref target) = self.remote_target {
            Command::new("sshpass")
                .arg("-p")
                .arg("kali56")
                .arg("ssh")
                .arg("-o")
                .arg("StrictHostKeyChecking=no")
                .arg(target)
                .arg("vcgencmd pmic_read_adc")
                .output()
                .ok()?
        } else {
            Command::new("vcgencmd")
                .arg("pmic_read_adc")
                .output()
                .ok()?
        };

        let raw = String::from_utf8_lossy(&output.stdout);

        if let Some(idx) = raw.find("3V3_SYS_V") {
            let rest = &raw[idx..];
            if let Some(eq_idx) = rest.find('=') {
                let val_part = &rest[eq_idx + 1..];
                if let Some(end_idx) = val_part.find('V') {
                    if let Ok(voltage) = val_part[..end_idx].parse::<f64>() {
                        // ADAPTIVE CALIBRATION: Slowly pull baseline towards current voltage if not in sag
                        let sag = self.baseline_voltage - voltage;

                        if sag > self.sag_threshold {
                            self.persistence = 0;
                            self.last_pulse = Instant::now();
                            return Some(sag);
                        } else {
                            // Update baseline slowly to follow thermal/power drift (Alpha = 0.1)
                            self.baseline_voltage = (self.baseline_voltage * 0.9) + (voltage * 0.1);

                            self.persistence += 1;
                            if self.persistence < self.persistence_threshold {
                                return Some(0.001);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
