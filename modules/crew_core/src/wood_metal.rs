use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use chrono;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use std::fs;
use std::path::Path;

/// Wood-Metal Cryptography: Rust Port
/// "Oxidation States for Autonomous Stability"

const ITERATIONS: u32 = 100_000;

pub struct WoodMetal;

impl WoodMetal {
    /// Derives the Resonant Seed from foundational constants.
    /// Anchors: TAU, PI, PHI, E
    pub fn get_resonant_seed() -> Vec<u8> {
        // In a real implementation, we'd pull these from a shared config or local file.
        // For this failover, we'll anchor to the constants defined in lib.rs.
        use crate::Talu64;
        format!(
            "{}|{}|{}|{}",
            Talu64::TAU,
            Talu64::PI,
            Talu64::PHI,
            Talu64::E
        )
        .into_bytes()
    }

    /// Derives a 256-bit key using PBKDF2-SHA256.
    pub fn derive_key(seed: &[u8], salt: &[u8; 16]) -> [u8; 32] {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(seed, salt, ITERATIONS, &mut key);
        key
    }

    /// Stabilizes "Wood" into "Metal" (Encryption).
    pub fn stabilize(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let plaintext = fs::read(path)?;
        let seed = Self::get_resonant_seed();

        // Generate random Salt and Nonce
        use rand::{thread_rng, RngCore};
        let mut salt = [0u8; 16];
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut salt);
        thread_rng().fill_bytes(&mut nonce_bytes);

        let key_bytes = Self::derive_key(&seed, &salt);
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| format!("Encryption failure: {}", e))?;

        // Payload: SALT(16) | NONCE(12) | CIPHERTEXT
        let mut payload = Vec::with_capacity(16 + 12 + ciphertext.len());
        payload.extend_from_slice(&salt);
        payload.extend_from_slice(&nonce_bytes);
        payload.extend_from_slice(&ciphertext);

        let metal_path = path.with_extension("metal");
        fs::write(&metal_path, payload)?;
        fs::remove_file(path)?;

        println!(
            "✨ [STABILIZED] {} -> {}",
            path.display(),
            metal_path.display()
        );
        Ok(())
    }

    /// Restores "Metal" back into "Wood" (Decryption).
    pub fn restore(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let payload = fs::read(path)?;
        if payload.len() < 28 {
            return Err("Payload too short".into());
        }

        let seed = Self::get_resonant_seed();
        let salt: [u8; 16] = payload[0..16].try_into()?;
        let nonce_bytes: [u8; 12] = payload[16..28].try_into()?;
        let ciphertext = &payload[28..];

        let key_bytes = Self::derive_key(&seed, &salt);
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| format!("Decryption failure: {}", e))?;

        let wood_path = path.with_extension("");
        fs::write(&wood_path, plaintext)?;
        fs::remove_file(path)?;

        println!("🍃 [GROWTH] {} -> {}", path.display(), wood_path.display());
        Ok(())
    }

    /// Oxidation Check: Flagging Spooky Relativity
    pub fn check_oxidation(path: &Path, half_life_days: i64) {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                let modified: chrono::DateTime<chrono::Local> = modified.into();
                let now = chrono::Local::now();
                let age = now.signed_duration_since(modified).num_days();

                if age > half_life_days {
                    println!(
                        "👻 [SPOOKY RELATIVITY] Metal at {} is oxidizing (Age: {} days).",
                        path.display(),
                        age
                    );
                }
            }
        }
    }
}
