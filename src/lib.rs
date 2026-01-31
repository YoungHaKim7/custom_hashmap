use std::{collections::HashMap, hash::Hash};

const CHACHE_SIZE: usize = 128;

struct Entry<K, V> {
    key: K,
    val: Option<V>,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LRUCache<K, V> {
    capacity: usize,
    head: Option<usize>,
    tail: Option<usize>,
    map: HashMap<K, usize>,
    entries: Vec<Entry<K, V>>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            head: None,
            tail: None,
            map: HashMap::new(),
            entries: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.map.contains_key(&key) {
            self.access(&key);
            let entry = &mut self.entries[self.head.unwrap()];
            let old_val = entry.val.take();
            entry.val = Some(value);
            old_val
        } else {
            self.ensure_room();

            let index = self.entries.len();
            if let Some(e) = self.head {
                self.entries[e].next = Some(index);
            }

            self.entries.push(Entry {
                key: key.clone(),
                val: Some(value),
                prev: None,
                next: self.head,
            });
            self.head = Some(index);
            self.tail = self.tail.or(self.head);
            self.map.insert(key, index);

            None
        }
    }

    fn access(&mut self, key: &K) {
        let i = *self.map.get(key).unwrap();
        self.remove_from_list(i);
        self.head = Some(i)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    fn remove_from_list(&mut self, i: usize) {
        let (prev, next) = {
            let entry = self.entries.get_mut(i).unwrap();
            (entry.prev, entry.next)
        };

        match (prev, next) {
            (Some(j), Some(k)) => {
                let head = &mut self.entries[j];
            }
            (Some(j), None) => {
                let head = &mut self.entries[j];
                head.next = None;
                self.tail = prev;
            }
            _ => {
                if self.len() > 1 {
                    let head = &mut self.entries[0];
                    head.next = None;
                    let next = &mut self.entries[1];
                    next.prev = None;
                }
            }
        }
    }

    fn ensure_room(&mut self) {
        if self.capacity == self.len() {
            self.remove_tail();
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|index| {
            self.remove_from_list(index);
            self.entries[index].val.take().unwrap()
        })
    }

    fn remove_tail(&mut self) {
        if let Some(index) = self.tail {
            self.remove_from_list(index);
            let key = &self.entries[index].key;
            self.map.remove(key);
        }

        if self.tail.is_none() {
            self.head = None;
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &self.entries;
        self.map
            .get(key)
            .and_then(move |&i| entries[i].val.as_ref())
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &mut self.entries;
        self.map
            .get(key)
            .and_then(move |&i| entries[i].val.as_mut())
    }

    fn empty(&mut self, key: &K) -> bool {
        self.map.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.map.len() == self.capacity
    }
}
