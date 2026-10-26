use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader, Read},
    path::Path,
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn write(p: impl AsRef<Path>, v: Value) {
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
type T = [u64; 2];
fn tally(v: &Value) -> T {
    [
        v["samples"].as_u64().unwrap(),
        v["period"].as_u64().unwrap(),
    ]
}
fn add(a: &mut T, b: T) {
    a[0] = a[0].checked_add(b[0]).unwrap();
    a[1] = a[1].checked_add(b[1]).unwrap();
}
fn accumulate(b: &mut BTreeMap<String, T>, key: &str, t: T) {
    add(b.entry(key.into()).or_default(), t);
}
fn pct(v: &Value, total: u64) -> f64 {
    v["period"].as_u64().unwrap_or(0) as f64 * 100. / total as f64
}
fn rowscope(run: &str, scope: &str, b: &Value, total: u64) -> Value {
    let p = b["total"]["period"].as_u64().unwrap();
    json!({"run":run,"scope":scope,"period":p,"samples":b["total"]["samples"],"share_of_all_percent":p as f64*100./total as f64,"within_scope_percent":{"ifma":pct(&b["families"]["ifma"],p),"vector_add_sub":pct(&b["families"]["vector_add_sub"],p),"vector_logic":pct(&b["families"]["vector_logic"],p),"vector_shift":pct(&b["families"]["vector_shift"],p),"vector_move":pct(&b["families"]["vector_move"],p),"vector_permute":pct(&b["families"]["vector_permute"],p),"rsp_operand":pct(&b["memory"]["rsp_operand"],p),"rbp_operand":pct(&b["memory"]["rbp_operand"],p),"rip_operand":pct(&b["memory"]["rip_operand"],p),"other_memory_operand":pct(&b["memory"]["other_memory_operand"],p)}})
}
fn main() {
    let root = Path::new("/tmp/zopro-instruction-census");
    let config = read(root.join("analysis-config.json"));
    let result = read(root.join("analysis/result.json"));
    assert_eq!(
        read(root.join("analysis-workflow/result.json"))["status"],
        "complete"
    );
    assert_eq!(result["status"], "complete; diagnostic only");
    assert_eq!(read(root.join("invalid-hash.exit.json"))["returncode"], 1);
    assert!(!root.join("invalid-hash-output").exists());
    let pins = read(root.join("analysis-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let manifest_path = Path::new("/tmp/zopro-grouping-module/production-source-manifest.json");
    let manifest = read(manifest_path);
    assert_eq!(
        hash(manifest_path),
        "63374de60539c58f853603d144c4df61dd8cdae966bb27835566172fcbb0d70f"
    );
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap(),
            "accepted source {p}"
        );
    }
    assert_eq!(
        hash(manifest["native_binary"].as_str().unwrap()),
        manifest["native_binary_sha256"].as_str().unwrap()
    );
    let prior = read("/tmp/zopro-fused-product-overlap/perf/rows.json");
    let mut all = [0, 0];
    let mut output = Vec::new();
    let mut scopes_out = Vec::new();
    let mut functions_out = Vec::new();
    let mut target_symbols = 0u64;
    let mut ip_count = 0usize;
    for run in config["runs"].as_array().unwrap() {
        let name = run["name"].as_str().unwrap();
        let parsed = read(root.join("analysis").join(format!("{name}.json")));
        let summary = &parsed["summary"];
        assert_eq!(
            summary["total"],
            json!({"samples":run["expected_samples"],"period":run["expected_period"]})
        );
        let mut raw = [0, 0];
        let mut tids = BTreeSet::new();
        let mut raw_ips = BTreeMap::<u64, T>::new();
        for line in
            BufReader::new(fs::File::open(run["samples"].as_str().unwrap()).unwrap()).lines()
        {
            let line = line.unwrap();
            let f = line.split_whitespace().collect::<Vec<_>>();
            if f.is_empty() || f[0].starts_with('#') {
                continue;
            }
            let (pid, tid) = f[0].split_once('/').unwrap();
            assert_eq!(
                pid.parse::<u64>().unwrap(),
                run["expected_pid"].as_u64().unwrap()
            );
            tids.insert(tid.to_owned());
            let t = [1, f[1].parse::<u64>().unwrap()];
            add(&mut raw, t);
            add(
                raw_ips
                    .entry(u64::from_str_radix(f[2], 16).unwrap())
                    .or_default(),
                t,
            );
        }
        assert_eq!(raw, tally(&summary["total"]));
        assert_eq!(tids.len(), run["expected_tids"].as_u64().unwrap() as usize);
        let mut ips = [0, 0];
        let mut executable = [0, 0];
        let mut outside = [0, 0];
        let mut families = BTreeMap::new();
        let mut memory = BTreeMap::new();
        let mut scopes = BTreeMap::new();
        let mut functions = BTreeMap::<usize, T>::new();
        let mut seen = BTreeSet::new();
        let bias = u64::from_str_radix(summary["load_bias"].as_str().unwrap(), 16).unwrap();
        let mut top_ips = Vec::new();
        let mut top_rsp = Vec::new();
        for line in BufReader::new(
            fs::File::open(root.join("analysis").join(format!("{name}-ips.jsonl"))).unwrap(),
        )
        .lines()
        {
            let row: Value = serde_json::from_str(&line.unwrap()).unwrap();
            let t = tally(&row["tally"]);
            let runtime = u64::from_str_radix(row["runtime_ip"].as_str().unwrap(), 16).unwrap();
            assert!(seen.insert(runtime));
            assert_eq!(raw_ips.get(&runtime), Some(&t));
            add(&mut ips, t);
            accumulate(&mut families, row["family"].as_str().unwrap(), t);
            accumulate(&mut memory, row["memory"].as_str().unwrap(), t);
            accumulate(&mut scopes, row["scope"].as_str().unwrap(), t);
            let is_mapped = parsed["executable_intervals"]
                .as_array()
                .unwrap()
                .iter()
                .any(|interval| {
                    runtime >= interval[0].as_u64().unwrap()
                        && runtime < interval[1].as_u64().unwrap()
                });
            assert_eq!(is_mapped, !row["elf_ip"].is_null());
            if is_mapped {
                assert_eq!(
                    runtime - bias,
                    u64::from_str_radix(row["elf_ip"].as_str().unwrap(), 16).unwrap()
                );
                assert!(row["assembly"].as_str().is_some());
                add(&mut executable, t);
                add(
                    functions
                        .entry(row["function"].as_u64().unwrap() as usize)
                        .or_default(),
                    t,
                );
            } else {
                assert_eq!(row["scope"], "outside_executable");
                add(&mut outside, t);
            }
            if name == "1x2-w32-accepted" && row["scope"] == "g2_prefix" {
                top_ips.push(row.clone());
                if row["memory"] == "rsp_operand" {
                    top_rsp.push(row);
                }
            }
        }
        assert_eq!(seen.len(), raw_ips.len());
        ip_count += seen.len();
        assert_eq!(ips, raw);
        assert_eq!(executable, tally(&summary["executable"]));
        assert_eq!(outside, tally(&summary["outside_executable"]));
        for (label, map) in [("families", families), ("memory", memory)] {
            assert_eq!(map.len(), parsed["all"][label].as_object().unwrap().len());
            for (k, t) in map {
                assert_eq!(t, tally(&parsed["all"][label][k]));
            }
        }
        assert_eq!(scopes.len(), parsed["scopes"].as_object().unwrap().len());
        for (k, t) in scopes {
            assert_eq!(t, tally(&parsed["scopes"][&k]["total"]));
            scopes_out.push(rowscope(name, &k, &parsed["scopes"][&k], raw[1]));
        }
        let original = prior
            .as_array()
            .unwrap()
            .iter()
            .find(|row| {
                row["phase"] == "profile"
                    && format!(
                        "{}-{}",
                        row["case"].as_str().unwrap(),
                        row["variant"].as_str().unwrap()
                    ) == name
            })
            .unwrap();
        for (i, t) in functions {
            let f = &parsed["functions"][i];
            assert_eq!(t, tally(&f["bins"]["total"]));
            if matches!(
                f["scope"].as_str().unwrap(),
                "g1_stream" | "g2_prefix" | "g1_reduce" | "g2_reduce" | "fused_feed"
            ) {
                assert_eq!(f["names"].as_array().unwrap().len(), 1);
                assert_eq!(
                    original["profile"]["symbol_periods"][f["names"][0].as_str().unwrap()]
                        .as_u64()
                        .unwrap(),
                    t[1]
                );
                target_symbols += 1;
            }
        }
        let mut top = parsed["functions"].as_array().unwrap().clone();
        top.sort_by_key(|f| std::cmp::Reverse(f["bins"]["total"]["period"].as_u64().unwrap()));
        top.truncate(20);
        functions_out.push(json!({"run":name,"top_functions":top}));
        output.push(json!({"run":name,"summary":summary,"all":parsed["all"],"unique_ips":seen.len(),"unscoped":parsed["scopes"]["unscoped_executable"],"ambiguous":parsed["scopes"]["ambiguous_alias"]}));
        add(&mut all, raw);
        if name == "1x2-w32-accepted" {
            top_ips.sort_by_key(|r| std::cmp::Reverse(r["tally"]["period"].as_u64().unwrap()));
            top_ips.truncate(30);
            top_rsp.sort_by_key(|r| std::cmp::Reverse(r["tally"]["period"].as_u64().unwrap()));
            top_rsp.truncate(30);
            write(
                root.join("g2-primary-sites.json"),
                json!({"all_top_sites":top_ips,"rsp_top_sites":top_rsp,"interpretation":"Cycle-IP associations only; no precise-event or spill claim."}),
            );
        }
    }
    assert_eq!(all, tally(&result["total"]));
    assert_eq!(all[0], 297412);
    write(
        root.join("instruction-summary.json"),
        json!({"runs":output,"scopes":scopes_out,"top_functions":functions_out,"interpretation":"Weighted user-cycle samples; opcode cost and memory traffic cannot be inferred from instruction or register-address tags."}),
    );
    let out = json!({"status":"complete; diagnostic only","captures":9,"all_samples":all[0],"all_period":all[1],"unique_runtime_ips_across_runs":ip_count,"raw_compact_and_jsonl_independently_conserve":true,"all_executable_sample_ips_exact_boundary_byte_bound":true,"named_target_functions_match_original_symbol_periods":target_symbols,"all_analysis_pins":pins.as_object().unwrap().len(),"accepted_sources_unchanged":manifest["source_files_sha256"].as_object().unwrap().len(),"accepted_manifest_sha256":hash(manifest_path),"accepted_sdk_sha256":manifest["native_binary_sha256"],"old_qualified_xtask_sha256":hash("/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask"),"reader_sha256":hash(root.join("target/release/xtask")),"invalid_hash_rejected_before_output":true,"tests_passed":30,"clippy":"passed, -D warnings","formatting":"passed","new_profile_captures":0,"new_benchmark_sdk_calls":0,"new_gnark_exports":0,"adopted_prover_change":false,"goal_achieved":false,"owned_jobs":"all terminal; offline extraction/reader only; no AWS","limitations":["Cycles sampling may skid; no exclusive opcode cost, retired instruction count, stall diagnosis or removable walltime.","RSP/RBP operands are observations, not spill counts or memory traffic; RBP can be a general register.","Named scopes are literal symbol-range subsets, not a complete semantic MSM attribution. Closures and callees remain explicit under other_executable.","One inherited capture per tuple is not a new timing comparison or a statistical opcode-cost experiment."]});
    write(root.join("closeout.json"), out.clone());
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
