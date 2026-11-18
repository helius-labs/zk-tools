//! Dependency readiness for existing solver leaves; private opt-in prototype.
use super::*;
use std::sync::{atomic::{AtomicBool,AtomicU32,Ordering},Mutex};
pub(super) fn enabled()->bool{static ON:std::sync::OnceLock<bool>=std::sync::OnceLock::new();*ON.get_or_init(||std::env::var("ZOPRO_CPU_READY_SOLVER").as_deref()==Ok("1"))}
#[derive(Clone,Copy)]struct Leaf{epoch:u32,task:u32,wave:u32,part:u32}
impl Leaf{fn get<'a>(&self,s:&'a Schedule)->&'a[u32]{let task=&s.epochs[self.epoch as usize][self.task as usize];if self.wave==u32::MAX{task}else{&s.refined[&(task[0]as usize)][self.wave as usize][self.part as usize]}}}
struct Graph{offsets:Vec<usize>,children:Vec<u32>,parents:Vec<u32>,roots:Vec<u32>}
struct Running{pending:Vec<AtomicU32>,failed:AtomicBool,error:Mutex<Option<String>>}
impl Graph{
    fn execute(&self,f:impl Fn(usize)->Result<(),String>+Sync)->Result<(),String>{
        let running=Running{pending:self.parents.iter().map(|&n|AtomicU32::new(n)).collect(),failed:AtomicBool::new(false),error:Mutex::new(None)};
        rayon::scope(|scope|{for &root in &self.roots{let f=&f;let running=&running;scope.spawn(move|scope|self.drive(scope,running,f,root as usize));}});
        let error=running.error.into_inner().unwrap();if let Some(e)=error{Err(e)}else{Ok(())}
    }
    fn drive<'s,F:Fn(usize)->Result<(),String>+Sync>(&'s self,scope:&rayon::Scope<'s>,running:&'s Running,f:&'s F,mut node:usize){
        loop{
            if !running.failed.load(Ordering::Acquire){if let Err(e)=f(node){let mut error=running.error.lock().unwrap();if error.is_none(){*error=Some(e);}running.failed.store(true,Ordering::Release);}}
            let mut next=None;
            for &child in &self.children[self.offsets[node]..self.offsets[node+1]]{
                // The last arriving parent acquires all preceding releases
                // on this counter before publishing the consumer job.
                if running.pending[child as usize].fetch_sub(1,Ordering::AcqRel)==1{
                    if next.is_none(){next=Some(child as usize)}else{scope.spawn(move|scope|self.drive(scope,running,f,child as usize));}
                }
            }
            let Some(child)=next else{return};node=child;
        }
    }
}
pub(super) struct Ready{leaves:Vec<Leaf>,graph:Graph,_reservation:(crate::preparation::Reservation,crate::preparation::Reservation)}
fn checked_sum(a:usize,b:usize)->Option<usize>{a.checked_add(b)}
fn checked_bytes(n:usize,s:usize)->Option<usize>{n.checked_mul(s)}
fn each_write(p:&Plan,op:&Op,mut f:impl FnMut(usize)){match op{Op::Hint(h)=>{for w in h.outputs(){f(w);}},Op::Constraint(s)=>{if let Some(a)=&s.assignment{f(a.wire());}let first=p.n_wires+s.first_slot();for w in first..first+p.stored(s){f(w);}}}}
fn read_count(p:&Plan,op:&Op)->Option<usize>{match op{
    Op::Hint(h)=>Some(h.input().len()),
    Op::Constraint(s)=>{let mut n=0usize;for sum in &p.sums[s.sums()]{n=n.checked_add(1+usize::from(matches!(sum.previous(),Previous::Slot(_))))?;}for e in &s.expressions{n=n.checked_add(match e{Expression::Zero|Expression::Constant(_)=>0,Expression::Wire(_)|Expression::Term(_)=>1,Expression::Sum(a,b)=>(b-a)as usize})?;}Some(n)}
}}
fn each_leaf(s:&Schedule,mut f:impl FnMut(Leaf)){for(e,epoch)in s.epochs.iter().enumerate(){for(t,task)in epoch.iter().enumerate(){if let Some(waves)=s.refined.get(&(task[0]as usize)){for(w,wave)in waves.iter().enumerate(){for(p,_)in wave.iter().enumerate(){f(Leaf{epoch:e as u32,task:t as u32,wave:w as u32,part:p as u32});}}}else{f(Leaf{epoch:e as u32,task:t as u32,wave:u32::MAX,part:0});}}}}
pub(super) fn build(p:&Plan,s:&Schedule)->Option<Ready>{
    if s.epochs.is_empty()||!*p.definite.get_or_init(||p.definitely_writes_all()){return None}
    // A tagged operation uses its low27bits; larger untagged images keep
    // their full index. Existing tagger already disables tags at2^27.
    let mask=if p.ops.len()<=(0x07ff_ffffusize){0x07ff_ffffu32}else{u32::MAX};
    let mut count=0usize;each_leaf(s,|_|count+=1);if count==0||count>=u32::MAX as usize{return None}
    let mut max_reads=0usize;for op in &p.ops{max_reads=max_reads.max(read_count(p,op)?);}
    let values=p.n_wires.checked_add(p.n_slots)?;
    // Base Vec capacities, excluding separately admitted exact CSR edges.
    // producer ownership, visited ops, read scratch, all retained arrays.
    let peak=checked_sum(checked_sum(checked_sum(checked_bytes(count,48)?,checked_bytes(count.checked_add(1)?,8)?)?,checked_bytes(values,4)?)?,checked_sum(checked_sum(p.ops.len(),checked_bytes(max_reads,8)?)?,12*64)?)?;
    let reservation=crate::preparation::Reservation::analysis(Some(peak))?;
    let mut leaves=Vec::with_capacity(count);each_leaf(s,|l|leaves.push(l));let mut producer=vec![u32::MAX;values];let mut seen=vec![false;p.ops.len()];
    for(id,l)in leaves.iter().enumerate(){for &raw in l.get(s){let op=(raw&mask)as usize;let visited=seen.get_mut(op)?;if *visited{return None}*visited=true;let mut valid=true;each_write(p,&p.ops[op],|wire|{if let Some(v)=producer.get_mut(wire){if *v!=u32::MAX{valid=false}else{*v=id as u32}}else{valid=false}});if !valid{return None}}}
    if seen.iter().any(|v|!*v){return None}drop(seen);
    let mut reads=Vec::with_capacity(max_reads);let mut marks=vec![u32::MAX;count];let mut parents=vec![0u32;count];let mut offsets=vec![0usize;count+1];
    for(id,l)in leaves.iter().enumerate(){for &raw in l.get(s){p.reads(&p.ops[(raw&mask)as usize],&mut reads);for &wire in &reads{let parent=*producer.get(wire)?;if parent==u32::MAX{if wire>=p.n_inputs{return None}continue}let parent=parent as usize;if parent==id{continue}if parent>=id{return None}if marks[parent]!=id as u32{marks[parent]=id as u32;parents[id]=parents[id].checked_add(1)?;offsets[parent+1]=offsets[parent+1].checked_add(1)?;}}}}
    for i in 1..offsets.len(){offsets[i]=offsets[i].checked_add(offsets[i-1])?;}
    let edges=offsets[count];let edge_bytes=edges.checked_mul(4)?.checked_add(64)?;
    let edge_reservation=crate::preparation::Reservation::analysis(Some(edge_bytes))?;
    let mut children=vec![0u32;edges];let mut cursor=offsets[..count].to_vec();marks.fill(u32::MAX);
    for(id,l)in leaves.iter().enumerate(){for &raw in l.get(s){p.reads(&p.ops[(raw&mask)as usize],&mut reads);for &wire in &reads{let parent=producer[wire];if parent==u32::MAX||parent==id as u32{continue}let parent=parent as usize;if marks[parent]!=id as u32{marks[parent]=id as u32;children[cursor[parent]]=id as u32;cursor[parent]+=1;}}}}
    debug_assert!((0..count).all(|i|cursor[i]==offsets[i+1]));
    let mut roots=Vec::with_capacity(count);for(i,&n)in parents.iter().enumerate(){if n==0{roots.push(i as u32)}}
    drop((producer,reads,marks,cursor));let retained=leaves.capacity().checked_mul(std::mem::size_of::<Leaf>())?.checked_add(offsets.capacity().checked_mul(std::mem::size_of::<usize>())?)?.checked_add(parents.capacity().checked_mul(4)?)?.checked_add(roots.capacity().checked_mul(4)?)?.checked_add(6*64)?;
    #[cfg(zopro_live_preparation_charge)]let reservation=reservation.retain(Some(retained));
    if std::env::var("ZOPRO_READY_TRACE").as_deref()==Ok("1"){eprintln!("READY_SOLVER leaves={} edges={} roots={} peak_charge={} retained_charge={}",count,children.len(),roots.len(),peak+edge_bytes,retained+edge_bytes);}
    Some(Ready{leaves,graph:Graph{offsets,children,parents,roots},_reservation:(reservation,edge_reservation)})
}
impl Ready{
    /// # Safety
    /// The existing schedule covers the Plan exactly; construction validated
    /// unique outputs, full op coverage, and an edge for every external read.
    /// No consumer runs until all producers publish through the atomic count.
    pub(super) unsafe fn execute(&self,p:&Plan,s:&Schedule,v:View)->Option<Result<(),String>>{
        let bytes=self.leaves.len().checked_mul(std::mem::size_of::<AtomicU32>())?.checked_add(4*64)?;let _scratch=crate::preparation::Reservation::analysis(Some(bytes))?;
        Some(self.graph.execute(|id|{let task=self.leaves[id].get(s);#[cfg(zopro_resource_schedule)]if p.poseidon_task(task){return unsafe{p.run_poseidon_task(task,v)}}for &op in task{unsafe{p.run(&p.ops[op as usize],v)?};}Ok(())}))
    }
}
#[cfg(test)]mod tests{
    use super::*;use std::sync::atomic::AtomicUsize;
    fn graph(n:usize,edges:&[(usize,usize)])->Graph{let mut offsets=vec![0;n+1];let mut parents=vec![0;n];for &(a,b)in edges{assert!(a<b&&b<n);offsets[a+1]+=1;parents[b]+=1;}for i in 1..=n{offsets[i]+=offsets[i-1];}let mut cursor=offsets.clone();let mut children=vec![0;edges.len()];for &(a,b)in edges{children[cursor[a]]=b as u32;cursor[a]+=1;}let roots=parents.iter().enumerate().filter_map(|(i,&n)|(n==0).then_some(i as u32)).collect();Graph{offsets,children,parents,roots}}
    #[test]fn diamond_publication_once_and_concurrent_reuse(){let edges=[(0,1),(0,2),(1,3),(2,3),(3,4),(3,5),(4,6),(5,6)];let g=graph(7,&edges);for threads in[1,4,16,96]{let pool=rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();pool.install(||{rayon::scope(|s|{for _ in 0..8{s.spawn(|_|{let values=(0..7).map(|_|AtomicUsize::new(0)).collect::<Vec<_>>();g.execute(|i|{for &(a,b)in &edges{if b==i{assert_eq!(values[a].load(Ordering::Relaxed),1);}}assert_eq!(values[i].fetch_add(1,Ordering::Relaxed),0);Ok(())}).unwrap();assert!(values.iter().all(|v|v.load(Ordering::Relaxed)==1));});}});});}}
    #[test]fn error_suppresses_descendants_and_scope_reuses(){let g=graph(5,&[(0,1),(1,2),(2,3),(3,4)]);for _ in 0..8{let calls=AtomicUsize::new(0);assert_eq!(g.execute(|i|{calls.fetch_add(1,Ordering::Relaxed);if i==1{Err("injected".into())}else{Ok(())}}),Err("injected".into()));assert_eq!(calls.load(Ordering::Relaxed),2);let calls=AtomicUsize::new(0);g.execute(|_|{calls.fetch_add(1,Ordering::Relaxed);Ok(())}).unwrap();assert_eq!(calls.load(Ordering::Relaxed),5);}}
    #[test]fn panic_propagates_without_waiting_for_unscheduled_nodes(){let g=graph(3,&[(0,1),(1,2)]);assert!(std::panic::catch_unwind(||{let _=g.execute(|i|{assert_ne!(i,1,"injected");Ok(())});}).is_err());g.execute(|_|Ok(())).unwrap();}
}

#[cfg(test)]
impl Ready {
    pub(super) fn check_edges(&self,p:&Plan,s:&Schedule){
        use std::collections::{BTreeMap,BTreeSet};
        let mask=if p.ops.len()<=0x07ff_ffff{0x07ff_ffff}else{u32::MAX};
        let mut owners=BTreeMap::new();
        for(i,leaf)in self.leaves.iter().enumerate(){for &op in leaf.get(s){for wire in p.writes(&p.ops[(op&mask)as usize]){assert!(owners.insert(wire,i).is_none());}}}
        let mut reference=BTreeSet::new();let mut reads=Vec::new();
        for(i,leaf)in self.leaves.iter().enumerate(){for &op in leaf.get(s){p.reads(&p.ops[(op&mask)as usize],&mut reads);for wire in &reads{if let Some(&parent)=owners.get(wire){if parent!=i{reference.insert((parent,i));}}}}}
        let mut actual=BTreeSet::new();for i in 0..self.leaves.len(){for &child in &self.graph.children[self.graph.offsets[i]..self.graph.offsets[i+1]]{assert!(actual.insert((i,child as usize)));}}
        assert_eq!(actual,reference);assert_eq!(actual.len(),self.graph.children.len());
        let mut parents=vec![0;self.leaves.len()];for &(_,child)in &reference{parents[child]+=1;}assert_eq!(parents,self.graph.parents);let roots=parents.iter().enumerate().filter_map(|(i,&n)|(n==0).then_some(i as u32)).collect::<Vec<_>>();assert_eq!(roots,self.graph.roots);
    }
}
