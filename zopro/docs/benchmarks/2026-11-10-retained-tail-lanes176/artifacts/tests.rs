use ark_ec::{AffineRepr,CurveGroup,PrimeGroup};
use ark_ff::{PrimeField,Field as ArkField};
use std::sync::atomic::Ordering::Relaxed;

macro_rules! curve_cases {
 ($one:ident,$thread:ident,$config:ty,$ark:ty,$convert:expr)=>{
 fn $one()->(usize,usize){
  type C=$config;type A=<$ark as CurveGroup>::Affine;
  let generator=<$ark>::generator();
  let offset=ark_bn254::Fr::from_le_bytes_mod_order(b"retained tails independent full width seed 2026-11-10");
  let mut p=generator*offset;let mut projective=Vec::new();for _ in 0..4096{projective.push(p);p+=generator;}
  let ark_points=<$ark>::normalize_batch(&projective);let convert=$convert;
  let mut sizes=(0usize..=64).collect::<Vec<_>>();sizes.extend([65,71,72,73,127,128,129,199,200,201,255,256,257,351,352,353,511,512,513,1023,1024,1025]);
  let mut cases=0;let mut oracle_outputs=0;
  for &m in &sizes{for pattern in 0..5{
   let lengths=if pattern==4{vec![0,2*m,3,0,5,1,2,7]}else{vec![2*m]};
   let mut starts=vec![];let mut offset=0;for &n in &lengths{starts.push(offset);offset+=n;}
   let mut input:Vec<A>=(0..offset).map(|i|ark_points[(i*37+101)%ark_points.len()]).collect();
   for i in(0..offset.saturating_sub(1)).step_by(2){match pattern{1 if i%14==0=>input[i]=A::identity(),2 if i%10==0=>input[i+1]=input[i],3 if i%10==0=>input[i+1]=-input[i],_=>{}}}
   let native:Vec<Affine<C>>=input.iter().map(|p|convert(*p)).collect();let raw=native.iter().copied().map(limbs::<C>).collect::<Vec<_>>();
   let mut sums=vec![];let mut destinations=vec![];for(b,&n)in lengths.iter().enumerate(){for j in 0..n/2{sums.push(input[starts[b]+2*j].into_group()+input[starts[b]+2*j+1]);destinations.push(starts[b]+j);}if n%2!=0{sums.push(input[starts[b]+n-1].into_group());destinations.push(starts[b]+n/2);}}
   let expected=<$ark>::normalize_batch(&sums).into_iter().map(convert).collect::<Vec<_>>();
   for capacity_mode in 0..4{for vector_inverse in[false,true]{
    let mut actual=raw.clone();let mut original=raw.clone();let mut ac=lengths.clone();let mut oc=lengths.clone();let capacity=(offset/2).div_ceil(8)*8+8;
    let mut scratch=Scratch::<C>::default();scratch.vector_inverse=vector_inverse;
    if capacity_mode&1!=0{scratch.pairs=Vec::with_capacity(capacity);}if capacity_mode&2!=0{scratch.groups=Vec::with_capacity(capacity/8);scratch.prefix_records=Vec::with_capacity(capacity/8);}
    let mut reference=Scratch::<C>::default();reference.vector_inverse=vector_inverse;
    reduce_layer::<C>(&mut actual,&starts,&mut ac,&mut scratch);reduce_layer_original::<C>(&mut original,&starts,&mut oc,&mut reference);
    assert_eq!(ac,oc);assert_eq!(scratch.pairs,reference.pairs);
    for(a,b)in actual.iter().zip(&original){assert_eq!(affine::<C>(*a),affine::<C>(*b));}
    for(&destination,want)in destinations.iter().zip(&expected){assert_eq!(affine::<C>(actual[destination]),*want);oracle_outputs+=1;}
    if capacity_mode==3{assert_eq!(scratch.pairs.capacity(),capacity);assert_eq!(scratch.groups.capacity(),capacity/8);assert_eq!(scratch.prefix_records.capacity(),capacity/8);}
    assert!(native.iter().zip(&input).all(|(a,b)|*a==convert(*b)));cases+=1;
   }}
  }}
  (cases,oracle_outputs)
 }
 fn $thread()->(Vec<LimbPoint<<$config as CurveConfig>::Base>>,Vec<usize>,Vec<usize>,Vec<Affine<$config>>){
  let generator=<$ark>::generator();let lengths=[0usize,257,17,64,1,0,135,8];let mut starts=vec![];let mut n=0;for &length in &lengths{starts.push(n);n+=length;}
  let points=(0..n).map(|i|generator*ark_bn254::Fr::from((i*37+1234567)as u64)).collect::<Vec<_>>();let points=<$ark>::normalize_batch(&points);let convert=$convert;
  let mut expected=vec![];for(b,&length)in lengths.iter().enumerate(){let sum: $ark=points[starts[b]..starts[b]+length].iter().copied().sum();expected.push(convert(sum.into_affine()));}
  let raw=points.into_iter().map(convert).map(limbs::<$config>).collect();(raw,starts,lengths.to_vec(),expected)
 }
 };
}
curve_cases!(g1_cases,g1_threads,zopro_bn254::G1Config,ark_bn254::G1Projective,|p:ark_bn254::G1Affine|{if p.infinity{zopro_bn254::G1Affine::IDENTITY}else{zopro_bn254::G1Affine::new(Fq::from_canonical(p.x.into_bigint().0).unwrap(),Fq::from_canonical(p.y.into_bigint().0).unwrap())}});
curve_cases!(g2_cases,g2_threads,zopro_bn254::G2Config,ark_bn254::G2Projective,|p:ark_bn254::G2Affine|{if p.infinity{zopro_bn254::G2Affine::IDENTITY}else{let f=|x:ark_bn254::Fq|Fq::from_canonical(x.into_bigint().0).unwrap();zopro_bn254::G2Affine::new(Fq2::new(f(p.x.c0),f(p.x.c1)),Fq2::new(f(p.y.c0),f(p.y.c1)))}});

#[test]fn g1_all_residues_exceptions_and_capacity_fallbacks(){let before=RETAINED_PADDED_LAYERS.load(Relaxed);let(cases,outputs)=g1_cases();assert!(RETAINED_PADDED_LAYERS.load(Relaxed)>before);println!("RETAINED_TAIL_G1 cases={cases} Arkworks_compacted_outputs={outputs};all_original_slots_match=true;capacity_guard=true");}
#[test]fn g2_all_residues_exceptions_and_capacity_fallbacks(){let before=RETAINED_PADDED_LAYERS.load(Relaxed);let(cases,outputs)=g2_cases();assert!(RETAINED_PADDED_LAYERS.load(Relaxed)>before);println!("RETAINED_TAIL_G2 cases={cases} Arkworks_compacted_outputs={outputs};all_original_slots_match=true;capacity_guard=true");}

fn layered<C:CurveConfig>(input:&[LimbPoint<C::Base>],starts:&[usize],counts:&[usize],want:&[Affine<C>])where C::Base:BatchInverse{
 let mut actual=input.to_vec();let mut original=input.to_vec();let mut ac=counts.to_vec();let mut oc=counts.to_vec();let mut scratch=Scratch::<C>::default();let mut reference=Scratch::<C>::default();
 while ac.iter().any(|n|*n>1){reduce_layer::<C>(&mut actual,starts,&mut ac,&mut scratch);reduce_layer_original::<C>(&mut original,starts,&mut oc,&mut reference);assert_eq!(ac,oc);for(a,b)in actual.iter().zip(&original){assert_eq!(affine::<C>(*a),affine::<C>(*b));}}
 for(b,&n)in counts.iter().enumerate(){if n!=0{assert_eq!(affine::<C>(actual[starts[b]]),want[b]);}}
}
#[test]fn changing_layers_actual_workers_and_shared_callers(){
 let g1=g1_threads();let g2=g2_threads();let before=RETAINED_PADDED_LAYERS.load(Relaxed);let run=||{layered::<zopro_bn254::G1Config>(&g1.0,&g1.1,&g1.2,&g1.3);layered::<zopro_bn254::G2Config>(&g2.0,&g2.1,&g2.2,&g2.3);};let mut workers=0;
 for n in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(n).stack_size(512*1024).build().unwrap();let seen=std::sync::Mutex::new(std::collections::BTreeSet::new());pool.broadcast(|ctx|{seen.lock().unwrap().insert(ctx.index());run();});assert_eq!(seen.into_inner().unwrap().len(),n);workers+=n;}
 let pool=rayon::ThreadPoolBuilder::new().num_threads(8).stack_size(512*1024).build().unwrap();std::thread::scope(|scope|{let mut handles=vec![];for _ in 0..4{let run=&run;let pool=&pool;handles.push(scope.spawn(move||pool.install(run)));}for h in handles{h.join().unwrap();}});assert_eq!(workers,53);let padded=RETAINED_PADDED_LAYERS.load(Relaxed)-before;assert!(padded>0);println!("RETAINED_TAIL_LAYERED worker_executions={workers};shared_callers=4;complete_bucket_trees=114;padded_layer_calls={padded};original_and_Arkworks_match=true");
}
