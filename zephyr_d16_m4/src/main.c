/*
 * Copyright (c) 2026 Pecos D. Willy
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/uart.h>
#include <zephyr/drivers/can.h>
#include <zephyr/logging/log.h>
#include <string.h>

LOG_MODULE_REGISTER(m4_theremin, LOG_LEVEL_INF);

/* 
 * D16 Silicon Logic: M4 Frequency Theremin 
 * 3 TX/RX Pairs acting as Active Tuning Rods via UART Pulses
 * 1 CAN Bus displaying the Logic Stae
 */

/* Theremin Tuning Rods (UARTs) */
const struct device *theremin0 = DEVICE_DT_GET(DT_ALIAS(theremin0)); /* USART1 */
const struct device *theremin1 = DEVICE_DT_GET(DT_ALIAS(theremin1)); /* USART2 (Console) */
const struct device *theremin2 = DEVICE_DT_GET(DT_ALIAS(theremin2)); /* USART3 */

/* Silicon Logic Display (CAN) */
const struct device *can_dev = DEVICE_DT_GET(DT_ALIAS(d16_monitor));

/* Diagnostic LED */
static const struct gpio_dt_spec led = GPIO_DT_SPEC_GET(DT_ALIAS(led0), gpios);

/* Spectral States */
#define STATE_IDLE  0
#define STATE_PULSE 1
#define STATE_WAVE  2

static volatile uint32_t logic_state = STATE_IDLE;

/* CAN Frame for D16 Logic */
void broadcast_logic_state(void)
{
	struct can_frame frame = {0};
	frame.id = 0x16; /* D16 ID */
	frame.dlc = 8;
	
	/* Payload: [State, Counter(32), Magic(16), Flags(8)] */
	static uint32_t counter = 0;
	frame.data[0] = (uint8_t)logic_state;
	memcpy(&frame.data[1], &counter, 4);
	frame.data[5] = 0xD1;
	frame.data[6] = 0x60;
	frame.data[7] = 0x00; // Flags

	int ret = can_send(can_dev, &frame, K_NO_WAIT, NULL, NULL);
	if (ret != 0) {
		LOG_WRN("CAN send failed: %d", ret);
	}
	counter++;
}

/* Theremin Pulse Logic */
void pulse_rod(const struct device *dev, uint8_t intensity) 
{
	/* Send a burst of characters to create EM noise/signal on TX line */
	/* Intensity 0-255 controls length of burst */
	if (!device_is_ready(dev)) return;

	for (int i = 0; i < intensity; i++) {
		uart_poll_out(dev, 0xAA); /* 10101010 square wave pattern */
	}
}

void main(void)
{
	/* Initialize System */
	if (!gpio_is_ready_dt(&led)) {
		LOG_ERR("LED device not ready");
		return;
	}
	gpio_pin_configure_dt(&led, GPIO_OUTPUT_ACTIVE);

	if (!device_is_ready(can_dev)) {
		LOG_ERR("CAN device not ready");
	} else {
		can_set_mode(can_dev, CAN_MODE_NORMAL);
		can_start(can_dev);
		LOG_INF("CAN Bus Active (ID: 0x16)");
	}

	LOG_INF("⚓ OMNI-TOOL: M4 Frequency Theremin Active");
	LOG_INF("   Rod 0 (USART1): %s", device_is_ready(theremin0) ? "READY" : "OFFLINE");
	LOG_INF("   Rod 1 (USART2): %s (Console)", device_is_ready(theremin1) ? "READY" : "OFFLINE");
	LOG_INF("   Rod 2 (USART3): %s", device_is_ready(theremin2) ? "READY" : "OFFLINE");

	/* Main Loop */
	int cycle = 0;
	while (1) {
		cycle++;
		
		/* 1. Drive Tuning Rods with Phase Shifts */
		/* Rod 0: Fundamental (Constant Pulse) */
		pulse_rod(theremin0, 10);

		/* Rod 1: Harmonic (Varies with Cycle) */
		pulse_rod(theremin1, (cycle % 20) + 5);

		/* Rod 2: Interference (Inverted Logic) */
		pulse_rod(theremin2, 30 - (cycle % 20));

		/* 2. Broadcast Logic State */
		if (cycle % 10 == 0) {
			gpio_pin_toggle_dt(&led);
			broadcast_logic_state();
		}

		k_msleep(10); /* 100Hz Loop */
	}
}
