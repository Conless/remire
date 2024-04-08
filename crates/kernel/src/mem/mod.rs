use core::alloc::GlobalAlloc;

use crate::sync::UPSafeCell;

use self::buddy::BuddyAllocator;

mod buddy;
mod avl;

const KERNEL_HEAP_SIZE: usize = 0x1000000;

static mut KERNEL_HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub struct UpBuddyAllocator {
    inner: UPSafeCell<BuddyAllocator>,
}

impl UpBuddyAllocator {
    pub const fn empty() -> Self {
        Self {
            inner: unsafe { UPSafeCell::new(BuddyAllocator::empty()) },
        }
    }

    pub fn init(&self) {
        unsafe {
            let start = KERNEL_HEAP_SPACE.as_ptr() as usize;
            let end = start + KERNEL_HEAP_SPACE.len();
            self.inner.borrow_mut().add_segment(start, end);
        }
    }
}

unsafe impl GlobalAlloc for UpBuddyAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.inner.borrow_mut().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.inner.borrow_mut().dealloc(ptr, layout);
    }
}

#[global_allocator]
pub static HEAP_ALLOCATOR: UpBuddyAllocator = UpBuddyAllocator::empty();
