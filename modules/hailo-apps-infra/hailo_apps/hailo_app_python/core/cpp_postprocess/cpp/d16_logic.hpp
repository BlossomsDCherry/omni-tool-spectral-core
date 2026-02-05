#pragma once
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iostream>

class NobleGasStabilizer {
public:
    static NobleGasStabilizer& instance() {
        static NobleGasStabilizer instance;
        return instance;
    }

    float get_coherence_multiplier() {
        // Increment moment
        moment_++;

        // Noble Gas Logic: (t % 30) in {2, 10, 18, 26}
        // Assuming 30fps, 30 frames ~ 1 second cycle.
        int shell_pos = moment_ % 30;
        bool is_stable = (shell_pos == 2 || shell_pos == 10 || shell_pos == 18 || shell_pos == 26);

        float multiplier = is_stable ? 1.5f : 1.0f;

        // Export State to Shared Memory (RAM Disk) for Bridge
        // Only write every few frames to save IO, or every frame for responsiveness.
        // /dev/shm is tmpfs, so it's fast.
        if (moment_ % 5 == 0) {
            std::ofstream shm_file("/dev/shm/hailo_coherence", std::ios::trunc);
            if (shm_file.is_open()) {
                shm_file << multiplier;
                shm_file.close();
            }
        }

        return multiplier;
    }

private:
    NobleGasStabilizer() : moment_(0) {}
    uint64_t moment_;
};
