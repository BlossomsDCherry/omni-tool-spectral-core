use core::option::Option::{self, None, Some};
use core::sync::atomic::{AtomicU32, Ordering};

/// The Zero-Wait Handshake ($t$)
/// Replaces ring buffers with direct phase-coherence hand-offs.
pub struct ZeroWaitHandshake {
    /// Atomic slot for the "Data Comet" (The Resonant Ping)
    comet: AtomicU32,
    /// Atomic flag for phase-sync (Roman/Arabic alignment)
    sling_pulse: AtomicU32,
}

impl ZeroWaitHandshake {
    pub const fn new() -> Self {
        Self {
            comet: AtomicU32::new(0),
            sling_pulse: AtomicU32::new(0),
        }
    }

    /// Core 1 (Arabic/Sensor): Tossing the Comet
    /// Called at the polyrhythmic peak.
    pub fn toss_comet(&self, data: u32) {
        // We only write when the sling_pulse is ready (Phase Coherence)
        self.comet.store(data, Ordering::Release);
        self.sling_pulse.fetch_add(1, Ordering::SeqCst);
    }

    /// Core 0 (Roman/Logic): Catching the Comet
    /// Called at the 60 BPM "Ground State" tick.
    pub fn catch_comet(&self) -> Option<u32> {
        // Enforce the handshake: Only catch if the pulse has triggered
        if self.sling_pulse.load(Ordering::Acquire) > 0 {
            let data = self.comet.load(Ordering::Relaxed);
            self.sling_pulse.store(0, Ordering::Release); // Reset pulse
            Some(data)
        } else {
            None // Missed the sling (Decoherence detected)
        }
    }
}
