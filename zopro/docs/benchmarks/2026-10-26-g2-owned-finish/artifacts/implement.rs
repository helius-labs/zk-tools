use std::{fs,path::Path};use serde_json::{Value,json};use sha2::{Digest,Sha256};
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let r=Path::new("/tmp/zopro-g2-owned-finish");let dst=r.join("candidate");let path="crates/zopro-core/src/msm.rs";let base=fs::read_to_string(dst.join(path)).unwrap();let mut s=base.clone();let guard="#[cfg(all(zopro_g2_owned_finish,zopro_g2_prefix_records,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]";
let module=format!("\n// BEGIN G2_OWNED_MODULE\n{guard}\nmod g2_owned_finish;\n// END G2_OWNED_MODULE\n");s.push_str(&module);
let method=r#"    // BEGIN G2_OWNED_APPLY
    /// Apply a batch whose point coordinates the caller discards afterward.
    #[cfg(zopro_g2_owned_finish)]
    #[inline]
    pub(super) fn apply_owned(&mut self,buckets:&mut[Affine<C>],batch:&mut[(u32,Affine<C>)]) {
        if self.direct {C::Base::stream_add_owned(buckets,batch,self);}
        else {self.apply_generic(buckets,batch);}
    }
    // END G2_OWNED_APPLY
"#;let anchor="    pub(super) fn apply_signed(&mut self, buckets:";assert_eq!(s.matches(anchor).count(),1);s=s.replacen(anchor,&(method.to_owned()+anchor),1);
let method=r#"    // BEGIN G2_OWNED_DEFAULT
    #[cfg(zopro_g2_owned_finish)]
    #[inline(always)]
    fn stream_add_owned<C:CurveConfig<Base=Self>>(buckets:&mut[Affine<C>],batch:&mut[(u32,Affine<C>)],s:&mut BatchAdd<C>) {
        Self::stream_add(buckets,batch,s);
    }
    // END G2_OWNED_DEFAULT
"#;let anchor="pub(super) trait BatchInverse: Field {\n";assert_eq!(s.matches(anchor).count(),1);s=s.replacen(anchor,&(anchor.to_owned()+method),1);
let method=format!(r#"    // BEGIN G2_OWNED_DISPATCH
    #[cfg(zopro_g2_owned_finish)]
    fn stream_add_owned<C:CurveConfig<Base=Self>>(buckets:&mut[Affine<C>],batch:&mut[(u32,Affine<C>)],s:&mut BatchAdd<C>) {{
        {guard}
        if crate::simd::cpu_option("ZOPRO_CPU_DIRECT_G2") && s.fq2.prefix_enabled && !batch.is_empty() && batch.len()%LANES==0 {{
            if s.fq2.canonical {{g2_owned_finish::apply::<C,true>(buckets,batch,&mut s.fq2);}}
            else {{g2_owned_finish::apply::<C,false>(buckets,batch,&mut s.fq2);}}
            return;
        }}
        Self::stream_add(buckets,batch,s);
    }}
    // END G2_OWNED_DISPATCH
"#);let anchor="impl BatchInverse for Fq2 {\n";s=s.replacen(anchor,&(anchor.to_owned()+&method),1);
let mut inverse=s.clone();for tag in["G2_OWNED_MODULE","G2_OWNED_APPLY","G2_OWNED_DEFAULT","G2_OWNED_DISPATCH"]{let indent=if tag=="G2_OWNED_MODULE"{""}else{"    "};let begin=format!("{indent}// BEGIN {tag}\n");let marker=format!("{indent}// END {tag}\n");let mut a=inverse.find(&begin).unwrap();if tag=="G2_OWNED_MODULE"{assert_eq!(inverse.as_bytes()[a-1],b'\n');a-=1;}let b=inverse.find(&marker).unwrap()+marker.len();inverse.replace_range(a..b,"");}assert_eq!(inverse,base);fs::write(dst.join(path),s).unwrap();
let path="crates/zopro-core/src/msm_affine.rs";let base=fs::read_to_string(dst.join(path)).unwrap();let anchor="            self.packed.apply(&mut self.affine, &self.batch);";assert_eq!(base.matches(anchor).count(),1);let replacement=format!("            // BEGIN G2_OWNED_CALL\n            #[cfg(zopro_g2_owned_finish)]\n            self.packed.apply_owned(&mut self.affine,&mut self.batch);\n            #[cfg(not(zopro_g2_owned_finish))]\n{anchor}\n            // END G2_OWNED_CALL");let s=base.replacen(anchor,&replacement,1);assert_eq!(s.replacen(&replacement,anchor,1),base);fs::write(dst.join(path),s).unwrap();
for(src,target)in[("g2_owned_finish.rs.txt","crates/zopro-core/src/msm/g2_owned_finish.rs"),("tests.rs.txt","crates/zopro-core/src/msm/g2_owned_finish/tests.rs")]{let p=dst.join(target);fs::create_dir_all(p.parent().unwrap()).unwrap();fs::copy(r.join(src),p).unwrap();}
let m:Value=serde_json::from_slice(&fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap()).unwrap();let mut changed=vec![];for(n,h)in m["source_files_sha256"].as_object().unwrap(){let got=hash(dst.join(n));if got!=h.as_str().unwrap(){assert!(n=="crates/zopro-core/src/msm.rs"||n=="crates/zopro-core/src/msm_affine.rs");changed.push(json!({"path":n,"original_sha256":h,"sha256":got}));}}assert_eq!(changed.len(),2);
for n in["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{let p=dst.join(n);fs::create_dir_all(p.parent().unwrap()).unwrap();fs::copy(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n),p).unwrap();}
fs::write(r.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"changed_existing":changed,"other_168_accepted_files_unchanged":true,"two_new_modules":["crates/zopro-core/src/msm/g2_owned_finish.rs","crates/zopro-core/src/msm/g2_owned_finish/tests.rs"],"whole_original_files_recovered_by_marker_removal_and_single_call_restore":true,"new_key_point_buffer_table_or_heap_storage":0,"records_and_batch_capacity_types_unchanged":true,"only_changed_batch_consumer":"Window::execute reads indices to clear busy bits then clears batch; no point-coordinate postreads","immutable_consumers_unchanged":true,"all_field_files_and_generator_unchanged":true,"scheduler_solver_key_plan_workspace_forecast_unchanged":true})).unwrap()).unwrap();println!("Two existing core files +2newfocused modules;168acceptedfilesunchanged; originalsrecover;174sources includingdriver/support");}
