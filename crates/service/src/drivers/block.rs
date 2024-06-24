// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use easy_fs::BlockDevice;
use ksync::UPSafeCell;
use alloc::{sync::Arc, vec::Vec};
use lazy_static::*;
use virtio_drivers::{Hal, VirtIOBlk, VirtIOHeader};

#[allow(unused)]
const VIRTIO0: usize = 0x10001000;

pub type BlockDeviceImpl = crate::drivers::block::VirtIOBlock;

pub struct VirtIOBlock(UPSafeCell<VirtIOBlk<'static, VirtioHal>>);

// lazy_static! {
//     static ref QUEUE_FRAMES: UPSafeCell<Vec<FrameTracker>> = unsafe { UPSafeCell::new(Vec::new()) };
// }

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0
            .borrow_mut()
            .read_block(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .borrow_mut()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}

impl VirtIOBlock {
    #[allow(unused)]
    pub fn new() -> Self {
        unsafe {
            Self(UPSafeCell::new(
                VirtIOBlk::<VirtioHal>::new(&mut *(VIRTIO0 as *mut VirtIOHeader)).unwrap(),
            ))
        }
    }
}

pub struct VirtioHal;

// impl Hal for VirtioHal {
//     fn dma_alloc(pages: usize) -> usize {
//         let mut ppn_base = PhysPageNum(0);
//         for i in 0..pages {
//             let frame = frame_alloc().unwrap();
//             if i == 0 {
//                 ppn_base = frame.ppn;
//             }
//             assert_eq!(frame.ppn.0, ppn_base.0 + i);
//             QUEUE_FRAMES.borrow_mut().push(frame);
//         }
//         let pa: PhysAddr = ppn_base.into();
//         pa.0
//     }

//     fn dma_dealloc(pa: usize, pages: usize) -> i32 {
//         let pa = PhysAddr::from(pa);
//         let mut ppn_base: PhysPageNum = pa.into();
//         for _ in 0..pages {
//             frame_dealloc(ppn_base);
//             ppn_base.step();
//         }
//         0
//     }

//     fn phys_to_virt(addr: usize) -> usize {
//         addr
//     }

//     fn virt_to_phys(vaddr: usize) -> usize {
//         PageTable::from_token(kernel_token())
//             .translate_va(VirtAddr::from(vaddr))
//             .unwrap()
//             .0
//     }
// }

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = Arc::new(BlockDeviceImpl::new());
}

#[allow(unused)]
pub fn block_device_test() {
    let block_device = BLOCK_DEVICE.clone();
    let mut write_buffer = [0u8; 512];
    let mut read_buffer = [0u8; 512];
    for i in 0..512 {
        for byte in write_buffer.iter_mut() {
            *byte = i as u8;
        }
        block_device.write_block(i as usize, &write_buffer);
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    println!("block device test passed!");
}
