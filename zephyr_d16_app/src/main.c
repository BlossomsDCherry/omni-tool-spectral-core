/*
 * Copyright 2026 Pecos D. Willy
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/logging/log.h>
#include <zephyr/usb/usb_device.h>
#include <math.h>

LOG_MODULE_REGISTER(d16_app, LOG_LEVEL_INF);

#define TAU 6.2831853f
#define SLEEP_TIME_MS 10

/* RGB LEDs from DeviceTree */
static const struct gpio_dt_spec led3_red = GPIO_DT_SPEC_GET(DT_NODELABEL(led3_red), gpios);
static const struct gpio_dt_spec led3_green = GPIO_DT_SPEC_GET(DT_NODELABEL(led3_green), gpios);
static const struct gpio_dt_spec led3_blue = GPIO_DT_SPEC_GET(DT_NODELABEL(led3_blue), gpios);

void main(void)
{
	LOG_INF("⚓ OMNI-TOOL: Zephyr D16 Firmware Active (Uno Q STM32U585) 🌊");
	LOG_INF("🏎️  Initializing D16 Harmonic Pulse on RGB LEDs...");

	/* Enable USB Console */
	if (usb_enable(NULL)) {
		LOG_ERR("Failed to enable USB");
		return;
	}

	if (!gpio_is_ready_dt(&led3_red) || !gpio_is_ready_dt(&led3_green) || !gpio_is_ready_dt(&led3_blue)) {
		LOG_ERR("RGB LED devices not ready");
		return;
	}

	gpio_pin_configure_dt(&led3_red, GPIO_OUTPUT_ACTIVE);
	gpio_pin_configure_dt(&led3_green, GPIO_OUTPUT_ACTIVE);
	gpio_pin_configure_dt(&led3_blue, GPIO_OUTPUT_ACTIVE);

	uint32_t moment = 0;
	while (1) {
		/* Calculate spectral intensity driven by TAU */
		float time = (float)moment * 0.01f; // 10ms steps
		
		/* Noble Gas Stability Check (Bridge Logic) */
		/* Shells: 2, 10, 18, 26. Scaled to seconds (moment/100) for visibility */
		int shell_pos = (moment / 100) % 30; 
		bool is_stable = (shell_pos == 2 || shell_pos == 10 || shell_pos == 18 || shell_pos == 26);

		if (is_stable) {
			/* Stability = White Light (Full Coherence) */
			gpio_pin_set_dt(&led3_red, 1);
			gpio_pin_set_dt(&led3_green, 1);
			gpio_pin_set_dt(&led3_blue, 1);
		} else {
			/* Channel 1 (Red): Fundamental */
			float val_r = (sinf(time) + 1.0f) / 2.0f;
			
			/* Channel 2 (Green): Harmonic (Perfect Fifth) */
			float val_g = (sinf(time * 1.5f) + 1.0f) / 2.0f;
			
			/* Channel 3 (Blue): Octave */
			float val_b = (sinf(time * 2.0f) + 1.0f) / 2.0f;

			gpio_pin_set_dt(&led3_red, val_r > 0.5f ? 1 : 0);
			gpio_pin_set_dt(&led3_green, val_g > 0.5f ? 1 : 0);
			gpio_pin_set_dt(&led3_blue, val_b > 0.5f ? 1 : 0);
		}
		
		moment++;
		k_msleep(SLEEP_TIME_MS);
	}
}
