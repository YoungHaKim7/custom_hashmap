pub trait AssmblyHash {
    fn new(capacity: usize) -> Self;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn access(&mut self, key: &K);

    fn contains(&self, key: &K) -> bool;

    fn remove_from_list(&mut self, i: usize);

    fn ensure_room(&mut self);

    fn len(&self) -> usize;

    fn remove(&mut self, key: &K) -> Option<V>;

    fn remove_tail(&mut self);

    fn get(&mut self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn empty(&self) -> bool;

    fn is_full(&self) -> bool;
}
