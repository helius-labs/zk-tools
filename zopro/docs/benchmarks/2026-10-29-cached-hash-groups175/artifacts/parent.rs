use crate::builders::Groups;
use hashbrown::{HashTable,hash_table::Entry};
use std::hash::BuildHasher;
#[derive(Clone,Copy)]struct Record{row:u32,count:u32}
fn cutoff(n:usize,percent:usize)->usize{assert!(percent<=100);n/100*percent+(n%100)*percent/100}
fn one_pass<S:BuildHasher>(scalars:&[[u64;4]],capacity:usize,percent:usize,state:S)->Option<Groups>{
    assert!(scalars.len()<u32::MAX as usize);
    let limit=cutoff(scalars.len(),percent);
    let mut table=HashTable::<u32>::with_capacity(capacity);
    let mut records=Vec::<Record>::new();
    let mut classes=vec![u32::MAX;scalars.len()];
    for (i,(scalar,class)) in scalars.iter().zip(&mut classes).enumerate(){
        if *scalar==[0;4]{continue}
        let hash=state.hash_one(scalar);
        let id=match table.entry(hash,|&id|scalars[records[id as usize].row as usize]==*scalar,|&id|state.hash_one(&scalars[records[id as usize].row as usize])){
            Entry::Occupied(e)=>*e.get(),
            Entry::Vacant(e)=>{
                if records.len()==limit{return None}
                let id=records.len()as u32;
                records.push(Record{row:i as u32,count:0});
                e.insert(id);id
            }
        };
        records[id as usize].count+=1;*class=id;
    }
    drop(table);
    // Full scalar copies and usize counts exist only after exact admission.
    let mut values=Vec::with_capacity(records.len());
    let mut counts=Vec::with_capacity(records.len());
    let mut starts=Vec::with_capacity(records.len()+1);starts.push(0);
    for record in &records{
        values.push(scalars[record.row as usize]);counts.push(record.count as usize);
        starts.push(starts.last().unwrap()+record.count as usize);
    }
    drop(records);
    if values.is_empty(){return Some(Groups{len:scalars.len(),values,counts,starts,order:vec![]})}
    let mut positions=starts[..counts.len()].to_vec();
    let mut order=vec![0u32;*starts.last().unwrap()];
    for(i,&class)in classes.iter().enumerate(){if class!=u32::MAX{order[positions[class as usize]]=i as u32;positions[class as usize]+=1;}}
    Some(Groups{len:scalars.len(),values,counts,starts,order})
}
pub fn candidate(s:&[[u64;4]],capacity:usize,percent:usize,state:ahash::RandomState)->Option<Groups>{one_pass(s,capacity,percent,state)}
