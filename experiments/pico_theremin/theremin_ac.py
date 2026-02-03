# D16 Soft FPGA: Theremin & AC Driver (Recreation)
# Hardware: Pico 2 W (RP2350)
# Setup:
#   - GPIO 16 (TX): Carrier Wave / Drive
#   - GPIO 17 (RX): Sensor / Hot
#   - Piezo: Connected to TX (or parallel to LED)
#   - Red LED: Connected across TX (16) and RX (17) (Anode to RX, Cathode to TX for 'Electro-Kinetic' flow)

import machine
import time
import math

# --- Constants ---
TX_PIN = 16
RX_PIN = 17 # ADC 1? No, 17 is just GPIO. ADC0=26, ADC1=27.
            # If 17 is RX, we might be doing digital polling or PIO.
            # Let's assume ADC behavior via "Soft FPGA" digital sampling or using 26/27 if able.
            # User said "GPIO 17". On Pico, GP17 is NOT an ADC pin.
            # So this must be a "Soft ADC" via digital timing or PIO?
            # Or just usage of the "Input Synchronizer".

# Reverting to "Spectral Scoping" logic:
# Measuring "Drift" or "Jitter" on a digital pin.

tx = machine.PWM(machine.Pin(TX_PIN))
rx = machine.Pin(RX_PIN, machine.Pin.IN, machine.Pin.PULL_DOWN)

# Piezo Driver (shared with TX or separate?)
# If LED is across TX/RX, TX provides the power.
# Let's use TX as the main driver.

def main():
    print("🌀 D16 Soft FPGA: Theremin AC Recreation Initialized")
    print("   Target: Recreate Piezo (Sound) and LED (Light) phenomena.")
    
    # Base Carrier Frequency (The "Field")
    # High enough to drive LED, audible for Piezo harmonics?
    # Or ultrasonic carrier with audible modulation?
    carrier_freq = 1000 # Start audible
    tx.freq(carrier_freq)
    tx.duty_u16(32768) # 50% Duty Cycle

    print(f"⚡ Carrier Active on GPIO {TX_PIN}: {carrier_freq} Hz")
    print(f"🔦 Connect LED Anode to GPIO {RX_PIN}, Cathode to GPIO {TX_PIN}")
    
    samples = 100
    
    while True:
        # "Theremin" Logic: Measure the "Field"
        # Since GP17 isn't ADC, we measure how often it flips or stays high?
        # Physical hand proximity changes capacitance/inductance.
        # Simple loop to measure "Input Density"
        
        high_count = 0
        for _ in range(samples):
            if rx.value():
                high_count += 1
        
        # Density Analysis (0.0 - 1.0)
        density = high_count / samples
        
        # Spectral Mapping: Density -> Frequency
        # Map 0.0 - 1.0 to 200Hz - 2000Hz
        target_freq = 200 + (density * 1800)
        
        # Dynamic Feedback
        tx.freq(int(target_freq))
        
        # Logging (The "Oscilloscope" View)
        # Use chars to show density bar
        bar = "#" * int(density * 20)
        print(f"\rRX Density: [{bar:<20}] {target_freq:.1f} Hz", end="")
        
        # The LED will brighten/dim based on frequency/duty coupling logic physically.
        # No extra code needed for LED if it's purely parasitic/complementary.

        time.sleep(0.05)

if __name__ == "__main__":
    main()
