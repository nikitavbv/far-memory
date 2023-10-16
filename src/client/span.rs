use {
    std::{alloc::{GlobalAlloc, Layout}, ops::Range},
    crate::utils::allocator::GLOBAL,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SpanId(u64);

pub struct LocalSpanData {
    ptr: *mut u8,
    size: usize,
    in_use: bool, // currently used by application and should not be swapped out
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
            in_use: false,
        }
    }

    pub fn new(size: usize) -> Self {
        Self::for_local_ptr_and_size(unsafe { GLOBAL.alloc(span_layout(size)) }, size)
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        // TODO: just use vec pointer?
        let size = data.len();
        let ptr = unsafe {
            GLOBAL.alloc(span_layout(size))
        };

        unsafe {
            std::ptr::copy(data.as_slice() as *const _ as *const u8, ptr, size);
        };

        Self::for_local_ptr_and_size(ptr, size)
    }

    pub fn free(mut self) {
        self.free_memory()
    }

    fn free_memory(&mut self) {
        unsafe {
            GLOBAL.dealloc(self.ptr, span_layout(self.size));
        }
    }

    pub fn shrink(self, shrink_by: usize) -> Self {
        if shrink_by > self.size {
            panic!("cannot shrink by more than the current size of span");
        }

        let new_size = self.size - shrink_by;

        Self {
            ptr: unsafe {
                GLOBAL.realloc(self.ptr, span_layout(self.size), new_size)
            },
            size: new_size,
            in_use: self.in_use,
        }
    }

    pub fn extend_with_vec(self, data: Vec<u8>) -> Self {
        let new_size = self.size + data.len();
        let ptr = unsafe {
            GLOBAL.realloc(self.ptr, span_layout(self.size), new_size)
        };

        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr() as *mut u8, ptr.add(self.size), data.len());
        }
        
        Self {
            ptr,
            size: new_size,
            in_use: self.in_use,
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

    pub fn ptr(&self) -> *mut u8 {
        self.ptr.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn mark_in_use(&mut self, in_use: bool) {
        self.in_use = in_use;
    }

    pub fn is_in_use(&self) -> bool {
        self.in_use
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

    pub fn mark_in_use(&mut self, set_in_use: bool) {
        match self {
            FarMemorySpan::Local { data } => data.mark_in_use(set_in_use),
            FarMemorySpan::Remote { local_part, total_size: _ } => if let Some(local_part) = local_part {
                local_part.mark_in_use(set_in_use);
            }
        }
    }

    pub fn is_in_use(&self) -> bool {
        match self {
            FarMemorySpan::Local { data } => data.is_in_use(),
            FarMemorySpan::Remote { local_part, total_size: _ } => local_part.as_ref().map(|v| v.is_in_use()).unwrap_or(false),
        }
    }
}

impl Drop for LocalSpanData {
    fn drop(&mut self) {
        self.free_memory()
    }
}

fn span_layout(span_size: usize) -> Layout {
    Layout::array::<u8>(span_size).unwrap()
}