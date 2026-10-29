// Proof-local dictionary: fingerprints and indices, with full-value equality.
use std::hash::BuildHasher;
const EMPTY: u64 = u64::MAX;
pub struct IndexMap<S> {
    slots: Vec<u64>,
    len: usize,
    state: S,
}
impl<S: BuildHasher> IndexMap<S> {
    pub fn new(capacity: usize, state: S) -> Self {
        let buckets = if capacity == 0 { 0 } else {
            capacity.checked_mul(8).unwrap().div_ceil(7).checked_next_power_of_two().unwrap().max(4)
        };
        Self { slots: vec![EMPTY; buckets], len: 0, state }
    }
    fn locate(&self, hash: u64, value: &[u64; 4], values: &[[u64; 4]]) -> (usize, Option<u32>) {
        let mask = self.slots.len() - 1;
        let mut at = hash as usize & mask;
        let fingerprint = hash as u32;
        loop {
            let slot = self.slots[at];
            if slot == EMPTY { return (at, None); }
            let id = slot as u32;
            if (slot >> 32) as u32 == fingerprint && values[id as usize] == *value {
                return (at, Some(id));
            }
            at = (at + 1) & mask;
        }
    }
    fn grow(&mut self, values: &[[u64; 4]]) {
        let buckets = self.slots.len().checked_mul(2).unwrap().max(4);
        let mut slots = vec![EMPTY; buckets];
        for (id, value) in values.iter().enumerate() {
            let hash = self.state.hash_one(value);
            let mut at = hash as usize & (buckets - 1);
            while slots[at] != EMPTY { at = (at + 1) & (buckets - 1); }
            slots[at] = ((hash as u32 as u64) << 32) | id as u64;
        }
        self.slots = slots;
    }
    pub fn insert(&mut self, value: &[u64; 4], values: &mut Vec<[u64; 4]>) -> (u32, bool) {
        let hash = self.state.hash_one(value);
        if self.slots.is_empty() { self.grow(values); }
        let (mut at, existing) = self.locate(hash, value, values);
        if let Some(id) = existing { return (id, false); }
        if (self.len + 1) * 8 > self.slots.len() * 7 {
            self.grow(values);
            at = self.locate(hash, value, values).0;
        }
        let id = u32::try_from(values.len()).unwrap();
        assert_ne!(id, u32::MAX);
        values.push(*value);
        self.slots[at] = ((hash as u32 as u64) << 32) | id as u64;
        self.len += 1;
        (id, true)
    }
}
