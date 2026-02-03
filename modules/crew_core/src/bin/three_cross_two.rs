use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;
use std::error::Error;
use std::fs;
use std::io::{self, Seek, Write};
use std::thread;
use std::time::Duration;

// --- Constants & Sensor Addresses ---
const FB_PATH: &str = "/dev/fb0";
const I2C_BUS: &str = "/dev/i2c-1";

// HTS221 (Humidity/Temp)
const HTS221_ADDR: u16 = 0x5F;
const HTS221_CTRL_REG1: u8 = 0x20;
const HTS221_TEMP_OUT_L: u8 = 0x2A;

// LPS25H (Pressure)
const LPS25H_ADDR: u16 = 0x5C;
const LPS25H_CTRL_REG1: u8 = 0x20;
const LPS25H_PRESS_OUT_XL: u8 = 0x28;

// LSM9DS1 (IMU - Accel/Gyro/Mag)
const LSM9DS1_GYRO_ADDR: u16 = 0x6A; // Gyro/Accel
const LSM9DS1_CTRL_REG1_G: u8 = 0x10;
const LSM9DS1_OUT_X_L_G: u8 = 0x18;

// --- Vector Struct ---
#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn cross(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Vector3 {
        let m = self.magnitude();
        if m == 0.0 {
            Vector3::new(0.0, 0.0, 0.0)
        } else {
            Vector3::new(self.x / m, self.y / m, self.z / m)
        }
    }
}

// --- Framebuffer Driver ---
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
}

// --- Sensor Drivers ---
struct SensorArray {
    hts: Option<LinuxI2CDevice>,
    lps: Option<LinuxI2CDevice>,
    gyro: Option<LinuxI2CDevice>,
}

impl SensorArray {
    fn new() -> Self {
        let hts = LinuxI2CDevice::new(I2C_BUS, HTS221_ADDR).ok();
        let lps = LinuxI2CDevice::new(I2C_BUS, LPS25H_ADDR).ok();
        let gyro = LinuxI2CDevice::new(I2C_BUS, LSM9DS1_GYRO_ADDR).ok();

        let mut sensors = Self { hts, lps, gyro };
        sensors.init();
        sensors
    }

    fn init(&mut self) {
        if let Some(dev) = &mut self.hts {
            let _ = dev.smbus_write_byte_data(HTS221_CTRL_REG1, 0x80);
        }
        if let Some(dev) = &mut self.lps {
            let _ = dev.smbus_write_byte_data(LPS25H_CTRL_REG1, 0x90);
        }
        if let Some(dev) = &mut self.gyro {
            let _ = dev.smbus_write_byte_data(LSM9DS1_CTRL_REG1_G, 0x20);
        }
    }

    // Static helper to avoid borrow conflict
    fn read_i16_static(dev: &mut LinuxI2CDevice, reg: u8) -> Result<i16, Box<dyn Error>> {
        let lo = dev.smbus_read_byte_data(reg)?;
        let hi = dev.smbus_read_byte_data(reg + 1)?;
        Ok(i16::from_le_bytes([lo, hi]))
    }

    fn read_environment(&mut self) -> Vector3 {
        let temp = self.read_temp().unwrap_or(25.0);
        let press = self.read_pressure().unwrap_or(1013.0);
        let hum = self.read_humidity().unwrap_or(40.0);
        Vector3::new(temp / 50.0, press / 1100.0, hum / 100.0)
    }

    fn read_inertial(&mut self) -> Vector3 {
        if let Some(dev) = &mut self.gyro {
            let x = Self::read_i16_static(dev, LSM9DS1_OUT_X_L_G).unwrap_or(0) as f64;
            let y = Self::read_i16_static(dev, LSM9DS1_OUT_X_L_G + 2).unwrap_or(0) as f64;
            let z = Self::read_i16_static(dev, LSM9DS1_OUT_X_L_G + 4).unwrap_or(0) as f64;
            Vector3::new(x, y, z).normalize()
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        }
    }

    fn read_temp(&mut self) -> Option<f64> {
        let dev = self.hts.as_mut()?;
        let t_out = Self::read_i16_static(dev, HTS221_TEMP_OUT_L).ok()?;
        Some(t_out as f64 / 100.0)
    }

    fn read_pressure(&mut self) -> Option<f64> {
        let dev = self.lps.as_mut()?;
        let xl = dev.smbus_read_byte_data(LPS25H_PRESS_OUT_XL).ok()? as u32;
        let l = dev.smbus_read_byte_data(LPS25H_PRESS_OUT_XL + 1).ok()? as u32;
        let h = dev.smbus_read_byte_data(LPS25H_PRESS_OUT_XL + 2).ok()? as u32;
        let p_raw = (h << 16) | (l << 8) | xl;
        Some(p_raw as f64 / 4096.0)
    }

    fn read_humidity(&mut self) -> Option<f64> {
        let dev = self.hts.as_mut()?;
        let h_out = Self::read_i16_static(dev, 0x28).ok()?;
        Some(h_out as f64 / 100.0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🌪️ 3x2x2 Maneuver Initiated...");
    println!("💎 Connecting to Hardware Substrate...");

    let mut sensors = SensorArray::new();
    let mut fb = SenseHatFb::new().ok();
    let ground_state = Vector3::new(0.0, 0.0, 1.0);

    println!("✅ Sensors Active. Beginning Loop...");

    loop {
        let set_a = sensors.read_environment();
        let set_b = sensors.read_inertial();

        let interaction_vector = set_a.cross(set_b);
        let torque_vector = interaction_vector.cross(ground_state);
        let torque_mag = torque_vector.magnitude();

        if let Some(hat) = &mut fb {
            let mut pixels = [(0, 0, 0); 64];

            let r_base = (torque_vector.x.abs() * 255.0).clamp(0.0, 255.0) as u8;
            let g_base = (torque_vector.y.abs() * 255.0).clamp(0.0, 255.0) as u8;
            let b_base = (torque_mag * 255.0).clamp(0.0, 255.0) as u8;

            for i in 0..64 {
                // Time-based ripple
                let t = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64;
                let ripple = ((i as f64 / 8.0 + t / 200.0).sin() * 50.0) as i16;
                let r = (r_base as i16 + ripple).clamp(0, 255) as u8;
                let g = (g_base as i16 + ripple).clamp(0, 255) as u8;
                pixels[i] = (r, g, b_base);
            }
            hat.set_pixels(&pixels).ok();
        }

        thread::sleep(Duration::from_millis(50));
    }
}
