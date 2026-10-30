fn walk(base: &Path, paths: &mut Vec<PathBuf>) {
    for e in fs::read_dir(base).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, paths)
        } else if matches!(
            p.extension().and_then(|x| x.to_str()),
            Some(
                "rs" | "json"
                    | "log"
                    | "proof"
                    | "public"
                    | "md"
                    | "toml"
                    | "lock"
                    | "glsl"
                    | "comp"
                    | "wgsl"
                    | "vk"
            )
        ) {
            paths.push(p);
        }
    }
}
fn pack(paths: &[PathBuf], base: &Path, out: &Path) -> Value {
    let mut values = BTreeMap::new();
    for p in paths {
        let b = fs::read(p).unwrap();
        let h = format!("{:x}", Sha256::digest(&b));
        let len = b.len();
        let record = match String::from_utf8(b) {
            Ok(s) => json!({"sha256":h,"length":len,"utf8":s}),
            Err(e) => {
                json!({"sha256":h,"length":len,"hex":e.into_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>()})
            }
        };
        assert!(values
            .insert(p.strip_prefix(base).unwrap().display().to_string(), record)
            .is_none());
    }
    let bytes = serde_json::to_vec(&values).unwrap();
    let tmp = out.with_extension("json");
    fs::write(&tmp, &bytes).unwrap();
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(&tmp)
        .stdout(Stdio::from(fs::File::create(out).unwrap()))
        .status()
        .unwrap()
        .success());
    let back = Command::new("gzip")
        .args(["-dc"])
        .arg(out)
        .output()
        .unwrap();
    assert!(back.status.success());
    assert_eq!(back.stdout, bytes);
    fs::remove_file(tmp).unwrap();
    json!({"members":paths.len(),"sha256":hash(out),"roundtrip_exact":true})
}
fn publish() {
    let a = read(r("analysis.json"));
    assert_eq!(a["complete"], true);
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let d = Path::new(Z).join("docs/benchmarks/2026-10-30-query-timeline175");
    assert!(!d.exists());
    fs::create_dir_all(d.join("artifacts")).unwrap();
    for f in [
        "capture.rs",
        "analysis.rs",
        "publication.rs",
        "native.rs",
        "setup.rs",
        "query_timeline.rs",
        "prepare-sdk.rs",
        "design.md",
        "capture-protocol.md",
        "review.rs",
        "pre-capture-review.json",
        "source-audit.json",
        "sdk/source-audit.json",
        "pins.json",
        "plan.json",
        "jobs.json",
        "cases.json",
        "analysis.json",
        "proof-inventory.json",
        "proof-sha256-after.json",
    ] {
        let to = d.join("artifacts").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(r(f), to).unwrap();
    }
    let mut archives = BTreeMap::new();
    let manifest = read(Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    ));
    let mut source: Vec<_> = manifest["source_files_sha256"]
        .as_object()
        .unwrap()
        .keys()
        .map(|p| r("source").join(p))
        .collect();
    source.push(r("source/crates/zopro-core/src/query_timeline.rs"));
    assert_eq!(source.len(), 176);
    archives.insert(
        "source",
        pack(&source, &r("source"), &d.join("artifacts/source.json.gz")),
    );
    let mut receipts = vec![];
    for dir in ["native", "sdk/runs", "runs"] {
        walk(&r(dir), &mut receipts);
    }
    archives.insert(
        "receipts",
        pack(&receipts, &r(""), &d.join("artifacts/receipts.json.gz")),
    );
    let mut proofs = vec![];
    walk(&r("proofs"), &mut proofs);
    assert_eq!(proofs.len(), 576);
    archives.insert(
        "proofs",
        pack(&proofs, &r(""), &d.join("artifacts/proofs.json.gz")),
    );
    save(
        d.join("artifacts/publication-review.json"),
        &json!({"archives":archives,"original_pins_current":true,"new_tests_or_captures":0,"production_changed":false}),
    );
    let mut md=String::from("# In-process query critical-path timeline\n\n30 October 2026. Diagnostic stage profiling of the exact accepted algorithm, with OFF and traced sharing one executable. This is not an optimization or adoption. The previous balanced G2 full SDK trial remains unadopted.\n\nThree primes precede151fresh verified SDK requests per process. Thirty-one fixed rotated/reversed blocks compare frozen accepted, OFF and traced at1/16/32workers, with whole-process affinity CPU2 / CPUs0–15 / CPUs0–31. Shared Ryzen9950X host. Timings include solving, randomized proving, SDK verification and serialization; loading is excluded from latency and included in lifetime RSS. All helpers compile/pin before capture; no owned build/helper/hash/archive/analyzer or independent workload overlaps captures.\n\n## Overhead qualification\n\nBoth OFF/accepted and traced/OFF paired median intervals must fit factor1.01 before treating a topology as unperturbed. Ranks10–22/31 provide nominal97.055%individual median coverage under independent/exchangeable blocks, without familywise guarantees. All rows and failed checks are retained, without exclusions/extensions/rerolls.\n\n|Case / variant|Median ms|CPU ms/request|Peak RSS MiB|\n|---|---:|---:|---:|\n");
    for (k, c) in a["cells"].as_object().unwrap() {
        md.push_str(&format!(
            "|{k}|{:.6}|{:.3}|{:.2}|\n",
            c["median_ms"].as_f64().unwrap(),
            c["CPU_ms"].as_f64().unwrap(),
            c["max_RSS_MiB"].as_f64().unwrap()
        ));
    }
    md.push_str("\n|Pair|Ratio|Bounds|Qualified|\n|---|---:|---|---|\n");
    for (k, p) in a["pairs"].as_object().unwrap() {
        let s = &p["statistics"];
        md.push_str(&format!(
            "|{k}|{:.9}|[{:.9}, {:.9}]|{}|\n",
            s["median"].as_f64().unwrap(),
            s["low"].as_f64().unwrap(),
            s["high"].as_f64().unwrap(),
            p["within_factor1_01"]
        ));
    }
    md.push_str(&format!("\nFailed overhead checks: `{}`.\n\n## Recorded stage intervals\n\nThese are medians across4681retained traced requests per topology. They overlap and must not be added. Query intervals include internal work/waits; start is function entry, not scheduler readiness. Last-return counts identify observed function ordering, not a causal CPU/cache/scheduler explanation or a removable wall-time fraction. A topology that fails overhead qualification remains instrumented-only evidence.\n\n",a["overhead_failed_checks"]));
    for (k, s) in a["stages"].as_object().unwrap() {
        md.push_str(&format!("### {k}\n\nUnperturbed overhead qualification: **{}**. Last query counts: `{}`.\n\n|Interval|Median ms|\n|---|---:|\n",s["interpret_as_unperturbed"],s["last_query"]));
        for (name, v) in s["median_intervals_ms"].as_object().unwrap() {
            md.push_str(&format!("|{name}|{:.6}|\n", v.as_f64().unwrap()));
        }
    }
    md.push_str("\n## Verification, memory and limits\n\nAll576workflow steps complete:43,002internally verified requests and288fresh164-byte exports independently pass Gnark with common image/public/VK bindings. All exports are distinct against24,943prior hashes; the new union is25,231. Exports are the last timed proof, written after timing. Native126core/eightprover tests pass, including two journal/order tests; all17field sources remain exact and field qualification is inherited. Source176has174exact originals; reversing one parent's edits recovers all175.\n\nThe fixed journal requests26,624payload bytes per process, allocated before primes in OFF and traced. No query-boundary locks occur. One journal push follows joined output serialization. Overflow rejects analysis; no records are silently dropped. Records contain only durations. Lifetime RSS is not a minimum-RAM or incremental-workspace bound. No new PMU or sampled-profile execution accompanies this timestamp diagnostic; existing full-proof profiles motivated the study.\n\nProduction and all nine accepted local/AWS rows remain unchanged. The original eight CPU10× targets remain unmet. No new AWS/Aeglos, mobile/ARM/TEE, huge-circuit, memory-pressure/swap or concurrent-throughput result is claimed. Full supplied CPU/Metal comparisons and AWS configurations remain in the [current benchmark report](../2026-10-30-current-bench-report/README.md). This diagnostic selects further investigation, not a proof speedup.\n\n[Analysis](artifacts/analysis.json), [all per-request samples/timestamps](artifacts/rows.json), [protocol](artifacts/design.md), [source audit](artifacts/source-audit.json), [original pins](artifacts/pins.json), [workflow](artifacts/plan.json), [source archive](artifacts/source.json.gz), [proofs](artifacts/proofs.json.gz), [receipts](artifacts/receipts.json.gz), [new ledger](artifacts/proof-sha256-after.json).\n");
    fs::copy(r("rows.json"), d.join("artifacts/rows.json")).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    let mut files = vec![];
    walk(&d, &mut files);
    for name in ["source.json.gz", "proofs.json.gz", "receipts.json.gz"] {
        files.push(d.join("artifacts").join(name));
    }
    files.sort();
    let sums = files
        .iter()
        .map(|p| format!("{}  {}\n", hash(p), p.strip_prefix(&d).unwrap().display()))
        .collect::<String>();
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!("Published query-timeline diagnostic; ledger25231; production unchanged");
}
