use memmap2::MmapMut;
use std::fs::OpenOptions;

use std::os::unix::fs::OpenOptionsExt;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct D16ShmLayout {
    pub channels: [u32; 16],
    pub timestamp: u32,
}

pub struct D16ShmWriter {
    mmap: MmapMut,
}

impl D16ShmWriter {
    pub fn new() -> Option<Self> {
        let path = "/dev/shm/d16_state";
        println!("   [SHM] Creating/Opening {}", path);

        let file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o666) // RW for everyone
            .open(path)
        {
            Ok(f) => f,
            Err(e) => {
                println!("   [SHM] Failed to open file: {}", e);
                return None;
            }
        };

        let size = std::mem::size_of::<D16ShmLayout>() as u64;
        if let Err(e) = file.set_len(size) {
            println!("   [SHM] Failed to set length: {}", e);
            return None;
        }

        let mmap = unsafe {
            match MmapMut::map_mut(&file) {
                Ok(m) => m,
                Err(e) => {
                    println!("   [SHM] Failed to mmap: {}", e);
                    return None;
                }
            }
        };

        println!("   [SHM] Shared Memory Initialized Successfully.");
        Some(Self { mmap })
    }

    pub fn write(&mut self, channels: [u32; 16], timestamp: u32) {
        let layout = D16ShmLayout {
            channels,
            timestamp,
        };

        unsafe {
            let src_ptr = &layout as *const D16ShmLayout as *const u8;
            let dst_ptr = self.mmap.as_mut_ptr();
            std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, std::mem::size_of::<D16ShmLayout>());
        }
    }
}
