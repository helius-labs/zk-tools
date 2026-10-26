use serde_json::Value;
use std::{fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn num(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32-sdk");
    let report =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-deferred-dif32-sdk");
    fs::create_dir_all(report).unwrap();
    let p = read(root.join("pilot-result.json"));
    let profile = read(root.join("profile-analysis.json"));
    let code = read(root.join("codegen.json"));
    let mut s=String::from("# Deferred FFT correction in complete SDK requests\n\n**The fixed full-proof screen fails; the candidate is not adopted.** The earlier [component](../2026-10-26-deferred-dif32-component/README.md) reduced isolated DIF32 tile time by32%, but the original-eight paired geometric SDK aggregate is essentially flat:1.000160998 versus accepted and1.000013540 versus fresh OFF. Each aggregate wins3/7 blocks. Production remains unchanged.\n\nPrimary process-median latency is8.580045ms accepted,8.590735ms OFF and8.569553ms ON. Paired primary ratios improve by0.45% and0.18%, with4/7 and5/7 wins. Those small primary improvements do not pass the complete gate: one primary block exceeds5% regression, the original-eight aggregates fail improvement, and the physical16 control and one candidate block fail separately. All six failed checks and all observations are retained. There is no unchanged retry or selective adoption.\n\n## Integration\n\nThe two field files are byte-identical to the pushed component. A native Plan samples `ZOPRO_CPU_DEFERRED_DIF32=1` once at construction; omission and zero are OFF. Only ordinary canonical roots can enable the new DIF32 path. Empty and multiplicatively twisted roots remain OFF: multiplication can leave twisted roots abovep, outside this routine's stronger precondition. DIT, shorter expanded leaves, portable/generic algorithms and the original OFF body remain.\n\nThe callback borrows exactly the existing cached or uncached Quad roots. It adds no31-root array, key data, retained root cache, preparation, worker or per-proof heap buffer. Standard environment parsing may allocate an ephemeral flag string during Plan construction. The arithmetic returns normalized values below2p before existing compact storage. Removing the additions recovers both original core files byte for byte.\n\nNative layout probes explicitly link the actual accepted and candidate libraries: both Plans are1,728 bytes with32-byte alignment. The candidate Twiddles type is448 bytes and Pair is320 bytes. The existing Plan footprint is preserved in these native builds; this is not a whole-process memory or minimum-RAM claim.\n\n## Complete SDK timing\n\nRyzen9950X,16physical cores/32logical CPUs. Seven rotated/reversed separate-process blocks compare accepted, fresh OFF and ON. Every32-worker case measures31 fresh requests after3primes; primary16 measures101 after3primes, pinned to CPUs0–15, one logical CPU per physical core. All latency finishes before profiles. Each request includes fresh solving/randomness/proving, SDK verification and serialization. Key loading and primes are excluded from latency. No other owned build, profile, archive or benchmark overlaps timing.\n\n| Case/workers | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for name in [
        "1x2-w32",
        "2x2-w32",
        "4x2-w32",
        "8x2-w32",
        "12x2-w32",
        "16x2-w32",
        "24x2-w32",
        "merge-24x1-w32",
        "merge-54x1-w32",
        "1x2-w16",
    ] {
        let c = &p["cases"][name];
        let m = &c["metrics"];
        s.push_str(&format!(
            "| {name} | {:.6} | {:.6} | {:.6} | {:.9} | {:.9} |\n",
            num(&m["accepted"], "median_ms"),
            num(&m["off"], "median_ms"),
            num(&m["on"], "median_ms"),
            num(&c["on_over_control"]["accepted"], "paired_median"),
            num(&c["on_over_control"]["off"], "paired_median")
        ));
    }
    s.push_str("\nPrimary32 and original-eight aggregate must improve against both controls with at least4/7winning blocks. Every case must stay within2% paired median and5% in every block of both controls; OFF calibration must stay within a factor1.03. There is no minimum positive-gain floor or sample exclusion. Primary block0 has ON/OFF ratio1.055719315. Physical16 OFF calibration is1.059895196, and its block1 ON ratios are1.085776969/1.090784326. These failures remain alongside the two flat original-eight aggregate failures. The overall decision is not caused solely by physical16 variability.\n\n| Unprofiled resource observation | Accepted | OFF | ON |\n| --- | ---: | ---: | ---: |\n");
    for (name, label, key) in [
        ("1x2-w32", "Primary CPU ms/request", "cpu_ms_per_request"),
        ("1x2-w32", "Primary maximum process RSS MiB", "maxrss_mib"),
        (
            "merge-54x1-w32",
            "Merge54 CPU ms/request",
            "cpu_ms_per_request",
        ),
        (
            "merge-54x1-w32",
            "Merge54 maximum process RSS MiB",
            "maxrss_mib",
        ),
    ] {
        let m = &p["cases"][name]["metrics"];
        s.push_str(&format!(
            "| {label} | {:.3} | {:.3} | {:.3} |\n",
            num(&m["accepted"], key),
            num(&m["off"], key),
            num(&m["on"], key)
        ));
    }
    s.push_str("\nCPU entries are medians of per-process means. RSS includes key loading, setup and primes. Finite RSS observations are not minimum RAM, live scratch bounds or pressure qualification.\n\n## Profiles and linked code\n\nAfter all latency, nine warm999Hz user-cycle captures cover every variant at primary101,12x251 and merge549 measured requests after3primes. All301,520 samples have zero lost records and one process ID per capture. The closing audit independently checks all nine declared tuples and their round counts.\n\nPrimary sampled FFT share is4.69% OFF and5.04% ON; the targeted inverse stored-column wrapper is1.015% and1.213% of sampled user-cycle weight. G1/G2 arithmetic together remain about67% in both captures. These percentages do not measure critical-path time, removable latency, or the causal effect of a particular load or spill. One capture per tuple does not establish a CPU-work improvement.\n\n| Primary profile | Accepted | OFF | ON |\n| --- | ---: | ---: | ---: |\n");
    let primary = profile["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["case"] == "1x2-w32")
        .collect::<Vec<_>>();
    let by = |v: &str| *primary.iter().find(|r| r["variant"] == v).unwrap();
    s.push_str(&format!(
        "| CPU ms/request | {:.3} | {:.3} | {:.3} |\n",
        num(by("accepted"), "cpu_ms_per_request"),
        num(by("off"), "cpu_ms_per_request"),
        num(by("on"), "cpu_ms_per_request")
    ));
    s.push_str("\nThe real Euler path has one packed inverse and one packed forward transform, plus separate weighting and product passes. Only eligible outer inverse column tiles change; expanded leaves and the forward transform retain their original work. Thus the isolated32% tile result describes a much narrower workload than complete proving. It also lacks the full caller's packing, dispatch and other code.\n\n| Actual packed DIF wrapper | Bytes | Instructions | Static IFMA sites | RSP operand lines |\n| --- | ---: | ---: | ---: | ---: |\n");
    for r in code["symbols"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["symbol"].as_str().unwrap().ends_with(", false>"))
    {
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            r["build"].as_str().unwrap(),
            r["size_bytes"],
            r["instructions"],
            r["ifma_sites"],
            r["rsp_operand_lines"]
        ));
    }
    s.push_str("\nThe candidate wrapper retains OFF/ON and cached/uncached alternatives. Its400 static IFMA sites do not mean an executed tile doubles its multiplications. Literal prologue stack subtractions sum to17,024 bytes in both versions, excluding alignment, pushes and callees. The forward wrapper's static counts match. Code growth and stack-reference counts are observations, not proof that instruction-cache or register costs explain the result. Exact disassemblies are retained.\n\n## Validation and decision\n\nBoth native modes pass113 unfiltered core tests; the raw prover passes2tests and the cubic API6. Four new tests check12,800 actual column outputs against the existing four-lane reference,1,209,320 actual root lanes for the canonical bound,12 cached columns, full packed/legacy-quad transform agreement through262,144 fields, default/explicit/empty/twisted dispatch policy, and four concurrent callers with512KiB stacks. Repeated OFF/ON runs reuse the same mathematical cases. The component's33fieldtests and6independent integer/Fourier oracles are inherited only after exact field-byte identity is verified; no new field-suite run is claimed here.\n\nAll nine Euler shapes pass OFF/ON preflight at4workers. The complete campaign contains **9,192 benchmark SDK requests,237 distinct exports independently verified by Gnark, and474proof/public files**. Every exported proof has the164-byte compressed format, distinct hash, expected fixture public inputs, and no collision with4,084 prior proof identities. The2fixed source fixtures are separate. All336 performance pins pass the closing audit.\n\nThe native build, preflight and fixed measurement sequence finish without a source, arithmetic or timing repair. Additional uncached/preparation-denied SDK compatibility, portable, concurrent whole-SDK, finite-pressure and large-circuit qualification are not run after the failed speed gate. No AWS, Aeglos or mobile comparison is added.\n\nThe tested integration is preserved without adoption or an unchanged reroll. The component result remains valid for its own scope. The next research priority is the G1/G2 arithmetic that dominates primary CPU work, with a derivation and actual linked-code/resource audit before another full trial. This result does not establish a general memory bottleneck or justify another FFT parameter sweep.\n\n## Supplied M5 references\n\nCandidate diagnostics below do not replace the accepted prover. CPU/Metal numbers are the supplied historical references, not fresh Apple measurements.\n\n| Shape | ON ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for r in p["m5_supplied_references"].as_array().unwrap() {
        s.push_str(&format!(
            "| {} | {:.6} | {:.0} | {:.2}× | {:.0} | {:.2}× |\n",
            r["shape"].as_str().unwrap(),
            num(r, "on_ms"),
            num(r, "cpu_reference_ms"),
            num(r, "on_cpu_speedup"),
            num(r, "metal_reference_ms"),
            num(r, "on_metal_speedup")
        ));
    }
    s.push_str("\nNo merge54 M5 reference was supplied. The original all-eight10×CPU objective remains unmet. Accepted historical primary is8.594267ms and merge54 is178.891799ms in the [production checkpoint](../2026-10-25-grouping-module/README.md).\n\n[Protocol](artifacts/experiment.json), [full decision and failed checks](artifacts/pilot-result.json), [all samples](artifacts/perf/rows.json), [source audit](artifacts/source-audit.json), [actual linked code](artifacts/codegen.json), [profile observations](artifacts/profile-analysis.json), [proof inventory](artifacts/proof-inventory.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifacts/SHA256SUMS). Nine raw captures and nine expanded symbol streams remain hash-indexed locally and are omitted from Git.\n");
    fs::write(report.join("README.md"), s).unwrap();
    println!("Wrote completeSDK negative report with all6failures and fullCPU/Metalcontext");
}
