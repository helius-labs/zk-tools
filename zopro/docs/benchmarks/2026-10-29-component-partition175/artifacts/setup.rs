fn setup(){
    assert!(!r("build-plan.json").exists());
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");let m=read(&manifest);let mut pins=BTreeMap::new();pin(&mut pins,&manifest);
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),175);
    for(p,h)in m["source_files_sha256"].as_object().unwrap(){let from=Path::new(Z).join(p);assert_eq!(hash(&from),h.as_str().unwrap());pin(&mut pins,&from);let to=r("source").join(p);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();}
    let phi="crates/zopro-core/src/msm_affine/g2_phi.rs";
    let before="    hb_raw(bases, words, 12, 2)";
    let after=format!("    #[cfg(zopro_phi_component_partition)]\n    if let Some(value)=partition::try_raw(bases,words) {{return value;}}\n{before}");
    let module="\n#[cfg(any(test,zopro_phi_component_partition))]\nmod partition;\n";
    let mut text=fs::read_to_string(r("source").join(phi)).unwrap();once(&mut text,before,&after);text+=module;fs::write(r("source").join(phi),text).unwrap();
    let borrowed="crates/zopro-core/src/msm_affine/g2_borrowed.rs";
    let suffix=r#"
#[cfg(any(test,zopro_phi_component_partition))]
pub(super) fn try_component(bases:&[G2Affine],short:&[[u64;4]],high:bool,w:usize,bits:usize)->Option<Vec<G2Affine>>{
    let packed=BatchAdd::default();if !packed.borrowed_g2_enabled(){return None;}
    let size=batch_size(bits).max(32);let mut state=Window::new(1usize<<(bits-1),size,packed);
    state.feed(bases,short,high,w,bits,size);Some(state.finish())
}
"#;
    let original=fs::read_to_string(r("source").join(borrowed)).unwrap();fs::write(r("source").join(borrowed),format!("{original}{suffix}")).unwrap();
    let workspace="crates/zopro-core/src/msm_affine/workspace.rs";
    let old="        q.leaf=q.leaf.max(top.leaf);p=p.maximum(q);";
    let revised=r#"        q.leaf=q.leaf.max(top.leaf);
        #[cfg(zopro_phi_component_partition)]
        {
            // Each independent component routes the full range in the narrow top.
            // Retain the original envelope as a floor and charge both full parents.
            let component_top=self::top::<C>(n,top_bits+1)?;
            q.held=add(q.held,mul(component_top.held,2)?)?;
            q.leaf=q.leaf.max(component_top.leaf);
        }
        p=p.maximum(q);"#;
    let test=r#"
#[cfg(test)]mod component_partition_envelope_tests{
    use super::*;
    #[test]fn full_range_top_is_conservatively_charged(){
        for n in (1024usize..65536).step_by(257).chain([1024,1025,30089,65534,65535]){
            let whole=top::<G2Config>(n,7).unwrap();let half=top::<G2Config>(n.div_ceil(2),7).unwrap();assert!(whole.held>=half.held&&whole.leaf>=half.leaf);
            let forecast=hb::<G2Config>(n).unwrap();
            if cfg!(zopro_phi_component_partition){let ordinary=shape::<G2Config>(2048,11,2,200).unwrap();assert!(forecast.held>=ordinary.held+2*whole.held);assert!(forecast.leaf>=whole.leaf);}
        }
    }
}
"#;
    let mut text=fs::read_to_string(r("source").join(workspace)).unwrap();once(&mut text,old,revised);text+=test;fs::write(r("source").join(workspace),text).unwrap();
    let child="crates/zopro-core/src/msm_affine/g2_phi/partition.rs";let to=r("source").join(child);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(r("partition.rs"),&to).unwrap();pin(&mut pins,&to);
    let(mut exact,mut fields)=(0,0);
    for(p,h)in m["source_files_sha256"].as_object().unwrap(){let path=r("source").join(p);let bytes=fs::read(&path).unwrap();if hash(&path)==h.as_str().unwrap(){exact+=1;}
        let restored=if p==phi{String::from_utf8(bytes).unwrap().strip_suffix(module).unwrap().replacen(&after,before,1).into_bytes()}else if p==borrowed{String::from_utf8(bytes).unwrap().strip_suffix(suffix).unwrap().as_bytes().to_vec()}else if p==workspace{String::from_utf8(bytes).unwrap().strip_suffix(test).unwrap().replacen(revised,old,1).into_bytes()}else{bytes};
        assert_eq!(format!("{:x}",Sha256::digest(restored)),h.as_str().unwrap());if p.starts_with("crates/zopro-bn254/"){assert_eq!(hash(&path),h.as_str().unwrap());fields+=1;}pin(&mut pins,path);
    }
    assert_eq!(exact,172);assert_eq!(fields,17);
    save(r("source-audit.json"),&json!({"source_files":176,"original_files":175,"originals_exact":172,"reverse175_exact":true,"all17field_exact":true,"accepted_KH_and_A_exact":true,"changed":[phi,borrowed,workspace],"new_helper":child,"cfg":"zopro_phi_component_partition","no_key_tables_or_point_copies":true,"ordinary_bucket_sets_per_window":2,"forecast_retains_old_floor_and_adds_full_top_envelopes":true}));
    let flags=m["rustflags"].as_str().unwrap();let selected=format!("{flags} --cfg zopro_phi_component_partition");fs::write(r("native-flags.txt"),flags).unwrap();fs::write(r("candidate-flags.txt"),&selected).unwrap();
    let required=["msm_affine::g2_phi::partition::tests::complete_queries_match_independent_generator","msm_affine::g2_phi::partition::tests::component_buckets_match_independent_signed_sums","msm_affine::g2_phi::partition::tests::compiled_dispatch_matches_policy","msm_affine::workspace::component_partition_envelope_tests::full_range_top_is_conservatively_charged"];
    let mut steps=Vec::new();for label in["control","candidate"]{
        let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(if label=="candidate"{selected.as_str()}else{flags});env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");let cargo=r("source/Cargo.toml");
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":required,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":300}));
    }
    for name in["design.md","history-audit.md","partition.rs","setup.rs","native.rs","native-helper","source-audit.json","native-flags.txt","candidate-flags.txt"]{pin(&mut pins,r(name));}
    for name in["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{pin(&mut pins,Path::new(Z).join(name));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared10 native steps;176private/172exact;reverse175;4newcoretests;field inherited");
}
