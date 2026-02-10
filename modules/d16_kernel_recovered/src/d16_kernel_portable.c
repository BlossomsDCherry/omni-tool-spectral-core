/*
 * D16 Soft FPGA Kernel - Portable C Implementation
 * Logic: "Soft FPGA" - Manual Unrolled Scalar Operations.
 * 
 * Inputs:
 *   tau: u32 (Global Pulse)
 *   results: *mut u32 (Buffer of 16 channels)
 * 
 * Output Structure:
 *   High 16 bits: Decay (Quotient)
 *   Low 16 bits: Phase (Remainder)
 */

#include <stdint.h>

void d16_soft_fpga(uint64_t tau, uint32_t *results) {
    // Channel 1 (Luffy) - Div 1
    results[0] = (tau << 16) | 0;

    // Channel 2 (Zoro) - Div 2
    results[1] = ((tau / 2) << 16) | (tau % 2);

    // Channel 3 (Nami) - Div 3
    results[2] = ((tau / 3) << 16) | (tau % 3);

    // Channel 4 (Usopp) - Div 4
    results[3] = ((tau / 4) << 16) | (tau % 4);

    // Channel 5 (Sanji) - Div 5
    results[4] = ((tau / 5) << 16) | (tau % 5);

    // Channel 6 (Chopper) - Div 6
    results[5] = ((tau / 6) << 16) | (tau % 6);

    // Channel 7 (Robin) - Div 7
    results[6] = ((tau / 7) << 16) | (tau % 7);

    // Channel 8 (Franky) - Div 8
    results[7] = ((tau / 8) << 16) | (tau % 8);

    // Channel 9 (Brook) - Div 9
    results[8] = ((tau / 9) << 16) | (tau % 9);

    // Channel 10 (Jinbe) - Div 10
    results[9] = ((tau / 10) << 16) | (tau % 10);

    // Channel 11 (Vivi) - Div 11
    results[10] = ((tau / 11) << 16) | (tau % 11);

    // Channel 12 (Carrot) - Div 12
    results[11] = ((tau / 12) << 16) | (tau % 12);

    // Channel 13 (Yamato) - Div 13
    results[12] = ((tau / 13) << 16) | (tau % 13);

    // Channel 14 (Momo) - Div 14
    results[13] = ((tau / 14) << 16) | (tau % 14);

    // Channel 15 (Kinemon) - Div 15
    results[14] = ((tau / 15) << 16) | (tau % 15);

    // Channel 16 (Law) - Div 16
    results[15] = ((tau / 16) << 16) | (tau % 16);
}
