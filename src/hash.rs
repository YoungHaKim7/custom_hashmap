#[cfg(target_os = "linux")]
#[inline(always)]
pub unsafe fn look_up_identifier(data: *const u8, len: usize) -> u32 {
    unsafe {
        let result: u32;
        std::arch::asm!(
            "cmp rsi, 4",
            "jb 2f",
            "movzx eax, WORD PTR [rdi]",
            "movzx edx, WORD PTR [rdi + rsi - 2]",
            "shl eax, 16",
            "or eax, edx",
            "jmp 3f",
            "2:",
            "mov eax, 80",
            "3:",
            in("rdi") data,
            in("rsi") len,
            lateout("eax") result,
            clobber_abi("system")
        );
        result
    }
}

#[cfg(target_os = "macos")]
#[inline(always)]
pub unsafe fn look_up_identifier(data: *const u8, len: usize) -> u32 {
    unsafe {
        let result: u32;
        std::arch::asm!(
            "cmp {len}, #4",
            "b.lo 2f",
            "ldrh w0, [{data}]",
            "sub x1, {len}, #2",
            "ldrh w2, [{data}, x1]",
            "lsl w0, w0, #16",
            "orr w0, w0, w2",
            "b 3f",
            "2:",
            "mov w0, #80",
            "3:",
            data = in(reg) data,
            len = in(reg) len,
            lateout("w0") result,
            clobber_abi("system")
        );
        result
    }
}

/// Trait for types that can be hashed using the assembly hash function
pub trait AssemblyHash {
    fn assembly_hash(&self) -> u32;
}

impl AssemblyHash for String {
    fn assembly_hash(&self) -> u32 {
        self.as_str().assembly_hash()
    }
}

impl AssemblyHash for &str {
    fn assembly_hash(&self) -> u32 {
        unsafe { look_up_identifier(self.as_ptr(), self.len()) }
    }
}

// Implement for common integer types
impl AssemblyHash for u32 {
    fn assembly_hash(&self) -> u32 {
        *self
    }
}

impl AssemblyHash for u64 {
    fn assembly_hash(&self) -> u32 {
        (*self >> 32) as u32 ^ *self as u32
    }
}

impl AssemblyHash for usize {
    fn assembly_hash(&self) -> u32 {
        *self as u32
    }
}

impl AssemblyHash for i32 {
    fn assembly_hash(&self) -> u32 {
        *self as u32
    }
}

impl AssemblyHash for i64 {
    fn assembly_hash(&self) -> u32 {
        (*self as u64).assembly_hash()
    }
}

impl AssemblyHash for isize {
    fn assembly_hash(&self) -> u32 {
        *self as u32
    }
}

/// Entry in the hash table bucket
enum BucketEntry<K, V> {
    Occupied(K, V),
    Removed,
}

impl<K: Clone, V: Clone> Clone for BucketEntry<K, V> {
    fn clone(&self) -> Self {
        match self {
            BucketEntry::Occupied(k, v) => BucketEntry::Occupied(k.clone(), v.clone()),
            BucketEntry::Removed => BucketEntry::Removed,
        }
    }
}

/// Custom HashMap using open addressing with linear probing and the assembly hash
pub struct CustomHashMap<K, V> {
    buckets: Vec<Option<BucketEntry<K, V>>>,
    len: usize,
}

impl<K, V> CustomHashMap<K, V>
where
    K: Eq + AssemblyHash + Clone,
{
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let cap = capacity.max(1).next_power_of_two();
        let mut buckets = Vec::with_capacity(cap);
        for _ in 0..cap {
            buckets.push(None);
        }
        Self { buckets, len: 0 }
    }

    fn bucket_index(&self, key: &K) -> usize {
        let hash = key.assembly_hash() as usize;
        hash % self.buckets.len()
    }

    fn find_slot(&self, key: &K) -> (usize, Option<usize>) {
        let mut idx = self.bucket_index(key);
        let first_removed = None;

        loop {
            match &self.buckets[idx] {
                None => return (idx, first_removed),
                Some(BucketEntry::Removed) => return (idx, Some(idx)),
                Some(BucketEntry::Occupied(k, _)) if k == key => return (idx, first_removed),
                _ => {}
            }
            idx = (idx + 1) % self.buckets.len();
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.ensure_capacity();

        let (idx, first_removed) = self.find_slot(&key);

        // If key exists, update value
        if let Some(BucketEntry::Occupied(k, v)) = &mut self.buckets[idx]
            && k == &key
        {
            return Some(std::mem::replace(v, value));
        }

        // Insert at first removed slot or the found slot
        let slot = first_removed.unwrap_or(idx);
        self.buckets[slot] = Some(BucketEntry::Occupied(key, value));
        self.len += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut idx = self.bucket_index(key);
        let start_idx = idx;

        loop {
            match &self.buckets[idx] {
                None => break,
                Some(BucketEntry::Removed) => {}
                Some(BucketEntry::Occupied(k, v)) if k == key => return Some(v),
                Some(_) => {}
            }
            idx = (idx + 1) % self.buckets.len();
            if idx == start_idx {
                break;
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut idx = self.bucket_index(key);
        let start_idx = idx;

        for _ in 0..self.buckets.len() {
            if let Some(BucketEntry::Occupied(k, _)) = &self.buckets[idx] {
                if k == key {
                    // SAFETY: We're at a valid index and the key matches
                    // We can safely return a mutable reference since we won't
                    // access this bucket again in this function
                    return match &mut self.buckets[idx] {
                        Some(BucketEntry::Occupied(_, v)) => Some(v),
                        _ => None,
                    };
                }
            } else if self.buckets[idx].is_none() {
                break;
            }
            idx = (idx + 1) % self.buckets.len();
            if idx == start_idx {
                break;
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut idx = self.bucket_index(key);
        let start_idx = idx;

        loop {
            match &self.buckets[idx] {
                None => break,
                Some(BucketEntry::Removed) => {}
                Some(BucketEntry::Occupied(k, _)) if k == key => {
                    self.len -= 1;
                    // Take ownership and mark as removed
                    let entry = self.buckets[idx].take();
                    self.buckets[idx] = Some(BucketEntry::Removed);
                    if let Some(BucketEntry::Occupied(_, v)) = entry {
                        return Some(v);
                    }
                }
                Some(_) => {}
            }
            idx = (idx + 1) % self.buckets.len();
            if idx == start_idx {
                break;
            }
        }

        None
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn ensure_capacity(&mut self) {
        // Resize at 75% load factor
        if self.len * 4 >= self.buckets.len() * 3 {
            self.resize(self.buckets.len() * 2);
        }
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_buckets = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            new_buckets.push(None);
        }
        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);
        self.len = 0;

        for entry in old_buckets {
            if let Some(BucketEntry::Occupied(key, value)) = entry {
                self.insert(key, value);
            }
        }
    }
}

impl<K, V> Default for CustomHashMap<K, V>
where
    K: Eq + AssemblyHash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clone for CustomHashMap<K, V>
where
    K: Eq + AssemblyHash + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        let mut new = Self::with_capacity(self.buckets.len());
        for entry in &self.buckets {
            if let Some(BucketEntry::Occupied(key, value)) = entry {
                new.insert(key.clone(), value.clone());
            }
        }
        new
    }
}
