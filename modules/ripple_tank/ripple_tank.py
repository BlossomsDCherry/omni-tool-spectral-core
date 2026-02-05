#!/usr/bin/env python3
import numpy as np
import time
import sys
import os
import mmap

# --- Constants ---
GRID_SIZE = 64
DAMPING = 0.99
C = 0.5  # Wave speed
DT = 0.1
DX = 1.0

# --- Shared Memory ---
SHM_PATH = "/dev/shm/current_wave_coherence"

class RippleTank:
    def __init__(self, size=GRID_SIZE):
        self.size = size
        self.u = np.zeros((size, size))      # Current state
        self.u_prev = np.zeros((size, size)) # Previous state
        self.u_next = np.zeros((size, size)) # Next state
        self.frame = 0

    def update(self):
        """
        Discrete Laplacian for 2D Wave Equation:
        d2u/dt2 = c^2 * (d2u/dx2 + d2u/dy2)
        """
        # Laplacian kernel [0, 1, 0; 1, -4, 1; 0, 1, 0]
        laplacian = (
            np.roll(self.u, 1, axis=0) +
            np.roll(self.u, -1, axis=0) +
            np.roll(self.u, 1, axis=1) +
            np.roll(self.u, -1, axis=1) -
            4 * self.u
        )
        
        # Wave Equation Update
        self.u_next = (2 * self.u) - self.u_prev + (C**2) * (DT**2 / DX**2) * laplacian
        
        # Damping
        self.u_next *= DAMPING
        
        # Shift buffers
        self.u_prev = self.u.copy()
        self.u = self.u_next.copy()
        
        self.frame += 1

    def inject_source(self, x, y, frequency, amplitude=1.0):
        """Oscillating point source."""
        val = amplitude * np.sin(self.frame * frequency)
        self.u[x, y] = val

    def calculate_coherence(self):
        """
        Metric: Spatial Coherence / Symmetry.
        Simple proxy: Ratio of energy in center vs edges (Focus).
        """
        energy = np.sum(np.abs(self.u))
        if energy == 0: return 0.0
        
        center_energy = np.sum(np.abs(self.u[28:36, 28:36]))
        return (center_energy / energy) * 10.0 # Scale up

    def render_ascii(self):
        """Renders grid to terminal using ASCII chars."""
        chars = " .:-=+*#%@"
        os.system('clear')
        print(f"--- RIPPLE TANK SIMULATION Frame: {self.frame} ---")
        
        # Downsample for terminal
        view = self.u[::2, ::2]
        
        for row in view:
            line = ""
            for val in row:
                idx = int((val + 1.0) * 4) # Map ~-1..1 to 0..8
                idx = max(0, min(len(chars)-1, idx))
                line += chars[idx]
            print(line)

    def write_shm(self, coherence):
        try:
            with open(SHM_PATH, "w") as f:
                f.write(f"{coherence:.4f}")
        except Exception as e:
            pass

    def run(self):
        print("Initializing Ripple Tank...")
        try:
            while True:
                # 1. Sim Physics
                self.update()
                
                # 2. Inject Sources (Simulating "Crew" Inputs)
                # Source A (Robin): Center
                self.inject_source(32, 32, frequency=0.2, amplitude=2.0)
                
                # Source B (Interference): Offset
                # Logic: Interference creates "texture"
                self.inject_source(16, 16, frequency=0.3, amplitude=1.0)

                # 3. Calculate metrics
                coherence = self.calculate_coherence()
                self.write_shm(coherence)
                
                # 4. Render
                if self.frame % 2 == 0:
                    self.render_ascii()
                    print(f"Coherence Metric: {coherence:.4f}")
                
                time.sleep(0.05)
                
        except KeyboardInterrupt:
            print("\nTank drained.")

if __name__ == "__main__":
    tank = RippleTank()
    tank.run()
