from pathlib import Path
import json,hashlib,shutil,time
r=Path(__file__).resolve().parent;repo=Path('/home/ubuntu/src/zk-tools/zopro')
a=json.loads(Path('/tmp/zopro-small-core-entry-adoption/production-source-manifest.json').read_text());component=Path('/tmp/zopro-prefix-records')
def sha(p):
    with Path(p).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
x=json.loads((component/'results.json').read_text());assert not x['passed']and x['cases']['g2-b200']['passed']
assert not(r/'workspace').exists();w=r/'workspace'
for n,h in a['source_files_sha256'].items():
    assert sha(repo/n)==h;dest=w/n;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(repo/n,dest)
save('accepted.json',a)
guard='all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma))'
source=(component/'workspace/crates/zopro-core/src/msm/prefix_records.rs').read_text();start=source.index('pub(super) fn g2<');end=source.index('\nfn points<',start)
body='//! G2 inversion prefix/norm records; native experimental path.\nuse super::*;\n'+source[start:end].replace('pub(super) fn g2<','pub(super) fn apply<',1)
body+='''
#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::G2Config;
    fn points(n:usize,offset:u64)->Vec<Affine<G2Config>>{let g=Projective::<G2Config>::generator();let mut p=g.mul(&Fr::from(offset));let mut out=Vec::new();for _ in 0..n{out.push(p);p+=g;}Projective::<G2Config>::batch_to_affine(&out)}
    #[test]fn g2_prefix_complete_curve_tails_reuse(){
        for n in [0,1,7,8,9,15,16,80,200,352,504]{
            let size=(n+1usize).next_power_of_two();let input=points(size,7);let incoming=points(n,50007);
            for canonical in [false,true]{for vector_inverse in [false,true]{
                let batch:Vec<_>=incoming.iter().enumerate().map(|(i,&p)|((i*37%size)as u32,if i%3==0{-p}else{p})).collect();
                let mut actual=input.clone();let mut expected=input.clone();let mut s=Fq2AddScratch{canonical,vector_inverse,..Default::default()};
                for _ in 0..3{for &(id,p)in &batch{let i=id as usize;assert_ne!(expected[i].x,p.x);expected[i]=(expected[i]+p).to_affine();}
                    if canonical{apply::<G2Config,true>(&mut actual,&batch,&mut s)}else{apply::<G2Config,false>(&mut actual,&batch,&mut s)}
                    assert_eq!(actual,expected);assert_eq!(s.groups.capacity(),0);
                }
            }}
        }
    }
    #[test]fn g2_prefix_dispatch_and_control(){
        let input=points(16,7);let incoming=points(9,50007);let batch:Vec<_>=incoming.into_iter().enumerate().map(|(i,p)|(i as u32,p)).collect();
        let expected:Vec<_>=input.iter().enumerate().map(|(i,p)|if i<9{(*p+batch[i].1).to_affine()}else{*p}).collect();
        for enabled in [false,true]{let mut actual=input.clone();let mut s=Fq2AddScratch{canonical:true,vector_inverse:true,prefix_enabled:enabled,..Default::default()};stream_g2_impl::<G2Config,true>(&mut actual,&batch,&mut s);assert_eq!(actual,expected);if enabled{assert_eq!(s.groups.capacity(),0);assert!(!s.prefix_records.is_empty());}else{assert_eq!(s.prefix_records.capacity(),0);assert!(!s.groups.is_empty());}}
    }
}
'''
dest=w/'crates/zopro-core/src/msm/g2_prefix.rs';dest.parent.mkdir(parents=True,exist_ok=True);dest.write_text(body)
n='crates/zopro-core/src/msm.rs';old=(w/n).read_text();new=old;insertions=[]
def once(anchor,addition):
    global new
    assert new.count(anchor)==1,anchor;new=new.replace(anchor,anchor+addition);insertions.append(addition)
once('''    groups: Vec<Fq2AddGroup>,''','''
    #[cfg('''+guard+''')] prefix_records:Vec<[FqBatch;2]>,
    #[cfg('''+guard+''')] prefix_enabled:bool,
    #[cfg('''+guard+''')] prefix_trace:bool,
    #[cfg('''+guard+''')] prefix_traced:bool,''')
once('''            fq2: Fq2AddScratch {
                vector_inverse,
                canonical,''','''
                #[cfg('''+guard+''')] prefix_enabled:std::env::var("ZOPRO_CPU_G2_PREFIX_RECORDS").as_deref()==Ok("1"),
                #[cfg('''+guard+''')] prefix_trace:std::env::var_os("ZOPRO_CPU_G2_PREFIX_TRACE").is_some(),''')
once('''fn stream_g2_impl<C: CurveConfig<Base = Fq2>, const FAST: bool>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, Affine<C>)],
    s: &mut Fq2AddScratch,
) {''','''
    #[cfg('''+guard+''')] {
        if s.prefix_trace && !s.prefix_traced {eprintln!("G2_PREFIX_RECORDS enabled={} batch={}",s.prefix_enabled,batch.len());s.prefix_traced=true;}
        if s.prefix_enabled {return g2_prefix::apply::<C,FAST>(buckets,batch,s);}
    }''')
module='\n#[cfg('+guard+')]\nmod g2_prefix;\n';new+=module
recovered=new.removesuffix(module)
for addition in insertions:recovered=recovered.replace(addition,'',1)
assert recovered==old
(w/n).write_text(new)
shutil.copy2('/tmp/zopro-small-core-entry-adoption/native-sdk.rs',r/'sdk.rs')
save('source-pins.json',{str(f.relative_to(w)):sha(f)for f in sorted(w.rglob('*'))if f.is_file()})
save('scope-audit.json',dict(parent_combined_component_failed=True,parent_results_sha256=sha(component/'results.json'),original_msm_body_recovered_byte_for_byte=True,g1_candidate_body_absent=True,only_g2_stream_dispatch_changed=True,scalar_tail_reused=True,extra_scratch_metadata='One Vec header plus three bool fields in private native Fq2 scratch; actual pressure/stack/forecast qualification pending. Old group capacity remains zero on the new path; constructor fixes path choice per scratch instance.'))
save('protocol.json',dict(declared=time.time(),scope='New G2-only complete-SDK hypothesis. Combined G1/G2 component remains failed and is not adopted. G1 stays on accepted algorithm; only native G2 streaming dispatch changes.',parent_failed_report='/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-25-prefix-records-component/README.md',evidence_sha256=sha(component/'results.json'),rustflags=a['rustflags']+' --cfg zopro_g2_prefix_records',driver_sha256=sha(r/'sdk.rs'),runtime_flag='ZOPRO_CPU_G2_PREFIX_RECORDS',default='OFF experimental, option read at scratch construction',trace_flag='ZOPRO_CPU_G2_PREFIX_TRACE',preflight='All9nativeOFF/ON4worker fullSDK plus defaultmissingflagprimary32;3primes+1captured each. Trace must confirm actual G2 stream selection for all9 shapes, tracing absent in timed cells.',cases=[dict(shape=shape,workers=32,rounds=31)for shape in ['1x2','2x2','4x2','8x2','12x2','16x2','24x2','merge-24x1','merge-54x1']]+[dict(shape='1x2',workers=16,rounds=31)],blocks=7,variants=['accepted','off','on'],gate='Fresh7block full9+primary16 campaign. Primary32 ON/accepted and ON/OFF pairedmedian<1 with>=4/7wins. All9geometricpairedON/accepted and ON/OFF<1. Every10case<=1.02pairedmedian and<=1.05everyblock vsboth. OFF/accepted<=1.03. No component result substitutes this gate; no minimum gain floor or unchanged reroll.',expected_sdk=7216,expected_distinct_gnark=229,limitations='No promotion from component or SDK pilot alone. Portable, concurrency, memory transitions/cold+warm limits, resource forecasts, codegen/stack and actual250/4999 qualification remain required. No2x speed prediction.',adopted=False,goal_achieved=False))
print('G2_ONLY_SDK_PREPARED')
