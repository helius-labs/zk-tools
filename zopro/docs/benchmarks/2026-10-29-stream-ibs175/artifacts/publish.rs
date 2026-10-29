use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/stream-ibs175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-stream-ibs175";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
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
fn med(mut a: Vec<f64>) -> f64 {
    a.sort_by(f64::total_cmp);
    assert_eq!(a.len(), 3);
    a[1]
}
fn main() {
    let analysis = read(r("analysis.json"));
    assert_eq!(analysis["independent_proof_count"], 17811);
    let rows = analysis["rows"].as_array().unwrap();
    let mut groups = BTreeMap::new();
    for cat in ["G1feed", "G2feed"] {
        let op: Vec<_> = rows
            .iter()
            .filter(|v| v["mode"] == "op")
            .map(|v| &v["IBS"]["groups"][format!("user/{cat}")])
            .collect();
        let fetch: Vec<_> = rows
            .iter()
            .filter(|v| v["mode"] == "fetch")
            .map(|v| &v["IBS"]["groups"][format!("user/{cat}")])
            .collect();
        for g in &op {
            assert_eq!(g["imprecise_excluded"], 0);
            assert_eq!(g["invalid_or_saturated_latency_pairs"], 0);
        }
        let value = |g: &[&Value], f: &str| med(g.iter().map(|v| v[f].as_f64().unwrap()).collect());
        let rate = |g: &[&Value], n: &str, d: &str| {
            med(g
                .iter()
                .map(|v| v[n].as_f64().unwrap() / v[d].as_f64().unwrap())
                .collect())
        };
        let nested = |g: &[&Value], n: &str, f: &str| {
            med(g.iter().map(|v| v[n][f].as_f64().unwrap()).collect())
        };
        let mut instructions = BTreeMap::new();
        for mnemonic in [
            "vpmadd52luq",
            "vpmadd52huq",
            "vpaddq",
            "vpsrlq",
            "vmovdqa64",
            "vpbroadcastq",
            "vpxor",
            "vpxord",
        ] {
            let rs: Vec<_> = rows
                .iter()
                .filter(|v| v["mode"] == "op")
                .map(|v| &v["IBS"]["groups"][format!("user/{cat}/mnemonic/{mnemonic}")])
                .collect();
            if rs.iter().all(|v| !v.is_null()) {
                instructions.insert(mnemonic,json!({"median_samples":value(&rs,"samples"),"median_tag_to_complete_mean":nested(&rs,"tag_to_complete","mean"),"median_complete_to_retire_mean":nested(&rs,"complete_to_retire","mean")}));
            }
        }
        let stack: Vec<_> = rows
            .iter()
            .filter(|v| v["mode"] == "op")
            .map(|v| &v["IBS"]["groups"][format!("user/{cat}/operand/stack-addressed")])
            .collect();
        let data = json!({"median_op_samples":value(&op,"samples"),"median_tag_to_complete_mean":nested(&op,"tag_to_complete","mean"),"median_complete_to_retire_mean":nested(&op,"complete_to_retire","mean"),"median_demand_L1_miss_fraction_of_load_ops":rate(&op,"demand_L1_misses","load_ops"),"median_demand_L2_miss_fraction_of_load_ops":rate(&op,"demand_L2_misses","load_ops"),"demand_RAM_count_all_blocks":op.iter().map(|v|v["demand_RAM"].as_u64().unwrap()).sum::<u64>(),"median_valid_demand_miss_mean":nested(&op,"valid_demand_miss_latency","mean"),"median_fetch_samples":value(&fetch,"samples"),"median_fetch_completed_mean":nested(&fetch,"complete_fetch_latency","mean"),"median_fetch_icache_miss_fraction":rate(&fetch,"icache_misses","samples"),"median_fetch_opcache_miss_fraction":rate(&fetch,"opcache_misses","samples"),"median_fetch_complete_fraction":rate(&fetch,"complete","samples"),"median_stack_addressed_samples":value(&stack,"samples"),"median_stack_tag_to_complete_mean":nested(&stack,"tag_to_complete","mean"),"mnemonics":instructions});
        groups.insert(cat, data);
    }
    save(
        r("summary.json"),
        &json!({"groups":groups,"sample_counts":analysis["sample_counts"],"aggregation":"median of3within-process values, not pooled or weighted; means in cycles; mnemonic samples include retireduop selection bias","scope":"descriptive accepted175 primary32 only"}),
    );
    let asm = fs::read_to_string(r("disassembly.txt")).unwrap();
    let mut selected = String::new();
    let mut keep = false;
    let mut names = Vec::new();
    for line in asm.lines() {
        if let Some((_, name)) = line.trim().split_once(" <") {
            if let Some(symbol) = name.strip_suffix(">:") {
                keep = symbol.contains("stream_g1_impl")
                    || (symbol.contains("g2_borrowed") && symbol.contains("5apply"));
                if keep {
                    names.push(symbol.to_owned());
                }
            }
        }
        if keep {
            writeln!(selected, "{line}").unwrap();
        }
    }
    fs::write(r("selected-stream-disassembly.txt"), selected).unwrap();
    let unresolved: Vec<_> = rows
        .iter()
        .filter(|v| v["mode"] == "op")
        .map(|v| {
            v["IBS"]["unresolved_SDK_symbols"]
                .as_object()
                .unwrap()
                .len()
        })
        .collect();
    assert_eq!(unresolved, vec![0, 0, 0]);
    save(
        r("postcapture-review.json"),
        &json!({"SDK_unresolved_symbol_counts":unresolved,"precise_feed_invalid_or_saturated_samples":0,"all_user_imprecise_counts":[2,0,1],"instruction_metadata_clarification":"Original decoder metadata says imprecise samples excluded from mnemonic statistics. It excludes their latency/load values but retains their sample/imprecise counts under the recorded interrupted-IP instruction. Both feed categories have zero such records in every process, so reported feed mnemonic metrics are unaffected. Other categories retain count limitation.","selected_disassembly_symbols":names,"full_disassembly_local_sha256":hash(r("disassembly.txt")),"symbols_local_sha256":hash(r("symbols.txt")),"finish_permission_recovery":"First finish could not hash root-owned newly captured perf.data. Changed ownership of these six local files after capture; bytes unchanged. Second closeout succeeded. No capture or verifier repeated.","decoder_capability_recovery":"First new fetch decode found missing L2Miss text on incomplete fetch; before SDK captures added validity guard and focused regression. Original op-capability interpretation unchanged; capability workload not repeated."}),
    );
    let mut md=String::from("# Current streaming-kernel operation and fetch profiles\n\nThe unchanged accepted175 SDK completed three operation and three instruction-fetch profiles on the local Ryzen9950X. This adds completion/retirement and instruction-class observations to the previous load-only IBS studies. No optimization is adopted.\n\n## Findings\n\nStreaming arithmetic has substantial tag-to-completion delay even on register-only IFMA instructions. Demand loads almost never miss L2, and completed instruction fetches usually have short latency. This makes shortening field-arithmetic dependency chains the next investigation, rather than another large lookup table, allocation-only change or scheduling-grain sweep. This is a hypothesis from the combined service trace and hardware observations, not a proven causal bottleneck or forecast speedup.\n\nThe preceding [service trace](../2026-10-29-service-trace175/README.md) put both K/H and G2 on the finishing path. G1feed below combines all `stream_g1_impl` instantiations, including A and K/H; IBS alone does not separate their query contexts. G2feed is the current borrowed `apply<FAST=true>` kernel.\n\n|Kernel|Median op samples/process|Tag→completion mean, cycles|Completion→retirement mean, cycles|L1 misses/load ops|L2 misses/load ops|RAM samples, all3blocks|Valid demand-miss mean, cycles|\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for (cat, g) in &groups {
        writeln!(
            md,
            "|{cat}|{:.0}|{:.2}|{:.2}|{:.3}%|{:.3}%|{}|{:.2}|",
            g["median_op_samples"].as_f64().unwrap(),
            g["median_tag_to_complete_mean"].as_f64().unwrap(),
            g["median_complete_to_retire_mean"].as_f64().unwrap(),
            100. * g["median_demand_L1_miss_fraction_of_load_ops"]
                .as_f64()
                .unwrap(),
            100. * g["median_demand_L2_miss_fraction_of_load_ops"]
                .as_f64()
                .unwrap(),
            g["demand_RAM_count_all_blocks"],
            g["median_valid_demand_miss_mean"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\nLoad denominators use precise raw load operations; software-prefetch counts are zero in both feed groups. Miss latency includes only demand loads with valid DC-miss data and excludes uncached accesses. These fractions describe samples, not shares of wall time or memory bandwidth.\n\n|Kernel|Instruction|Median samples/process|Tag→completion mean, cycles|Completion→retirement mean, cycles|\n|---|---|---:|---:|---:|\n");
    for (cat, g) in &groups {
        for mnemonic in [
            "vpmadd52luq",
            "vpmadd52huq",
            "vpaddq",
            "vpsrlq",
            "vmovdqa64",
            "vpxor",
        ] {
            let m = &g["mnemonics"][mnemonic];
            writeln!(
                md,
                "|{cat}|`{mnemonic}`|{:.0}|{:.2}|{:.2}|",
                m["median_samples"].as_f64().unwrap(),
                m["median_tag_to_complete_mean"].as_f64().unwrap(),
                m["median_complete_to_retire_mean"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    md.push_str("\nThese are waits from tagging through completion, including operand and resource availability, **not intrinsic instruction execution latencies**. A zeroing XOR can complete immediately yet wait for earlier operations to retire. The same selection/sampling bias prevents interpreting the arithmetic means as exclusive stalls or summing them into removable proof time. Memory-operand instructions can comprise multiple ops; raw load/store flags identify sampled ops. Stack-addressed operands do not by themselves establish compiler spills.\n\n|Kernel|Median fetch samples/process|Completed fetch mean, cycles|Completed fraction|I-cache misses/all fetch samples|Op-cache misses/all fetch samples|\n|---|---:|---:|---:|---:|---:|\n");
    for (cat, g) in &groups {
        writeln!(
            md,
            "|{cat}|{:.0}|{:.3}|{:.2}%|{:.3}%|{:.3}%|",
            g["median_fetch_samples"].as_f64().unwrap(),
            g["median_fetch_completed_mean"].as_f64().unwrap(),
            100. * g["median_fetch_complete_fraction"].as_f64().unwrap(),
            100. * g["median_fetch_icache_miss_fraction"].as_f64().unwrap(),
            100. * g["median_fetch_opcache_miss_fraction"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\nG2 has more I-cache/op-cache misses than G1. Completed-fetch latency omits incomplete fetches, which remain counted explicitly; low completed means cannot rule out frontend stalls. Fetch samples are separate from retired-op samples. No generic frontend-stall alias or unsupported event interpretation is used.\n\n## Protocol and verification\n\nSix fixed fresh processes,32workers on CPUs0–31,3priming proofs plus61sampled complete SDK requests each. Blocks alternate op/fetch, fetch/op, op/fetch; period524288, `ibs_op/cnt_ctl=1/` and `ibs_fetch/rand_en=1/`. FIFO acknowledgments enable after key loading/priming and disable before export. No owned helper/build/hash/archive/secondCPU workload overlapped capture; every capture finished before any independent verifier or decoder. All30workflow steps passed. No successful capture or verifier was rerun, no rounds extended or observations excluded.\n\n**384SDK requests internally verify; six fresh164-byte exported proofs independently pass pinned Gnark, image/public binding and freshness checks. Ledger17805→17811,12proof/public members.** Operation samples517186; fetch samples67569; zero lost records. Exact raw/script PID/TID/ns/IP joins, op weight/source agreement and raw-vs-pretty field checks pass. All SDK instruction addresses resolve by exact mangled symbol+offset into the pinned PIE disassembly. Both feed groups have zero imprecise or saturated/invalid latency pairs. Three imprecise user records outside these feeds retain sample-count attribution limits documented in [postcapture review](artifacts/postcapture-review.json).\n\nFour new Rust decoder tests pass, plus one focused incomplete-fetch regression after a capability decode exposed perf's omitted invalid field. A new nonSDK32MiB SHA256 fetch capability yields31samples; the existing239op records validate the new decoder without repeating that workload. Original prover arithmetic/runtime tests are inherited because source and executable are unchanged. Initial closeout encountered root-owned perf data; ownership changed after capture, then closeout completed with original bytes and no rerun.\n\nSDK medians under op sampling were7.991270/8.157612/8.154106ms; fetch8.054909/7.973145/8.012890ms. These are diagnostic, without unprofiled latency or observer-overhead qualification. No RSS improvement, minimum RAM, AWS/Aeglos, phone/ARM/TEE, huge-circuit, concurrency or throughput result follows.\n\nRegister semantics follow [AMD's Zen4/Zen5 IBS derived events](https://docs.amd.com/r/en-US/57368-uProf-user-guide/7.9.3.2.4.-IBS-Derived-Events), the installed6.8.0-147 [header](artifacts/references/installed-amd-ibs.h), and [upstream Linuxv6.8 IBS handling](https://github.com/torvalds/linux/blob/v6.8/arch/x86/events/amd/ibs.c). Upstream is a reference, not asserted byte-identical to Ubuntu's runtime kernel. Current hardware exposes Zen4 IBS extensions; no old-family erratum is assumed.\n\n## CPU and Metal comparison\n\nInherited accepted local timings against the supplied historical M5 references; not newly measured by this diagnostic. Accepted primary peak RSS remains134MiB. The prior physical16 baseline-calibration engineering exception remains; the full performance gate must not be relabeled passed.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for shape in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let m = &analysis["inherited_M5"][shape];
        writeln!(
            md,
            "|{shape}|{:.6}|{:.0}|{:.3}×|{:.0}|{:.3}×|{:.1}|",
            m["accepted_ms"].as_f64().unwrap(),
            m["cpu_reference_ms"].as_f64().unwrap(),
            m["cpu_speedup"].as_f64().unwrap(),
            m["metal_reference_ms"].as_f64().unwrap(),
            m["metal_speedup"].as_f64().unwrap(),
            m["required_ms_for_CPU10x"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\nAll-eight CPU10× goal remains active/unmet,0/8. Accepted54→1 is162.250464ms without a supplied M5 reference. No production code or accepted measurement changes.\n\n[Design](artifacts/design.md), [declaration](artifacts/declaration.json), [summary](artifacts/summary.json), [all observations](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [source/tool pins](artifacts/pins.json), [raw local-only hashes](artifacts/raw-local-only-sha256.json). Raw perf data/register dumps/scripts and complete49MiB disassembly stay local; selected streaming disassembly is archived.\n");
    let d = Path::new(D);
    fs::create_dir_all(d.join("artifacts")).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    let mut files = vec![
        "decode.rs",
        "legacy-decoder.rs",
        "decoder-build.log",
        "decoder-build-v2.log",
        "decoder-tests.log",
        "decoder-incomplete-test.log",
        "flow.rs",
        "flow-build.log",
        "publish.rs",
        "publish-build.log",
        "design.md",
        "hardware.json",
        "kernel.txt",
        "perf-version.txt",
        "references/installed-amd-ibs.h",
        "capabilities.json",
        "case.json",
        "declaration.json",
        "jobs.json",
        "pins.json",
        "run-plan.json",
        "prepare.log",
        "finish.log",
        "proof-inventory.json",
        "raw-local-only-sha256.json",
        "analysis.json",
        "summary.json",
        "postcapture-review.json",
        "selected-stream-disassembly.txt",
        "capability/analysis.json",
        "capability/inherited-op-analysis.json",
        "capability/capture.stdout.log",
        "capability/capture.stderr.log",
        "capability/raw.stderr.log",
        "capability/script.stderr.log",
        "runs/result.json",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();
    for e in fs::read_dir(r("runs")).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            let name = p.file_name().unwrap().to_str().unwrap();
            for f in ["stdout.log", "stderr.log", "command.json", "exit.json"] {
                let rel = format!("runs/{name}/{f}");
                if r(&rel).exists()
                    && !(f == "stdout.log"
                        && (name.starts_with("raw-") || name.starts_with("script-")))
                {
                    files.push(rel);
                }
            }
        }
    }
    for j in read(r("jobs.json")).as_array().unwrap() {
        for f in ["0.proof", "0.public"] {
            files.push(format!(
                "captures/{}/proofs/{f}",
                j["name"].as_str().unwrap()
            ));
        }
    }
    let mut index = BTreeMap::new();
    for rel in files {
        let src = r(&rel);
        let dest = d.join("artifacts").join(&rel);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(&src, &dest).unwrap();
        assert_eq!(hash(&src), hash(&dest));
        index.insert(format!("artifacts/{rel}"), hash(&dest));
    }
    index.insert("README.md".to_owned(), hash(d.join("README.md")));
    save(d.join("artifact-index.json"), &json!(index));
    index.insert(
        "artifact-index.json".to_owned(),
        hash(d.join("artifact-index.json")),
    );
    let mut sums = String::new();
    for (p, h) in &index {
        writeln!(sums, "{h}  {p}").unwrap();
    }
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!("Published{}hashes; ledger17811", index.len());
}
