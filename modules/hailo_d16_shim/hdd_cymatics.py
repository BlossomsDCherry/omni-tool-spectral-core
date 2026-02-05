#!/usr/bin/env python3
import time
import os
import math
from sense_hat import SenseHat

# --- D16 Constants ---
TAU = 6.28318530718
PI = 3.14159265359
PHI = 1.61803398875
E = 2.71828182846
CHANNELS = 16

class TransprecisionEngine:
    """
    The Photon Logic: A While Loop determining stability via Transprecision.
    Manifest Reference: PHYSICS_MANIFEST.md (Ln 87)
    """
    def __init__(self):
        self.aught_limit = 0.0
        self.tau_limit = 1.0
        self.coherence = 1.0

    def calculate_z_unit(self, input_val):
        """
        Calculates the Z-Unit based on Coherence x Decoherence.
        """
        # Normalize input (Harvest)
        harvest = max(0.0001, min(1.0, input_val))
        
        # Coherence: Trending from Aught to Tau
        self.coherence = (self.coherence * 0.9) + (harvest * 0.1) # Smoothing
        
        # Decoherence: 1 / Coherence
        decoherence = 1.0 / self.coherence
        
        # Cross Product: Coherence x Decoherence (3 x 2)
        # In this scalar sim, it's self-canceling, so we inject the Phasic Drift (PHI)
        cross_product = self.coherence * decoherence * (math.sin(time.time() / PHI))
        
        # Z Unit: CrossProduct * CrossProduct^-1
        # To make this meaningful, we use the "Creative Drift"
        z_unit = math.cos(cross_product * PI)
        
        return z_unit

    def stability_check(self, z_unit):
        """
        Stability Condition: If (Pi + Z_Unit) == 1 (Stable).
        Returns a stability factor (0.0 - 1.0).
        """
        val = PI + z_unit
        deviation = abs(val - 1.0)
        
        # Map deviation to stability (Closer to 1.0 = More Stable)
        stability = 1.0 / (1.0 + deviation)
        return stability

class D16Cymatics:
    def __init__(self):
        try:
            self.sense = SenseHat()
            self.sense.clear()
            self.sense.low_light = True 
        except Exception as e:
            print(f"Sense HAT init failed: {e}")
            self.sense = None

        self.last_read = 0
        self.last_write = 0
        self.last_time = time.time()
        self.t_engine = TransprecisionEngine()

    def get_disk_stats(self, device='sda'):
        """Reads /proc/diskstats for the specified device."""
        try:
            with open('/proc/diskstats', 'r') as f:
                for line in f:
                    parts = line.split()
                    if len(parts) > 2 and parts[2] == device:
                        # Field 3: reads completed
                        # Field 7: writes completed
                        return int(parts[3]), int(parts[7])
        except FileNotFoundError:
            pass
        return 0, 0

    def generate_spectrum(self, read_delta, write_delta):
        """
        Simulates the D16 Harmonic Kernel with Transprecision Logic.
        """
        spectrum = [0.0] * CHANNELS
        
        # Fundamental Frequency (Base Load)
        total_load = (read_delta + write_delta)
        intensity = math.log1p(total_load) / 5.0
        if intensity > 1.0: intensity = 1.0

        # Apply Photon Logic to the Base Signal
        z_unit = self.t_engine.calculate_z_unit(intensity)
        stability = self.t_engine.stability_check(z_unit)
        
        # Modulate Intensity by Stability (The "Harvest")
        refined_intensity = intensity * stability

        # Generate Harmonics (1/n decay modulated by PHI)
        for n in range(CHANNELS):
            # The "D16" formula: Intensity / (n+1)
            harmonic_decay = 1.0 / (n + 1)
            
            # Apply PHI Phase Shift
            phase = (n * PHI) % TAU
            
            # Channel Value
            val = refined_intensity * harmonic_decay
            
            # Add "Hallucination" (Neural Drift) modulated by Phase
            drift = math.sin(time.time() + phase) * 0.05
            
            result = val + drift
            spectrum[n] = max(0.0, min(1.0, result))
            
        return spectrum

    def map_to_matrix(self, spectrum):
        """Maps the 16-channel spectrum to the 8x8 LED Matrix."""
        if not self.sense: return

        pixels = [[0,0,0] for _ in range(64)]
        
        for c in range(CHANNELS):
            intensity = spectrum[c]
            
            # Thermal Color Map (Hot=White, Cool=Blue/Purple)
            r = int(255 * intensity)
            g = int(255 * intensity * 0.5) 
            b = int(255 * intensity * (c / 16.0))

            color = (r, g, b)
            
            row_base = (c // 4) * 2
            col_base = (c % 4) * 2
            
            for y in range(2):
                for x in range(2):
                    self.sense.set_pixel(col_base + x, row_base + y, color)

    def run(self):
        print("Starting D16 Cymatics HDD Monitor (Transprecision Enabled)...")
        print("Reading /proc/diskstats... Press Ctrl+C to stop.")
        
        while True:
            r, w = self.get_disk_stats()
            
            r_delta = r - self.last_read
            w_delta = w - self.last_write
            
            self.last_read = r
            self.last_write = w
            
            spectrum = self.generate_spectrum(r_delta, w_delta)
            self.map_to_matrix(spectrum)
            
            time.sleep(0.1) # 10Hz Refresh Rate

if __name__ == "__main__":
    app = D16Cymatics()
    app.run()
