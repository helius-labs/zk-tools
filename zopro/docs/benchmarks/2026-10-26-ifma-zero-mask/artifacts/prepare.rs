use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-ifma-zero-mask");
    let p = root.join("candidate/crates/zopro-bn254/src/ifma.rs");
    let original = fs::read_to_string(&p).unwrap();
    let a="                    let m = _mm256_and_si256(\n                        _mm256_madd52lo_epu64(zero, t[$i], inv),\n                        Self::splat((1u64 << $bits) - 1),\n                    );";
    let b="                    let m = _mm256_madd52lo_epu64(zero, t[$i], inv);\n                    let m = if cfg!(zopro_unmask_m52) && $bits == 52 {\n                        m\n                    } else {\n                        _mm256_and_si256(m, Self::splat((1u64 << $bits) - 1))\n                    };";
    let c="                    let m = _mm256_and_si256(_mm256_madd52lo_epu64(zero, t[0], inv), mask);";
    let d="                    let m = _mm256_madd52lo_epu64(zero, t[0], inv);\n                    let m = if cfg!(zopro_unmask_m52) { m } else { _mm256_and_si256(m, mask) };";
    assert_eq!(original.matches(a).count(), 1);
    assert_eq!(original.matches(c).count(), 1);
    let current = original.replace(a, b).replace(c, d);
    assert_eq!(current.replace(d, c).replace(b, a), original);
    fs::write(&p, current).unwrap();
    fs::write(root.join("edits.json"),serde_json::to_vec_pretty(&json!({"ifma.rs":[{"from":a,"to":b},{"from":c,"to":d}],"last48bitblock_unchanged":true})).unwrap()).unwrap();
    let mut driver =
        String::from("use std::hint::black_box;\nuse zopro_bn254::{Fq,Fr,FqConfig,FrConfig};\n");
    let mut calls = String::new();
    let mut names = Vec::new();
    for (field, cfg, host) in [("fq", "FqConfig", "Fq"), ("fr", "FrConfig", "Fr")] {
        for (mixed, radix) in [(true, 256), (false, 260)] {
            for lanes in [4, 8] {
                let ty = format!(
                    "zopro_bn254::ifma{}::Ifma{}<{cfg},{mixed}>",
                    if lanes == 8 { "8" } else { "" },
                    if lanes == 8 { "8" } else { "" }
                );
                for op in ["mul", "sum2", "difference2"] {
                    let name = format!("audit_{field}_r{radix}_{op}{lanes}");
                    let body = if op == "mul" {
                        "a.mul(b)".to_owned()
                    } else {
                        format!("a.{op}(b,c,d)")
                    };
                    driver.push_str(&format!("#[no_mangle]\n#[inline(never)]\npub fn {name}(a:&{ty},b:&{ty},c:&{ty},d:&{ty})->{ty}{{{body}}}\n"));
                    calls.push_str(&format!("{{let v=<{ty}>::new(&[{host}::from_u64(black_box(7));{lanes}]);black_box({name}(&v,&v,&v,&v));}}\n"));
                    names.push(json!({"name":name,"field":field,"radix":radix,"lanes":lanes,"operation":op}));
                }
            }
        }
    }
    driver.push_str("fn main(){\n");
    driver.push_str(&calls);
    driver.push_str("}\n");
    fs::write(root.join("field-probe.rs"), driver).unwrap();
    fs::write(
        root.join("wrappers.json"),
        serde_json::to_vec_pretty(&json!(names)).unwrap(),
    )
    .unwrap();
    let base: Value = serde_json::from_slice(
        &fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("accepted-manifest.json"),
        serde_json::to_vec_pretty(&base).unwrap(),
    )
    .unwrap();
    let mut changed = Vec::new();
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        let got = hash(root.join("candidate").join(n));
        if got != h.as_str().unwrap() {
            changed.push(json!({"path":n,"sha256":got,"original_sha256":h}));
        }
    }
    assert_eq!(changed.len(), 1);
    println!("Prepared guarded two-site IFMA52 mask edit; generator will produce eightwide version;24 actual wrappers");
}
