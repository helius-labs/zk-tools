#[cfg(test)]mod linear_carry_tests{
 use super::*;
 fn masks(a:&mut G1Caller,b:&mut G2Caller,mask:u8){
  for (i,(id,_))in a.owned.iter_mut().enumerate(){*id=(*id&0x7fffffff)|if mask>>(i%8)&1!=0{1<<31}else{0};}
  for (i,(id,_))in b.owned.iter_mut().enumerate(){*id=(*id&0x7fffffff)|if mask>>(i%8)&1!=0{1<<31}else{0};}
  for (out,&(id,p))in a.unsigned.iter_mut().zip(&a.owned){*out=(id&0x7fffffff,if id>>31!=0{-p}else{p});}
 }
 #[test]fn linear_carry_all_masks_modes_tails_and_reuse(){
  assert_eq!(core::mem::size_of::<FqAddGroup>(),1600);assert_eq!(core::mem::align_of::<FqAddGroup>(),64);
  let mut cases=0;
  for n in[0,1,7,8,9,15,16,17,199,200,201,351,352,353,511]{
   let mut a=G1Caller::new(n,31);let mut b=G2Caller::new(n,31);let source_a=a.source.clone();let source_b=b.source.clone();let incoming_a=a.incoming.clone();let incoming_b=b.incoming.clone();
   for mask in if n==8{(0..=255u8).collect::<Vec<_>>()}else{vec![0,1,0x55,0xaa,0xff]}{
    masks(&mut a,&mut b,mask);
    for fast in[false,true]{for inverse in[false,true]{for store in[0,1,2]{
     a.scratch.canonical=fast;a.scratch.vector_inverse=inverse;a.scratch.write_groups=store==2;a.scratch.reserve_groups=store>0;b.scratch.vector_inverse=inverse;b.scratch.canonical=fast;
     for borrowed in[false,true]{a.reset();for rounds in[1,2]{if borrowed{let refs=a.owned.iter().zip(&a.incoming).map(|(&(id,_),p)|(id,p)).collect::<Vec<_>>();for _ in 0..rounds{if fast{stream_g1_impl::<G1Config,true,true>(&mut a.buckets,&refs,&mut a.scratch)}else{stream_g1_impl::<G1Config,false,true>(&mut a.buckets,&refs,&mut a.scratch)}}}else{for _ in 0..rounds{if fast{stream_g1_impl::<G1Config,true,false>(&mut a.buckets,&a.unsigned,&mut a.scratch)}else{stream_g1_impl::<G1Config,false,false>(&mut a.buckets,&a.unsigned,&mut a.scratch)}}}a.check(if rounds==1{1}else{3});cases+=1;}}
     b.reset();let refs=b.owned.iter().zip(&b.incoming).map(|(&(id,_),p)|(id,p)).collect::<Vec<_>>();for rounds in[1,2]{for _ in 0..rounds{if fast{super::super::g2_borrowed::apply::<G2Config,true>(&mut b.buckets,&refs,&mut b.scratch)}else{super::super::g2_borrowed::apply::<G2Config,false>(&mut b.buckets,&refs,&mut b.scratch)}}b.check(if rounds==1{1}else{3});cases+=1;}
    }}}
   }
   assert_eq!(a.source,source_a);assert_eq!(b.source,source_b);assert_eq!(a.incoming,incoming_a);assert_eq!(b.incoming,incoming_b);
  }
  println!("LINEAR_CALLER_ORACLE complete_states={cases} group_bytes=1600 align=64");
 }
 #[test]fn linear_carry_small_stacks_and_stable_capacity(){
  let mut workers_total=0;
  for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();pool.broadcast(|ctx|{let n=[8,9,200,201,352,353][ctx.index()%6];let mut a=G1Caller::new(n,71+ctx.index()as u64);let mut b=G2Caller::new(n,71+ctx.index()as u64);a.run(1,false,true);b.run(1,false,true);let ca=a.capacities();let cb=b.capacities();for rounds in[2,3,5]{a.reset();b.reset();a.run(rounds,false,true);b.run(rounds,false,true);a.check(rounds);b.check(rounds);assert_eq!(a.capacities(),ca);assert_eq!(b.capacities(),cb);}});workers_total+=workers;}
  println!("LINEAR_CALLER_STACK workers={workers_total} stack_bytes=524288 modes=1,4,16,32");
 }
}
