use crate::resonance::{ResonanceAnalyzer, ResonanceReport};
use log::{debug, error, info};

use std::io::{self, BufRead, BufReader};
use std::time::Duration;

pub struct SerialBridge {
    port_name: String,
    baud_rate: u32,
}

impl SerialBridge {
    pub fn new(port_name: &str, baud_rate: u32) -> Self {
        Self {
            port_name: port_name.to_string(),
            baud_rate,
        }
    }

    pub fn listen<F>(&self, on_report: F) -> io::Result<()>
    where
        F: Fn(ResonanceReport),
    {
        info!(
            "Attempting to connect to Serial Port: {} @ {}",
            self.port_name, self.baud_rate
        );

        let port = serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_millis(100))
            .open()?;

        info!(
            "Connected to {}. Listening for D16 Telemetry...",
            self.port_name
        );

        let mut reader = BufReader::new(port);
        let mut buffer = String::new();
        let mut age_seconds = 0;

        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        let clean_line = buffer.trim();
                        if !clean_line.is_empty() {
                            debug!("[RX]: {}", clean_line);

                            // Basic Resonance Analysis on the raw bytes of the log line
                            match self.process_line(clean_line, age_seconds) {
                                Ok(report) => on_report(report),
                                Err(_) => {} // Ignore parse errors / noise
                            }
                            age_seconds += 1;
                        }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    continue;
                }
                Err(e) => {
                    error!("Error reading from serial port: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    fn process_line(&self, line: &str, age: u64) -> Result<ResonanceReport, ()> {
        // Simple mass calculation: length of the log line
        let mass = line.len();
        if mass == 0 {
            return Err(());
        }

        // Analyze
        let report = ResonanceAnalyzer::analyze(line.as_bytes(), mass, age);
        Ok(report)
    }
}
