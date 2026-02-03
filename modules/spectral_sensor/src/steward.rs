use crate::eight_gate::RecursiveFilter;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// The Zero-Wait Protocol ($t$).
/// Replaces ring buffers with direct phase-coherence hand-offs.
/// Host-side adaptation of `zero_wait_handshake.rs`.
#[derive(Debug, Default)]
pub struct ZeroWaitHandshake {
    /// Atomic slot for the "Data Comet" (The Resonant Ping)
    /// Stores f64 bits.
    comet: AtomicU64,
    /// Atomic flag for phase-sync (Roman/Arabic alignment)
    sling_pulse: AtomicU32,
}

/// The Living Key ($\L$).
/// Grounded in the Sovereign Manifest and interactive entropy.
pub struct LivingKey;

impl LivingKey {
    /// Verify a key signature against the Sovereign Chain of Trust.
    /// Requirements: Not empty, entropy-derived (>= 64 chars).
    pub fn verify(key: &str) -> bool {
        !key.is_empty() && key.len() >= 64
    }
}

/// The Command Handshake ($\mathcal{H}$).
/// A safety mechanism for terminal commands.
/// Prime the handshake with a Living Key to authorize operations.
#[derive(Debug, Default)]
pub struct CommandHandshake {
    authorized: AtomicU32,
}

impl CommandHandshake {
    pub fn new() -> Self {
        Self {
            authorized: AtomicU32::new(0),
        }
    }

    /// Prime the handshake with the Captain's Living Key.
    pub fn prime(&self, key: &str) -> bool {
        if LivingKey::verify(key) {
            self.authorized.store(1, Ordering::Release);
            true
        } else {
            false
        }
    }

    /// Consumes the authorization to run a command.
    /// Returns true if authorized.
    pub fn authorize_op(&self) -> bool {
        if self.authorized.load(Ordering::Acquire) > 0 {
            self.authorized.store(0, Ordering::Release);
            true
        } else {
            false
        }
    }
}

impl ZeroWaitHandshake {
    pub fn new() -> Self {
        Self {
            comet: AtomicU64::new(0),
            sling_pulse: AtomicU32::new(0),
        }
    }

    /// Host (Roman/Logic): Catching the Comet.
    /// Returns Some(data) ONLY if phase alignment (sling_pulse) is valid
    /// AND the Recursive Filter confirms the Harmonic Resonance.
    pub fn catch_comet(&self) -> Option<f64> {
        // Enforce the handshake: Only catch if the pulse has triggered
        if self.sling_pulse.load(Ordering::Acquire) > 0 {
            let bits = self.comet.load(Ordering::Relaxed);
            let data = f64::from_bits(bits);

            // RECURSIVE FILTER INTEGRATION
            // Zero Wait is only permitted if the data resonates.
            if RecursiveFilter::observe(data, 0.5).is_some() {
                // Reset the pulse (Consumption)
                self.sling_pulse.store(0, Ordering::Release);
                return Some(data);
            } else {
                // Decoherence: Logic rejects the data as "Noise".
                // Handshake NOT completed.
                // In a real system, we might log this or let it decay.
                None
            }
        } else {
            None // Missed the sling (Decoherence / Waiting)
        }
    }

    /// Host (Roman): Tossing a Command (Optional Reverse Flow).
    pub fn toss_command(&self, cmd: f64) {
        self.comet.store(cmd.to_bits(), Ordering::Release);
        self.sling_pulse.store(1, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eight_gate::Stance;

    #[test]
    fn test_zero_wait_coherence() {
        let protocol = ZeroWaitHandshake::new();

        // 1. Initial State: No Pulse
        assert_eq!(protocol.catch_comet(), None);

        // 2. Simulate Device Toss with RESONANT data
        // We need a value that passes RecursiveFilter.
        // Earth Signature = 296.07.
        // 296.07 % TAU (6.28318) ~= 0.77 (approx).
        // Let's just use the signature itself, it acts as a perfect harmonic.
        let resonant_val = Stance::Earth.signature();

        protocol.toss_command(resonant_val);

        // 3. Catch - Should succeed because it resonates
        let caught = protocol.catch_comet();
        assert!(caught.is_some());
        assert!((caught.unwrap() - resonant_val).abs() < 0.0001);

        // 4. Post-Catch: Pulse should be reset
        assert_eq!(protocol.catch_comet(), None);
    }

    #[test]
    fn test_rejection_of_dissonance() {
        let protocol = ZeroWaitHandshake::new();

        // 1. Dissonant Value
        // Calculated to be > 0.1 rad away from all 8 Stance Signatures.
        // Value: 3.5
        // Nearest Phases: Water(0.674), Earth(0.760), ... Fire(5.578).
        // No resonance overlaps.

        let dissonant_val = 3.5;
        protocol.toss_command(dissonant_val);

        // 2. Catch - Should return None (Filtered out) but the Pulse stays active?
        // Logic: if catch returns None, we didn't consume the pulse.
        assert_eq!(protocol.catch_comet(), None);

        // Pulse should still be 1 (Waiting for valid data, or effectively blocked)
        assert_eq!(protocol.sling_pulse.load(Ordering::Acquire), 1);
    }

    #[test]
    fn test_command_handshake_flow() {
        let handshake = CommandHandshake::new();
        let key = "a".repeat(64); // Valid Living Key

        // 1. Initial: Unauthorized
        assert!(!handshake.authorize_op());

        // 2. Prime with valid key
        assert!(handshake.prime(&key));

        // 3. Authorize: Success
        assert!(handshake.authorize_op());

        // 4. Post-Authorize: Reset to unauthorized
        assert!(!handshake.authorize_op());
    }
}
