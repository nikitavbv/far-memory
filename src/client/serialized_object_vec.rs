use {
    std::collections::HashMap,
    serde::{Serialize, de::DeserializeOwned},
    tracing::{span, Level},
    rand::Rng,
    super::{
        serialized_object::FarMemorySerialized,
        client::FarMemoryClient,
    },
};

pub struct FarMemorySerializedObjectVec<T> {
    client: FarMemoryClient,
    objects: Vec<FarMemorySerialized<T>>,
}

impl<T> FarMemorySerializedObjectVec<T> {
    pub fn new(client: FarMemoryClient) -> Self {
        Self {
            client,
            objects: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl<T: Serialize> FarMemorySerializedObjectVec<T> {
    pub fn push(&mut self, object: T) {
        self.objects.push(FarMemorySerialized::from_value(self.client.clone(), object));
    }
}

impl<T: DeserializeOwned> FarMemorySerializedObjectVec<T> {
    pub fn get(&self, index: usize, trace: bool) -> Option<T> {
        self.objects.get(index).map(|v| v.to_local(trace))
    }

    pub fn iter(&self, trace: bool) -> FarMemorySerializedObjectVecIterator<T> {
        FarMemorySerializedObjectVecIterator::new(self.objects.clone(), trace)
    }
}

pub struct FarMemorySerializedObjectVecIterator<T> {
    objects: Vec<FarMemorySerialized<T>>,
    remote_objects_by_span: HashMap<u64, Vec<FarMemorySerialized<T>>>,
    trace: bool,
}

impl<T> FarMemorySerializedObjectVecIterator<T> {
    pub fn new(objects: Vec<FarMemorySerialized<T>>, trace: bool) -> Self {
        Self {
            objects,
            remote_objects_by_span: HashMap::new(),
            trace,
        }
    }
}

impl<T: DeserializeOwned> Iterator for FarMemorySerializedObjectVecIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let span = span!(Level::DEBUG, "serialized object iterator - next");

        let _span = if self.trace {
            Some(span.enter())
        } else {
            None
        };

        if let Some(object) = self.objects.pop() {
            // we have objects that are not sorted yet
            if object.is_local() {
                return Some(object.to_local(self.trace));
            }

            // save remote object to remote objects by span
            let span_id = object.span().id();
            if !self.remote_objects_by_span.contains_key(&span_id) {
                self.remote_objects_by_span.insert(span_id, vec![]);
            }
            self.remote_objects_by_span.get_mut(&span_id).unwrap().push(object);
        }

        // go over remote objects by span and check if any of those is local yet
        let spans = self.remote_objects_by_span.keys().cloned().collect::<Vec<_>>();
        for span in spans {
            if self.remote_objects_by_span.get(&span).unwrap().is_empty() {
                self.remote_objects_by_span.remove(&span);
                continue;
            }

            let objects_in_span = self.remote_objects_by_span.get_mut(&span).unwrap();
            // found local span, lets return objects from it
            if objects_in_span.last().unwrap().is_local() {
                return Some(objects_in_span.pop().unwrap().to_local(self.trace));
            }
        }

        // time to swap something in
        while !self.remote_objects_by_span.is_empty() {
            let span = *self.remote_objects_by_span.keys().next().unwrap();
            let objects_in_span = self.remote_objects_by_span.get_mut(&span).unwrap();
            if objects_in_span.is_empty() {
                self.remote_objects_by_span.remove(&span);
                continue;
            }

            // swap in happens here
            return Some(objects_in_span.pop().unwrap().to_local(self.trace));
        }

        // no objects left
        None
    }
}
