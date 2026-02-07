from d16_sdk import D16Engine, ConnectionConfig

print("🐍 Python: Testing D16 SDK bindings...")

config = ConnectionConfig(udp_port=1337, hub_ip="127.0.0.1")
engine = D16Engine(config)

print("Status:", engine.get_status())
print("Connecting...")
engine.connect()
print("Status:", engine.get_status())
print("Disconnecting...")
engine.disconnect()
print("Test Complete.")
