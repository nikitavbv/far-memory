use {
    std::{marker::PhantomData, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher},
    super::{
        object::{FarMemory, FarMemoryLocal},
        client::FarMemoryClient,
    },
};

pub struct FarMemoryHashMap<K, V> {
    client: FarMemoryClient,
    index: Vec<Option<FarMemoryHashMapNode<K, V>>>,

    _phantom: PhantomData<(K, V)>,
}

impl<K: Hash + PartialEq, V> FarMemoryHashMap<K, V> {
    pub fn new(client: FarMemoryClient, slots: usize) -> Self {
        Self {
            client,
            index: (0..slots).into_iter().map(|_| None).collect(),

            _phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.index_for_key(&key);
        let key = FarMemory::from_value(self.client.clone(), key);
        let value = FarMemory::from_value(self.client.clone(), value);

        let slot = self.index.get_mut(index).unwrap();
        if let Some(node) = slot {
            node.insert(key, value);
        } else {
            *slot = Some(FarMemoryHashMapNode::new(key, value));
        }
    }

    pub fn get(&self, key: &K, trace: bool) -> Option<FarMemoryLocal<V>> {
        let index = self.index_for_key(&key);

        let slot = self.index.get(index).unwrap();
        if let Some(node) = slot {
            let node_key = node.key.to_local(trace);
            if *node_key == *key {
                Some(node.value.to_local(trace))
            } else {
                node.get(key, trace)
            }
        } else {
            None
        }
    }

    fn index_for_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        (hash % self.index.len() as u64) as usize
    }
}

pub struct FarMemoryHashMapNode<K, V> {
    key: FarMemory<K>,
    value: FarMemory<V>,

    next: Option<Box<FarMemoryHashMapNode<K, V>>>,
}

impl<K: PartialEq, V> FarMemoryHashMapNode<K, V> {
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

    pub fn get(&self, key: &K, trace: bool) -> Option<FarMemoryLocal<V>> {
        if let Some(node) = &self.next {
            let node_key = node.key.to_local(trace);
            if *node_key == *key {
                Some(node.value.to_local(trace))
            } else {
                node.get(key, trace)
            }
        } else {
            None
        }
    }
}
