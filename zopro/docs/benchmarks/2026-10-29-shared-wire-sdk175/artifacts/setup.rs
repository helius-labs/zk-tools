fn setup() {
    assert!(!r("build-plan.json").exists());
    let m = read(MANIFEST);
    let mut pins = BTreeMap::new();
    let mut edits = BTreeMap::new();
    for (file, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(file);
        assert_eq!(hash(&p), h.as_str().unwrap());
        let to = r("source").join(file);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(p, to).unwrap();
    }
    let mut edit = |file: &str, changes: Vec<(String, String)>| {
        let p = r("source").join(file);
        let old = fs::read_to_string(&p).unwrap();
        let mut new = old.clone();
        for (before, after) in &changes {
            assert_eq!(new.matches(before).count(), 1, "{file}: {before}");
            new = new.replacen(before, after, 1);
        }
        let mut inverse = new.clone();
        for (before, after) in changes.iter().rev() { inverse = inverse.replacen(after, before, 1); }
        assert_eq!(inverse, old);
        fs::write(p,new).unwrap();
        edits.insert(file.to_string(),json!(changes));
    };
    let file="crates/zopro-core/src/msm_affine/grouping.rs";
    edit(file,vec![("#[cfg(test)]\nmod tests;".into(),"#[cfg(zopro_shared_wire_groups)]\nmod wire_union;\n#[cfg(zopro_shared_wire_groups)]\npub(crate) use wire_union::scalar_groups_pair;\n\n#[cfg(test)]\nmod tests;".into())]);
    let child="crates/zopro-core/src/msm_affine/grouping/wire_union.rs";
    fs::create_dir_all(r("source").join(child).parent().unwrap()).unwrap();
    fs::copy(r("wire_union.rs"),r("source").join(child)).unwrap();
    let file="crates/zopro-core/src/msm_affine.rs";
    let s=fs::read_to_string(r("source").join(file)).unwrap();
    let start=s.find("pub(super) fn g2_query_exclusive(\n").unwrap();
    let end=start+s[start..].find("\n#[cfg(test)]\nmod exclusive_g2_tests").unwrap();
    let original=&s[start..end];
    let marker="    #[cfg(all(\n        zopro_g2_horner_buckets,";
    let tail=&original[original.find(marker).unwrap()..];
    let wrapper=r#"#[cfg(zopro_shared_wire_groups)]
pub(super) fn g2_query_exclusive(bases: &[zopro_bn254::G2Affine], words: &mut [[u64;4]]) -> zopro_bn254::G2Projective {
    let groups=crate::simd::cpu_option("ZOPRO_CPU_DEDUP").then(||scalar_groups(words)).flatten();
    g2_query_exclusive_prepared(bases, words, groups)
}
#[cfg(zopro_shared_wire_groups)]
pub(super) fn g2_query_exclusive_prepared(bases: &[zopro_bn254::G2Affine], words: &mut [[u64;4]], groups: Option<ScalarGroups>) -> zopro_bn254::G2Projective {
    assert_eq!(bases.len(), words.len());
"#;
    edit(file,vec![
        ("pub(super) use grouping::{group_witness, scalar_groups, ScalarGroups};".into(),"pub(super) use grouping::{group_witness, scalar_groups, ScalarGroups};\n#[cfg(zopro_shared_wire_groups)]\npub(super) use grouping::scalar_groups_pair;".into()),
        (original.into(),format!("#[cfg(not(zopro_shared_wire_groups))]\n{original}\n{wrapper}{tail}"))
    ]);
    let file="crates/zopro-core/src/lib.rs";
    let s=fs::read_to_string(r("source").join(file)).unwrap();
    let start=s.find("    fn solve_canonical_wires(").unwrap();
    let end=start+s[start..].find("\n}\n\nimpl Key").unwrap();
    let original=&s[start..end];
    let mut new=original.to_string();
    let before="        let mut product=queries.product_values(&self.plan,a,b);";
    assert_eq!(new.matches(before).count(),1);
    new=new.replace(before,r#"        #[cfg(not(zopro_shared_wire_groups))]
        let mut product=queries.product_values(&self.plan,a,b);
        #[cfg(zopro_shared_wire_groups)]
        let (mut product,shared)=rayon::join(
            ||queries.product_values(&self.plan,a,b),
            ||msm_affine::scalar_groups_pair(words,[&plan.a_indices,&plan.b_indices],[sa,&sb[..]]));
        #[cfg(zopro_shared_wire_groups)]
        let (a_groups,mut b_groups)=match shared {Some([a,b])=>(Some(a),Some(b)),None=>(None,None)};
        #[cfg(not(zopro_shared_wire_groups))]
        let a_query=||msm(&self.a[..],sa);
        #[cfg(zopro_shared_wire_groups)]
        let a_query=||match a_groups.as_ref() {
            Some(g)=>g.as_ref().map_or_else(||msm_raw(&self.a[..],sa),|g|msm_grouped(&self.a[..],g)),
            None=>msm(&self.a[..],sa),
        };
"#);
    // Replace both caller A branches, leaving the closure definitions above.
    new=new.replace("rayon::join(|| msm(&self.a[..],sa),","rayon::join(a_query,")
        .replace("rayon::join(|| msm(&self.a[..], sa),","rayon::join(a_query,");
    let before="msm_affine::g2_query_exclusive(&self.b2,sb)";
    assert_eq!(new.matches(before).count(),2);
    new=new.replace(before,r#"{
                            #[cfg(not(zopro_shared_wire_groups))]
                            {msm_affine::g2_query_exclusive(&self.b2,sb)}
                            #[cfg(zopro_shared_wire_groups)]
                            {let g=b_groups.take().unwrap_or_else(||crate::simd::cpu_option("ZOPRO_CPU_DEDUP").then(||scalar_groups(sb)).flatten());
                             msm_affine::g2_query_exclusive_prepared(&self.b2,sb,g)}
                        }"#);
    for before in [
        "                        let groups=crate::simd::cpu_option(\"ZOPRO_CPU_DEDUP\")\n                            .then(||scalar_groups(sb)).flatten();",
        "                        let groups = crate::simd::cpu_option(\"ZOPRO_CPU_DEDUP\")\n                            .then(|| scalar_groups(sb))\n                            .flatten();"
    ] {
        assert_eq!(new.matches(before).count(),1);
        new=new.replace(before,&format!("                        #[cfg(not(zopro_shared_wire_groups))]\n{before}\n                        #[cfg(zopro_shared_wire_groups)]\n                        let groups=b_groups.take().unwrap_or_else(||crate::simd::cpu_option(\"ZOPRO_CPU_DEDUP\").then(||scalar_groups(sb)).flatten());"));
    }
    edit(file,vec![(original.into(),new)]);
    let cargo="crates/zopro-core/Cargo.toml";
    let needle="[target.'cfg(not(target_arch = \"wasm32\"))'.dependencies]\n";
    edit(cargo,vec![(needle.into(),format!("{needle}hashbrown = {{ version = \"=0.16.1\", default-features = false, features = [\"inline-more\"] }}\n"))]);
    let lock="Cargo.lock";
    let needle="name = \"zopro-core\"\nversion = \"0.1.0\"\ndependencies = [\n \"ahash\",\n \"futures-channel\",\n \"getrandom 0.2.17\",\n";
    edit(lock,vec![(needle.into(),format!("{needle} \"hashbrown 0.16.1\",\n"))]);
    let mut exact=0;let mut field=0;
    for (f,h) in m["source_files_sha256"].as_object().unwrap() {
        let p=r("source").join(f);
        if hash(&p)==h.as_str().unwrap() {exact+=1;} else {assert!(edits.contains_key(f));}
        if f.starts_with("crates/zopro-bn254/") {assert_eq!(hash(&p),h.as_str().unwrap());field+=1;}
        pin(&mut pins,p);
    }
    assert_eq!(exact,170);assert_eq!(field,17);pin(&mut pins,r("source").join(child));
    save(r("source-audit.json"),&json!({"originals":175,"originals_exact":exact,"private_files":176,"modified_originals":edits.keys().collect::<Vec<_>>(),"edits":edits,"new_child":child,"field_originals_exact":field,"production_changed":false,"scope":"canonical-wire entry: first two actual fixtures; larger Montgomery paths unchanged"}));
    let mut steps=vec![];
    for (label,on) in [("control",false),("candidate",true)] {
        let flags=if on {format!("{} --cfg zopro_shared_wire_groups",m["rustflags"].as_str().unwrap())}else{m["rustflags"].as_str().unwrap().to_string()};
        fs::write(r(format!("native-flags-{label}.txt")),&flags).unwrap();
        let mut env=m["environment"].clone();
        env["RUSTFLAGS"]=json!(flags);env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));
        env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");env["ZOPRO_CPU_DEDUP"]=json!("1");
        for (name,args) in [
            ("core",vec!["cargo","test","--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"]),
            ("prover",vec!["cargo","test","--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"]),
            ("libraries",vec!["cargo","build","--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"]),
        ] {
            let mut args:Vec<String>=args.into_iter().map(String::from).collect(); args.splice(2..2,["--manifest-path".into(),r("source/Cargo.toml").display().to_string()]);
            steps.push(json!({"kind":"command","name":format!("{name}-{label}"),"argv":args,"environment":env,"unfiltered":name!="libraries","timeout_seconds":1800}));
        }
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    for file in ["setup.rs","native.rs","native-helper","wire_union.rs","design.md","source-audit.json","native-flags-control.txt","native-flags-candidate.txt"] {pin(&mut pins,r(file));}
    pin(&mut pins,MANIFEST);
    save(r("build-pins.json"),&json!(pins));
    save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
    println!("Prepared10steps/176privatefiles170originalsexact; no production edits");
}
