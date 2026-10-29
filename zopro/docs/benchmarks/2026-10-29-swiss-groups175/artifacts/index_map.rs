// Index-only dictionary using hashbrown's grouped control-byte probing.
use std::hash::BuildHasher;
use hashbrown::{HashTable, hash_table::Entry};
pub struct IndexMap<S> { table: HashTable<u32>, state: S }
impl<S: BuildHasher> IndexMap<S> {
    pub fn new(capacity: usize, state: S) -> Self { Self { table: HashTable::with_capacity(capacity), state } }
    pub fn insert(&mut self, value: &[u64; 4], values: &mut Vec<[u64; 4]>) -> (u32, bool) {
        let state = &self.state;
        let hash = state.hash_one(value);
        match self.table.entry(hash,
            |&id| values[id as usize] == *value,
            |&id| state.hash_one(&values[id as usize])) {
            Entry::Occupied(entry) => (*entry.get(), false),
            Entry::Vacant(entry) => {
                let id = u32::try_from(values.len()).unwrap();
                assert_ne!(id, u32::MAX);
                values.push(*value);
                entry.insert(id);
                (id, true)
            }
        }
    }
}
