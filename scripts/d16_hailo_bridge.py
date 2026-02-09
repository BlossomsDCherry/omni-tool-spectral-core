#!/usr/bin/env python3
import time
import serial
import math
import random
import argparse
import sys
import os

# Bridge Configuration
DEFAULT_PORT = '/dev/ttyACM0'
BAUD_RATE = 115200

def get_hailo_coherence():
    """
    Tries to get coherence from Hailo-8 inference via shared memory (RAM disk).
    Location: /dev/shm/hailo_coherence
    """
    try:
        if os.path.exists('/dev/shm/hailo_coherence'):
            with open('/dev/shm/hailo_coherence', 'r') as f:
                val = float(f.read().strip())
                return val
    except Exception:
        pass
    return None

def get_zrr_fidelity():
    """
    Tries to get fidelity from Z-RR annealing.
    """
    return None

def mock_coherence(t):
    """
    Generates a synthetic coherence wave based on D16 physics.
    Shells at t % 30 in {2, 10, 18, 26}.
    """
    seconds = int(t)
    shell_pos = seconds % 30
    
    # Base chaotic signal
    base = 0.5 + (math.sin(t * 5.0) * 0.2) + (random.random() * 0.1)
    
    # Stability Nodes
    if shell_pos in [2, 10, 18, 26]:
        # Resonance spike
        return 1.5
    return base

def get_voltage_differential():
    """
    Reads the '3x2' Voltage Differential (Saturation Contrast) from the Sovereign Receiver.
    Location: /dev/shm/d16_saturation_voltage
    """
    try:
        if os.path.exists('/dev/shm/d16_saturation_voltage'):
            with open('/dev/shm/d16_saturation_voltage', 'r') as f:
                val = float(f.read().strip())
                return val
    except Exception:
        pass
    return None

def main():
    parser = argparse.ArgumentParser(description='D16 <-> Hailo/Z-RR Bridge')
    parser.add_argument('--port', default=DEFAULT_PORT, help='Serial port')
    parser.add_argument('--dry-run', action='store_true', help='Do not connect to serial, just print')
    args = parser.parse_args()

    ser = None
    if not args.dry_run:
        try:
            ser = serial.Serial(args.port, BAUD_RATE, timeout=1)
            print(f"✅ Bridge Connected to {args.port}")
        except Exception as e:
            print(f"❌ Failed to connect to {args.port}: {e}")
            sys.exit(1)
    else:
        print("⚠️ DRY RUN MODE")

    print("🌊 Spectral Bridge Active. Listening for Advertiser...")
    
    start_time = time.perf_counter()
    
    try:
        while True:
            now = time.perf_counter()
            elapsed = now - start_time
            
            # 1. Gather Data (Advertiser Field)
            hailo_val = get_hailo_coherence()
            zrr_val = get_zrr_fidelity()
            voltage_diff = get_voltage_differential()
            
            # 2. Synthesize Coherence (Listener Collapse)
            if hailo_val is not None:
                coherence = hailo_val
                source = "HAILO"
            elif zrr_val is not None:
                coherence = zrr_val
                source = "Z-RR"
            else:
                coherence = mock_coherence(elapsed)
                source = "MOCK"

            # 3. Transmit to D16 Firmware
            packet = f"C:{coherence:.2f}\n"
            
            if ser:
                ser.write(packet.encode('utf-8'))
            
            # Visual Log
            bar = "▓" * int(coherence * 10)
            voltage_str = f"V_Diff:{voltage_diff:.2f}" if voltage_diff is not None else "V_Diff:--"
            
            # High-Precision Timestamp (T+ss.mmm)
            elapsed_micros = elapsed * 1000000
            
            # Harmonic Timing: 216Hz (10^3 / 6^3) - Hollow Sphere
            # 256 (Potential) - 216 (Real) = 40 (Gap/Wooten Shift)
            # Cross Product Logic: Perfect(256), Real(216), Base(Gap), Parsimony(Result)
            
            w_shift = 40.0
            perfect_val = coherence * 2.56 # 256 Space
            real_val = coherence * 2.16    # 216 Space
            base_val = abs(perfect_val - real_val) # Gap
            parsimony_val = (real_val / perfect_val) * 100.0 if perfect_val > 0 else 0.0
            
            print(f"[{source}] T+{elapsed:08.5f}s | {packet.strip()} | {voltage_str} | Base:{base_val:.2f} Pars:{parsimony_val:.1f}%")

            # 216Hz Harmonic Timing
            # 1000ms / 216 = ~4.629ms
            time.sleep(0.004629) 

    except KeyboardInterrupt:
        print("\n👋 Bridge Closed.")
        if ser: ser.close()

if __name__ == "__main__":
    main()
