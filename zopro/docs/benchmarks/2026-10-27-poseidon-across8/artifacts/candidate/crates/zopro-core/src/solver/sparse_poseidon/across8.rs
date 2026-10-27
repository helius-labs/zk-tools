//! Eight independent tasks; callers retain all input/output lifetime guarantees.
use super::*;
use zopro_bn254::{FrConfig,ifma8::Ifma8};
type V=Ifma8<FrConfig,true>;
#[inline(always)]fn constant(x:Fr)->V{V::new(&[x;8])}
fn dense(s:&mut[V;3],a:&[Fr],offset:usize){let mut next=[V::zero();3];for i in 0..3{for j in 0..3{next[i]=next[i].add(&constant(a[offset+i*3+j]).mul(&s[j]));}}*s=next;}
unsafe fn emit(plan:&Plan,tasks:[&[u32];8],views:[View;8],partial:usize,k:usize,x:V)->V{
 let square=x.mul(&x);let fourth=square.mul(&square);let fifth=fourth.mul(&x);
 let(xs,squares,fourths,fifths)=(x.get(),square.get(),fourth.get(),fifth.get());
 for lane in 0..8{let[f,m,l]=positions(3,partial,k);let a=step(plan,tasks[lane][f]).unwrap();let b=step(plan,tasks[lane][m]).unwrap();let c=step(plan,tasks[lane][l]).unwrap();let v=views[lane];let x=xs[lane];let square=squares[lane];let fourth=fourths[lane];let fifth=fifths[lane];
  // SAFETY: recognized task indices and independently writable views are the caller's contract.
  unsafe{*v.w.add(plan.n_wires+a.first_slot())=x;*v.w.add(a.assignment.as_ref().unwrap().wire())=square;*v.w.add(b.assignment.as_ref().unwrap().wire())=fourth;*v.w.add(c.assignment.as_ref().unwrap().wire())=fifth;
   *v.a.add(a.constraint())=x;*v.b.add(a.constraint())=x;*v.a.add(b.constraint())=square;*v.b.add(b.constraint())=square;
   let normal=wire(&c.expressions[0])==Some(plan.n_wires+a.first_slot());*v.a.add(c.constraint())=if normal{x}else{fourth};*v.b.add(c.constraint())=if normal{fourth}else{x};
   if !v.c.is_null(){*v.c.add(a.constraint())=square;*v.c.add(b.constraint())=fourth;*v.c.add(c.constraint())=fifth;}
  }
 }fifth
}
unsafe fn hash(plan:&Plan,tasks:[&[u32];8],views:[View;8])->Result<(),String>{
 let(partial,a)=parameters(3);let count=24+partial;let mds=count;let pre=mds+9;let sparse=pre+9;
 for lane in 0..8{for &id in &tasks[lane][..9]{unsafe{plan.run(&plan.ops[(id&INDEX)as usize],views[lane])?;}}}
 let mut s:[V;3]=core::array::from_fn(|i|V::new(&core::array::from_fn(|lane|{let row=step(plan,tasks[lane][6+i]).unwrap();unsafe{*views[lane].w.add(row.assignment.as_ref().unwrap().wire())}})));
 dense(&mut s,a,mds);let mut k=3;
 for r in [2,3]{for i in 0..3{s[i]=unsafe{emit(plan,tasks,views,partial,k,s[i].add(&constant(a[r*3+i])))};k+=1;}dense(&mut s,a,if r==3{pre}else{mds});}
 let mut c=12;let mut x=s[0].add(&constant(a[c]));c+=1;
 for r in 0..partial{let offset=sparse+r*5;let mut rest=if r+1<partial{let z=constant(a[c]);c+=1;z}else{V::zero()};for j in 1..3{rest=rest.add(&constant(a[offset+j]).mul(&s[j]));}
  let head=unsafe{emit(plan,tasks,views,partial,k,x)};k+=1;for i in 1..3{s[i]=s[i].add(&constant(a[offset+3+i-1]).mul(&head));}x=constant(a[offset]).mul(&head).add(&rest);
 }s[0]=x;
 for r in 0..4{for i in 0..3{s[i]=unsafe{emit(plan,tasks,views,partial,k,s[i].add(&constant(a[c+i])))};k+=1;}c+=3;if r!=3{dense(&mut s,a,mds);}}
 Ok(())
}
unsafe fn remainder(plan:&Plan,task:&[u32],mut pos:usize,v:View)->Result<(),String>{
 while pos<task.len(){let t=(task[pos]>>28)as usize;if t==0{unsafe{plan.run(&plan.ops[(task[pos]&INDEX)as usize],v)?;}pos+=1;}else{let(partial,a)=parameters(t);unsafe{plan.run_poseidon(&task[pos..],t,partial,a,v)?;}pos+=3*(7*t+partial);}}Ok(())
}
/// All tasks must be valid for this plan. Every view must remain initialized;
/// distinct tasks' writes/reads must satisfy the original epoch contract.
pub(super) unsafe fn run(plan:&Plan,tasks:[&[u32];8],views:[View;8])->Result<(),String>{
 if tasks.iter().any(|task|!plan.poseidon_task(task)){for lane in 0..8{unsafe{plan.run_poseidon_task(tasks[lane],views[lane])?;}}return Ok(());}
 let mut pos=[0usize;8];let(partial,_)=parameters(3);let span=3*(21+partial);
 loop{for lane in 0..8{while pos[lane]<tasks[lane].len()&&tasks[lane][pos[lane]]>>28==0{unsafe{plan.run(&plan.ops[(tasks[lane][pos[lane]]&INDEX)as usize],views[lane])?;}pos[lane]+=1;}}
  if (0..8).all(|i|pos[i]<tasks[i].len()&&tasks[i][pos[i]]>>28==3){let slices=core::array::from_fn(|i|&tasks[i][pos[i]..pos[i]+span]);unsafe{hash(plan,slices,views)?;}for p in &mut pos{*p+=span;}}
  else{for lane in 0..8{unsafe{remainder(plan,tasks[lane],pos[lane],views[lane])?;}}return Ok(());}
 }
}
