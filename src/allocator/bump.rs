use core::ptr;

use alloc::alloc::{GlobalAlloc, Layout};

use super::{align_up, Locked};

pub type BumpAllocator = Locked<_BumpAllocator>;

impl BumpAllocator {
    pub const fn create() -> Self {
        Locked::new(_BumpAllocator::new())
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        let heap_end = bump.heap_start + bump.heap_size - 1;

        if alloc_end > heap_end {
            ptr::null_mut()
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut bump = self.lock();

        if bump.next == (ptr as usize + layout.size()) {
            bump.next = ptr as usize;
            bump.allocations -= 1;
        } else {
            bump.allocations -= 1;
            if bump.allocations == 0 {
                bump.next = bump.heap_start;
            }
        }
    }
}

pub struct _BumpAllocator {
    heap_start: usize,
    heap_size: usize,
    next: usize,
    allocations: usize,
}

impl _BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_size: 0,
            next: 0,
            allocations: 0,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_size = heap_size;
        self.next = heap_start;
    }
}
