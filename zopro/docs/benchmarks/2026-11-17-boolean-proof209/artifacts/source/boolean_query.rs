// Test-only complete-query experiment. Static positions come from HintBits,
// but every value and index is guarded before any input word is changed.
use super::*;
use zopro_bn254::{G2Affine,G2Projective};
use std::sync::atomic::{AtomicUsize,Ordering};
static ROUTES:[AtomicUsize;4]=[const{AtomicUsize::new(0)};4];

fn peel(bases:&[G2Affine],words:&mut[[u64;4]],positions:&[u32])->Option<Vec<u32>>{
    if bases.len()!=words.len()||positions.is_empty()
        ||positions.windows(2).any(|p|p[0]>=p[1])
        ||positions.iter().any(|&i|i as usize>=words.len()
           ||words[i as usize]!=[0;4]&&words[i as usize]!=[1,0,0,0]) {return None;}
    let n=positions.iter().filter(|&&i|words[i as usize]==[1,0,0,0]).count();
    let mut selected=Vec::new(); selected.try_reserve_exact(n).ok()?;
    if crate::preparation::allocation_bytes(&selected)?>workspace::vector::<u32>(positions.len(),false)? {return None;}
    selected.extend(positions.iter().copied().filter(|&i|words[i as usize]==[1,0,0,0]));
    for&i in positions {words[i as usize]=[0;4];}
    Some(selected)
}

fn boolean_sum(bases:&[G2Affine],selected:&[u32])->G2Projective{
    if selected.is_empty(){G2Projective::IDENTITY}else{
        let starts=[0,selected.len()];
        crate::msm::reduce_indexed_buckets(bases,selected,&starts)[0].to_projective()
    }
}
pub(crate) fn query(
    bases: &[zopro_bn254::G2Affine],
    words: &mut [[u64; 4]],
    positions: &[u32], on: bool,
) -> zopro_bn254::G2Projective {
    assert_eq!(bases.len(), words.len());
    let groups = crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
        .then(|| scalar_groups(words))
        .flatten();
    if on && groups.is_none() && (1024..65536).contains(&bases.len()) {
        if let Some(selected)=peel(bases,words,positions) {
            ROUTES[1].fetch_add(1,Ordering::Relaxed);
            let(a,b)=rayon::join(||g2_phi::raw_exclusive(bases,words),||boolean_sum(bases,&selected));
            return a+b;
        }
        ROUTES[3].fetch_add(1,Ordering::Relaxed);
    } else {ROUTES[if on{2}else{0}].fetch_add(1,Ordering::Relaxed);}
    #[cfg(all(
        zopro_g2_horner_buckets,
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        feature = "ifma8",
        not(no_ifma)
    ))]
    if (1024..65536).contains(&bases.len()) {
        if let Some(mut g) = groups {
            if !(1024..65536).contains(&g.values.len()) {
                return crate::msm_grouped_b2(bases, &g);
            }
            let points = crate::msm::group_affine(bases, &g.order, &g.starts, &g.counts);
            return g2_phi::raw_exclusive(&points, &mut g.values);
        }
        return g2_phi::raw_exclusive(bases, words);
    }
    match groups {
        Some(g) => crate::msm_grouped_b2(bases, &g),
        None => crate::msm_raw_b2(bases, words),
    }
}
pub(crate) struct Prepared {
    pub(crate) positions:Vec<u32>,
    pub(crate) workspace:usize,
    _owner:crate::preparation::Reservation,
}
static POSITIONS:AtomicUsize=AtomicUsize::new(0);
static WORKSPACE:AtomicUsize=AtomicUsize::new(0);
pub(crate)fn snapshot()->([usize;4],usize,usize){(std::array::from_fn(|i|ROUTES[i].load(Ordering::Relaxed)),POSITIONS.load(Ordering::Relaxed),WORKSPACE.load(Ordering::Relaxed))}
pub(crate)fn prepare(plan:&crate::solver::Plan,n:usize,normal:usize)->Option<Prepared>{
    if std::env::var("ZOPRO_CPU_BOOLEAN_QUERY").as_deref()!=Ok("1") || !(1024..65536).contains(&n) || n!=plan.b_indices.len(){return None;}
    let out=build(plan,normal)?;
    POSITIONS.fetch_max(out.positions.len(),Ordering::Relaxed);
    WORKSPACE.fetch_max(out.workspace,Ordering::Relaxed);
    Some(out)
}
fn merge_ranges(ranges:&mut Vec<core::ops::Range<usize>>){
    ranges.sort_unstable_by_key(|r|(r.start,r.end));
    let mut used=0;
    for i in 0..ranges.len(){let r=ranges[i].clone();if used!=0&&r.start<=ranges[used-1].end{ranges[used-1].end=ranges[used-1].end.max(r.end);}else{ranges[used]=r;used+=1;}}
    ranges.truncate(used);
}
fn each_position(indices:&[u32],ranges:&[core::ops::Range<usize>],mut f:impl FnMut(u32))->Option<usize>{
    if indices.windows(2).any(|p|p[0]>=p[1]) || ranges.iter().any(|r|r.start>=r.end) || ranges.windows(2).any(|r|r[0].end>=r[1].start){return None;}
    let mut r=0;let mut count=0;
    for(i,&wire)in indices.iter().enumerate(){while r<ranges.len()&&ranges[r].end<=wire as usize{r+=1;}if r==ranges.len(){break;}if ranges[r].start<=wire as usize{f(u32::try_from(i).ok()?);count+=1;}}
    Some(count)
}
fn build(plan:&crate::solver::Plan,normal:usize)->Option<Prepared>{
    use crate::preparation::{Reservation,allocation_bytes};
    let mut count=0usize;plan.for_each_boolean_range(|_|count+=1);
    if count==0{return None;}
    let range_peak=workspace::vector::<core::ops::Range<usize>>(count,false)?;
    let _scratch_owner=Reservation::analysis(Some(range_peak))?;
    let mut ranges=Vec::new();ranges.try_reserve_exact(count).ok()?;
    if allocation_bytes(&ranges)?>range_peak{return None;}
    plan.for_each_boolean_range(|r|ranges.push(r));merge_ranges(&mut ranges);
    let n=each_position(&plan.b_indices,&ranges,|_|{})?;
    if n==0{return None;}
    let extra=crate::msm::workspace_boolean_indexed(n)?;
    let meta=workspace::vector::<u32>(n,false)?;
    // Independent owner guarantees baseline+overlap remain forecast even when
    // the ordinary solver schedule chooses its allocation-free fallback.
    let owner=Reservation::schedule(0,Some(meta),Some(normal.checked_add(extra)?))?;
    let mut positions=Vec::new();positions.try_reserve_exact(n).ok()?;
    if allocation_bytes(&positions)?>meta{return None;}
    assert_eq!(each_position(&plan.b_indices,&ranges,|i|positions.push(i))?,n);
    let owner=owner.retain(allocation_bytes(&positions));
    Some(Prepared{positions,workspace:extra,_owner:owner})
}
#[cfg(test)]mod tests{
 use super::*;use zopro_bn254::{Fq,Fq2,Fr};use sha2::{Sha256,Digest};use std::{fs,time::Instant};
 fn positions(name:&str)->Vec<u32>{let b=fs::read(format!("/home/ubuntu/src/zopro-perf-work/boolean-g2-208/{name}.positions")).unwrap();let sha=match name{"1x2"=>"73fbf326bb45062ba8990b12058e1b2ed5dc4761bbf7055005bf411388cda303","2x2"=>"efc9fb8919dee9135d4154cb12f90c4f24c7319f8e4b5a78134197ffebc10636",_=>panic!()};assert_eq!(format!("{:x}",Sha256::digest(&b)),sha);b.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect()}
 fn fixture(name:&str)->(Vec<G2Affine>,Vec<[u64;4]>,String){
  let b=fs::read(format!("/home/ubuntu/src/zopro-perf-work/g2-queue-scan175/inputs/{name}.b2q")).unwrap();let(sha,out)=match name{"1x2"=>("421d6ba37e8b3a6f5feb23829c174d11bc034cb9bd8874adf672d65038fb0ad0","8ff656ce44bbebe641139dbc019ae10fc8b3146db4e3af5ad45a0ed3832d4dd7"),"2x2"=>("2b325a5850f1fc9ea21e715e6027f14cc7a29d2c602d0acc683bbfb8377921bc","0fb52c0dece236bf024cfb1afade3d403618730859e99e60bb2bea90b866ecae"),_=>panic!()};assert_eq!(format!("{:x}",Sha256::digest(&b)),sha);assert_eq!(&b[..8],b"B2SCAN01");let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(b.len(),16+160*n);let p=b[16..16+128*n].chunks_exact(128).map(|b|{let f=b.chunks_exact(32).map(|v|Fq::from_be_bytes(v.try_into().unwrap()).unwrap()).collect::<Vec<_>>();G2Affine::new(Fq2::new(f[1],f[0]),Fq2::new(f[3],f[2]))}).collect();let w=b[16+128*n..].chunks_exact(32).map(|v|std::array::from_fn(|i|u64::from_le_bytes(v[i*8..i*8+8].try_into().unwrap()))).collect();(p,w,out.into())
 }
 fn point_hash(p:&G2Affine)->String{let mut h=Sha256::new();for f in[p.x.c0,p.x.c1,p.y.c0,p.y.c1]{for w in f.to_canonical(){h.update(w.to_le_bytes());}}format!("{:x}",h.finalize())}
 fn array_hash(p:&[G2Affine])->String{let mut h=Sha256::new();for p in p{h.update(point_hash(p).as_bytes());}format!("{:x}",h.finalize())}
 fn pool(workers:usize)->rayon::ThreadPool{rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).start_handler(move|i|unsafe{let mut s:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut s);libc::CPU_SET(if workers==1{2}else{i},&mut s);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&s),&s),0);}).build().unwrap()}
 #[test]fn guards_complete_sums_and_query_routes(){
  let g=G2Projective::generator();let coeff=(0..1024).map(|i|if i%16==0{Fr::from([1u64,1,1,0,2,2][i/16%6])*if [1usize,5].contains(&(i/16%6)){-Fr::ONE}else{Fr::ONE}}else{match i%19{0=>Fr::ZERO,1=>-Fr::from((i+22)as u64),_=>Fr::from((i+22)as u64)}}).collect::<Vec<_>>();let points=G2Projective::batch_to_affine(&coeff.iter().map(|&s|g*s).collect::<Vec<_>>());let original=points.clone();let positions=(0..1024).step_by(16).map(|i|i as u32).collect::<Vec<_>>();
  for workers in[1,4,16,32]{let pool=pool(workers);pool.install(||{for mode in[0,1,2]{let words=(0..1024).map(|i|if i%16==0{[u64::from(mode==1||mode==2&&i%3==0),0,0,0]}else{Fr::from((i*739+100003)as u64).to_canonical()}).collect::<Vec<_>>();let expected=(g*coeff.iter().zip(&words).fold(Fr::ZERO,|s,(&c,&w)|s+c*Fr::from_canonical(w).unwrap())).to_affine();for on in[false,true]{assert_eq!(query(&points,&mut words.clone(),&positions,on).to_affine(),expected);}
   let mut bad=words.clone();bad[positions[1]as usize]=[2,0,0,0];let before=bad.clone();assert!(peel(&points,&mut bad,&positions).is_none());assert_eq!(bad,before);
   for invalid in[vec![],vec![3,3],vec![4,3],vec![1024]]{let mut w=words.clone();assert!(peel(&points,&mut w,&invalid).is_none());assert_eq!(w,words);}
  }
  let words=vec![[1,0,0,0];1024];let expected=(g*coeff.iter().copied().fold(Fr::ZERO,|a,b|a+b)).to_affine();assert_eq!(query(&points,&mut words.clone(),&positions,true).to_affine(),expected);
  });}assert_eq!(points,original);
 }
 #[test]fn complete_queries_match_retained_oracles(){for name in["1x2","2x2"]{let(p,w,e)=fixture(name);let before=array_hash(&p);let pos=positions(name);let pool=pool(4);for on in[false,true]{assert_eq!(point_hash(&pool.install(||query(&p,&mut w.clone(),&pos,on).to_affine())),e);}assert_eq!(array_hash(&p),before);}}

 #[test]fn ranges_cover_overlap_ordering_and_empty(){
  let mut r=vec![9..12,3..8,7..10,20..24,24..27];merge_ranges(&mut r);assert_eq!(r,vec![3..12,20..27]);
  let idx=[0,3,4,11,12,20,26,27,99];let mut out=vec![];assert_eq!(each_position(&idx,&r,|i|out.push(i)),Some(5));assert_eq!(out,vec![1,2,3,5,6]);
  assert_eq!(each_position(&[],&r,|_|panic!()),Some(0));assert_eq!(each_position(&idx,&[],|_|panic!()),Some(0));
  for idx in[vec![1,1],vec![4,3]]{assert_eq!(each_position(&idx,&r,|_|{}),None);}
  assert!(crate::msm::workspace_boolean_indexed(usize::MAX).is_none());
  assert!(crate::msm::workspace_boolean_indexed(1753).unwrap()>1753*4);
 }
 fn solver(name:&str)->crate::solver::Plan{
  let file=match name{"1x2"=>"transfer_confidential_1_2.image","2x2"=>"transfer_confidential_2_2.image",_=>panic!()};
  let b=fs::read(format!("/home/ubuntu/src/zopro-perf-work/accepted-c7a201/bundle/images/{file}")).unwrap();
  let u=|p:usize|u32::from_le_bytes(b[p..p+4].try_into().unwrap())as usize;let v=|p:usize|u64::from_le_bytes(b[p..p+8].try_into().unwrap())as usize;let align=|n:usize|n.div_ceil(64)*64;
  let meta=align(24+v(16));let(na,nb,nk,nz)=(u(56),u(60),u(64),u(68));let(ns,np)=(v(meta),v(meta+8));let at=align(meta+128+64*(na+nb+nk+ns+nz+1)+128*nb+64*np+8*ns);assert_eq!(b.len(),at+8+v(at));
  let bytes=&b[at+8..];let expected=if name=="1x2"{"0f6851718a9faa3c0d6f2107696f1ef7d4862bec100f7501b4f55f071165a6a4"}else{"8693f80ae587e73f51aa99f7dffcef81e7e82cdb9890359a7b140cde70df1288"};assert_eq!(format!("{:x}",Sha256::digest(bytes)),expected);
  crate::solver::Plan::from_image(bytes).unwrap()
 }
 #[test]fn metadata_matches_real_solver_and_denial_is_clean(){
  for name in["1x2","2x2"]{let plan=solver(name);let before=plan.image();let p=build(&plan,1<<20).unwrap();assert_eq!(p.positions,positions(name));assert_eq!(p.workspace,crate::msm::workspace_boolean_indexed(p.positions.len()).unwrap());assert_eq!(p.positions.len(),p.positions.capacity());drop(p);assert_eq!(plan.image(),before);
   assert!(build(&plan,usize::MAX).is_none());assert_eq!(plan.image(),before);
   let params=[Fr::ONE,Fr::from(5u64)].iter().flat_map(|f|f.0.iter().flat_map(|w|w.to_le_bytes())).collect::<Vec<_>>();
   let mut key=crate::Key::build(crate::Points::Kernel([&[];5]),&params).unwrap();
   key.boolean_query=Some(build(&plan,1<<20).unwrap());key.solver=Some(crate::solver::Plan::from_image(&before).unwrap());
   key.invalidate_solver();assert!(key.boolean_query.is_none());assert!(key.solver.is_none());
   key.basis_only=true;key.boolean_query=Some(build(&plan,1<<20).unwrap());key.solver=Some(crate::solver::Plan::from_image(&before).unwrap());
   key.invalidate_solver();assert!(key.boolean_query.is_none());assert!(key.basis_fallback.is_some());
   let previous=std::env::var_os("ZOPRO_CPU_PREPARE_MAX_BYTES");
   unsafe{std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES","0");}
   let denied=build(&plan,1<<20).is_none();
   unsafe{if let Some(p)=previous{std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES",p);}else{std::env::remove_var("ZOPRO_CPU_PREPARE_MAX_BYTES");}}
   assert!(denied);assert_eq!(plan.image(),before);

  }
 }
}
