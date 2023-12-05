use super::backend::SwapOutOperationData;

use {
    std::{alloc::{GlobalAlloc, Layout}, ops::Range},
    tracing::{span, Level},
    serde::{Serialize, Deserialize},
    crate::utils::allocator::GLOBAL,
};

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SpanId(u64);

// not sure if there is a cheap way to avoid this
unsafe impl Send for LocalSpanData {}
unsafe impl Sync for LocalSpanData {}

pub struct LocalSpanData {
    ptr: *mut u8,
    size: usize,
}

pub enum FarMemorySpan {
    Local {
        data: LocalSpanData,
    },
    Remote {
        // spans can be large, so it is possible that span is only partially swapped out (to optimize latency). For example, it does not
        // make sense to swap out the full 180MB span if the system requires just 10MB more free memory.
        local_part: Option<LocalSpanData>,
        // remote + local
        total_size: usize,
    },
}

impl SpanId {
    pub fn from_id(id: u64) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u64 {
        self.0
    }
}

impl LocalSpanData {
    pub fn for_local_ptr_and_size(ptr: *mut u8, size: usize) -> Self {
        Self {
            ptr,
            size,
        }
    }

    pub fn new(size: usize) -> Self {
        Self::for_local_ptr_and_size(unsafe { GLOBAL.alloc(span_layout(size)) }, size)
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        let ptr = data.as_ptr() as *mut u8;
        let size = data.len();
        std::mem::forget(data);

        Self::for_local_ptr_and_size(ptr, size)
    }

    pub fn free(mut self) {
        self.free_memory()
    }

    fn free_memory(&mut self) {
        if self.ptr != std::ptr::null_mut() {
            span!(Level::DEBUG, "free span memory").in_scope(|| {
                unsafe {
                    GLOBAL.dealloc(self.ptr, span_layout(self.size));
                }
                self.ptr = std::ptr::null_mut();
            })
        }
    }

    pub fn shrink(mut self, shrink_by: usize) -> Self {
        if shrink_by > self.size {
            panic!("cannot shrink by more than the current size of span");
        }

        let new_size = self.size - shrink_by;
        let prev_ptr = self.ptr;
        self.ptr = std::ptr::null_mut();

        Self {
            ptr: unsafe {
                GLOBAL.realloc(prev_ptr, span_layout(self.size), new_size)
            },
            size: new_size,
        }
    }

    pub fn extend_with_vec(mut self, data: Vec<u8>) -> Self {
        let new_size = self.size + data.len();
        let ptr = unsafe {
            GLOBAL.realloc(self.ptr, span_layout(self.size), new_size)
        };
        self.ptr = std::ptr::null_mut();

        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr() as *mut u8, ptr.add(self.size), data.len());
        }

        Self {
            ptr,
            size: new_size,
        }
    }

    pub fn read_to_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.size)
        }
    }

    pub fn read_to_slice_with_range(&self, range: Range<usize>) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.add(range.start), range.len())
        }
    }

    pub fn into_vec(mut self) -> Vec<u8> {
        let vec = unsafe {
          Vec::from_raw_parts(self.ptr, self.size, self.size)
        };
        self.ptr = std::ptr::null_mut();
        vec
    }

    pub fn to_swap_out_operation_data_with_range(&self, range: Range<usize>) -> SwapOutOperationData {
        // not safe and not nice, but good enough for now
        unsafe {
            SwapOutOperationData::ReadFrom { ptr: self.ptr.add(range.start), size: range.len() }
        }
    }

    pub fn ptr(&self) -> *mut u8 {
        self.ptr.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl FarMemorySpan {
    pub fn for_local_ptr_and_size(ptr: *mut u8, size: usize) -> Self {
        Self::Local {
            data: LocalSpanData::for_local_ptr_and_size(ptr, size),
        }
    }

    pub fn new_local(size: usize) -> Self {
        Self::Local {
            data: LocalSpanData::new(size),
        }
    }

    pub fn ptr(&self) -> *mut u8 {
        match self {
            FarMemorySpan::Local { data  } => data.ptr.clone(),
            FarMemorySpan::Remote { .. } => panic!("cannot provide a ptr for remote span"),
        }
    }

    pub fn is_local(&self) -> bool {
        match self {
            FarMemorySpan::Local { .. } => true,
            FarMemorySpan::Remote { .. } => false,
        }
    }

    pub fn is_remote(&self) -> bool {
        match self {
            FarMemorySpan::Local { .. } => false,
            FarMemorySpan::Remote { .. } => true,
        }
    }

    pub fn total_size(&self) -> usize {
        self.local_memory_usage() + self.remote_memory_usage()
    }

    pub fn local_memory_usage(&self) -> usize {
        match self {
            FarMemorySpan::Local { data  } => data.size(),
            FarMemorySpan::Remote { local_part, total_size: _ } => local_part.as_ref().map(|v| v.size()).unwrap_or(0),
        }
    }

    pub fn remote_memory_usage(&self) -> usize {
        match self {
            FarMemorySpan::Local { .. } => 0,
            FarMemorySpan::Remote { local_part, total_size } => total_size - local_part.as_ref().map(|v| v.size()).unwrap_or(0),
        }
    }
}

impl Drop for LocalSpanData {
    fn drop(&mut self) {
        span!(Level::DEBUG, "LocalSpanData drop").in_scope(|| self.free_memory())
    }
}

fn span_layout(span_size: usize) -> Layout {
    Layout::array::<u8>(span_size).unwrap()
}
