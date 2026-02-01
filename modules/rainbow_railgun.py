import subprocess
import time
import re
import sys
import math

# Rainbow Railgun: Antigravity Hull Celebration & Touch Test 🌈🔫
# "Parsimonious Metadata Sorter"

def get_pin_states():
    try:
        # Read GPIO 0-27
        result = subprocess.run(["pinctrl", "get", "0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27"], capture_output=True, text=True)
        return result.stdout
    except Exception as e:
        return ""

def parse_states(output):
    # Regex to find 'hi' or 'lo'
    # Format: " 0: ip    pu | hi // ID_SDA/GPIO0 = input"
    # We want to trap the pin number and the state (hi/lo)
    states = {}
    lines = output.split('\n')
    for line in lines:
        match = re.search(r'(\d+):.*\| (hi|lo)', line)
        if match:
            pin = int(match.group(1))
            state = 1 if match.group(2) == 'hi' else 0
            states[pin] = state
    return states

def calculate_knots(states):
    # Knots = Entropy of the system. 
    # Simple metric: Sum of HIGH pins * Complexity Factor
    high_count = sum(states.values())
    if len(states) == 0: return 0
    
    # Fake 'entropy' based on pattern
    entropy = 0
    for i in range(len(states)-1):
        if states.get(i,0) != states.get(i+1,0):
            entropy += 1
            
    knots = high_count * (entropy + 1) * math.pi
    return knots

def main():
    print("🌈 RAINBOW RAILGUN INITIATED on Antigravity Hull (10.0.0.80)")
    print("💎 Sorting Metadata Parsimoniously...")
    
    previous_states = {}
    
    # Pins to watch for "Touch" (Noisy/Floating)
    # Based on earlier local scan: 12, 13, 22, 23, 24, 25 were 'pd' (pull-down) 'no' (function none)
    # If user touches them, they might flicker if configured as input, or if we just read them.
    # We will just watch for ANY change in the unassigned block.
    watch_pins = [12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27]
    
    start_time = time.time()
    
    while True:
        raw_output = get_pin_states()
        current_states = parse_states(raw_output)
        
        knots = calculate_knots(current_states)
        
        # Visual Bar
        bar_len = int((knots % 100) / 5)
        bar = "█" * bar_len
        
        # Check for Changes in Watch Pins
        touch_alert = ""
        if previous_states:
            for pin in watch_pins:
                if current_states.get(pin) != previous_states.get(pin):
                    touch_alert += f" [TOUCH GPIO {pin}]"
        
        # Parsimonious Output: Only print frame
        print(f"R:{knots:.4f} | {bar} {touch_alert}")
        
        previous_states = current_states
        time.sleep(0.1) # 10Hz Refresh

if __name__ == "__main__":
    main()
