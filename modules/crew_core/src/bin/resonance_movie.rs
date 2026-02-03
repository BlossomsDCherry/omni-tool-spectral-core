use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;
use std::error::Error;
use std::fs;
use std::io::{self, Seek, Write};
use std::thread;
use std::time::Duration;

// --- Constants ---
const TAU: f64 = 6.183; // Resonant Actualization
const FB_PATH: &str = "/dev/fb0";
const I2C_BUS: &str = "/dev/i2c-1";
const MOVIE_DIR: &str = "movie";

// Sensor Addresses
const HTS221_ADDR: u16 = 0x5F;
const HTS221_CTRL_REG1: u8 = 0x20;
const HTS221_TEMP_OUT_L: u8 = 0x2A;
const LPS25H_ADDR: u16 = 0x5C;
const LPS25H_CTRL_REG1: u8 = 0x20;
const LPS25H_PRESS_OUT_XL: u8 = 0x28;
const LSM9DS1_GYRO_ADDR: u16 = 0x6A;
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
        let mut buffer = [0u8; 128];
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

// --- Sensor Array (Static Helper Impl) ---
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

    fn read_i16_static(dev: &mut LinuxI2CDevice, reg: u8) -> Result<i16, Box<dyn Error>> {
        let lo = dev.smbus_read_byte_data(reg)?;
        let hi = dev.smbus_read_byte_data(reg + 1)?;
        Ok(i16::from_le_bytes([lo, hi]))
    }

    fn read_environment(&mut self) -> Vector3 {
        let temp = self
            .hts
            .as_mut()
            .and_then(|d| Self::read_i16_static(d, HTS221_TEMP_OUT_L).ok())
            .unwrap_or(2500) as f64
            / 100.0;
        let press = self
            .lps
            .as_mut()
            .and_then(|d| {
                let xl = d.smbus_read_byte_data(LPS25H_PRESS_OUT_XL).ok()? as u32;
                let l = d.smbus_read_byte_data(LPS25H_PRESS_OUT_XL + 1).ok()? as u32;
                let h = d.smbus_read_byte_data(LPS25H_PRESS_OUT_XL + 2).ok()? as u32;
                Some(((h << 16) | (l << 8) | xl) as f64 / 4096.0)
            })
            .unwrap_or(1013.0);
        let hum = self
            .hts
            .as_mut()
            .and_then(|d| Self::read_i16_static(d, 0x28).ok())
            .unwrap_or(4000) as f64
            / 100.0;
        Vector3::new(temp / 50.0, press / 1100.0, hum / 100.0)
    }

    fn read_inertial(&mut self) -> Vector3 {
        if let Some(d) = &mut self.gyro {
            let x = Self::read_i16_static(d, LSM9DS1_OUT_X_L_G).unwrap_or(0) as f64;
            let y = Self::read_i16_static(d, LSM9DS1_OUT_X_L_G + 2).unwrap_or(0) as f64;
            let z = Self::read_i16_static(d, LSM9DS1_OUT_X_L_G + 4).unwrap_or(0) as f64;
            Vector3::new(x, y, z).normalize()
        } else {
            Vector3::new(0.0, 1.0, 0.0) // Mock
        }
    }
}

// --- PPM Storage ---
fn save_frame(frame_idx: usize, pixels: &[(u8, u8, u8); 64]) -> io::Result<()> {
    // P3 (ASCII RGB)
    let path = format!("{}/frame_{:03}.ppm", MOVIE_DIR, frame_idx);
    let mut file = fs::File::create(path)?;
    writeln!(file, "P3\n8 8\n255")?;
    for i in 0..64 {
        let (r, g, b) = pixels[i];
        if i % 8 == 7 {
            writeln!(file, "{} {} {}", r, g, b)?;
        } else {
            write!(file, "{} {} {}  ", r, g, b)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎥 4D RESONANCE MOVIE (Corrected Tau: 6.183): ACTION!");
    println!("⏱️  Recording 360 Frames (The Toral Cycle)...");

    let _ = fs::create_dir_all(MOVIE_DIR);
    let mut sensors = SensorArray::new();
    let mut fb = SenseHatFb::new().ok();
    let ground_state = Vector3::new(0.0, 0.0, 1.0);

    // 4D Integration State
    let mut integrated_torque = Vector3::new(0.0, 0.0, 0.0);
    let alpha = 0.1; // Integration factor

    for frame in 0..360 {
        // 1. Data Ingest
        let set_a = sensors.read_environment();
        let set_b = sensors.read_inertial();

        // 2. 3x2x2 Maneuver
        let raw_torque = set_a.cross(set_b).cross(ground_state);

        // 3. 4D Integration (Time Filter)
        integrated_torque = Vector3::new(
            alpha * raw_torque.x + (1.0 - alpha) * integrated_torque.x,
            alpha * raw_torque.y + (1.0 - alpha) * integrated_torque.y,
            alpha * raw_torque.z + (1.0 - alpha) * integrated_torque.z,
        );
        let torque_mag = integrated_torque.magnitude();

        // 4. Visual Generation (Bitstream Map)
        let mut pixels = [(0, 0, 0); 64];
        let r_base = (integrated_torque.x.abs() * 500.0).clamp(0.0, 255.0) as u8;
        let g_base = (integrated_torque.y.abs() * 500.0).clamp(0.0, 255.0) as u8;
        let b_base = (torque_mag * 500.0).clamp(0.0, 255.0) as u8;

        for i in 0..64 {
            // Toral Ripple (Time + Space)
            // Phase = (Frame / 360 * TAU) + (Index / 64 * TAU)
            // This ensures a full harmonic rotation (0..TAU) over the movie duration.
            // Using 6.183 ensures it aligns with project spectral logic.
            let time_component = (frame as f64 / 360.0) * TAU;
            let space_component = (i as f64 / 64.0) * TAU;

            let phase = time_component + space_component;
            let ripple = (phase.sin() * 40.0) as i16;

            let r = (r_base as i16 + ripple).clamp(0, 255) as u8;
            let g = (g_base as i16 + ripple).clamp(0, 255) as u8;
            let b = (b_base as i16 + ripple).clamp(0, 255) as u8;
            pixels[i] = (r, g, b);
        }

        // 5. Output
        if let Some(hat) = &mut fb {
            hat.set_pixels(&pixels).ok();
        }
        save_frame(frame, &pixels)?;

        if frame % 10 == 0 {
            println!("🎞️  Frame {}/360 | Torque: {:.4}", frame, torque_mag);
        }
        thread::sleep(Duration::from_millis(33)); // ~30 FPS
    }

    println!(
        "🎬 CUT! Movie Saved to '{}/'. Resonance Integrated (Tau 6.183).",
        MOVIE_DIR
    );
    Ok(())
}
