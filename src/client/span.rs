use {
    std::alloc::{GlobalAlloc, Layout},
    crate::utils::allocator::GLOBAL,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SpanId(u64);

pub struct LocalSpanData {
    ptr: *mut u8,
    size: usize,
}

pub enum FarMemorySpan {
    Local(LocalSpanData),
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

    pub fn free(self) {
        unsafe {
            GLOBAL.dealloc(self.ptr, span_layout(self.size));
        }
    }

    pub fn read_to_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.size)
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl FarMemorySpan {
    pub fn for_local_ptr_and_size(ptr: *mut u8, size: usize) -> Self {
        Self::Local(LocalSpanData::for_local_ptr_and_size(ptr, size))
    }

    pub fn new_local(size: usize) -> Self {
        Self::Local(LocalSpanData::new(size))
    }

    pub fn ptr(&self) -> *mut u8 {
        match self {
            FarMemorySpan::Local(local_data) => local_data.ptr.clone(),
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

    pub fn local_memory_usage(&self) -> usize {
        match self {
            FarMemorySpan::Local(local_data) => local_data.size(),
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

fn span_layout(span_size: usize) -> Layout {
    Layout::array::<u8>(span_size).unwrap()
}