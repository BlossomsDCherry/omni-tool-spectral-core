
import time
import sys
from millennium_optimization import inject_optimization, _GlobalVoxel

# Mock Class simulating High-Latency IPC (AT-SPI)
class MockAccessible:
    def __init__(self, name, role):
        self.name = name
        self.role = role

    def get_name(self):
        # Simulate IPC roundtrip latency (e.g., 2ms)
        time.sleep(0.002) 
        return self.name

    def get_role(self):
        # Simulate IPC roundtrip latency
        time.sleep(0.002)
        return self.role

class MockGenerator:
    def __init__(self):
        self.obj = MockAccessible("OK Button", "ROLE_PUSH_BUTTON")

    def generate_packet(self):
        # A typical complex generator might call get_name() and get_role() multipled times
        # to determine context, braille, speech, etc.
        n = self.obj.get_name()      # Call 1
        r = self.obj.get_role()      # Call 2
        n2 = self.obj.get_name()     # Call 3 (Redundant check)
        r2 = self.obj.get_role()     # Call 4 (Redundant check, common in nested logic)
        return f"{n} is a {r}"

def run_benchmark():
    print("--- Orca Voxel Cache Benchmark ---")
    
    generator = MockGenerator()
    iterations = 50
    
    # 1. Baseline Run
    start_time = time.time()
    for _ in range(iterations):
        generator.generate_packet()
    baseline_duration = time.time() - start_time
    print(f"🐢 Baseline Duration: {baseline_duration:.4f}s")

    # 2. Inject Optimization
    print("\n💉 Injecting Millennium Shim...")
    inject_optimization(MockAccessible, ["get_name", "get_role"])

    # 3. Optimized Run
    _GlobalVoxel.clear()
    start_time = time.time()
    for _ in range(iterations):
        generator.generate_packet()
    optimized_duration = time.time() - start_time
    
    print(f"🐇 Optimized Duration: {optimized_duration:.4f}s")
    print(f"📊 Cache Stats: {_GlobalVoxel.stats()}")
    
    improvement = (baseline_duration - optimized_duration) / baseline_duration * 100
    print(f"\n🚀 Performance Improvement: {improvement:.1f}%")

if __name__ == "__main__":
    run_benchmark()
