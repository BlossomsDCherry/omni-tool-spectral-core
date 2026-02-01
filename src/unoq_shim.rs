// Copyright 2026 Pecos D. Willy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Uno Q Shim: Interfacing with the QRB2210 + STM32U585 board (10.0.0.1).

use std::io::{Read, Write};
use std::net::TcpStream;

pub const UNOQ_IP: &str = "10.0.0.1";
pub const UNOQ_AGENT_PORT: u16 = 21515;

pub struct UnoQShim {
    pub target_ip: String,
    pub port: u16,
}

impl UnoQShim {
    pub fn new() -> Self {
        Self {
            target_ip: UNOQ_IP.to_string(),
            port: UNOQ_AGENT_PORT,
        }
    }

    /// Establish a spectral link with the Uno Q Agentic OS.
    pub fn connect_agent(&self) -> std::io::Result<TcpStream> {
        println!(
            "📡 [UNOQ] Linking to Agent at {}:{}...",
            self.target_ip, self.port
        );
        TcpStream::connect(format!("{}:{}", self.target_ip, self.port))
    }

    /// Deploy D16 payload to the Uno Q.
    pub fn deploy_payload(&self, payload: &str) -> std::io::Result<()> {
        let mut stream = self.connect_agent()?;
        println!("🚀 [UNOQ] Deploying D16 Payload...");
        stream.write_all(payload.as_bytes())?;

        // Wait for acknowledgment
        let mut buffer = [0; 128];
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let response = String::from_utf8_lossy(&buffer[..n]);
                println!("✅ [UNOQ] Agent Response: {}", response.trim());
            }
            _ => {
                println!("⚠️ [UNOQ] No immediate ACK. Payload streaming...");
            }
        }
        Ok(())
    }

    /// Synchronize the Uno Q to the Atomic Precision Layer.
    pub fn sync_precision(&self) {
        println!("⚖️ [UNOQ] Synchronizing to 8-sig-fig Atomic Precision...");
        let sync_msg = "COMMAND:SYNC_PRECISION:TAU=6.2831853";
        let _ = self.deploy_payload(sync_msg);
    }
}
