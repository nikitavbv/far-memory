use std::marker::PhantomData;

pub struct FarMemoryHashMap<K, V> {
    index: Vec<Option<FarMemoryHashMapNode<K, V>>>,

    _phantom: PhantomData<(K, V)>,
}

impl<K, V> FarMemoryHashMap<K, V> {
    pub fn new(slots: usize) -> Self {
        Self {
            index: Vec::new(),

            _phantom: PhantomData,
        }
    }
}

pub struct FarMemoryHashMapNode<K, V> {
    key: K,
    value: V,
}
