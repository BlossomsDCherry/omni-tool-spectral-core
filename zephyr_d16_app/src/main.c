/*
 * Copyright 2026 Pecos D. Willy
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/logging/log.h>
#include <zephyr/usb/usb_device.h>
#include <zephyr/drivers/uart.h>
#include <zephyr/bluetooth/bluetooth.h>
#include <hubble/hubble.h>
#include <stdlib.h>
#include <math.h>

LOG_MODULE_REGISTER(d16_app, LOG_LEVEL_INF);

#define TAU 6.2831853f
#define SLEEP_TIME_MS 10

/* RGB LEDs from DeviceTree */
static const struct gpio_dt_spec led3_red = GPIO_DT_SPEC_GET(DT_NODELABEL(led3_red), gpios);
static const struct gpio_dt_spec led3_green = GPIO_DT_SPEC_GET(DT_NODELABEL(led3_green), gpios);
static const struct gpio_dt_spec led3_blue = GPIO_DT_SPEC_GET(DT_NODELABEL(led3_blue), gpios);

/* Global Bridge State */
static volatile float external_coherence = 0.0f;
static const struct device *const uart_dev = DEVICE_DT_GET(DT_CHOSEN(zephyr_console));

/* Hubble Network State */
static uint8_t hubble_key[16] = {0}; // Default 0-key for phase stabilization
static struct bt_data ad[] = {
	BT_DATA_BYTES(BT_DATA_FLAGS, (BT_LE_AD_GENERAL | BT_LE_AD_NO_BREDR)),
	BT_DATA_BYTES(BT_DATA_UUID16_ALL, 0xA6, 0xFC), // Hubble UUID 0xFCA6
	BT_DATA(BT_DATA_SVC_DATA16, NULL, 0), // Placeholder for Hubble Payload
};

/* Harmonic Oscillator (D7 Archetype) */
#define GRAVITY_G 9.80665f

struct HarmonicOscillator {
	float length_l;
	float period_t1;
	float period_t2;
};

static void oscillator_init(struct HarmonicOscillator *h, float length) {
	h->length_l = length;
	h->period_t1 = 0.0f;
	h->period_t2 = 0.0f;
}

static float drive_mass(struct HarmonicOscillator *h, float mass) {
	/* Mass affects Period T1 (simulated via log scaling) */
	/* Mass range: 1.0 (Zero Coherence) to 100.0 (High Coherence at 1.0) */
	float mass_factor = logf(fmaxf(mass, 1.0f)) * 0.1f; 
	h->period_t1 = (TAU * sqrtf(h->length_l / GRAVITY_G)) * (1.0f + mass_factor);
	return h->period_t1;
}

static float apply_entropy(struct HarmonicOscillator *h, float entropy) {
	/* Entropy affects Period T2 (Drag) */
	float drag_factor = entropy * 0.1f;
	h->period_t2 = (TAU * sqrtf(h->length_l / GRAVITY_G)) * (1.0f + drag_factor);
	return h->period_t2;
}

static bool check_stability(struct HarmonicOscillator *h, float tolerance) {
	return fabsf(h->period_t1 - h->period_t2) < tolerance;
}

/* Serial Callback */
#define RX_BUF_SIZE 32
static char rx_buf[RX_BUF_SIZE];
static int rx_pos = 0;

void serial_cb(const struct device *dev, void *user_data)
{
	uint8_t c;
	if (!uart_irq_update(dev)) {
		return;
	}

	if (uart_irq_rx_ready(dev)) {
		while (uart_fifo_read(dev, &c, 1) == 1) {
			if (c == '\n' || c == '\r') {
				rx_buf[rx_pos] = '\0';
				if (rx_pos > 2 && rx_buf[0] == 'C' && rx_buf[1] == ':') {
					external_coherence = strtof(&rx_buf[2], NULL);
				}
				rx_pos = 0;
			} else if (rx_pos < RX_BUF_SIZE - 1) {
				rx_buf[rx_pos++] = c;
			}
		}
	}
}

int main(void)
{
	LOG_INF("⚓ OMNI-TOOL: Zephyr D16 Firmware Active (Uno Q STM32U585) [Status: GREEN] 🌊");
	LOG_INF("🏎️  Initializing D16 Harmonic Pulse on RGB LEDs (Aliases: led3_red, led3_green, led3_blue)...");

	/* Enable USB Console */
	if (usb_enable(NULL)) {
		LOG_ERR("Failed to enable USB");
		return 0;
	}

	/* Initialize Serial Interrupt */
	if (!device_is_ready(uart_dev)) {
		LOG_ERR("UART device not found!");
		return 0;
	}
	uart_irq_callback_user_data_set(uart_dev, serial_cb, NULL);
	uart_irq_rx_enable(uart_dev);

	if (!gpio_is_ready_dt(&led3_red) || !gpio_is_ready_dt(&led3_green) || !gpio_is_ready_dt(&led3_blue)) {
		LOG_ERR("RGB LED devices not ready");
		return 0;
	}

	gpio_pin_configure_dt(&led3_red, GPIO_OUTPUT_ACTIVE);
	gpio_pin_configure_dt(&led3_green, GPIO_OUTPUT_ACTIVE);
	gpio_pin_configure_dt(&led3_blue, GPIO_OUTPUT_ACTIVE);

	/* Initialize Hubble Network SDK */
	if (hubble_init(0, hubble_key) != 0) {
		LOG_ERR("Failed to initialize Hubble SDK");
	} else {
		LOG_INF("🔭 Hubble SDK Initialized (Phase Stabilization Mode)");
	}

	/* Initialize Bluetooth */
	int err = bt_enable(NULL);
	if (err) {
		LOG_ERR("Bluetooth init failed (err %d)", err);
	} else {
		LOG_INF("Bluetooth initialized");
	}

	uint32_t moment = 0;
	uint8_t hubble_payload[32];
	uint8_t adv_payload[32];
	size_t adv_len = 0;
	
	/* Initialize Harmonic Oscillator */
	static struct HarmonicOscillator oscillator;
	oscillator_init(&oscillator, 0.993f); // Seconds pendulum approx

	while (1) {
		/* Hubble Phase Broadcast (Every 100ms) */
		if (moment % 10 == 0) {
			/* Pack Coherence into Hubble Payload */
			memcpy(hubble_payload, (void *)&external_coherence, sizeof(float));
			
			/* Get formatted advertisement payload from SDK */
			int ret = hubble_ble_advertise_get(hubble_payload, sizeof(float), adv_payload, &adv_len);
			if (ret == 0) {
				if (adv_len > 4) {
					ad[2].data_len = adv_len - 2; // Subtract Len and Type byte
					ad[2].data = &adv_payload[2]; // Start at UUID
					
					/* Update Advertisement (Non-Connectable Beacon) */
					err = bt_le_adv_start(BT_LE_ADV_NCONN, ad, ARRAY_SIZE(ad), NULL, 0);
					if (err && err != -EALREADY) {
						bt_le_adv_stop();
						bt_le_adv_start(BT_LE_ADV_NCONN, ad, ARRAY_SIZE(ad), NULL, 0);
					}
				}
			}
		}

		/* Calculate spectral intensity driven by TAU */
		float time = (float)moment * 0.01f; // 10ms steps
		
        /* 1. Drive Harmonic Mass (Coherence) */
        /* Scale: 0.0-1.0 coherence -> 1.0-100.0 Mass */
        drive_mass(&oscillator, 1.0f + (external_coherence * 100.0f));

        /* 2. Apply Entropy (Constant for now) */
        apply_entropy(&oscillator, 0.05f); // "Water" Stance Entropy

        /* 3. Check Stability (Resonance) */
        bool is_stable = check_stability(&oscillator, 0.02f);
		
		if (external_coherence > 1.2f) {
			is_stable = true; /* High coherence override */
		}

		if (is_stable) {
			/* Stability = White Light (Harmonic Resonance) */
			gpio_pin_set_dt(&led3_red, 1);
			gpio_pin_set_dt(&led3_green, 1);
			gpio_pin_set_dt(&led3_blue, 1);
		} else {
			/* Instability = Pulse at T1 Frequency */
            /* Frequency = 1.0 / T1 */
			/* Safe divide: T1 should never be 0, but good to check */
			float freq_mod = (oscillator.period_t1 > 0.0f) ? (1.0f / oscillator.period_t1) : 1.0f;
            
			/* Channel 1 (Red): Fundamental (T1) -> Modulated by Frequency */
			float val_r = (sinf(time * freq_mod * TAU) + 1.0f) / 2.0f;
			
			/* Channel 2 (Green): Harmonic (Perfect Fifth) */
			float val_g = (sinf(time * freq_mod * 1.5f * TAU) + 1.0f) / 2.0f;
			
			/* Channel 3 (Blue): Octave */
			float val_b = (sinf(time * freq_mod * 2.0f * TAU) + 1.0f) / 2.0f;

			gpio_pin_set_dt(&led3_red, val_r > 0.5f ? 1 : 0);
			gpio_pin_set_dt(&led3_green, val_g > 0.5f ? 1 : 0);
			gpio_pin_set_dt(&led3_blue, val_b > 0.5f ? 1 : 0);
		}
		
		moment++;
		k_msleep(SLEEP_TIME_MS);
	}
}
