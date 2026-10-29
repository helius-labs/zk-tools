use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/mask-carry175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-mask-carry175";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["gate_passed"], false);
    let m=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let mut code = BTreeMap::new();
    for build in ["frozen", "candidate"] {
        let text = fs::read_to_string(format!("{R}/{build}-assembly.txt")).unwrap();
        let mut keep = false;
        let mut selected = String::new();
        for l in text.lines() {
            if l.ends_with(">:") {
                keep = ["chain", "streams", "fused"]
                    .iter()
                    .any(|p| l.contains(&format!("<probe::{p}::<zopro_bn254::FqConfig, true,")));
            }
            if keep {
                writeln!(selected, "{l}").unwrap();
            }
        }
        fs::write(format!("{R}/{build}-selected.s"), &selected).unwrap();
        let records: Vec<_> = a["static_code"][build]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| {
                ["chain", "streams", "fused"].iter().any(|p| {
                    v["symbol"]
                        .as_str()
                        .unwrap()
                        .contains(&format!("<probe::{p}::<zopro_bn254::FqConfig, true,"))
                })
            })
            .map(|v| {
                let mut v = v.clone();
                let frames: Vec<_> = v["stack_subtractions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|v| {
                        let body = v.as_str().unwrap().split_once(':').unwrap().1.trim();
                        body.starts_with("sub ")
                    })
                    .cloned()
                    .collect();
                v["literal_stack_reservations"] = json!(frames);
                v.as_object_mut().unwrap().remove("stack_subtractions");
                v
            })
            .collect();
        code.insert(build, records);
    }
    save(
        format!("{R}/hot-code.json"),
        &json!({"records":code,"original_stack_subtractions_field_is_broad":"Original analysis counts any subtract mnemonic with RSP including vpsubq. Only scalar sub immediate,RSP is a literal frame reservation; new records filter that distinction.","selected_disassembly":"Fq/R256 complete linked loops; static sites include setup/final copy/cold paths, not executed counts or measured spills","complete_local_assembly_hashes":{"frozen":hash(format!("{R}/frozen-assembly.txt")),"candidate":hash(format!("{R}/candidate-assembly.txt"))}}),
    );
    let mut md=String::from("# Branchless final carry normalization\n\n**Rejected at the component gate; no SDK integration or adoption.** The intended Fq/R256 aggregate is 1.091876 against fresh ordinary arithmetic, bounds [1.089922, 1.094193], with 0/31 winning blocks. All 12 contexts regress; every fresh/frozen calibration passes. Accepted production remains unchanged.\n\n## Change\n\nA private eight-lane CIOS method replaces only the final serial carry pass with independent quotient transfers followed by exact AVX-512 mask carry propagation. Generate/propagate masks handle every cascade without a conditional ripple branch. Product rows, Montgomery cancellation, mixed-radix packing, representation and fused correction retain the accepted code. The [bounds](artifacts/bounds.md) show exact integer equivalence for coefficients below 2^61. No heap allocation, lookup table, key copy or retained setup is introduced.\n\nThe [24 October parallel normalization](../2026-10-24-parallel-normalize-component/README.md) and [prefix-only integration](../2026-10-24-prefix-controlled/README.md) used a conditional correction path and remain rejected. This experiment changes the correction algorithm rather than repeating either implementation. The current [IBS diagnostic](../2026-10-29-stream-ibs175/README.md) motivated examining arithmetic dependencies; it did not predict a speedup.\n\n## Fixed measurements\n\nRyzen 9950X, one thread pinned to CPU 2. Thirty-one fixed rotated/reversed blocks compare a frozen ordinary executable built before the include with fresh ordinary and masked methods sharing one new executable. Each process measures 100,000 iterations after three 2,000-iteration primes. Conversions and output checking are outside the timer. No owned build, helper, hash, archive or second workload overlaps capture.\n\nUnits are ns per field-lane update; four-stream cases divide by four. Percentage changes are medians of paired per-block ratios against fresh ordinary, not ratios of displayed medians. Positive means slower.\n\n|Case|Frozen|Fresh|Masked|Masked change|\n|---|---:|---:|---:|---:|\n");
    for row in a["rows"].as_array().unwrap() {
        writeln!(
            md,
            "|{}|{:.6}|{:.6}|{:.6}|+{:.3}%|",
            row["case"].as_str().unwrap(),
            row["median_ns"]["frozen"].as_f64().unwrap(),
            row["median_ns"]["fresh"].as_f64().unwrap(),
            row["median_ns"]["masked"].as_f64().unwrap(),
            100. * (row["masked_ratios"]["fresh"]["median"].as_f64().unwrap() - 1.)
        )
        .unwrap();
    }
    md.push_str("\nFq/R256 aggregate versus frozen is 1.091435 [1.089644, 1.093233], also 0/31 wins. Calibration bounds for every case stay within factor 1.03; all 24 candidate/control nonregression checks fail. The predeclared intended aggregate requires upper ratio below 1 and at least 21/31 wins against both controls; every context must remain within factor 1.03. There is no minimum gain floor. Individual order ranks 10–22 of 31 give nominal 97.055% coverage conditional on exchangeable independent blocks, without a familywise guarantee. No exclusions, extensions, successful reruns or threshold changes occur. The exact recipe is closed.\n\n## Correctness and profiles\n\nThe exact model checks **122,880 BigUint integer/normalization equalities**, including boundary and fixed random arrays at radices 16, 256 and 2^52. It also exhausts all **1,048,576** radix-4 arrays with every coefficient below B². Native tests check **40,960** coefficient arrays and **172,544** changing-chain field-lane results against Arkworks across both primes/radices, ordinary/fused products, inclusive 2p inputs, lazy outputs and output bounds. Two new tests pass; the 33 unchanged field tests are filtered out and inherited. These counts are comparisons, not proofs.\n\nAll 1,116 latency processes, 12 preflights and four separate profiles check their returned canonical fields against a scalar closed-form affine-power oracle outside the timer. Each returns 32 fields, but chain/fused duplicate eight independent final states into four slots; those are not 32 independent chains. Original arithmetic and the probe are byte-retained. A temporary post-build probe formatting change was restored to the exact original build bytes before pins/capture; neither successful build was repeated.\n\nFour whole-process cycles:u profiles run after latency, 30,000,000 iterations at 199 Hz. They include setup/primes/oracle and do not enter the gate.\n\n|Profile|Samples|Lost records|\n|---|---:|---:|\n");
    for (k, p) in a["profiles"].as_object().unwrap() {
        writeln!(md, "|{k}|{}|{}|", p["samples"], p["lost"]).unwrap();
    }
    md.push_str("\nNearly all sampled periods are attributed to the expected matching loop; one PID per profile, zero lost records. Profiles confirm the intended code runs, without establishing an exclusive stall cause. Full raw perf files remain local with hashes; decoded samples and loss dumps are archived.\n\nActual Fq/R256 dependent-loop static instruction sites increase **307→327**, with the same **100 IFMA sites**, **42 RSP operand lines** and **384-byte literal stack reservation**. These are static code observations, not dynamic instructions, measured spills or an RSS bound. The new [hot-code audit](artifacts/hot-code.json) distinguishes literal stack reservation from the original observer's broad subtract-with-RSP field, which also includes vector subtracts. This normalization removes a serial source carry chain but adds work; the measured regression rejects this implementation rather than proving all dependency-reduction approaches unhelpful.\n\n## Scope and provenance\n\nAll 175 production source hashes remain exact. The private field copy contains the original 17 files; removing one include recovers the original eight-lane module, while other 16 originals remain exact. The appended helper copies the accepted CIOS body with only its name and final normalization block changed. No four-lane dispatch or production consumer changes. Frozen/fresh probes use the same source, seeds and oracle. Sources, libraries, executables, observer and xtask have 281 pins closed before/after. All workflow steps complete.\n\nNo SDK requests or new exported proofs occur; independent proof ledger remains **17,811**. No new proving-time, RSS, minimum RAM, AWS/Aeglos, mobile/ARM/TEE, large-circuit, memory-pressure, concurrency or throughput qualification follows.\n\n## Inherited CPU and Metal comparison\n\nAccepted local timings against supplied historical M5 references; these are not component measurements. Primary peak RSS remains 134 MiB. The accepted full-performance-gate failure and 16-worker baseline-calibration engineering exception remain unchanged.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
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
        let v = &m["m5"][shape];
        writeln!(
            md,
            "|{shape}|{:.6}|{:.0}|{:.3}×|{:.0}|{:.3}×|{:.1}|",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap(),
            v["required_ms_for_CPU10x"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\nAll-eight CPU 10× goal remains active and unmet (0/8). Accepted 54→1 remains 162.250464 ms without a supplied M5 reference.\n\n[Design and gate](artifacts/design.md), [declaration](artifacts/declaration.json), [exact model](artifacts/model.rs), [native kernel](artifacts/kernel.rs), [all results](artifacts/analysis.json), [hot-code audit](artifacts/hot-code.json), [source/tool pins](artifacts/pins.json), [artifact hashes](artifact-index.json).\n");
    fs::write(format!("{D}/README.md"), md).unwrap();
    let mut index: BTreeMap<String, String> =
        serde_json::from_value(read(format!("{D}/artifact-index.json"))).unwrap();
    for (p, h) in &index {
        assert_eq!(hash(format!("{D}/artifacts/{p}")), *h);
    }
    for file in [
        "report.rs",
        "report-build.log",
        "hot-code.json",
        "frozen-selected.s",
        "candidate-selected.s",
    ] {
        let out = Path::new(D).join("artifacts").join(file);
        fs::copy(format!("{R}/{file}"), &out).unwrap();
        index.insert(file.into(), hash(out));
    }
    save(format!("{D}/artifact-index.json"), &json!(index));
    let mut sums = String::new();
    for (p, h) in &index {
        writeln!(sums, "{h}  artifacts/{p}").unwrap();
    }
    for p in ["artifact-index.json", "README.md"] {
        writeln!(sums, "{}  {p}", hash(format!("{D}/{p}"))).unwrap();
    }
    fs::write(format!("{D}/SHA256SUMS"), sums).unwrap();
    println!(
        "Reportwritten;{}artifactmembers+README+index hashes",
        index.len()
    );
}
