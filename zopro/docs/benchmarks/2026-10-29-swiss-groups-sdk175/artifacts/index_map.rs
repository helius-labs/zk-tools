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

#[cfg(test)]
mod tests {
 use super::*;
 use std::hash::Hasher;
 struct Constant;
 struct ConstantHasher;
 impl Hasher for ConstantHasher { fn write(&mut self,_:&[u8]){} fn finish(&self)->u64{u64::MAX} }
 impl BuildHasher for Constant { type Hasher=ConstantHasher; fn build_hasher(&self)->ConstantHasher{ConstantHasher} }
 #[test] fn forced_collision_growth_and_complete_key_equality() {
  for capacity in [0,1,3,17,128] {
   let mut table=IndexMap::new(capacity,Constant);let mut values=Vec::new();
   for i in 0..1025u64 { let v=[7,i/3,i%3,9];assert_eq!(table.insert(&v,&mut values),(i as u32,true));assert_eq!(table.insert(&v,&mut values),(i as u32,false)); }
   for i in (0..1025u64).rev(){assert_eq!(table.insert(&[7,i/3,i%3,9],&mut values),(i as u32,false));}
   assert_eq!(values.len(),1025);
  }
 }
}
