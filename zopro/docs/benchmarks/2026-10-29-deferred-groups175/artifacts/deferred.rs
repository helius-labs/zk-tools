use crate::builders::Groups;
use hashbrown::{HashTable,hash_table::Entry};
use std::hash::BuildHasher;
const TAG:u32=1<<31;
fn eligible(n:usize)->bool{n<TAG as usize}
fn cutoff(n:usize,percent:usize)->usize{assert!(percent<=100);n/100*percent+(n%100)*percent/100}
fn deferred_with<S:BuildHasher>(scalars:&[[u64;4]],capacity:usize,percent:usize,state:S)->Option<Groups>{
    assert!(eligible(scalars.len()));
    let limit=cutoff(scalars.len(),percent);
    let mut table=HashTable::<u32>::with_capacity(capacity);
    for (i,scalar) in scalars.iter().enumerate(){
        if *scalar==[0;4]{continue}
        let hash=state.hash_one(scalar);
        let distinct=table.len();
        match table.entry(hash,|&row|scalars[row as usize]==*scalar,|&row|state.hash_one(&scalars[row as usize])){
            Entry::Occupied(_)=>{},
            Entry::Vacant(v)=>{if distinct==limit{return None}v.insert(i as u32);}
        }
    }
    // Output arrays are born only after exact successful admission.
    let mut values=Vec::<[u64;4]>::new();let mut counts=Vec::<usize>::new();
    let mut classes=vec![u32::MAX;scalars.len()];
    for (scalar,class) in scalars.iter().zip(&mut classes){
        if *scalar==[0;4]{continue}
        let hash=state.hash_one(scalar);
        // No insertion or rehash after tags are introduced. A tag changes
        // the source of the equality value but never its hash or identity.
        let slot=table.find_mut(hash,|&entry|{
            let v=if entry&TAG==0{&scalars[entry as usize]}else{&values[(entry&!TAG)as usize]};
            *v==*scalar
        }).expect("first pass indexed every nonzero scalar");
        let id=if *slot&TAG==0{
            let id=u32::try_from(values.len()).unwrap();assert!(id<TAG);
            values.push(*scalar);counts.push(0);*slot=id|TAG;id
        }else{*slot&!TAG};
        *class=id;counts[id as usize]+=1;
    }
    drop(table);
    if values.is_empty(){return Some(Groups{len:scalars.len(),values,order:vec![],starts:vec![0],counts})}
    let mut starts=Vec::with_capacity(counts.len()+1);starts.push(0);
    for &count in &counts{starts.push(starts.last().unwrap()+count);}
    let mut positions=starts[..counts.len()].to_vec();let mut order=vec![0u32;*starts.last().unwrap()];
    for(i,&class)in classes.iter().enumerate(){if class!=u32::MAX{order[positions[class as usize]]=i as u32;positions[class as usize]+=1;}}
    Some(Groups{len:scalars.len(),values,order,starts,counts})
}
pub fn candidate(s:&[[u64;4]],capacity:usize,percent:usize,state:ahash::RandomState)->Option<Groups>{
    if !eligible(s.len()){return crate::builders::original(s,capacity,percent,state)}
    deferred_with(s,capacity,percent,state)
}
#[cfg(test)]mod tests{
use super::*;use std::hash::Hasher;
#[derive(Clone)]struct Constant;struct H;
impl Hasher for H{fn finish(&self)->u64{u64::MAX}fn write(&mut self,_:&[u8]){}}
impl BuildHasher for Constant{type Hasher=H;fn build_hasher(&self)->H{H}}
#[test]fn collisions_tag_transition_and_full_value_oracle(){for n in[0,1,7,255,256,1025,4097]{for percent in[0,1,80,95,100]{for cap in[0,1,17,32768]{let s:Vec<_>=(0..n).map(|i|if i%11==0{[0;4]}else{[7,(i%257)as u64,(i%17)as u64,((i/7)%3)as u64]}).collect();let snapshot=s.clone();assert_eq!(deferred_with(&s,cap,percent,Constant),crate::oracle(&s,percent));assert_eq!(s,snapshot);}}}}
#[test]fn exact_cutoff_occupied_boundary_and_late_new_value(){for n in[100usize,101,257,1001]{for percent in[1,80,95,100]{let limit=cutoff(n,percent);let mut s:Vec<_>=(0..n).map(|i|[(i.min(limit.saturating_sub(1))+1)as u64,2,3,4]).collect();assert_eq!(deferred_with(&s,0,percent,Constant),crate::oracle(&s,percent));if limit<n{s[n-1]=[999999,2,3,4];assert_eq!(deferred_with(&s,0,percent,Constant),None);}}}}
#[test]fn tag_and_cutoff_integer_boundaries(){for n in[0,1,99,100,101,(TAG as usize)-1,TAG as usize,u32::MAX as usize]{assert_eq!(eligible(n),n<TAG as usize);for percent in[0,1,80,95,100]{assert_eq!(cutoff(n,percent)as u128,(n as u128*percent as u128)/100);}}assert_eq!(std::mem::size_of::<u32>(),4);}
}
