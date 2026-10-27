//! Component-only fixture/probe. This is absent from production builds.
use super::*;
fn u32_at(data:&[u8],pos:&mut usize)->usize{let v=u32::from_le_bytes(data[*pos..*pos+4].try_into().unwrap());*pos+=4;v as usize}
fn field_at(data:&[u8],pos:&mut usize)->Fr{let words=core::array::from_fn(|i|u64::from_le_bytes(data[*pos+8*i..*pos+8*i+8].try_into().unwrap()));*pos+=32;Fr::from_canonical(words).unwrap()}
pub(super) fn forms_width(width:usize)->(usize,Vec<Vec<(usize,Fr)>>){let bytes=include_bytes!("all-widths.bin");let mut pos=0;let widths=u32_at(bytes,&mut pos);for _ in 0..widths{let t=u32_at(bytes,&mut pos);let n=u32_at(bytes,&mut pos);let mut forms=Vec::new();for _ in 0..n{let len=u32_at(bytes,&mut pos);let mut terms=Vec::new();for _ in 0..len{terms.push((u32_at(bytes,&mut pos),field_at(bytes,&mut pos)));}forms.push(terms);}if t==width{return(n,forms);}}panic!("fixture width absent")}
pub struct Probe{pub(super) plan:Plan,pub(super) tasks:Vec<Vec<u32>>,pub(super) w:Vec<Fr>,pub(super) a:Vec<Fr>,pub(super) b:Vec<Fr>,pub(super) c:Vec<Fr>,pub(super) chains:Vec<usize>,width:usize,grouped_enabled:bool,arranged_workers:usize}
impl Probe{
 pub fn new(groups:usize,reverse:bool,materialize:bool,ragged:bool)->Self{
  Self::with_width(groups,reverse,materialize,ragged,3)
 }
 pub(super) fn with_width(groups:usize,reverse:bool,materialize:bool,ragged:bool,t:usize)->Self{
  assert!(groups>0);let lanes=groups.checked_mul(8).unwrap();let(n,forms)=forms_width(t);let(partial,params)=parameters(t);assert_eq!(n,7*t+partial);let stride=t+3*n;let chains:Vec<_>=(0..lanes).map(|i|if ragged{i%3+1}else{1}).collect();let hashes:usize=chains.iter().sum();let inputs=1+t*lanes;let operations=hashes.checked_mul(stride).unwrap();assert!(operations<=INDEX as usize);let n_wires=inputs+operations;
  let mut coefficients=vec![Fr::ZERO,Fr::ONE,Fr::from(2u64),-Fr::ONE,-Fr::from(2u64)];let mut sums=Vec::new();let mut ordered:Vec<Option<Op>>=(0..operations).map(|_|None).collect();let mut tasks=Vec::new();let mut hashbase=0;
  let assignment=|wire:usize|Some(Assignment{side:2,wire:wire.try_into().unwrap(),coefficient:1,inverse:super::super::INVERSE_ONE});
  for(lane,&length)in chains.iter().enumerate(){let start=hashbase*stride;
   for h in hashbase..hashbase+length{let base=h*stride;let private=inputs+base;
    for i in 0..t{let source=if h==hashbase{1+lane*t+i}else{inputs+(h-1)*stride+t+3*(n-t+i)+2};ordered[base+i]=Some(Op::Constraint(Step{constraint:(base+i)as u32,sums:(0,0),first_slot:0,expressions:[Expression::Wire(source as u32),Expression::Constant(1),Expression::Zero],assignment:assignment(private+i)}));}
    for k in 0..n{let[f,m,l]=positions(t,partial,k);let begin=sums.len();for(j,(symbol,value))in forms[k].iter().enumerate(){let wire=if *symbol==0{0}else if *symbol<=n{private+t+3*(*symbol-1)+2}else{private+(*symbol-n-1)};let coefficient=coefficients.len();coefficients.push(*value);sums.push(Sum{previous:(if j==0{NONE}else{CHAIN})|if j+1==forms[k].len(){STORED}else{0},term:Term::new(coefficient,wire)});}let end=sums.len();let slot=h*n+k;let x=(n_wires+slot)as u32;let square=private+t+3*k;let fourth=square+1;let fifth=square+2;
     ordered[base+t+f]=Some(Op::Constraint(Step{constraint:(base+t+f)as u32,sums:(begin as u32,end as u32),first_slot:slot as u32,expressions:[Expression::Wire(x),Expression::Wire(x),Expression::Zero],assignment:assignment(square)}));
     ordered[base+t+m]=Some(Op::Constraint(Step{constraint:(base+t+m)as u32,sums:(end as u32,end as u32),first_slot:(slot+1)as u32,expressions:[Expression::Wire(square as u32),Expression::Wire(square as u32),Expression::Zero],assignment:assignment(fourth)}));
     ordered[base+t+l]=Some(Op::Constraint(Step{constraint:(base+t+l)as u32,sums:(end as u32,end as u32),first_slot:(slot+1)as u32,expressions:if reverse{[Expression::Wire(fourth as u32),Expression::Wire(x),Expression::Zero]}else{[Expression::Wire(x),Expression::Wire(fourth as u32),Expression::Zero]},assignment:assignment(fifth)}));
    }
   }hashbase+=length;tasks.push((start as u32..(hashbase*stride)as u32).collect::<Vec<_>>());
  }
  let plan=Plan{n_wires,n_inputs:inputs,n_public:inputs,a_indices:Vec::new(),b_indices:Vec::new(),coefficients,inverses:super::super::Inverses::default(),terms:Vec::new(),sums:super::super::super::Store::Owned(sums),n_slots:hashes*n,n_constraints:operations,ops:ordered.into_iter().map(Option::unwrap).collect(),fuse_sums:2,schedule:std::sync::OnceLock::new(),definite:std::sync::OnceLock::new()};
  let pattern=Pattern::new(t,partial,params).unwrap();for(task,&count)in tasks.iter_mut().zip(&chains){assert_eq!(pattern.tag(&plan,task),count);}
  let mut out=Self{w:vec![Fr::ZERO;plan.n_wires+plan.n_slots],a:vec![Fr::ZERO;operations],b:vec![Fr::ZERO;operations],c:if materialize{vec![Fr::ZERO;operations]}else{Vec::new()},plan,tasks,chains,width:t,grouped_enabled:false,arranged_workers:0};out.reset(0);out
 }
 pub fn reset(&mut self,seed:usize){self.w.fill(Fr::from(0x5a5au64));self.w[0]=Fr::ONE;self.a.fill(Fr::from(77u64));self.b.fill(Fr::from(77u64));self.c.fill(Fr::from(77u64));for lane in 0..self.tasks.len(){let family=seed+lane;let mut random=0x2545f4914f6cdd1du64.wrapping_mul(family as u64+1);for i in 0..self.width{self.w[1+self.width*lane+i]=match family{0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,3=>Fr::from(2u64),_=>{let mut words=[0u64;4];for x in &mut words{random^=random<<13;random^=random>>7;random^=random<<17;*x=random;}words[3]&=0x1fff_ffff_ffff_ffff;Fr::from_canonical(words).unwrap()}};}}}
 pub(super)fn view(&mut self)->View{View{w:self.w.as_mut_ptr(),a:self.a.as_mut_ptr(),b:self.b.as_mut_ptr(),c:if self.c.is_empty(){std::ptr::null_mut()}else{self.c.as_mut_ptr()}}}
 pub fn hashes(&self)->usize{self.chains.iter().sum()}
 pub fn buffer_elements(&self)->usize{self.w.len()+self.a.len()+self.b.len()+self.c.len()}
 pub fn snapshot(&self)->Vec<Fr>{self.w.iter().chain(&self.a).chain(&self.b).chain(&self.c).copied().collect()}
 fn task_id(plan:&Plan,task:&[u32])->usize{let first=task.first().copied().expect("nonempty fixture task");if plan.ops.len()<=INDEX as usize{(first&INDEX)as usize}else{first as usize}}
 fn task_key(plan:&Plan,task:&[u32])->(usize,usize,usize){let id=Self::task_id(plan,task);if !plan.poseidon_task(task){return(1,0,id);}let mut pos=0;let mut hashes=0;let(partial,_)=parameters(3);let span=3*(21+partial);while pos<task.len(){let t=task[pos]>>28;if t==0{pos+=1;}else if t==3&&pos+span<=task.len(){hashes+=1;pos+=span;}else{return(1,0,id);}}if hashes==0{(1,0,id)}else{(0,hashes,id)}}
 pub fn arrange(&mut self,grouped:bool,workers:usize){assert!(workers>0);let plan=&self.plan;let eligible=self.tasks.iter().filter(|task|Self::task_key(plan,task).0==0).count();self.grouped_enabled=grouped&&eligible>workers.saturating_mul(2);self.arranged_workers=workers;if self.grouped_enabled{self.tasks.sort_unstable_by_key(|task|Self::task_key(plan,task));}else{self.tasks.sort_unstable_by_key(|task|Self::task_id(plan,task));}}
 pub fn grouping_enabled(&self)->bool{self.grouped_enabled}
 pub fn layout_counts(&self,use_batch:bool)->[usize;3]{let total=self.hashes();let mut vector=0;if use_batch{for group in self.tasks.chunks(8){if group.len()==8{let mut minimum=usize::MAX;let mut eligible=true;for task in group{let key=Self::task_key(&self.plan,task);eligible&=key.0==0;minimum=minimum.min(key.1);}if eligible{vector+=minimum*8;}}}}assert!(vector<=total);[total,vector,total-vector]}
 #[inline(never)]pub fn parallel(&mut self,mode:u8)->Result<(),String>{use rayon::prelude::*;assert!(mode<=2);let view=self.view();let use_batch=mode==1||mode==2&&self.grouped_enabled;if mode==2{assert_eq!(self.arranged_workers,rayon::current_num_threads());}let plan=&self.plan;
  // SAFETY: fixture tasks own disjoint private wires, slots and rows; their
  // input wires are immutable. Their Vec descriptors alone may be reordered.
  if !use_batch{return self.tasks.par_iter().try_for_each(|task|unsafe{plan.run_poseidon_task(task,view)});}
  self.tasks.par_chunks(8).try_for_each(|group|{if group.len()==8&&group.iter().all(|task|Self::task_key(plan,task).0==0){let tasks=core::array::from_fn(|i|group[i].as_slice());unsafe{super::across8::run(plan,tasks,[view;8])}}else{group.par_iter().try_for_each(|task|unsafe{plan.run_poseidon_task(task,view)})}})
 }
 #[inline(never)]pub fn run(&mut self,mode:u8)->Result<(),String>{let view=self.view();match mode{
  0=>{for task in &self.tasks{unsafe{self.plan.run_poseidon_task(task,view)?;}}},
  1=>{for group in self.tasks.chunks_exact(8){let tasks=core::array::from_fn(|i|group[i].as_slice());unsafe{super::across8::run(&self.plan,tasks,[view;8])?;}}},
  2=>{for op in &self.plan.ops{unsafe{self.plan.run(op,view)?;}}},_=>return Err("unknown component mode".into())}Ok(())}
}
