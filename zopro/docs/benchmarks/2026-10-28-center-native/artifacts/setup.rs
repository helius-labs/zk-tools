use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/center-native";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let r=Path::new(R);assert!(!r.join("source").exists());let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    for(f,h)in m["source_files_sha256"].as_object().unwrap(){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());let out=r.join("source").join(f);fs::create_dir_all(out.parent().unwrap()).unwrap();fs::copy(p,out).unwrap();}
    let mut edits=BTreeMap::<String,Vec<(String,String)>>::new();
    edits.insert("crates/zopro-core/src/lib.rs".into(),vec![
        ("let (sa_vec,mut sb_vec):(Vec<_>,Vec<_>)=".into(),"let (mut sa_vec,mut sb_vec):(Vec<_>,Vec<_>)=".into()),
        ("|| rayon::join(|| msm(&self.a[..],sa),\n".into(),"|| rayon::join(|| msm_affine::centered::exclusive(&self.a[..],&mut sa_vec),\n".into())]);
    edits.insert("crates/zopro-core/src/msm_affine.rs".into(),vec![("mod grouping;\n".into(),"mod grouping;\npub(super) mod centered;\n".into())]);
    let pending="crates/zopro-core/src/msm_affine/borrowed_pending.rs";
    let orig=fs::read_to_string(r.join("source").join(pending)).unwrap();let begin=orig.find("pub(crate) fn msm<C:").unwrap();let body=orig[begin..].find("    assert_eq!(bases.len(), scalars.len());").unwrap()+begin;
    let signature=orig[begin..body].to_string();let replacement=format!("{signature}    msm_impl::<C,false>(bases,scalars,parts,width)\n}}\n\npub(super)fn msm_centered<C:CurveConfig<Base=Fq>>(bases:&[Affine<C>],scalars:&[[u64;4]],parts:usize,width:usize)->Projective<C>{{msm_impl::<C,true>(bases,scalars,parts,width)}}\n\nfn msm_impl<C:CurveConfig<Base=Fq>,const CENTER:bool>(bases:&[Affine<C>],scalars:&[[u64;4]],parts:usize,width:usize)->Projective<C>{{\n");
    edits.insert(pending.into(),vec![
        ("    fn feed(\n".into(),"    fn feed<const CENTER:bool>(\n".into()),
        ("            let d = digit(scalar, w, bits);\n".into(),"            let d = if CENTER {super::centered::digit(scalar,w,bits)} else {digit(scalar,w,bits)};\n".into()),
        ("            state.feed(k, sk, w, bits, size);\n".into(),"            state.feed::<false>(k, sk, w, bits, size);\n".into()),
        ("            state.feed(h, sh, w, bits, size);\n".into(),"            state.feed::<false>(h, sh, w, bits, size);\n".into()),
        (signature,replacement),
        ("            if let Some(sum) = top_window::<C, true>(bases, scalars, w, width) {\n".into(),"            let top=if CENTER {super::centered::top_window::<C>(bases,scalars,w,width)}else{top_window::<C,true>(bases,scalars,w,width)};\n            if let Some(sum) = top {\n".into()),
        ("                    state.feed(&bases[range.clone()], &scalars[range], w, width, size);\n".into(),"                    state.feed::<CENTER>(&bases[range.clone()], &scalars[range], w, width, size);\n".into())]);
    let file="crates/zopro-core/src/msm.rs";let original=fs::read_to_string(r.join("source").join(file)).unwrap();let begin=original.find("pub(super) fn reduce_indexed_buckets<C:").unwrap();let body=original[begin..].find("    let tile = ").unwrap()+begin;let signature=original[begin..body].to_string();
    let replacement=format!("{signature}    reduce_indexed_buckets_impl::<C,false>(bases,order,starts)\n}}\n\n#[cfg(not(target_arch=\"wasm32\"))]\npub(super)fn reduce_indexed_buckets_signed<C:CurveConfig>(bases:&[Affine<C>],order:&[u32],starts:&[usize])->Vec<Affine<C>> where C::Base:BatchInverse {{reduce_indexed_buckets_impl::<C,true>(bases,order,starts)}}\n\nfn reduce_indexed_buckets_impl<C:CurveConfig,const SIGNED:bool>(bases:&[Affine<C>],order:&[u32],starts:&[usize])->Vec<Affine<C>> where C::Base:BatchInverse {{\n");
    edits.insert(file.into(),vec![(signature,replacement),("                .map(|&i| limbs::<C>(bases[i as usize]))\n".into(),"                .map(|&i| {let index=if SIGNED{i&0x7fffffff}else{i};let p=bases[index as usize];limbs::<C>(if SIGNED&&i>>31!=0{-p}else{p})})\n".into())]);
    for(f,list)in &edits{let p=r.join("source").join(f);let original=fs::read_to_string(&p).unwrap();let mut next=original.clone();for(a,b)in list{assert_eq!(next.matches(a).count(),1,"{f}:{a}");next=next.replacen(a,b,1);}let mut back=next.clone();for(a,b)in list.iter().rev(){assert_eq!(back.matches(b).count(),1);back=back.replacen(b,a,1);}assert_eq!(back,original);fs::write(p,next).unwrap();}
    fs::copy(r.join("centered.rs"),r.join("source/crates/zopro-core/src/msm_affine/centered.rs")).unwrap();fs::write(r.join("inverse-edits.json"),serde_json::to_vec_pretty(&edits).unwrap()).unwrap();
    fs::write(r.join("native-flags.txt"),format!("{}\n",m["rustflags"].as_str().unwrap())).unwrap();fs::write(r.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"production_exact":172,"private_original_exact":168,"reversible_original_files":edits.keys().collect::<Vec<_>>(),"new_module":"msm_affine/centered.rs","new_scalar_or_point_buffer":false,"production_changed":false})).unwrap()).unwrap();
    println!("Prepared exclusive G1 candidate:172production exact,168private exact,4inverse edits recover");
}
