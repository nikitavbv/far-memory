use {
    std::{marker::PhantomData, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher},
    super::{
        object::FarMemory,
        client::FarMemoryClient,
    },
};

pub struct FarMemoryHashMap<K, V> {
    client: FarMemoryClient,
    index: Vec<Option<FarMemoryHashMapNode<K, V>>>,

    _phantom: PhantomData<(K, V)>,
}

impl<K: Hash, V> FarMemoryHashMap<K, V> {
    pub fn new(client: FarMemoryClient, slots: usize) -> Self {
        Self {
            client,
            index: (0..slots).into_iter().map(|_| None).collect(),

            _phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        let key = FarMemory::from_value(self.client.clone(), key);
        let value = FarMemory::from_value(self.client.clone(), value);

        let index = (hash % self.index.len() as u64) as usize;
        let mut slot = self.index.get_mut(index).unwrap();
        if let Some(node) = slot {
            node.insert(key, value);
        } else {
            *slot = Some(FarMemoryHashMapNode::new(key, value));
        }
    }
}

pub struct FarMemoryHashMapNode<K, V> {
    key: FarMemory<K>,
    value: FarMemory<V>,

    next: Option<Box<FarMemoryHashMapNode<K, V>>>,
}

impl<K, V> FarMemoryHashMapNode<K, V> {
    pub fn new(key: FarMemory<K>, value: FarMemory<V>) -> Self {
        Self {
            key,
            value,

            next: None,
        }
    }

    pub fn insert(&mut self, key: FarMemory<K>, value: FarMemory<V>) {
        if let Some(node) = &mut self.next {
            node.insert(key, value);
        } else {
            self.next = Some(Box::new(FarMemoryHashMapNode::new(key, value)));
        }
    }
}
