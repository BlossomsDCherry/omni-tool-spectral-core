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
    
    start_time = time.time()
    
    try:
        while True:
            now = time.time()
            elapsed = now - start_time
            
            # 1. Gather Data (Advertiser Field)
            hailo_val = get_hailo_coherence()
            zrr_val = get_zrr_fidelity()
            
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
            print(f"[{source}] T+{elapsed:04.1f}s | {packet.strip()} | {bar}")
            
            time.sleep(0.1) # 10Hz Refresh

    except KeyboardInterrupt:
        print("\n👋 Bridge Closed.")
        if ser: ser.close()

if __name__ == "__main__":
    main()
