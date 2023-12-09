use {
    std::{marker::PhantomData, ops::Deref},
    serde::Serialize,
    super::{FarMemoryClient, object::ObjectId},
};

pub struct FarMemorySerialized<T> {
    client: FarMemoryClient,
    object: ObjectId,
    _phantom: PhantomData<T>,
}

impl <T: Serialize> FarMemorySerialized<T> {
    pub fn from_value(client: FarMemoryClient, value: T) -> Self {
        let serialized = bincode::serialize(&value).unwrap();
        let object = client.put_object(serialized);

        Self {
            client,
            object,
            _phantom: PhantomData,
        }
    }
}

impl<T> Deref for FarMemorySerialized<T> {
    type Target = FarMemorySerializedLocal<T>;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

pub struct FarMemorySerializedLocal<T> {
    client: FarMemoryClient,
    object: ObjectId,
    data: T,
}

impl<T> Deref for FarMemorySerializedLocal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}
