use {
    std::{sync::{atomic::{AtomicU64, Ordering}, RwLock}, collections::HashMap},
    super::span::SpanId,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ObjectId(u64);

pub struct ObjectLocation {
    pub span_id: SpanId,
    pub offset: usize,
}

impl ObjectLocation {
    pub fn new(span_id: SpanId, offset: usize) -> Self {
        Self {
            span_id,
            offset,
        }
    }
}

pub struct ObjectRegistry {
    object_id_counter: AtomicU64,

    object_mapping: RwLock<HashMap<ObjectId, ObjectLocation>>,
}

impl ObjectRegistry {
    pub fn new() -> Self {
        Self {
            object_id_counter: AtomicU64::new(0),
            object_mapping: RwLock::new(HashMap::new()),
        }
    }

    pub fn next_object_id(&self) -> ObjectId {
        ObjectId(self.object_id_counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn put_object(&self, object_id: ObjectId, object_size: usize) -> Option<ObjectLocation> {
        let size_class = size_class_for_object(object_size);
        // TODO: pick location by size class
        unimplemented!()
    }

    pub fn add_span_for_object(&self, span_id: SpanId, span_size: usize, object_id: ObjectId, object_size: usize) -> ObjectLocation {
        unimplemented!()
    }
}

fn size_class_for_object(object_size: usize) -> usize {
    if object_size != 8200 {
        panic!("this object size is not supported");
    }

    // for now, all objects have the same size class
    8200
}
