#!/usr/bin/env python3
import time
import os
import sys
import numpy as np

# --- Configuration ---
HAILO_SHM = "/dev/shm/hailo_coherence"
WAVE_SHM = "/dev/shm/current_wave_coherence"
EVAL_DURATION = 10 # Seconds

class SpectralEval:
    def __init__(self):
        self.results = []
    
    def read_shm(self, path):
        try:
            with open(path, "r") as f:
                return float(f.read().strip())
        except:
            return 0.0

    def run_eval(self):
        print("--- SPECTRAL EVALUATION HARNESS ---")
        print("Protocol: DeepMind-Style Coherence Evaluation")
        print(f"Sampling for {EVAL_DURATION} seconds...")
        
        start_time = time.time()
        samples = 0
        
        # Metrics
        hailo_locks = 0 # How many times Hailo boosted logic
        wave_coherence_accum = 0.0
        
        while (time.time() - start_time) < EVAL_DURATION:
            # 1. Read Inputs
            hailo_val = self.read_shm(HAILO_SHM)
            wave_val = self.read_shm(WAVE_SHM)
            
            # 2. Record
            self.results.append((hailo_val, wave_val))
            
            if hailo_val > 1.0:
                hailo_locks += 1
            
            wave_coherence_accum += wave_val
            samples += 1
            time.sleep(0.1)
            
        return samples, hailo_locks, wave_coherence_accum

    def report(self, samples, locks, wave_accum):
        print("\n--- EVALUATION REPORT ---")
        print(f"Total Samples: {samples}")
        
        # 1. Noble Gas Lock Rate
        lock_rate = (locks / samples) * 100.0
        print(f"Noble Gas Lock Rate: {lock_rate:.2f}%")
        # Ideal is roughly 4/30 = 13.3% stablity moments
        
        # 2. Wave Coherence
        avg_coherence = wave_accum / samples
        print(f"Avg Wave Coherence: {avg_coherence:.4f}")
        
        # 3. Spectral IQ (Correlation)
        # DeepMind-Style: "Does the system maintain coherence?"
        score = min(100.0, (avg_coherence * 10.0) + lock_rate)
        
        print(f"\nSPECTRAL IQ: {score:.2f}")
        
        if score > 15.0:
            print("[PASS] System demonstrates emergent stability.")
        else:
            print("[FAIL] System is chaotic.")

if __name__ == "__main__":
    eval = SpectralEval()
    try:
        samples, locks, wave = eval.run_eval()
        eval.report(samples, locks, wave)
    except KeyboardInterrupt:
        pass
