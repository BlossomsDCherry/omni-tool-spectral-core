import asyncio
from bleak import BleakScanner

async def scan():
    print("📡 Scanning for D16 Telemetry (Hubble / Pico)...")
    devices = await BleakScanner.discover()
    for d in devices:
        name = d.name or "Unknown"
        # Filter for known D16 names or just log everything for now
        if "Hubble" in name or "Pico" in name or "Zephyr" in name:
            print(f"✅ FOUND: {name} [{d.address}]")
            print(f"   RSSI: {d.rssi}")
            print(f"   Details: {d.details}") 
        else:
             # Optional: print all to find hidden ones
             # print(f"   Found: {name} [{d.address}]")
             pass

if __name__ == "__main__":
    asyncio.run(scan())
