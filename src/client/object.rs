use {
    std::{
        sync::{atomic::{AtomicU64, Ordering}, RwLock, Mutex},
        collections::HashMap,
        marker::PhantomData,
        ops::Deref,
    },
    super::{
        span::SpanId,
        client::FarMemoryClient,
    },
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ObjectId(u64);

#[derive(Clone)]
pub struct ObjectLocation {
    pub span_id: SpanId,
    pub offset: usize,
    pub len: usize,
}

impl ObjectLocation {
    pub fn new(span_id: SpanId, offset: usize, len: usize) -> Self {
        Self {
            span_id,
            offset,
            len,
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
        let size_class = self.size_class_for_object(object_size);
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
            let remaining_len = slots_by_size_class[0].len - size_class;

            if remaining_len > 0 {
                let remaining = ObjectSlot {
                    span_id: slots_by_size_class[0].span_id.clone(),
                    offset: slots_by_size_class[0].offset + size_class,
                    len: remaining_len,
                };
                std::mem::replace(&mut slots_by_size_class[0], remaining)
            } else {
                slots_by_size_class.remove(0)
            }
        } else if slots_by_size_class[0].len == object_size {
            slots_by_size_class.remove(0)
        } else {
            panic!("it is not expected that slot size is smaller than object size: {}, size class is {}", slots_by_size_class[0].len, size_class);
        };

        let location = ObjectLocation {
            span_id: slot.span_id,
            offset: slot.offset,
            len: object_size,
        };
        self.object_mapping.write().unwrap().insert(object_id, location.clone());

        Some(location)
    }

    pub fn add_span_for_object(&self, span_id: SpanId, span_size: usize, object_id: ObjectId, object_size: usize) -> ObjectLocation {
        let size_class = self.size_class_for_object(object_size);
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

    pub fn get_object(&self, object_id: &ObjectId) -> ObjectLocation {
        self.object_mapping.read().unwrap().get(object_id).unwrap().clone()
    }

    pub fn size_class_for_object(&self, object_size: usize) -> usize {
        // TODO: implement actual size classes

        // for flights in dataframe demo
        if object_size >= 110 && object_size <= 120 {
            return 120;
        } else if object_size >= 310 && object_size <= 400 {
            return 400;
        } else if object_size >= 400 && object_size <= 500 {
            return 500;
        }

        if object_size != 8200 && object_size != 8 {
            panic!("this object size is not supported: {:?}", object_size);
        }

        object_size
    }
}

pub struct FarMemory<T> {
    client: FarMemoryClient,
    object: ObjectId,
    _phantom: PhantomData<T>,
}

impl<T> FarMemory<T> {
    pub fn from_value(client: FarMemoryClient, value: T) -> Self {
        let object = client.put_object(unsafe {
            std::slice::from_raw_parts(
                (&value as *const _) as *const u8,
                std::mem::size_of::<T>()
            ).to_vec()
        });

        Self {
            client,
            object,
            _phantom: PhantomData,
        }
    }

    pub fn to_local(&self) -> FarMemoryLocal<T> {
        FarMemoryLocal {
            client: self.client.clone(),
            object: self.object.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Deref for FarMemory<T> {
    type Target = FarMemoryLocal<T>;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

pub struct FarMemoryLocal<T> {
    client: FarMemoryClient,
    object: ObjectId,
    _phantom: PhantomData<T>,
}

impl<T> Deref for FarMemoryLocal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let location = self.client.get_object(&self.object);
        unsafe {
            &*(self.client.span_ptr(&location.span_id).add(location.offset) as *const T)
        }
    }
}

impl<T> Drop for FarMemoryLocal<T> {
    fn drop(&mut self) {
        self.client.decrease_refs_for_span(&self.client.get_object(&self.object).span_id);
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::client::InMemoryBackend,
        super::*,
    };

    struct TestValue {
        v: u64,
    }

    #[test]
    fn simple() {
        let client = FarMemoryClient::new(Box::new(InMemoryBackend::new()), 10 * 1024 * 1024);
        let object = FarMemory::from_value(client, TestValue { v: 42 });

        assert_eq!(42, object.to_local().v);
    }
}
