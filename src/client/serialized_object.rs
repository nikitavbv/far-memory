use {
    std::{marker::PhantomData, ops::Deref},
    serde::{Serialize, de::DeserializeOwned},
    super::{FarMemoryClient, object::ObjectId},
};

pub struct FarMemorySerialized<T> {
    client: FarMemoryClient,
    object: ObjectId,
    _phantom: PhantomData<T>,
}

impl <'a, T: Serialize> FarMemorySerialized<T> {
    pub fn from_value(client: FarMemoryClient, value: T) -> Self {
        // TODO: use rkyv instead for better performance?
        let serialized = bincode::serialize(&value).unwrap();
        let object = client.put_object(serialized);

        Self {
            client,
            object,
            _phantom: PhantomData,
        }
    }
}

impl <T: DeserializeOwned> FarMemorySerialized<T> {
    pub fn to_local(&self) -> T {
        let location = self.client.get_object(&self.object);
        let bytes = unsafe {
            let ptr = self.client.span_ptr(&location.span_id).add(location.offset);
            std::slice::from_raw_parts(ptr, location.len)
        };

        let data = bincode::deserialize_from(bytes).unwrap();
        self.client.decrease_refs_for_span(&location.span_id);

        // returning just data, because it is owned, and spans refs are already decreased
        data
    }
}
