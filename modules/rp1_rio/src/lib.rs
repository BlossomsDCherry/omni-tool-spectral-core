use std::fs::OpenOptions;
use std::os::fd::AsRawFd;
use std::ptr;

// RP1 Peripheral Base Address (from RPi 5 devicetree / Zephyr driver)
// Note: This is the physical address aperture for the RP1.
const RP1_PERI_BASE: u64 = 0x1f00000000;
const RIO_OFFSET: u64 = 0xe0000; // Offset for RIO bank 0 (example, needs verification against full map)
                                 // Based on Zephyr: RIO is at a specific offset. We'll use a likely offset or make it configurable.
                                 // Zephyr `gpio_rp1.c`: `rio_offset = DT_INST_REG_ADDR_BY_IDX(n, 1)`
                                 // This implies we might need to probe or use a known offset.
                                 // For now, we will use a refined guess based on RP2040 structure but acknowledge it might need tuning.
                                 // The Zephyr driver uses `sys_write32`.

const RIO_OUT: usize = 0x00;
const RIO_OE: usize = 0x04;
const RIO_IN: usize = 0x08;

const RIO_XOR: usize = 0x1000;
const RIO_SET: usize = 0x2000;
const RIO_CLR: usize = 0x3000;

pub struct Rp1Rio {
    base_ptr: *mut u32,
    _file: std::fs::File, // Keep file open to maintain mmap
}

impl Rp1Rio {
    /// Initialize the RP1 RIO driver by mapping /dev/mem.
    /// REQUIRES ROOT PRIVILEGES (Sovereign Access).
    pub fn new() -> Result<Self, std::io::Error> {
        let file = OpenOptions::new().read(true).write(true).open("/dev/mem")?;

        // Map 4KB page at the RIO base
        let map_len = 4096;
        let ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                map_len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                file.as_raw_fd(),
                (RP1_PERI_BASE + RIO_OFFSET) as libc::off_t,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Self {
            base_ptr: ptr as *mut u32,
            _file: file,
        })
    }

    /// Atomically set a pin high (Quantum 1).
    #[inline(always)]
    pub fn set_pin(&mut self, pin: u32) {
        unsafe {
            let reg = self.base_ptr.add(RIO_OUT + RIO_SET / 4); // Pointer arithmetic is in T (u32), so divide byte offset by 4
            ptr::write_volatile(reg, 1 << pin);
        }
    }

    /// Atomically clear a pin low (Quantum 0).
    #[inline(always)]
    pub fn clr_pin(&mut self, pin: u32) {
        unsafe {
            let reg = self.base_ptr.add(RIO_OUT + RIO_CLR / 4);
            ptr::write_volatile(reg, 1 << pin);
        }
    }

    /// Atomically toggle a pin (Quantum Superposition / Phase Shift).
    #[inline(always)]
    pub fn xor_pin(&mut self, pin: u32) {
        unsafe {
            let reg = self.base_ptr.add(RIO_OUT + RIO_XOR / 4);
            ptr::write_volatile(reg, 1 << pin);
        }
    }

    /// Enable output driver for a pin.
    pub fn enable_output(&mut self, pin: u32) {
        unsafe {
            let reg = self.base_ptr.add(RIO_OE + RIO_SET / 4);
            ptr::write_volatile(reg, 1 << pin);
        }
    }

    /// Read the current state of the pin.
    pub fn read_pin(&self, pin: u32) -> bool {
        unsafe {
            let reg = self.base_ptr.add(RIO_IN / 4);
            let val = ptr::read_volatile(reg);
            (val & (1 << pin)) != 0
        }
    }
}

// Ensure clean unmap on drop
impl Drop for Rp1Rio {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.base_ptr as *mut _, 4096);
        }
    }
}
