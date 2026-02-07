
import sys
import os

# POC: Using installed alphagenome package
# ALPHAGENOME_PATH = "/home/pecosdwilly/Downloads/alphagenome-main/src"
# sys.path.append(ALPHAGENOME_PATH)

from alphagenome_shim import MillenniumClient
from alphagenome.data import genome

def verify():
    print("--- Verifying Millennium Shim ---")
    
    # Initialize Wrapper
    client = MillenniumClient(api_key="TEST_KEY")
    
    # Create some "Messy" intervals (Prime numbers, off-grid)
    raw_intervals = [
        genome.Interval("chr1", 17, 23),    # Primes, likely non-resonant
        genome.Interval("chr1", 512, 520),  # 512 is resonant
        genome.Interval("chr1", 99, 150),
        genome.Interval("chr1", 1000, 1005)
    ]
    
    print("\n--- Feeding The Machine (4 Cycles) ---")
    for i, interval in enumerate(raw_intervals):
        print(f"\n[Input #{i+1}] {interval}")
        client.queue_prediction(interval)
        
    print("\n--- Verification Complete ---")

if __name__ == "__main__":
    verify()
