use {
    serde::Serialize,
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
}

impl<T: Serialize> FarMemorySerializedObjectVec<T> {
    pub fn push(&mut self, object: T) {
        self.objects.push(FarMemorySerialized::from_value(self.client.clone(), object));
    }
}
