use {
    std::{sync::{atomic::{AtomicU64, Ordering}, RwLock, Mutex}, collections::HashMap},
    super::span::SpanId,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ObjectId(u64);

#[derive(Clone)]
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

pub struct ObjectSlot {
    span_id: SpanId,
    offset: usize,
    len: usize,
}

pub struct ObjectRegistry {
    object_id_counter: AtomicU64,

    object_mapping: RwLock<HashMap<ObjectId, ObjectLocation>>,
    slots_by_size_class: Mutex<HashMap<usize, Vec<ObjectSlot>>>,
}

impl ObjectRegistry {
    pub fn new() -> Self {
        Self {
            object_id_counter: AtomicU64::new(0),
            object_mapping: RwLock::new(HashMap::new()),
            slots_by_size_class: Mutex::new(HashMap::new()),
        }
    }

    pub fn next_object_id(&self) -> ObjectId {
        ObjectId(self.object_id_counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn put_object(&self, object_id: ObjectId, object_size: usize) -> Option<ObjectLocation> {
        let size_class = size_class_for_object(object_size);
        let mut slots_map = self.slots_by_size_class.lock().unwrap();
        let slots_by_size_class = match slots_map.get_mut(&size_class) {
            Some(v) => v,
            None => return None, // no spans were allocated for this size class yet
        };
        if slots_by_size_class.is_empty() {
            // no free space left in this size class
            return None;
        }

        let slot = if slots_by_size_class[0].len > object_size {
            let remaining = ObjectSlot {
                span_id: slots_by_size_class[0].span_id.clone(),
                offset: slots_by_size_class[0].offset + object_size,
                len: slots_by_size_class[0].len - object_size,
            };
            std::mem::replace(&mut slots_by_size_class[0], remaining)
        } else if slots_by_size_class[0].len == object_size {
            slots_by_size_class.remove(0)
        } else {
            panic!("it is not expected that slot size is smaller than object size: {}", slots_by_size_class[0].len);
        };

        let location = ObjectLocation {
            span_id: slot.span_id,
            offset: slot.offset,
        };
        self.object_mapping.write().unwrap().insert(object_id, location.clone());

        Some(location)
    }

    pub fn add_span_for_object(&self, span_id: SpanId, span_size: usize, object_id: ObjectId, object_size: usize) -> ObjectLocation {
        let size_class = size_class_for_object(object_size);
        {
            let mut slots_by_size_class = self.slots_by_size_class.lock().unwrap();

            if !slots_by_size_class.contains_key(&size_class) {
                slots_by_size_class.insert(size_class, vec![]);
            }

            slots_by_size_class.get_mut(&size_class).unwrap().push(ObjectSlot {
                span_id,
                offset: 0,
                len: span_size,
            });
        }

        self.put_object(object_id, object_size).unwrap()
    }
}

fn size_class_for_object(object_size: usize) -> usize {
    if object_size != 8200 {
        panic!("this object size is not supported");
    }

    // for now, all objects have the same size class
    8200
}
