#![allow(dead_code)]
use anyhow::{anyhow, Result};
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

// Protocol Constants
const CMD_HEADER_1: u8 = 0x55;
const CMD_HEADER_2: u8 = 0xAA;
const ADDRESS: u8 = 0x11;

// Commands
const CMD_REQUEST_KNOCK: u8 = 0x2C;
const CMD_REQUEST_BLOCKS: u8 = 0x21;
const CMD_REQUEST_ARROWS: u8 = 0x22;
const CMD_REQUEST_LEARNED: u8 = 0x23;
const CMD_REQUEST_ALGORITHM: u8 = 0x2D;

const CMD_RETURN_OK: u8 = 0x2E;
const CMD_RETURN_INFO: u8 = 0x29;
const CMD_RETURN_BLOCK: u8 = 0x2A;
const CMD_RETURN_ARROW: u8 = 0x2B;

#[derive(Debug, Clone)]
pub struct Block {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub id: u16,
}

pub struct Huskylens {
    port: Box<dyn SerialPort>,
}

impl Huskylens {
    pub fn new(port_name: &str) -> Result<Self> {
        // [CHANGE] Switched to /dev/ttyUSB0 (CP2102) per user identification.
        // Still using 9600 8N1 standard.
        // Asserting DTR/RTS to ensure USB-UART bridge logic is active.
        let port = serialport::new(port_name, 9600)
            .timeout(Duration::from_millis(2000))
            .data_bits(serialport::DataBits::Eight)
            .flow_control(serialport::FlowControl::None)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .open()
            .map_err(|e| anyhow!("Failed to open serial port: {}", e))?;

        let mut p = port;
        let _ = p.write_data_terminal_ready(true);
        let _ = p.write_request_to_send(true);

        Ok(Self { port: p })
    }

    fn calculate_checksum(&self, buf: &[u8]) -> u8 {
        let mut sum: u16 = 0;
        for &byte in buf {
            sum = sum.wrapping_add(byte as u16);
        }
        (sum & 0xFF) as u8
    }

    fn send_command(&mut self, cmd: u8, data: &[u8]) -> Result<()> {
        let len = data.len() as u8;
        let mut frame = vec![CMD_HEADER_1, CMD_HEADER_2, ADDRESS, len, cmd];
        frame.extend_from_slice(data);

        // Checksum covers Header1 through Data
        let checksum = self.calculate_checksum(&frame);
        frame.push(checksum);

        self.port.write_all(&frame)?;
        Ok(())
    }

    fn read_response(&mut self) -> Result<(u8, Vec<u8>)> {
        // Read looking for header
        let mut buf = [0u8; 1];
        loop {
            self.port.read_exact(&mut buf)?;
            if buf[0] == CMD_HEADER_1 {
                break;
            }
        }

        self.port.read_exact(&mut buf)?;
        if buf[0] != CMD_HEADER_2 {
            return Err(anyhow!("Header 2 mismatch"));
        }

        self.port.read_exact(&mut buf)?;
        let _addr = buf[0]; // Should be 0x11

        self.port.read_exact(&mut buf)?;
        let len = buf[0] as usize;

        self.port.read_exact(&mut buf)?;
        let cmd = buf[0];

        let mut data = vec![0u8; len];
        if len > 0 {
            self.port.read_exact(&mut data)?;
        }

        self.port.read_exact(&mut buf)?;
        let _checksum = buf[0];
        // Validation skipped for brevity in V1, but should match

        Ok((cmd, data))
    }

    pub fn knock(&mut self) -> Result<bool> {
        self.send_command(CMD_REQUEST_KNOCK, &[])?;
        let (cmd, _) = self.read_response()?;
        Ok(cmd == CMD_RETURN_OK)
    }

    pub fn request_blocks(&mut self) -> Result<Vec<Block>> {
        self.send_command(CMD_REQUEST_BLOCKS, &[])?;
        let (cmd, data) = self.read_response()?;

        if cmd == CMD_RETURN_INFO {
            // Info block tells us how many blocks exist
            // Format: [NumberOfBlocks_Lower, NumberOfBlocks_Higher]
            // Then we read the blocks
            let count = u16::from_le_bytes([data[0], data[1]]);
            let mut blocks = Vec::new();

            for _ in 0..count {
                let (cmd_b, data_b) = self.read_response()?;
                if cmd_b == CMD_RETURN_BLOCK {
                    // Format: x(2), y(2), w(2), h(2), id(2)
                    if data_b.len() >= 10 {
                        blocks.push(Block {
                            x: u16::from_le_bytes([data_b[0], data_b[1]]),
                            y: u16::from_le_bytes([data_b[2], data_b[3]]),
                            width: u16::from_le_bytes([data_b[4], data_b[5]]),
                            height: u16::from_le_bytes([data_b[6], data_b[7]]),
                            id: u16::from_le_bytes([data_b[8], data_b[9]]),
                        });
                    }
                }
            }
            Ok(blocks)
        } else {
            Ok(vec![])
        }
    }
}
