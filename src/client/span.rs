#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SpanId(u64);

pub enum FarMemorySpan {
    Local {
        ptr: *mut u8,
        size: usize,   
    },
    Remote {
        size: usize,
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

impl FarMemorySpan {
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
            FarMemorySpan::Local { ptr: _, size } => *size,
            FarMemorySpan::Remote { .. } => 0,
        }
    }

    pub fn remote_memory_usage(&self) -> usize {
        match self {
            FarMemorySpan::Local { .. } => 0,
            FarMemorySpan::Remote { size } => *size,
        } 
    }
}