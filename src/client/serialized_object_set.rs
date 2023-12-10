use {
    serde::{Serialize, de::DeserializeOwned},
    super::{
        serialized_object::FarMemorySerialized,
        client::FarMemoryClient,
    },
};

pub struct FarMemorySerializedObjectSet<T> {
    client: FarMemoryClient,
    objects: Vec<FarMemorySerialized<T>>,
}

impl<T> FarMemorySerializedObjectSet<T> {
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

impl<T: Serialize> FarMemorySerializedObjectSet<T> {
    pub fn push(&mut self, object: T) {
        self.objects.push(FarMemorySerialized::from_value(self.client.clone(), object));
    }
}

impl<T: DeserializeOwned> FarMemorySerializedObjectSet<T> {
    pub fn get(&self, index: usize) -> T {
        self.objects.get(index).unwrap().to_local()
    }

    pub fn iter(&self) -> FarMemorySerializedObjectSetIterator<T> {
        FarMemorySerializedObjectSetIterator::new()
    }
}

pub struct FarMemorySerializedObjectSetIterator<T> {
    objects: Vec<FarMemorySerialized<T>>,
}

impl<T> FarMemorySerializedObjectSetIterator<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl<T> Iterator for FarMemorySerializedObjectSetIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
