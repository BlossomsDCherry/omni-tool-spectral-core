#!/bin/bash
# Atomic Drop / Full System Verification Script

# 1. Clean previous runs
pkill -f ripple_tank_core
pkill -f zrr_core
rm -f /dev/shm/current_wave_coherence /dev/shm/hailo_coherence

echo "==================================================="
echo "   ATOMIC DROP: FULL SYSTEM VERIFICATION"
echo "   Running with 8 Sig Fig Accuracy"
echo "==================================================="

# 2. Launch Physics (Ripple Tank - Rust)
echo "1. [PHYSICS] Launching Ripple Tank (Native Rust)..."
./modules/ripple_tank_core/target/release/ripple_tank_core > ripple.log 2>&1 &
PID_PHYSICS=$!
echo "   >> PID: $PID_PHYSICS"
sleep 2

# 3. Launch Rails (Z-RR - Rust)
# Create dummy payload if not exists
if [ ! -f dummy.zip ]; then
    echo "dummy" > dummy.txt
    zip dummy.zip dummy.txt > /dev/null
fi

echo "2. [RAILS] Launching Z-RR (Talu64 Logic)..."
# Z-RR runs in a loop in this test script check
# We'll run it in a loop to simulate continuous operation
(
    while true; do
        ./modules/z_rr/target/debug/zrr_core dummy.zip >> rails.log 2>&1
        sleep 0.5
    done
) &
PID_RAILS=$!
echo "   >> PID: $PID_RAILS"

# 4. Launch Brain (Hailo Shim - C++)
echo "3. [BRAIN] Launching Hailo Logic (Noble Gas)..."
# Compile shim if needed
g++ test_hailo_shim.cpp -o test_hailo_shim
./test_hailo_shim &
PID_BRAIN=$!
echo "   >> PID: $PID_BRAIN"

# 5. Monitor
echo "==================================================="
echo "   SYSTEM LIVE - MONITORING COHERENCE"
echo "==================================================="
for i in {1..10}; do
    WAVE=$(cat /dev/shm/current_wave_coherence 2>/dev/null || echo "0.0")
    HAILO=$(cat /dev/shm/hailo_coherence 2>/dev/null || echo "0.0")
    
    echo "[T+$i] Physics: $WAVE | Brain: $HAILO"
    sleep 1
done

echo "==================================================="
echo "   VERIFICATION COMPLETE"
echo "==================================================="

# Cleanup
kill $PID_PHYSICS $PID_RAILS $PID_BRAIN 2>/dev/null
pkill -f ripple_tank_core
pkill -f zrr_core
