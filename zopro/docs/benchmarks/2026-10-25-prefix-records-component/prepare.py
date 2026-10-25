from pathlib import Path
import hashlib,json,shutil,time
r=Path(__file__).resolve().parent;repo=Path('/home/ubuntu/src/zk-tools/zopro')
a=json.loads(Path('/tmp/zopro-small-core-entry-adoption/production-source-manifest.json').read_text())
def sha(p):
    with Path(p).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
assert not(r/'workspace').exists();w=r/'workspace'
for n,h in a['source_files_sha256'].items():
    assert sha(repo/n)==h;f=w/n;f.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(repo/n,f)
save('accepted.json',a)
msm=(w/'crates/zopro-core/src/msm.rs').read_text()
def tail(fn,next_marker):
    start=msm.index(fn);end=msm.index(next_marker,start);body=msm[start:end]
    return body[body.index('    s.tail.clear();'):body.index('    for (j, g) in s.groups.iter().enumerate().rev()')]
g1tail=tail('fn stream_g1_impl','#[derive(Clone, Copy)]\nstruct Fq2AddGroup')
g2tail=tail('fn stream_g2_impl','// W(B) =')
header='''//! Prefix records for nonaliasing streaming batches; private component only.
use super::*;

pub(super) fn g1<C:CurveConfig<Base=Fq>,const FAST:bool,const SIGNED:bool>(
    buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut FqAddScratch,
) {
    let full=batch.len()/LANES*LANES;
    let mut chains=FqBatch::one();
    s.prefix_records.clear();s.prefix_records.reserve(full/LANES);
    for k in (0..full).step_by(LANES) {
        let ax=FqBatch::new(&core::array::from_fn(|i|buckets[batch_index::<SIGNED>(batch[k+i].0)].x));
        let bx=FqBatch::new(&core::array::from_fn(|i|batch[k+i].1.x));
        let denominator=canonical_sub::<FAST>(&bx,&ax);
        s.prefix_records.push(chains);chains=chains.mul(&denominator);
    }
'''
g1finish='''    for j in (0..full/LANES).rev() {
        let k=j*LANES;
        let a:[Affine<C>;LANES]=core::array::from_fn(|i|buckets[batch_index::<SIGNED>(batch[k+i].0)]);
        let b:[Affine<C>;LANES]=core::array::from_fn(|i|batch[k+i].1);
        let ax=FqBatch::new(&a.map(|p|p.x));let ay=FqBatch::new(&a.map(|p|p.y));
        let bx=FqBatch::new(&b.map(|p|p.x));let by=FqBatch::new(&b.map(|p|p.y));
        let denominator=canonical_sub::<FAST>(&bx,&ax);
        let factor=s.prefix_records[j].mul(&inverse);
        inverse=inverse.mul(&denominator);
        let slope=scaled_signed_dy::<FAST,SIGNED>(&by,&ay,&factor,signs::<SIGNED,C>(&batch[k..k+LANES]));
        let x=slope.mul(&slope).sub(&canonical_add::<FAST>(&ax,&bx));
        let y=slope.mul(&ax.sub(&x)).sub(&ay);
        let(x,y)=(x.get(),y.get());
        for i in 0..LANES {buckets[batch_index::<SIGNED>(batch[k+i].0)]=Affine::new(x[i],y[i]);}
    }
}

pub(super) fn g2<C:CurveConfig<Base=Fq2>,const FAST:bool>(
    buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut Fq2AddScratch,
) {
    let full=batch.len()/LANES*LANES;
    let mut chains=FqBatch::one();
    s.prefix_records.clear();s.prefix_records.reserve(full/LANES);
    for k in (0..full).step_by(LANES) {
        let ax=fq2_pack(core::array::from_fn(|i|buckets[batch[k+i].0 as usize].x));
        let bx=fq2_pack(core::array::from_fn(|i|batch[k+i].1.x));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let norm=Fq2::lanes_norm(&denominator);
        s.prefix_records.push([chains,norm]);chains=chains.mul(&norm);
    }
'''
g2finish='''    for j in (0..full/LANES).rev() {
        let k=j*LANES;
        let a:[Affine<C>;LANES]=core::array::from_fn(|i|buckets[batch[k+i].0 as usize]);
        let b:[Affine<C>;LANES]=core::array::from_fn(|i|batch[k+i].1);
        let ax=fq2_pack(a.map(|p|p.x));let ay=fq2_pack(a.map(|p|p.y));
        let bx=fq2_pack(b.map(|p|p.x));let by=fq2_pack(b.map(|p|p.y));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let dy=core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]));
        let factor=s.prefix_records[j][0].mul(&inverse);
        inverse=inverse.mul(&s.prefix_records[j][1]);
        let slope=Fq2::lanes_scaled_numerator(&dy,&denominator,&factor);
        let x=Fq2::lanes_sub(&Fq2::lanes_square(&slope),&Fq2::lanes_add(&ax,&bx));
        let y=Fq2::lanes_sub(&Fq2::lanes_mul(&slope,&Fq2::lanes_sub(&ax,&x)),&ay);
        let(x,y)=(fq2_unpack(x),fq2_unpack(y));
        for i in 0..LANES {buckets[batch[k+i].0 as usize]=Affine::new(x[i],y[i]);}
    }
}
'''
source=header+g1tail+g1finish+g2tail+g2finish
assert g1tail in source and g2tail in source
# Explicit correctness checks use the complete scalar curve formulas, not
# the old affine batch kernel, as the expected result.
tests='''
fn points<C:CurveConfig>(n:usize,offset:u64)->Vec<Affine<C>> {
    let generator=Projective::<C>::generator();let mut p=generator.mul(&Fr::from(offset));
    let mut out=Vec::with_capacity(n);for _ in 0..n{out.push(p);p+=generator;}
    Projective::<C>::batch_to_affine(&out)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn prefix_g1_reference_signed_tails_and_reuse(){
        use zopro_bn254::G1Config;
        for n in [0,1,7,8,9,15,16,80,200,352,504]{
            let size=(n+1usize).next_power_of_two();let input=points::<G1Config>(size,7);let additions=points::<G1Config>(n,50007);
            for canonical in [false,true]{for vector_inverse in [false,true]{for signed in [false,true]{
                let batch:Vec<_>=additions.iter().enumerate().map(|(i,&p)|((i*37%size)as u32|if signed&&i%3==0{1<<31}else{0},p)).collect();
                let mut actual=input.clone();let mut expected=input.clone();let mut s=FqAddScratch{canonical,vector_inverse,..Default::default()};
                for _ in 0..3{
                    for &(id,p)in &batch{let index=(id&0x7fffffff)as usize;let p=if id>>31!=0{-p}else{p};assert_ne!(expected[index].x,p.x);expected[index]=(expected[index]+p).to_affine();}
                    if signed{if canonical{g1::<G1Config,true,true>(&mut actual,&batch,&mut s)}else{g1::<G1Config,false,true>(&mut actual,&batch,&mut s)}}else if canonical{g1::<G1Config,true,false>(&mut actual,&batch,&mut s)}else{g1::<G1Config,false,false>(&mut actual,&batch,&mut s)}
                    assert_eq!(actual,expected,"n={n} canonical={canonical} vector={vector_inverse} signed={signed}");assert_eq!(s.groups.capacity(),0);
                }
            }}}
        }
    }
    #[test]fn prefix_g2_reference_tails_and_reuse(){
        use zopro_bn254::G2Config;
        for n in [0,1,7,8,9,15,16,80,200,352,504]{
            let size=(n+1usize).next_power_of_two();let input=points::<G2Config>(size,7);let additions=points::<G2Config>(n,50007);
            for canonical in [false,true]{for vector_inverse in [false,true]{
                let batch:Vec<_>=additions.iter().enumerate().map(|(i,&p)|((i*37%size)as u32,if i%3==0{-p}else{p})).collect();
                let mut actual=input.clone();let mut expected=input.clone();let mut s=Fq2AddScratch{canonical,vector_inverse,..Default::default()};
                for _ in 0..3{
                    for &(id,p)in &batch{let index=id as usize;assert_ne!(expected[index].x,p.x);expected[index]=(expected[index]+p).to_affine();}
                    if canonical{g2::<G2Config,true>(&mut actual,&batch,&mut s)}else{g2::<G2Config,false>(&mut actual,&batch,&mut s)}
                    assert_eq!(actual,expected,"n={n} canonical={canonical} vector={vector_inverse}");assert_eq!(s.groups.capacity(),0);
                }
            }}
        }
    }
}
'''
source+=tests
# The component has no normal-prover dispatch. Inputs and scratch setup are
# outside the timer. Existing and new kernels update the same valid buckets.
for group in [1,2]:
    config=f'zopro_bn254::G{group}Config';scratch='FqAddScratch'if group==1 else'Fq2AddScratch'
    old='stream_g1_impl::<'+config+',true,true>'if group==1 else'stream_g2_impl::<'+config+',true>'
    new='g1::<'+config+',true,true>'if group==1 else'g2::<'+config+',true>'
    source+=f'''
fn bench_g{group}(mode:u8,n:usize,iterations:usize)->(u64,usize,Vec<u8>){{
    let mut buckets=points::<{config}>({4096 if group==1 else 2048},7);
    let incoming=points::<{config}>(n,50007);
    let batch:Vec<_>=incoming.into_iter().enumerate().map(|(i,p)|{{let id=(i*37%buckets.len())as u32;'''+('''(id|if i%3==0{1<<31}else{0},p)'''if group==1 else'''(id,if i%3==0{-p}else{p})''')+f'''}}).collect();
    let mut scratch={scratch}{{canonical:true,vector_inverse:true,'''+('''write_groups:true,reserve_groups:true,'''if group==1 else''' ''')+f'''..Default::default()}};
    for _ in 0..3{{if mode==0{{{old}(&mut buckets,&batch,&mut scratch)}}else{{{new}(&mut buckets,&batch,&mut scratch)}}}}
    let started=std::time::Instant::now();
    for _ in 0..iterations{{if mode==0{{{old}(std::hint::black_box(&mut buckets),std::hint::black_box(&batch),&mut scratch)}}else{{{new}(std::hint::black_box(&mut buckets),std::hint::black_box(&batch),&mut scratch)}}}}
    let elapsed=started.elapsed().as_nanos()as u64;
    let bytes=if mode==0{{scratch.groups.capacity()*std::mem::size_of_val(&scratch.groups[0])}}else{{scratch.prefix_records.capacity()*std::mem::size_of_val(&scratch.prefix_records[0])}};
    let mut checksum=Vec::new();for p in buckets{{for f in [p.x,p.y]{{'''+('''for v in [f]{'''if group==1 else'''for v in [f.c0,f.c1]{''')+'''for word in v.to_canonical(){checksum.extend_from_slice(&word.to_le_bytes());}}}}
    (elapsed,bytes,checksum)
}
'''
source+='''pub(crate) fn benchmark(curve:u8,mode:u8,n:usize,iterations:usize)->(u64,usize,Vec<u8>){assert!(mode<=1&&n>0);match curve{1=>bench_g1(mode,n,iterations),2=>bench_g2(mode,n,iterations),_=>panic!("curve")}}\n'''
dest=w/'crates/zopro-core/src/msm/prefix_records.rs';dest.parent.mkdir(parents=True,exist_ok=True);dest.write_text(source)
guard='all(zopro_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma))'
msm+='\n#[cfg('+guard+')]\nmod prefix_records;\n#[cfg('+guard+')]\npub(crate) use prefix_records::benchmark as prefix_component;\n'
msm=msm.replace('    groups: Vec<FqAddGroup>,','    groups: Vec<FqAddGroup>,\n    #[cfg('+guard+')]\n    prefix_records:Vec<FqBatch>,',1)
msm=msm.replace('    groups: Vec<Fq2AddGroup>,','    groups: Vec<Fq2AddGroup>,\n    #[cfg('+guard+')]\n    prefix_records:Vec<[FqBatch;2]>,',1)
(w/'crates/zopro-core/src/msm.rs').write_text(msm)
lib=w/'crates/zopro-core/src/lib.rs';lib.write_text(lib.read_text()+'\n#[cfg('+guard+')]\npub fn prefix_records_component(curve:u8,mode:u8,n:usize,iterations:usize)->(u64,usize,Vec<u8>){msm::prefix_component(curve,mode,n,iterations)}\n')
save('tail-reuse-audit.json',dict(g1_tail_sha256=hashlib.sha256(g1tail.encode()).hexdigest(),g2_tail_sha256=hashlib.sha256(g2tail.encode()).hexdigest(),original_scalar_tail_code_reused_verbatim=True,normal_prover_dispatch_unchanged=True))
save('source-pins.json',{str(f.relative_to(w)):sha(f)for f in sorted(w.rglob('*'))if f.is_file()})
save('protocol.json',dict(declared=time.time(),scope='Native private stream-kernel component. Inversion prefixes replace materialized coordinates; no SDK dispatch, key change, point table, C or ASM.',base_manifest='/tmp/zopro-small-core-entry-adoption/production-source-manifest.json',evidence='/tmp/zopro-operation-model/analysis.json',evidence_sha256=sha('/tmp/zopro-operation-model/analysis.json'),rustflags=a['rustflags']+' --cfg zopro_prefix_records',hypothesis='Reduce G1 group records5to1vectors and G2 groups9to2. Repack original nonaliasing coordinates on reverse. G2 combines prefix*inverse once for both components, saving one Fq product per point. Added packing/dependencies may regress.',oracle='Two new native tests: independent complete scalar curve formula for signed/unsigned G1 and signed-point G2, sizes0/1/7/8/9/15/16/80/200/352/504, both canonical and inverse modes,3scratch reuses. Existing old group vectors must remain unallocated on candidate.',cases=[dict(curve=1,batch=200),dict(curve=1,batch=352),dict(curve=2,batch=200)],blocks=31,iterations=256,primes=3,gate='Per-case paired new/old median<1 and>=21of31wins; geometric mean<1. No component adoption. If anycase fails, close this exact design with no full SDK integration or unchanged reroll.',limits='Bounded component only. Old kernel direct call from accepted body in same private build is control. Native point coordinates and batch alias contract retained. Logical scratch payload is not whole-process RAM; no portable/pressure/concurrency/large/AWS qualification.',adopted=False,goal_achieved=False))
print('PREFIX_RECORD_COMPONENT_PREPARED')
