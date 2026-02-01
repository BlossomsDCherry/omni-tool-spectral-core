/*
 * D16 Soft FPGA Kernel - Thumb-2 Port
 * Target: Cortex-M4/M7/M33 (Thumb-2 ISA)
 * 
 * Logic: "Soft FPGA" - Manual Unrolled Scalar Operations.
 * We use the hardware divider (sdiv/udiv) which is available on M4/M7/M33.
 * 
 * Inputs:
 *   r0: Tau (u32)
 *   r1: *results (u32[16])
 *
 * Register Map:
 *   r0: Tau (Input), preserved or restored if needed? No, we can just keep it in r0 if we are careful, 
 *       but we might need it for calculations. Let's move Tau to a high register or keep it in r0 
 *       and use other regs for scratch.
 *       Actually, we have plenty of registers:
 *       r0: Tau (Keep constant)
 *       r1: Results Ptr (Keep constant)
 *       r2: Quotient (q)
 *       r3: Remainder (r)
 *       r4: Divisor (n)
 *       r5: Temporary / Scratch
 *
 *   Note: r4-r11 are callee-saved. We must push them if we use them.
 */

.syntax unified
.cpu cortex-m4
.thumb

.global d16_soft_fpga_thumb
.type d16_soft_fpga_thumb, %function

.section .text
.align 2

d16_soft_fpga_thumb:
    // Prologue
    push    {r4-r7, lr}  
    // We will use r4 for the divisor. r5 for scratch.

    // r0 = Tau
    // r1 = Results Ptr

    // --- Channel 1 (Luffy) - Div 1 ---
    // q = Tau / 1 = Tau
    // r = Tau % 1 = 0
    mov     r2, r0          // Decay = Tau
    mov     r3, #0          // Phase = 0
    // Pack: (Decay << 16) | Phase
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #0]    // Store Luffy

    // --- Channel 2 (Zoro) - Div 2 ---
    mov     r4, #2
    udiv    r2, r0, r4      // q = Tau / 2
    mls     r3, r2, r4, r0  // r = Tau - (q * 2)
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #4]    // Store Zoro

    // --- Channel 3 (Nami) - Div 3 ---
    mov     r4, #3
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #8]    // Store Nami

    // --- Channel 4 (Usopp) - Div 4 ---
    mov     r4, #4
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #12]   // Store Usopp

    // --- Channel 5 (Sanji) - Div 5 ---
    mov     r4, #5
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #16]

    // --- Channel 6 (Chopper) - Div 6 ---
    mov     r4, #6
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #20]

    // --- Channel 7 (Robin) - Div 7 ---
    mov     r4, #7
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #24]

    // --- Channel 8 (Franky) - Div 8 ---
    mov     r4, #8
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #28]

    // --- Channel 9 (Brook) - Div 9 ---
    mov     r4, #9
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #32]

    // --- Channel 10 (Jinbe) - Div 10 ---
    mov     r4, #10
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #36]

    // --- Channel 11 (Vivi) - Div 11 ---
    mov     r4, #11
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #40]

    // --- Channel 12 (Carrot) - Div 12 ---
    mov     r4, #12
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #44]

    // --- Channel 13 (Yamato) - Div 13 ---
    mov     r4, #13
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #48]

    // --- Channel 14 (Momo) - Div 14 ---
    mov     r4, #14
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #52]

    // --- Channel 15 (Kinemon) - Div 15 ---
    mov     r4, #15
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #56]

    // --- Channel 16 (Law) - Div 16 ---
    mov     r4, #16
    udiv    r2, r0, r4
    mls     r3, r2, r4, r0
    lsl     r5, r2, #16
    orr     r5, r5, r3
    str     r5, [r1, #60]

    // Epilogue
    pop     {r4-r7, pc}
