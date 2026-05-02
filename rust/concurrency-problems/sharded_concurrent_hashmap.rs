// LLD - Sharded Concurrent HashMap (lock striping)
// Design a thread-safe map that scales under contention better than a single
// Mutex<HashMap>. Support get, insert, remove and atomic update-in-place.
// Approach: N shards, each an RwLock<HashMap>; the key's hash picks the shard, so
// operations on different shards proceed in parallel and reads share a shard.
// Time: O(1) average per op. Space: O(entries + shards).
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::sync::{Arc, RwLock};
use std::thread;

pub struct ShardedMap<K, V> {
    shards: Vec<RwLock<HashMap<K, V>>>,
    hasher: RandomState,
}

impl<K: Hash + Eq, V: Clone> ShardedMap<K, V> {
    pub fn new(shards: usize) -> Self {
        Self { shards: (0..shards.max(1)).map(|_| RwLock::new(HashMap::new())).collect(), hasher: RandomState::new() }
    }
    fn shard(&self, k: &K) -> &RwLock<HashMap<K, V>> {
        &self.shards[(self.hasher.hash_one(k) as usize) % self.shards.len()]
    }
    pub fn get(&self, k: &K) -> Option<V> {
        self.shard(k).read().unwrap().get(k).cloned()
    }
    pub fn insert(&self, k: K, v: V) -> Option<V> {
        self.shard(&k).write().unwrap().insert(k, v)
    }
    pub fn remove(&self, k: &K) -> Option<V> {
        self.shard(k).write().unwrap().remove(k)
    }
    /// Atomically read-modify-write the value for `k` (inserting `default` first).
    pub fn upsert(&self, k: K, default: V, f: impl FnOnce(&mut V)) {
        let mut g = self.shard(&k).write().unwrap();
        f(g.entry(k).or_insert(default));
    }
    pub fn len(&self) -> usize {
        self.shards.iter().map(|s| s.read().unwrap().len()).sum()
    }
}

fn main() {
    let m = Arc::new(ShardedMap::<u32, u64>::new(16));
    let hs: Vec<_> = (0..8)
        .map(|_| {
            let m = m.clone();
            thread::spawn(move || {
                for i in 0..10_000u32 {
                    m.upsert(i % 100, 0, |v| *v += 1);
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(m.len(), 100);
    assert_eq!(m.get(&7), Some(800));
    m.insert(1000, 1);
    assert_eq!(m.remove(&1000), Some(1));
    println!("keys={}, m[7]={:?}", m.len(), m.get(&7));
}
