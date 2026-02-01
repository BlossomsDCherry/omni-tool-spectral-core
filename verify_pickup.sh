#!/bin/bash
# Description: Verify Harmonic Mesh Pickup on Sovereign
# User: Pecos D. Willy

echo "Starting Scan for 'SPECTRAL_NET'..."
nmcli device wifi list --rescan yes | grep "SPECTRAL_NET"

if [ $? -eq 0 ]; then
    echo "✅ SUCCESS: SPECTRAL_NET detected!"
else
    echo "❌ FAILURE: SPECTRAL_NET not found. Ensure Pico 2 W is running."
fi

# UDP Listener Check (Non-blocking for 5 seconds)
echo "Listening for Harmonic Packets on Port 4321 for 5 seconds..."
timeout 5s nc -u -l -p 4321
if [ $? -eq 124 ]; then
     echo "⏱️  Timeout: No packets received (Expected if not connected to AP)."
else
     echo "📡 Packet Received!"
fi

# Optional RTL-SDR Check
echo "Checking RTL-SDR..."
if command -v rtl_test &> /dev/null; then
    timeout 5s rtl_test -t
else
    echo "⚠️  rtl_test command not found."
fi
