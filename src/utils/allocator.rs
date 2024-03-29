use std::{sync::atomic::{AtomicU64, Ordering}, alloc::{GlobalAlloc, System, Layout}};

#[global_allocator]
pub static GLOBAL: TrackingAllocator = TrackingAllocator {
    memory_usage: AtomicU64::new(0),
};

pub struct TrackingAllocator {
    memory_usage: AtomicU64,
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.memory_usage.fetch_add(layout.size() as u64, Ordering::Relaxed);

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.memory_usage.fetch_sub(layout.size() as u64, Ordering::Relaxed);

        System.dealloc(ptr, layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            self.memory_usage.fetch_add((new_size - layout.size()) as u64, Ordering::Relaxed);
        } else {
            self.memory_usage.fetch_sub((layout.size() - new_size) as u64, Ordering::Relaxed);
        }

        System.realloc(ptr, layout, new_size)
    }
}

pub fn current_memory_usage() -> u64 {
    GLOBAL.memory_usage.load(Ordering::Relaxed)
}
