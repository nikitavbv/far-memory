use {
    serde::{Serialize, de::DeserializeOwned},
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
    pub fn get(&self, index: usize) -> T {
        self.objects.get(index).unwrap().to_local()
    }

    pub fn iter(&self) -> FarMemorySerializedObjectVecIterator<T> {
        FarMemorySerializedObjectVecIterator::new()
    }
}

pub struct FarMemorySerializedObjectVecIterator<T> {
    objects: Vec<FarMemorySerialized<T>>,
}

impl<T> FarMemorySerializedObjectVecIterator<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl<T> Iterator for FarMemorySerializedObjectVecIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
