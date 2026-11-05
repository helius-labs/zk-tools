//! Diagnostic-only export of the schedule already prepared by Key loading.
use super::*;
const WIDTH:usize=5;
type Stamp=[u64;WIDTH];
fn precedes(a:Stamp,b:Stamp)->bool {
    a[0]<b[0] || a[0]==b[0] && a[1]==b[1] &&
        (a[2]<b[2] || a[2]==b[2] && a[3]==b[3] && a[4]<b[4])
}
fn place(map:&mut[Stamp],op:usize,stamp:Stamp)->Result<(),String>{
    let out=map.get_mut(op).ok_or("schedule operation outside plan")?;
    if out[0]!=u64::MAX{return Err("operation scheduled twice".into());}
    *out=stamp;Ok(())
}
fn complete(map:&[Stamp])->Result<(),String>{
    if map.iter().any(|s|s[0]==u64::MAX){return Err("unscheduled operation".into());}Ok(())
}
pub(crate)fn census(p:&Plan,domain:usize,workspace:Option<usize>)->Result<Vec<u64>,String>{
    if !p.definitely_writes_all(){return Err("plan does not have definite unique writes".into());}
    let mut map=vec![[u64::MAX;WIDTH];p.ops.len()];
    let(mut roots,mut leaves,mut tagged,mut epochs)=(0u64,0u64,0u64,0u64);
    let parallel=p.ops.len()>=4096&&solve_epochs()!=0&&p.schedule.get().is_some_and(|s|!s.epochs.is_empty());
    if parallel {
        let schedule=p.schedule.get().unwrap();
        for(epoch,tasks)in schedule.epochs.iter().enumerate(){
            for task in tasks {
                let root=roots;roots+=1;
                let first=*task.first().ok_or("empty root task")? as usize;
                if let Some(waves)=schedule.refined.get(&first){
                    for(wave,parts)in waves.iter().enumerate(){for part in parts {
                        leaf(p,&mut map,part,epoch as u64,root,wave as u64,leaves,&mut tagged)?;leaves+=1;
                    }}
                }else{leaf(p,&mut map,task,epoch as u64,root,0,leaves,&mut tagged)?;leaves+=1;}
            }
            epochs+=1;
        }
    }else{
        for op in 0..p.ops.len(){place(&mut map,op,[0,0,0,0,op as u64])?;}
        roots=1;leaves=1;epochs=1;
    }
    complete(&map)?;
    let mut producer=vec![usize::MAX;p.n_wires+p.n_slots];
    for i in 0..p.n_inputs{producer[i]=p.ops.len();}
    for(i,op)in p.ops.iter().enumerate(){for wire in p.writes(op){
        if producer[wire]!=usize::MAX{return Err("multiple writer or overwritten input".into());}
        producer[wire]=i;
    }}
    if producer.iter().any(|&x|x==usize::MAX){return Err("missing producer".into());}
    let mut reads=Vec::new();for(i,op)in p.ops.iter().enumerate(){p.reads(op,&mut reads);for &wire in&reads{
        let writer=producer[wire];if writer==p.ops.len(){continue;}
        if writer==i&&wire>=p.n_wires{continue;}
        if !precedes(map[writer],map[i]){return Err(format!("schedule violates producer ordering for op {i}"));}
    }}
    let mut out=vec![1,p.ops.len()as u64,p.n_wires as u64,p.n_slots as u64,p.n_inputs as u64,p.n_constraints as u64,p.a_indices.len()as u64,p.b_indices.len()as u64,epochs,roots,leaves,tagged,u64::from(parallel),domain as u64,workspace.map_or(u64::MAX,|n|n as u64)];
    out.extend(map.into_iter().flatten());Ok(out)
}
fn leaf(p:&Plan,map:&mut[Stamp],task:&[u32],epoch:u64,root:u64,wave:u64,id:u64,tagged:&mut u64)->Result<(),String>{
    if task.is_empty(){return Err("empty leaf task".into());}
    let encoded=p.poseidon_task(task);*tagged+=u64::from(encoded);
    for(pos,&raw)in task.iter().enumerate(){
        // Exact guard/mask from sparse_poseidon.rs: TASK=1<<27, INDEX=TASK-1.
        // Untagged/ineligible task IDs retain all their original bits.
        let op=if encoded{raw&((1<<27)-1)}else{raw};
        place(map,op as usize,[epoch,root,wave,id,pos as u64])?;
    }Ok(())
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn separate_roots_and_same_wave_leaves_are_unordered(){
  let a=[2,7,0,20,0];assert!(!precedes(a,[2,8,1,21,9]));assert!(!precedes(a,[2,7,0,21,9]));assert!(precedes(a,[3,8,0,21,0]));assert!(precedes(a,[2,7,1,21,0]));assert!(precedes(a,[2,7,0,20,1]));assert!(!precedes(a,a));
 }
 #[test]fn missing_duplicate_and_out_of_range_ops_fail(){let mut map=vec![[u64::MAX;WIDTH];3];assert!(complete(&map).is_err());place(&mut map,2,[0;WIDTH]).unwrap();assert!(place(&mut map,2,[1;WIDTH]).is_err());assert!(place(&mut map,3,[0;WIDTH]).is_err());place(&mut map,0,[0;WIDTH]).unwrap();place(&mut map,1,[0;WIDTH]).unwrap();complete(&map).unwrap();}
 #[test]fn local_wave_barriers_order_only_their_own_root(){for root in 0..4{for wave in 0..3{for leaf in 0..2{let a=[0,root,wave,leaf,1];let b=[0,root,wave+1,leaf+2,0];assert!(precedes(a,b));for other in 0..4{if other!=root{assert!(!precedes(a,[0,other,wave+2,leaf,9]));}}}}}}
}
