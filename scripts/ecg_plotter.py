import serial
import time
import sys

# D16 ECG Plotter
# Reads "ECG,tau,heart,inv_cymatics" from Serial
# Prints a simple ASCII graph

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 ecg_plotter.py <serial_port>")
        sys.exit(1)

    port = sys.argv[1]
    print(f"❤️  Connecting to D16 Heart Monitor on {port}...")
    
    try:
        ser = serial.Serial(port, 115200, timeout=1)
    except Exception as e:
        print(f"❌ Connection failed: {e}")
        sys.exit(1)

    print("✅ Connected! Listening for heartbeat...")
    print("   [ Pulse ] [ ADC Value ] [ Graph ]")

    try:
        while True:
            line = ser.readline().decode('utf-8', errors='ignore').strip()
            if line.startswith("ECG,"):
                parts = line.split(",")
                if len(parts) >= 3:
                     # ECG, tau, adc, cymatics
                     tau = parts[1]
                     adc = int(parts[2])
                     
                     # Simple Auto-Scale (12-bit ADC: 0-4095)
                     # Baseline ~2048? Or 0?
                     # Let's assume raw values.
                     bar_len = int((adc / 4096.0) * 50)
                     bar = "♥" * bar_len
                     
                     print(f"   {tau:<8} {adc:<6} | {bar}")
            elif line:
                print(f"   [LOG] {line}")
                
    except KeyboardInterrupt:
        print("\n👋 Monitoring stopped.")
        ser.close()

if __name__ == "__main__":
    main()
