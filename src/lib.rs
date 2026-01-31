use std::collections::HashMap;
use std::hash::Hash;

mod hash;
pub use hash::AssemblyHash;

pub const CACHE_SIZE: usize = 128;

struct Entry<K, V> {
    key: K,
    val: Option<V>,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LRUCache<K, V> {
    capacity: usize,
    head: Option<usize>, // MRU
    tail: Option<usize>, // LRU
    map: HashMap<K, usize>,
    entries: Vec<Entry<K, V>>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            head: None,
            tail: None,
            map: HashMap::new(),
            entries: Vec::with_capacity(capacity),
        }
    }

    /* ---------------- internal list helpers ---------------- */

    fn detach(&mut self, i: usize) {
        let (prev, next) = {
            let e = &self.entries[i];
            (e.prev, e.next)
        };

        if let Some(p) = prev {
            self.entries[p].next = next;
        } else {
            // i was head
            self.head = next;
        }

        if let Some(n) = next {
            self.entries[n].prev = prev;
        } else {
            // i was tail
            self.tail = prev;
        }

        self.entries[i].prev = None;
        self.entries[i].next = None;
    }

    fn push_front(&mut self, i: usize) {
        self.entries[i].prev = None;
        self.entries[i].next = self.head;

        if let Some(old_head) = self.head {
            self.entries[old_head].prev = Some(i);
        } else {
            // list was empty
            self.tail = Some(i);
        }

        self.head = Some(i);
    }

    fn access(&mut self, key: &K) {
        let i = self.map[key];

        if Some(i) == self.head {
            return; // already MRU
        }

        self.detach(i);
        self.push_front(i);
    }

    fn remove_tail(&mut self) {
        if let Some(i) = self.tail {
            let key = self.entries[i].key.clone();
            self.detach(i);
            self.map.remove(&key);
            self.entries[i].val = None;
        }
    }

    /* ---------------- public API ---------------- */

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(&i) = self.map.get(&key) {
            let old = self.entries[i].val.replace(value);
            self.detach(i);
            self.push_front(i);
            return old;
        }

        if self.len() == self.capacity {
            self.remove_tail();
        }

        let i = self.entries.len();
        self.entries.push(Entry {
            key: key.clone(),
            val: Some(value),
            prev: None,
            next: None,
        });

        self.push_front(i);
        self.map.insert(key, i);

        None
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.access(key);
        }

        self.map
            .get(key)
            .and_then(|&i| self.entries[i].val.as_ref())
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            self.access(key);
        }

        self.map
            .get(key)
            .and_then(|&i| self.entries[i].val.as_mut())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|i| {
            self.detach(i);
            self.entries[i].val.take().unwrap()
        })
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
