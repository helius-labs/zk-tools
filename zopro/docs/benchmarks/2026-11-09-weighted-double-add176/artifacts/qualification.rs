mod kernel;
use ark_ec::{AffineRepr,CurveGroup,PrimeGroup};
use ark_ff::{BigInt,PrimeField,Zero};
use zopro_bn254::{Field,Fq,Fr,G1Affine};

fn af(v:Fr)->ark_bn254::Fr{ark_bn254::Fr::from_bigint(BigInt(v.to_canonical())).unwrap()}
fn fq(v:ark_bn254::Fq)->Fq{Fq::from_canonical(v.into_bigint().0).unwrap()}
fn point(v:ark_bn254::G1Affine)->G1Affine{if v.is_zero(){G1Affine::IDENTITY}else{G1Affine::new(fq(v.x),fq(v.y))}}
fn check(v:G1Affine,w:ark_bn254::G1Affine){assert_eq!(v,point(w));assert!(v.is_on_curve());}
fn random(s:&mut u64)->Fr {let mut b=[0;32];for chunk in b.chunks_exact_mut(8){*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;chunk.copy_from_slice(&s.to_le_bytes());}Fr::from_le_bytes_mod_order(&b)}

#[test]
fn changing_full_width_packets_and_all_denominator_guards(){
 let g=ark_bn254::G1Projective::generator();let mut seed=0xd124836a91bc7d65;let mut count=0;let mut guards=[0usize;3];
 for round in 0..64 {let p=core::array::from_fn::<_,8,_>(|_|(g*af(random(&mut seed))).into_affine());
  let q=core::array::from_fn::<_,8,_>(|i|match (round+i)%8 {0=>ark_bn254::G1Affine::zero(),1=>p[i],2=>-p[i],3=>(-(p[i].into_group()+p[i])).into_affine(),_=>(g*af(random(&mut seed))).into_affine()});
  let p=if round%9==0{core::array::from_fn(|i|if i%3==0{ark_bn254::G1Affine::zero()}else{p[i]})}else{p};
  let (np,nq)=(p.map(point),q.map(point));let before=(np,nq);
  for active in 0..=8 {let out=kernel::apply(&np,&nq,active);assert_eq!(out.routes.fast+out.routes.first_guard+out.routes.second_guard,active);
   for i in 0..8 {if i<active {check(out.points[i],(p[i].into_group()+p[i]+q[i]).into_affine());count+=1;}else{assert_eq!(out.points[i],np[i]);}}
   guards[0]+=out.routes.fast;guards[1]+=out.routes.first_guard;guards[2]+=out.routes.second_guard;
  }assert_eq!((np,nq),before);
 }assert_eq!(count,2304);assert!(guards.iter().all(|n|*n>0));println!("PACKET_OUTPUTS={count};GUARDS={guards:?}");
}

fn fixture(kind:usize,seed:u64)->([[G1Affine;9];8],[[ark_bn254::Fr;9];8],[ark_bn254::Fr;8]){
 let g=ark_bn254::G1Projective::generator();let mut state=seed;let mut sums=[[ark_bn254::Fr::zero();9];8];let mut expected=[ark_bn254::Fr::zero();8];
 for lane in 0..8 {for index in 0..256 {let x=match kind {0=>ark_bn254::Fr::zero(),1=>ark_bn254::Fr::from(if index%2==0{1u64}else{0}),2=>if index%2==0{ark_bn254::Fr::from(1u64)}else{-ark_bn254::Fr::from(1u64)},_=>af(random(&mut state))};
   sums[lane][0]+=x;expected[lane]+=x*ark_bn254::Fr::from(index as u64+1);
   for step in 1..9 {if (index>>(8-step))&1==0{sums[lane][step]+=x;}}
 }}
 let roots=sums.map(|r|r.map(|s|point((g*s).into_affine())));(roots,sums,expected)
}

fn check_fold(roots:&[[G1Affine;9];8],sums:&[[ark_bn254::Fr;9];8],expected:&[ark_bn254::Fr;8],active:usize)->usize {
 let g=ark_bn254::G1Projective::generator();let mut values=roots.map(|r|r[0]);let mut coefficients=sums.map(|r|r[0]);let before=*roots;let mut outputs=0;
 for step in 1..9 {let q=roots.map(|r|-r[step]);let out=kernel::apply(&values,&q,active);values=out.points;
   for i in 0..active {coefficients[i]=coefficients[i]+coefficients[i]-sums[i][step];check(values[i],(g*coefficients[i]).into_affine());outputs+=1;}
 }
 let (finals,_)=kernel::fold(roots,active);assert_eq!(values,finals);
 for i in 0..active {assert_eq!(coefficients[i],expected[i]);check(finals[i],(g*expected[i]).into_affine());check(kernel::original_fold(&roots[i]).to_affine(),(g*expected[i]).into_affine());}
 assert_eq!(*roots,before);outputs
}

#[test]
fn weighted_256_bucket_dot_products_and_active_tails(){let mut comparisons=0;for kind in 0..4 {for offset in 0..4 {let(r,s,e)=fixture(kind,0xbac312449f6d8357+offset);for active in 0..=8 {comparisons+=check_fold(&r,&s,&e,active);}}}assert_eq!(comparisons,4608);println!("WEIGHTED_STAGE_OUTPUTS={comparisons};GROUPS=576;FOLD_STEPS=8");}

#[test]
fn every_small_stack_worker_checks_the_weighted_fold(){let(r,s,e)=fixture(3,0x731abcde3251f817);let mut workers_seen=0;for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();let seen=std::sync::Mutex::new(std::collections::BTreeSet::new());pool.broadcast(|ctx|{assert_eq!(check_fold(&r,&s,&e,8),64);seen.lock().unwrap().insert(ctx.index());});assert_eq!(seen.into_inner().unwrap().len(),workers);workers_seen+=workers;}assert_eq!(workers_seen,53);println!("ACTUAL_WORKERS=53;THREADED_STAGE_OUTPUTS=3392");}
