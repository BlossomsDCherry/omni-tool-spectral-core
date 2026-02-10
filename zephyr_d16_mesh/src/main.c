#include <zephyr/types.h>
#include <stddef.h>
#include <zephyr/sys/printk.h>
#include <zephyr/sys/util.h>
#include <zephyr/bluetooth/bluetooth.h>
#include <zephyr/bluetooth/hci.h>

#define HUBBLE_UUID_VAL 0xFCA6
#define SOVEREIGN_UUID_VAL 0xFCA7

static struct bt_le_scan_param scan_param = {
    .type       = BT_LE_SCAN_TYPE_PASSIVE,
    .options    = BT_LE_SCAN_OPT_NONE,
    .interval   = BT_GAP_SCAN_FAST_INTERVAL,
    .window     = BT_GAP_SCAN_FAST_WINDOW,
};

static const struct bt_data ad[] = {
    BT_DATA_BYTES(BT_DATA_FLAGS, (BT_LE_AD_GENERAL | BT_LE_AD_NO_BREDR)),
    BT_DATA(BT_DATA_NAME_COMPLETE, "D16 Mesh Node", 13),
};

static void scan_cb(const bt_addr_le_t *addr, int8_t rssi, uint8_t type,
            struct net_buf_simple *ad)
{
    // Minimal logic: Just print RSSI of everything for now to prove visibility
    // In a real mesh, we would filter for HUBBLE_UUID_VAL
    // char addr_str[BT_ADDR_LE_STR_LEN];
    // bt_addr_le_to_str(addr, addr_str, sizeof(addr_str));
    // printk("Device found: %s (RSSI %d)\n", addr_str, rssi);
}

void main(void)
{
    int err;

    printk("Starting D16 Mesh Node (ESP32)...\n");

    /* Initialize the Bluetooth Subsystem */
    err = bt_enable(NULL);
    if (err) {
        printk("Bluetooth init failed (err %d)\n", err);
        return;
    }

    printk("Bluetooth initialized\n");

    /* Start Advertising */
    err = bt_le_adv_start(BT_LE_ADV_NCONN, ad, ARRAY_SIZE(ad), NULL, 0);
    if (err) {
        printk("Advertising failed to start (err %d)\n", err);
        return;
    }

    printk("Advertising 'D16 Mesh Node'...\n");

    /* Start Scanning */
    err = bt_le_scan_start(&scan_param, scan_cb);
    if (err) {
         printk("Scanning failed to start (err %d)\n", err);
         return;
    }
    
    printk("Scanning for Hubble Network...\n");
}
