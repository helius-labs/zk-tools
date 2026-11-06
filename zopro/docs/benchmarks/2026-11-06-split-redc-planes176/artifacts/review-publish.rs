use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-planes176";
const G: &str = "/home/ubuntu/src/zopro-perf-work/accepted-dependencies176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-split-redc-planes176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hex(s: &str) -> u64 {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}
fn reg(s: &str) -> Option<u32> {
    let s = s.trim();
    for p in ["zmm", "ymm", "xmm"] {
        if let Some(t) = s.strip_prefix(p) {
            let n = t
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>();
            return n.parse().ok();
        }
    }
    None
}
fn args(s: &str) -> Vec<&str> {
    s.split_once(char::is_whitespace)
        .unwrap()
        .1
        .split('#')
        .next()
        .unwrap()
        .split(',')
        .map(str::trim)
        .collect()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = vec![];
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p));
        } else {
            v.push(p);
        }
    }
    v.sort();
    v
}
fn main() {
    assert!(!Path::new(D).exists());
    assert!(!Path::new(R).join("review.json").exists());
    let m = read(format!("{R}/model.json"));
    let g = read(format!("{G}/analysis.json"));
    assert_eq!(m["status"], "pass");
    assert_eq!(m["necessary_dependency_work_screen_passed"], true);
    assert_eq!(g["status"], "pass");
    let mut pins = BTreeMap::new();
    for root in [R, G] {
        for (p, h) in read(format!("{root}/pins.json")).as_object().unwrap() {
            assert_eq!(h, &json!(hash(p)));
            pins.insert(p.clone(), h.as_str().unwrap().to_string());
        }
    }
    let contexts = m["contexts"].as_array().unwrap();
    assert_eq!(contexts.len(), 12);
    assert_eq!(
        contexts
            .iter()
            .map(|v| v["outputs"].as_u64().unwrap())
            .sum::<u64>(),
        80844
    );
    assert_eq!(m["exact_reduced_outputs"], 80844);
    assert_eq!(m["exact_row_states"], 404220);
    assert_eq!(m["reduced_radix_cancellation_checks"], 2796192);
    let mut seen = BTreeSet::new();
    for c in contexts {
        assert!(seen.insert((
            c["field"].to_string(),
            c["radix_bits"].to_string(),
            c["operation"].to_string()
        )));
        assert_eq!(
            c["outputs"],
            if c["operation"] == "ordinary" {
                5169
            } else {
                7521
            }
        );
    }
    let b = 1u128 << 52;
    let mut e = 0u128;
    let mut bounds = vec![];
    for _ in 0..4 {
        let z = e + 4 * (b - 1);
        e = z + 2 * (b - 1) + (z + b - 1) / b;
        bounds.push(e);
    }
    assert!(e < 25 * b);
    let final52 = e + 6 * (b - 1) + (e + 4 * (b - 1) + b - 1) / b;
    let final48 = e + 6 * (b - 1) + (e + 4 * (b - 1) + (1 << 48) - 1) / (1 << 48);
    assert!(final52 < 32 * b && final48 < 32 * b);
    for k in ["max_plane", "max_sum", "max_frontier", "max_finish"] {
        assert!(m[k].as_str().unwrap().parse::<u128>().unwrap() < 32 * b);
    }
    for p in [
        "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47",
        "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001",
    ] {
        let p = BigUint::parse_bytes(p.as_bytes(), 16).unwrap();
        assert!(&p * BigUint::from(4u32) < (BigUint::from(1u32) << 256usize));
        assert!(&p * BigUint::from(8u32) < (BigUint::from(1u32) << 260usize));
        assert!(&p * BigUint::from(3u32) < (BigUint::from(1u32) << 256usize));
    }
    let dag = m["dependency_dag"].as_array().unwrap();
    assert_eq!(dag.len(), 4);
    for pair in dag.chunks(2) {
        assert_eq!(pair[0]["split_planes"], false);
        assert_eq!(pair[1]["split_planes"], true);
        assert_eq!(
            pair[0]["first_four_rows_ifma"],
            pair[1]["first_four_rows_ifma"]
        );
        assert!(
            pair[1]["quotient_depths"][3].as_u64().unwrap()
                < pair[0]["quotient_depths"][3].as_u64().unwrap()
        );
        assert!(
            pair[1]["max_merged_depth"].as_u64().unwrap()
                < pair[0]["max_merged_depth"].as_u64().unwrap()
        );
    }
    let q = read("/home/ubuntu/src/zopro-perf-work/g2-validated-batch176/qualification.json");
    let mut paths = 0;
    let mut blocks = 0;
    let mut instructions = 0;
    for fun in g["static_function_categories"].as_array().unwrap() {
        let row = q["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["kind"] == fun["kind"] && r["role"] == fun["role"])
            .unwrap();
        let file = row["inherited_disassembly"]
            .as_str()
            .map(String::from)
            .unwrap_or_else(|| {
                format!(
                    "/home/ubuntu/src/zopro-perf-work/g2-validated-batch176/disassembly/{}-{}.txt",
                    fun["role"].as_str().unwrap(),
                    fun["kind"].as_str().unwrap()
                )
            });
        let mut ins = BTreeMap::new();
        let end = row["start"].as_u64().unwrap() + row["size_bytes"].as_u64().unwrap();
        for l in fs::read_to_string(&file).unwrap().lines() {
            if let Some((a, s)) = l.trim().split_once(':') {
                if let Ok(a) = u64::from_str_radix(a, 16) {
                    if a >= row["start"].as_u64().unwrap() && a < end {
                        ins.insert(a, s.trim().to_string());
                    }
                }
            }
        }
        let mut used = BTreeSet::new();
        for block in fun["basic_blocks"].as_array().unwrap() {
            blocks += 1;
            let start = hex(block["start_hex"].as_str().unwrap());
            let last = hex(block["last_hex"].as_str().unwrap());
            let mut n = 0;
            for (ip, _) in ins.range(start..=last) {
                assert!(used.insert(*ip));
                n += 1;
            }
            assert_eq!(block["instruction_rows"], n);
            let chain = block["selected_chain"].as_array().unwrap();
            let mut weight = 0;
            let mut old = 0;
            let mut prev = None;
            for node in chain {
                paths += 1;
                let ip = hex(node["ip_hex"].as_str().unwrap());
                assert!(ip >= start && ip <= last);
                let s = &ins[&ip];
                assert_eq!(node["instruction"], *s);
                let op = s.split_whitespace().next().unwrap();
                let a = args(s);
                let is_ifma = op.starts_with("vpmadd52");
                let w = if is_ifma { 4 } else { 2 };
                weight += w;
                old += if is_ifma { 4 } else { 1 };
                assert_eq!(node["calibrated_weight"], w);
                if let Some((previous, regn)) = prev {
                    assert!(previous < ip);
                    let inputs = if is_ifma { &a[..] } else { &a[1..] };
                    assert!(inputs.iter().filter_map(|s| reg(s)).any(|r| r == regn));
                    for (_, middle) in ins.range(previous + 1..ip) {
                        let op = middle.split_whitespace().next().unwrap();
                        assert!(op != "vzeroall" && op != "vzeroupper");
                        if op.starts_with('v') {
                            if let Some((_, tail)) = middle.split_once(char::is_whitespace) {
                                assert_ne!(
                                    tail.split(',').next().and_then(reg),
                                    Some(regn),
                                    "intervening overwrite {middle}"
                                );
                            }
                        }
                    }
                }
                prev = Some((ip, reg(a[0]).unwrap()));
            }
            assert_eq!(block["max_connected_calibrated_score"], weight);
            assert_eq!(block["inherited_weights_on_same_path"], old);
        }
        assert_eq!(used.len(), ins.len());
        assert_eq!(used.len() as u64, row["instruction_rows"].as_u64().unwrap());
        instructions += used.len();
    }
    assert_eq!(g["samples"], 119638);
    let old = read("/home/ubuntu/src/zopro-perf-work/g2-validated-capture176/review.json");
    let mut total = 0;
    let mut profiles = BTreeSet::new();
    for p in g["panels"].as_array().unwrap() {
        let name = p["profile"].as_str().unwrap();
        assert!(profiles.insert(name));
        assert_eq!(p["samples"], old["profiles"][name]["samples"]);
        assert_eq!(p["normalized_instruction_samples"], p["named_core_samples"]);
        total += p["samples"].as_u64().unwrap();
    }
    assert_eq!(profiles.len(), 18);
    assert_eq!(total, 119638);
    assert!(fs::read(format!("{R}/model.stderr.log"))
        .unwrap()
        .is_empty());
    assert!(fs::read(format!("{G}/analyze.stderr.log"))
        .unwrap()
        .is_empty());
    let review = json!({"status":"pass","model_output_counts":80844,"coefficient_bound_first_four":bounds.iter().map(|v|v.to_string()).collect::<Vec<_>>(),"coefficient_bound_final52":final52.to_string(),"coefficient_bound_final48":final48.to_string(),"instruction_rows_partitioned":instructions,"basic_blocks":blocks,"selected_chain_nodes_checked":paths,"profiles_reconciled":18,"samples":total,"selected_chain_edges_match_register_def_use":true,"native_code_qualified":false,"timing_claim":false,"new_proofs":0,"production_changed":false});
    save(format!("{R}/review.json"), &review);
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut members = BTreeMap::new();
    for root in [R, G] {
        for file in walk(Path::new(root)) {
            let ext = file.extension().and_then(|s| s.to_str()).unwrap_or("");
            if ["rs", "md", "json", "log"].contains(&ext) {
                let s = fs::read_to_string(&file).unwrap();
                pins.insert(file.display().to_string(), hash(&file));
                members.insert(
                    format!(
                        "{}/{}",
                        if root == R {
                            "model"
                        } else {
                            "dependency-census"
                        },
                        file.strip_prefix(root).unwrap().display()
                    ),
                    s,
                );
            }
        }
    }
    let archive = json!({"files":members});
    save(format!("{R}/source-and-receipts.json"), &archive);
    let dest = art.join("source-and-receipts.json.gz");
    let out = fs::File::create(&dest).unwrap();
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(format!("{R}/source-and-receipts.json"))
        .stdout(out)
        .status()
        .unwrap()
        .success());
    let back = Command::new("gzip").arg("-dc").arg(&dest).output().unwrap();
    assert!(back.status.success());
    assert_eq!(
        back.stdout,
        fs::read(format!("{R}/source-and-receipts.json")).unwrap()
    );
    let mut index = BTreeMap::new();
    for (k, v) in archive["files"].as_object().unwrap() {
        index.insert(
            k.clone(),
            format!("{:x}", Sha256::digest(v.as_str().unwrap().as_bytes())),
        );
    }
    save(art.join("archive-index.json"), &json!(index));
    save(art.join("input-pins.json"), &json!(pins));
    save(art.join("model.json"), &m);
    save(art.join("review.json"), &review);
    for f in ["model.rs", "design.md", "bounds.md", "review-publish.rs"] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    fs::copy(format!("{G}/analyze.rs"), art.join("dependency-census.rs")).unwrap();
    fs::copy(
        format!("{G}/protocol.md"),
        art.join("dependency-protocol.md"),
    )
    .unwrap();
    let mut md=String::from("# Split product/reduction coefficient planes: mathematical screen\n\nThe split-accumulator recurrence passes **80,844 exact output comparisons and 404,220 row-state checks**, plus 2,796,192 reduced-radix carry checks. Its source-level dependency screen improves with unchanged IFMA count. **No native candidate has been qualified or timed, and no production optimization is adopted.**\n\n## Why this hypothesis\n\nThe [hardware calibration](../2026-11-06-instruction-calibration176/README.md) established IFMA accumulator latency about4 cycles and vector add/carry-shift latency about2 cycles on this Ryzen9950X. A new offline census reuses all9 saved caller decodes and all18 saved profile panels (119,638 samples), with exact ELF executable LOAD/MMAP2 address translation and instruction-boundary/symbol agreement. No recording, decode or proof is repeated.\n\nThe prominent OFF G2 block at+0x8aa2 contains a selected connected chain of28 IFMAs and one add. Its partial score changes only113→114 when substituting calibrated weights. This cautions against inferring from slower carry instructions that final normalization is the main opportunity. Across the three G2 OFF panels this block contains32.45–33.07% of sample locations. These are block-location counts with sampling skid, not exclusive cycles, a proven hardware critical path or removable proof time.\n\n| OFF consumer | Instruction rows | Basic blocks | Calibrated register-only rows | Largest connected score |\n|---|---:|---:|---:|---:|\n");
    for f in g["static_function_categories"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["role"] == "OFF")
    {
        let bs = f["basic_blocks"].as_array().unwrap();
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            f["kind"].as_str().unwrap(),
            f["rows"],
            bs.len(),
            bs.iter()
                .map(|v| v["calibrated_register_only_rows"].as_u64().unwrap())
                .sum::<u64>(),
            bs.iter()
                .map(|v| v["max_connected_calibrated_score"].as_u64().unwrap())
                .max()
                .unwrap()
        ));
    }
    md.push_str("\nUnknown vector writers, masks, memory operands, uncalibrated instructions and block boundaries break the scored chains. Loads/stores, register copies and inter-block dependencies are absent. The scores are neither a complete schedule nor a proven latency lower bound. The graph parser is checked against16 already decoded instruction-calibration loops and explicit alias/mask/memory exclusions. The independent reviewer checks every reported chain edge against actual intervening register writes and partitions every instruction exactly once. The first offline pass rejected the renamed G2 validated-ON symbol; the saved-symbol repair is retained and no performance evidence changes. All18 panels and the rejected ON arm remain in the archive.\n\n## Recurrence\n\nFor the first four radix52 CIOS rows, product halves update a plane P and Montgomery-reduction halves update Q. The quotient uses their low-coefficient sum z=P0+Q0. The existing cancelled-low carry identity updates Q1, then both planes shift. Their sum recovers the accepted unnormalized coefficient state exactly. Merge them before the fifth row, preserving the original final52/mixed48 step, normalization and correction.\n\nThis changes accumulation dependencies, not the field representation or the number of products. It adds at most nine vector additions and six live vector coefficients; compiler allocation and stack traffic may erase the opportunity. No heap buffer, table, key/point copy, additional plan or pool is proposed. This differs from the rejected FIPS, paired cancellation, row-quotient and independent-complex-output mechanisms; those results stay closed. The earlier zero-accumulator mask audit also found that LLVM already removes the proposed52-bit masks, so it was not rerun.\n\nThe checked-u128 digit interpreter agrees after each row with an independent BigUint whole-state recurrence, full-R quotient and field reference. Coverage includes both BN254 fields, R256/R260, ordinary/sum2/original-complement difference2, inclusive[0,2p] boundaries,4,096 random tuples and1,024 dependent chain cases per context. The [range argument](artifacts/bounds.md) bounds coefficients strictly below64B=2^58. Finite tests and source algebra are not a formal proof of future native code.\n\n## Necessary dependency screen\n\nThe idealized source DAG uses calibrated weights IFMA4/add2/right52shift2 and includes extra frontier and merge additions. Constants/input limbs start ready. Register allocation, spills, uncalibrated final48 instructions and the common final normalization are omitted. Values below are **dependency scores, not measured cycles or speed predictions**.\n\n| Operation | Fourth quotient, ordinary layout | Fourth quotient, split planes | Merged next-row frontier, ordinary | Merged next-row frontier, split | First-four-row IFMAs, either |\n|---|---:|---:|---:|---:|---:|\n| Ordinary multiply |54|44|62|52|80|\n| Sum/difference of products |82|62|94|70|120|\n\nThe final common row brings nominal totals to100/150 IFMAs. The necessary screen passes; the next stage is an actual native implementation, independent output tests and linked-caller codegen inspection. A positive model alone does not justify SDK timing or adoption. Actual caller/topology and full all-shape SDK/Gnark/resource/portable gates remain necessary.\n\nPreparation retained one pre-execution JSON-macro compilation correction and a corrected bound-text equality; the corrected mathematical model ran once successfully. No mathematical result, successful native capture, gate or input subset was replaced.\n\n## Unchanged full-proof baseline\n\nInherited accepted176 local32-worker single-proof times below are not new measurements. The previously documented physical16 calibration engineering exception remains. The original all-eight CPU10× goal is active and unmet0/8.\n\n| Shape | Accepted CPU ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n|---|---:|---:|---:|---:|---:|\n");
    for (sh, ms, cpu, metal) in [
        ("1x2", 7.928660, 37., 24.),
        ("2x2", 11.151932, 59., 30.),
        ("4x2", 16.086386, 109., 48.),
        ("8x2", 24.179453, 189., 74.),
        ("12x2", 33.884455, 284., 101.),
        ("16x2", 40.077381, 335., 124.),
        ("24x2", 65.850777, 528., 176.),
        ("merge24", 65.060937, 507., 175.),
    ] {
        md.push_str(&format!(
            "|{sh}|{ms:.6}|{cpu:.0}|{:.3}×|{metal:.0}|{:.3}×|\n",
            cpu / ms,
            metal / ms
        ));
    }
    md.push_str("\nInherited merge54 remains161.883899ms with no supplied M5 reference. Inherited132.75MiB RSS is not a new peak or minimum RAM. Proof ledger stays30,846. No fresh AWS/Aeglos, SDK/Gnark proof, portability, memory-pressure or mobile result is claimed.\n\n[Model result](artifacts/model.json), [design](artifacts/design.md), [range argument](artifacts/bounds.md), [integer/work model](artifacts/model.rs), [dependency census source](artifacts/dependency-census.rs), [census protocol](artifacts/dependency-protocol.md), [independent review](artifacts/review.json), [review/publication source](artifacts/review-publish.rs), [source/results archive](artifacts/source-and-receipts.json.gz), [member hashes](artifacts/archive-index.json) and [input pins](artifacts/input-pins.json). The archive contains all basic blocks, selected chains,18 panels, new source and repair records; original recorded streams and decodes remain at their pinned local paths and prior reports.\n");
    fs::write(Path::new(D).join("README.md"), md).unwrap();
    let files = walk(Path::new(D));
    let mut checks = String::new();
    for file in &files {
        checks.push_str(&format!(
            "{}  {}\n",
            hash(file),
            file.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), checks).unwrap();
    save(
        format!("{R}/publication.json"),
        &json!({"status":"pass","report":D,"archive_members":archive["files"].as_object().unwrap().len(),"files_including_checksum":files.len()+1,"archive_roundtrip_exact":true,"new_proofs":0}),
    );
    println!(
        "PASS model/range/{} instruction rows/{} blocks/{} selected nodes; archive {} members",
        instructions,
        blocks,
        paths,
        members.len()
    );
}
