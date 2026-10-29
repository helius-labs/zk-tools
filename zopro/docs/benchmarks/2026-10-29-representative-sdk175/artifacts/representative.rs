use super::ScalarGroups;
use hashbrown::{HashTable,hash_table::Entry};
use std::hash::BuildHasher;
#[derive(Clone,Copy)]struct Record{row:u32,count:u32}
fn cutoff(n:usize,percent:usize)->usize{assert!(percent<=100);n/100*percent+(n%100)*percent/100}
pub(super) fn build<S:BuildHasher>(scalars:&[[u64;4]],capacity:usize,percent:usize,state:S,reservation:crate::preparation::Reservation)->Option<ScalarGroups>{
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
    if values.is_empty(){return Some(ScalarGroups{_reservation:reservation,len:scalars.len(),values,counts,starts,order:vec![]})}
    let mut positions=starts[..counts.len()].to_vec();
    let mut order=vec![0u32;*starts.last().unwrap()];
    for(i,&class)in classes.iter().enumerate(){if class!=u32::MAX{order[positions[class as usize]]=i as u32;positions[class as usize]+=1;}}
    Some(ScalarGroups{_reservation:reservation,len:scalars.len(),values,counts,starts,order})
}
#[cfg(test)]mod tests{

type Shape=(usize,Vec<[u64;4]>,Vec<u32>,Vec<usize>,Vec<usize>);
fn shape(g:Option<ScalarGroups>)->Option<Shape>{g.map(|g|(g.len,g.values,g.order,g.starts,g.counts))}
fn test_build<S:BuildHasher>(s:&[[u64;4]],cap:usize,pct:usize,state:S)->Option<Shape>{shape(build(s,cap,pct,state,crate::preparation::Reservation::analysis(Some(0)).unwrap()))}
fn oracle(s:&[[u64;4]],pct:usize)->Option<Shape>{use std::collections::BTreeMap;let mut m=BTreeMap::<[u64;4],Vec<u32>>::new();let mut first=vec![];for(i,&v)in s.iter().enumerate(){if v==[0;4]{continue}if !m.contains_key(&v){first.push(v)}m.entry(v).or_default().push(i as u32);}if first.len()*100>s.len()*pct{return None}let mut starts=vec![0];let mut counts=vec![];let mut order=vec![];for v in&first{let rows=&m[v];counts.push(rows.len());order.extend(rows);starts.push(order.len());}Some((s.len(),first,order,starts,counts))}
use super::*;use std::hash::Hasher;
#[derive(Clone)]struct Constant;struct H;
impl Hasher for H{fn finish(&self)->u64{u64::MAX}fn write(&mut self,_:&[u8]){}}
impl BuildHasher for Constant{type Hasher=H;fn build_hasher(&self)->H{H}}
#[test]fn collisions_representative_counts_and_full_value_oracle(){for n in[0,1,7,255,256,1025,4097]{for percent in[0,1,80,95,100]{for cap in[0,1,17,32768]{let s:Vec<_>=(0..n).map(|i|if i%11==0{[0;4]}else{[7,(i%257)as u64,(i%17)as u64,((i/7)%3)as u64]}).collect();let snapshot=s.clone();assert_eq!(test_build(&s,cap,percent,Constant),oracle(&s,percent));assert_eq!(s,snapshot);}}}}
#[test]fn exact_cutoff_occupied_boundary_and_late_new_value(){for n in[100usize,101,257,1001]{for percent in[1,80,95,100]{let limit=cutoff(n,percent);let mut s:Vec<_>=(0..n).map(|i|[(i.min(limit.saturating_sub(1))+1)as u64,2,3,4]).collect();assert_eq!(test_build(&s,0,percent,Constant),oracle(&s,percent));if limit<n{s[n-1]=[999999,2,3,4];assert_eq!(test_build(&s,0,percent,Constant),None);}}}}
#[test]fn representative_bounds_and_original_forecast(){
use crate::msm_affine::grouping_cost::{vector,hash_table};
assert_eq!(std::mem::size_of::<Record>(),8);
for n in[1usize,3,7,99,100,101,256,32768,65536,1<<20,u32::MAX as usize-1]{for percent in[0usize,1,80,95,100]{let u=cutoff(n,percent);assert_eq!(u as u128,n as u128*percent as u128/100);assert!(u<=n&&n<u32::MAX as usize);
let old_build=hash_table::<([u64;4],u32)>(n).unwrap()+vector::<[u64;4]>(n,true).unwrap()+vector::<usize>(n,true).unwrap()+vector::<u32>(n,false).unwrap();
let first=hash_table::<u32>(n).unwrap()+vector::<Record>(n,true).unwrap()+vector::<u32>(n,false).unwrap();assert!(first<=old_build);
let old_lists=vector::<[u64;4]>(n,true).unwrap()+vector::<usize>(n,true).unwrap()+vector::<u32>(n,false).unwrap()+2*vector::<usize>(n+1,false).unwrap()+vector::<u32>(n,false).unwrap();
let materialize=vector::<Record>(u,true).unwrap()+vector::<[u64;4]>(u,false).unwrap()+vector::<usize>(u,false).unwrap()+vector::<usize>(u+1,false).unwrap()+vector::<u32>(n,false).unwrap();assert!(materialize<=old_lists,"n={n},u={u}");
// Independent payload inequality for both 32- and 64-bit usize, with
// records released before positions/order. No large buffers allocated.
for word in[4u128,8]{let v=n as u128;let k=u as u128;assert!(8*k+32*k+word*k+word*(k+1)+4*v<=32*v+word*v+4*v+2*word*(v+1)+4*v);}
}}
assert_eq!(hash_table::<u32>(usize::MAX),None);assert_eq!(vector::<Record>(usize::MAX,true),None);
}
}
