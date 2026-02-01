/*
 * D16 Soft FPGA Kernel - Fixed for Real Iron
 * Target: ARMv8-A (AArch64)
 * 
 * Logic: "Soft FPGA" - Manual Unrolled Scalar Operations for Precision.
 * NEON lacks integer vector divide. We use scalar registers w4-w7 for the ALU ops,
 * maximizing throughput via pipeline depth.
 * 
 * Inputs:
 *   x0: Tau (u32)
 *   x1: *results (u32[16])
 */

.global d16_soft_fpga
.type d16_soft_fpga, %function

.section .text
.align 4

d16_soft_fpga:
    // Prologue
    stp     x29, x30, [sp, #-16]!
    mov     x29, sp

    // x0 = Tau
    // x1 = Results Ptr

    // We proceed in banks of 4 to mimic the "Crewmate" parallel logic.
    // Bank 1: 1, 2, 3, 4
    
    // Channel 1 (Luffy) - Div 1
    // q = Tau / 1 = Tau
    // r = Tau % 1 = 0
    mov     w2, w0        // Decay = Tau
    mov     w3, #0        // Phase = 0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #0]  // Store Luffy

    // Channel 2 (Zoro) - Div 2
    mov     w4, #2
    udiv    w2, w0, w4    // q = Tau / 2
    msub    w3, w2, w4, w0 // r = Tau - (q*2)
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #4]  // Store Zoro

    // Channel 3 (Nami) - Div 3
    mov     w4, #3
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #8]  // Store Nami

    // Channel 4 (Usopp) - Div 4
    mov     w4, #4
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #12] // Store Usopp

    // Bank 2: 5, 6, 7, 8
    // Channel 5 (Sanji)
    mov     w4, #5
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #16]

    // Channel 6 (Chopper)
    mov     w4, #6
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #20]

    // Channel 7 (Robin)
    mov     w4, #7
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #24]

    // Channel 8 (Franky)
    mov     w4, #8
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #28]

    // Bank 3: 9, 10, 11, 12
    // Channel 9 (Brook)
    mov     w4, #9
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #32]

    // Channel 10 (Jinbe)
    mov     w4, #10
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #36]

    // Channel 11 (Vivi)
    mov     w4, #11
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #40]

    // Channel 12 (Carrot)
    mov     w4, #12
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #44]

    // Bank 4: 13, 14, 15, 16
    // Channel 13 (Yamato)
    mov     w4, #13
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #48]

    // Channel 14 (Momo)
    mov     w4, #14
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #52]

    // Channel 15 (Kinemon)
    mov     w4, #15
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #56]

    // Channel 16 (Law)
    mov     w4, #16
    udiv    w2, w0, w4
    msub    w3, w2, w4, w0
    lsl     w2, w2, #16
    orr     w2, w2, w3
    str     w2, [x1, #60]

    // Epilogue
    ldp     x29, x30, [sp], #16
    ret
