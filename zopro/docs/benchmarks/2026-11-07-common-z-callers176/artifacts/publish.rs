use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/common-z-callers176";
const OUT: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-07-common-z-callers176";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn copy(a: impl AsRef<Path>, b: impl AsRef<Path>) {
    fs::create_dir_all(b.as_ref().parent().unwrap()).unwrap();
    fs::copy(a, b).unwrap();
}
fn gzip(from: &Path, to: &Path) {
    let output = Command::new("gzip")
        .args(["-n", "-c"])
        .arg(from)
        .output()
        .unwrap();
    assert!(output.status.success());
    fs::write(to, output.stdout).unwrap();
    let round = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(to)
        .output()
        .unwrap();
    assert!(round.status.success());
    assert_eq!(round.stdout, fs::read(from).unwrap());
}
fn text(v: &Value) -> String {
    format!(
        "{:.9} [{:.9}, {:.9}], {}/15",
        v["median"].as_f64().unwrap(),
        v["low"].as_f64().unwrap(),
        v["high"].as_f64().unwrap(),
        v["wins"]
    )
}
fn main() {
    let a = read(r("analysis.json"));
    let review = read(r("independent-review.json"));
    assert_eq!(a["gate_passed"], false);
    assert_eq!(review["passed"], true);
    assert_eq!(review["statistical_checks_reconstructed"], 243);
    assert!(!Path::new(OUT).exists());
    let art = Path::new(OUT).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let files = [
        "analysis.json",
        "declaration.json",
        "design.md",
        "source-audit-after-test-repair.json",
        "source-audit.json",
        "identities.json",
        "build-review.json",
        "independent-review.json",
        "profile-symbols.json",
        "capture-pins.json",
        "capture-plan.json",
        "build-pins.json",
        "build-plan.json",
        "continuation-pins.json",
        "continuation-plan.json",
        "review-pins.json",
        "review-plan.json",
        "review2-pins.json",
        "review2-plan.json",
        "host-before.json",
        "setup.rs",
        "repair.rs",
        "finish.rs",
        "finish-before-test-assert-repair.rs",
        "segmented.rs",
        "driver.rs",
        "perf_support.rs",
        "campaign.rs",
        "analysis.rs",
        "review.rs",
        "review2.rs",
        "publish.rs",
    ];
    for file in files {
        copy(r(file), art.join(file));
    }
    for file in ["observations.json", "receipts.json"] {
        gzip(&r(file), &art.join(format!("{file}.gz")));
    }
    let mut raw = BTreeMap::new();
    for (out, plan, limit) in [
        ("capture", "capture-plan.json", None),
        ("build-runs", "build-plan.json", Some(3)),
        ("continuation-runs", "continuation-plan.json", None),
        ("review-runs", "review-plan.json", None),
        ("review2-runs", "review2-plan.json", None),
    ] {
        let plan = read(r(plan));
        for step in plan["steps"]
            .as_array()
            .unwrap()
            .iter()
            .take(limit.unwrap_or(usize::MAX))
        {
            let name = step["name"].as_str().unwrap();
            let dir = r(&format!("{out}/{name}"));
            let cmd = read(dir.join("command.json"));
            let exit = read(dir.join("exit.json"));
            let mut streams = BTreeMap::new();
            for name in ["stdout", "stderr"] {
                let p = dir.join(format!("{name}.log"));
                assert_eq!(hash(&p), exit[format!("{name}_sha256")].as_str().unwrap());
                streams.insert(name, fs::read_to_string(p).unwrap());
            }
            raw.insert(
                format!("{out}/{name}"),
                json!({"command":cmd,"exit":exit,"streams":streams}),
            );
        }
        let p = r(&format!("{out}/result.json"));
        if p.exists() {
            copy(&p, art.join(format!("{out}-result.json")));
        }
    }
    assert_eq!(raw.len(), 1959);
    let p = r("publication-raw-streams.json");
    assert!(!p.exists());
    fs::write(&p, serde_json::to_vec(&raw).unwrap()).unwrap();
    gzip(&p, &art.join("raw-streams.json.gz"));
    let mut local = BTreeMap::new();
    for arm in ["frozen", "fresh", "candidate"] {
        let ids = read(r("identities.json"));
        for kind in ["binary", "core"] {
            let p = ids[arm][kind].as_str().unwrap();
            local.insert(p.to_string(), hash(p));
        }
    }
    for name in a["profiles"].as_object().unwrap().keys() {
        let p = r(&format!("perf/{name}.data"));
        local.insert(p.display().to_string(), hash(&p));
    }
    for p in [
        "core-tests",
        "campaign",
        "review",
        "review2",
        "setup",
        "repair",
    ] {
        let path = r(p);
        local.insert(path.display().to_string(), hash(&path));
    }
    fs::write(
        art.join("local-artifacts.json"),
        serde_json::to_vec_pretty(&local).unwrap(),
    )
    .unwrap();
    // Complete public report, with all fixed contexts and failed checks retained.
    let mut report=String::from("# Segmented common-Z G1 reducer callers\n\n**Not adopted.** The fixed integration fails four control-calibration checks and six nonregression checks. Calibrated small-frontier gains remain useful: sixteen-point reductions are about twice as fast, and257–511-point callers gain roughly6–9% at32workers. These are reducer waves; accepted full-proof latency is unchanged.\n\n");
    writeln!(report,"The declared equal-log all-ten-case32-worker aggregate ON/control ratios are:\n\n|Control|Paired ratio [bounds], winning blocks|\n|---|---|").unwrap();
    for control in ["frozen", "fresh", "OFF"] {
        writeln!(
            report,
            "|{control}|{}|",
            text(&a["all10cases32_aggregate"][control])
        )
        .unwrap();
    }
    report.push_str("\nAll three aggregate gain checks pass, with15/15 winning blocks. That equal-case aggregate is not weighted by production proof work and does not override failed calibration/nonregression checks. All thirty same-binary ON/OFF nonregression checks pass. Six ON/original-or-fresh upper bounds exceed the declared1.03 limit in three contexts; those contexts also have control-calibration failures. No SDK trial, favorable-subset adoption, threshold/layout/worker/compiler rescue, extension or unchanged reroll follows this fixed implementation.\n\n## Algorithm and integration\n\n");
    report.push_str("The [parent mathematical screen](../2026-11-07-common-z-product-tree176/README.md) and [standalone IFMA prototype](../2026-11-07-common-z-small-native176/README.md) remain immutable. The new segmented implementation finishes all live groups together when their entire task frontier fits sixteen points and some group has at least three points. Those rules come from one eight-lane pair vector and at least two remaining reduction levels, before timing.\n\nFor each pair, d=dx and e=dy. The shared denominator product D and all-other cofactors q produce u=e*q, X'=u²−(X1+X2)D², Y'=u(X1D²−X')−Y1D³, Z'=ZD. Odd carried and singleton-group points rescale with D²/D³. All final groups share one scalar inverse for normalization. Equal-x/identity cases at any level decline transactionally; the original reducer continues from its untouched frontier. No partial output escapes.\n\nThe hooks sit in group_affine_impl and indexed-tile unweighted loops, with the same hook in the diagnostic reducer. reduce_layer and weighted_sum retain their original one-level contract. G2 retains its original arithmetic. The runtime option is read once in Scratch::default. The kernel adds no heap scratch, field implementation, unsafe code, key table/copy, retained plan or production pool. Its fixed stack arrays and compiler spills are additional to the observed heap counters.\n\n## Qualification and provenance\n\nAll131 changed native core tests pass, including segmented compositions through sixteen points, odd/carry/zero/singleton/mixed groups, discontiguous slots, unchanged inputs, late equal/opposite/identity fallback, and complete production grouping/indexed outputs on1/4/16/32-worker pools requesting512KiB stacks. Parent field and standalone suites are inherited rather than rerun. The unchanged field library is1200e635… .\n\nThe private snapshot has180files:178 diagnostic originals recover exactly after reversing eight recorded edits, with two new child files. The earlier diagnostic snapshot retains all176 accepted originals, including174 byte-exact original files and two append-only diagnostic parents. Successful runtime libraries are built once. A failed test-only compile required replacing assert_eq! with assert! for private limbs lacking Debug; runtime source before cfg(test) remains exact and both successful libraries are reused. The failure and repair remain archived.\n\nThe reused original diagnostic library, a fresh cfg-disabled library, and one candidate library supply four arms: original/frozen, fresh, candidate OFF and the same candidate ON. Actual consumers use O3, native CPU, fat LTO and one codegen unit. The qualified TLS allocation/perf harness changes only the three added cases and fresh label. Seven older fixture output digests are inherited and matched; unchanged harness qualification is reused.\n\n## Fixed measurements\n\nRyzen9950X,16physical/32logical CPUs, shared host. Workers pin individually to CPU2, CPUs0–15 or0–31, with512KiB stacks. Each worker owns a fixture and reducer. Three checked eight-wave primes precede the timed loop. Timers include dispatch/wait, gathers/conversions, ordinary reduction, common-Z finish, final normalization and the case's normal allocation/drop. Pool/fixture construction and final output checks are outside timing. These are complete reducer caller waves, not SDK requests or concurrent-proof throughput.\n\nFifteen fixed rotated/reversed blocks compare all ten cases at all three worker counts. Calibration requires fresh/original and OFF/original intervals inside[1/1.03,1.03]; ON/control nonregression requires upper<=1.03 in every context. Three all-case32-worker gains require upper<1 and at least11wins. Observed heap capacity must not grow>5% against any control. There is no gain floor, excluded block, extension, parameter sweep or reroll. Ranks4..12of15 give nominal96.484375% marginal coverage under independent exchangeable blocks, with no familywise guarantee.\n\nCapture completes all1,949commands on their first executions:120preflights, one preflight review,1,800latency processes, four counter processes, four profile processes and twenty offline commands. Every one of1,928caller processes checks its complete output; intermediate timed iterations are not each separately oracled.\n\nTimes below are microseconds per wave. Paired ratios need not equal ratios of marginal medians.\n\n|Context|Original / fresh / OFF / ON µs|ON/OFF ratio [bounds], wins|\n|---|---:|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let name = row["context"].as_str().unwrap();
        let check = a["checks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["name"] == format!("{name}/ON/OFF/nonregression"))
            .unwrap();
        let arms = &row["arms"];
        writeln!(
            report,
            "|{name}|{:.6} / {:.6} / {:.6} / {:.6}|{}|",
            arms["frozen"]["median_us"].as_f64().unwrap(),
            arms["fresh"]["median_us"].as_f64().unwrap(),
            arms["OFF"]["median_us"].as_f64().unwrap(),
            arms["ON"]["median_us"].as_f64().unwrap(),
            text(&check["statistics"])
        )
        .unwrap();
    }
    report.push_str("\n### Failed checks\n\n");
    for name in a["failed_checks"].as_array().unwrap() {
        writeln!(report, "- `{}`", name.as_str().unwrap()).unwrap();
    }
    report.push_str("\nThe32-worker tile2048 control interval is too wide for fresh/original and systematically above the declared band for OFF/original. OFF/original also fails at group35973/16workers and tails/32workers. The corresponding ON/original-or-fresh comparisons retain their actual unfavorable bounds; no universal nonregression or whole-proof gain is claimed.\n\n## CPU profiles and memory\n\nAll four PMU sets run100% of their measurement windows. All four warm-gated profiles pass:115,598samples, zero lost records, worker sample share100%, maximum unknown share1.7866%. They use cycles:u at a fixed4,000,000period,512pages, and FIFO enable after checked primes / disable before final oracles. Independent review reconstructs sample PID/TID/CPU/period headers. It does not independently reparse exact ELF/IP ownership or LOST records.\n\n|Counter context, ON/original|Cycles|Instructions|Cache misses|\n|---|---:|---:|---:|\n");
    for case in ["tile511", "groups3x4"] {
        let on = &a["counters"][format!("stat-{case}-w32-ON")];
        let original = &a["counters"][format!("stat-{case}-w32-frozen")];
        let change = |key: &str| {
            100. * (on[key]["value"].as_f64().unwrap() / original[key]["value"].as_f64().unwrap()
                - 1.)
        };
        writeln!(
            report,
            "|{case}-w32|{:+.2}%|{:+.2}%|{:+.2}%|",
            change("cycles:u"),
            change("instructions:u"),
            change("cache-misses:u")
        )
        .unwrap();
    }
    report.push_str("\nThe shorter small-group arithmetic and reduced inverse-bridge samples support the intended mechanism. Flat samples and generic cache counters do not establish removable wall fractions or a causal cache/DRAM explanation. Tile511 cache misses increase even while its measured cycles decrease.\n\nRequested worker heap capacity is unchanged for the seven inherited cases. At small16 it falls3,640→1,976bytes, groups3x4 falls1,752→1,720, and small-mixed falls1,888→1,856. Scratch::default's diagnostic option read adds one worker allocation in fresh-buffer cases; that cost is timed. Retained-buffer cases have zero timed worker allocations. TLS counters exclude main/coordinator allocations.\n\nAt32workers the largest synthetic group has355MiB whole-process high-water RSS in both original and ON; tile511 has3MiB in both. These counters include fixture/setup lifetimes and do not establish a rigorous incremental peak, prover RAM, admission bound or minimum RAM. Small fixed stack arrays and spills are outside heap capacity. No pressure, portable/mobile, huge-circuit or concurrent-proof qualification is added.\n\n## Review, decision and accepted comparison\n\nOffline review independently reconstructs all243timing/capacity checks, the same ten failures, all1,928output/worker receipts,1,949command/stream receipts, source reversals and all115,598sample headers. An initial reader used exact floating equality on a JSON round-trip and stopped at a one-ULP difference; the corrected separate reader permits1e-14 for reconstructed floating summaries, with unchanged thresholds and gate decisions. Both versions and failed receipts are retained. Neither analysis nor successful arithmetic, capture or decode is rerun.\n\nThe tested fixed integration is closed without adoption or an SDK trial. Calibrated small-frontier gains remain established component evidence. A future mathematical construction must justify new work rather than rescue this fixed gate through a worker/layout/compiler change.\n\nThe following latest accepted local32-worker full-proof observations and supplied historical M5 references are inherited, not this experiment's timings. [Accepted source manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json), [full-proof evidence](../2026-11-06-window-batch-proof-capture176/README.md).\n\n|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU/local|Metal/local|CPU10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (name, t, cpu, metal) in [
        ("1×2", 7.915095, 37., 24.),
        ("2×2", 11.169812, 59., 30.),
        ("4×2", 16.085882, 109., 48.),
        ("8×2", 24.517520, 189., 74.),
        ("12×2", 33.838626, 284., 101.),
        ("16×2", 40.083672, 335., 124.),
        ("24×2", 65.506198, 528., 176.),
        ("merge24", 65.200982, 507., 175.),
    ] {
        writeln!(
            report,
            "|{name}|{t:.6}|{cpu:.0}|{metal:.0}|{:.3}×|{:.3}×|{:.1}|",
            cpu / t,
            metal / t,
            cpu / 10.
        )
        .unwrap();
    }
    report.push_str("\nAccepted merge54 is161.718956ms, without a supplied M5 reference. All-eight CPU10× goal remains active and unmet,0/8. No new SDK request, proof, Gnark export or AWS/Aeglos result is produced; the canonical distinct-proof ledger stays31,387. Production source176 and SDK6d83… remain unchanged.\n\n[Design](artifacts/design.md), [complete results](artifacts/analysis.json), [independent review](artifacts/independent-review.json), [lossless observations](artifacts/observations.json.gz), [lossless command receipts](artifacts/receipts.json.gz), [exact raw streams including failed stages](artifacts/raw-streams.json.gz), [source audit and inverse edits](artifacts/source-audit-after-test-repair.json), [capture plan](artifacts/capture-plan.json), [capture pins](artifacts/capture-pins.json), [local binary/raw-profile identities](artifacts/local-artifacts.json), [checksums](SHA256SUMS). Public artifacts contain no actual key, witness, proof, native executable or library payload.\n");
    fs::write(Path::new(OUT).join("README.md"), report).unwrap();
    let mut publication = json!({"passed":true,"raw_command_stream_receipts":raw.len(),"gzip_roundtrips_exact":true,"native_payloads_published":false,"keys_witnesses_or_proofs_published":false,"goal_achieved":false});
    let mut members = Vec::new();
    fn walk(base: &Path, p: &Path, out: &mut Vec<String>) {
        for e in fs::read_dir(p).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(base, &p, out);
            } else {
                out.push(p.strip_prefix(base).unwrap().display().to_string());
            }
        }
    }
    walk(Path::new(OUT), Path::new(OUT), &mut members);
    members.sort();
    publication["members_before_publication_json"] = json!(members);
    fs::write(
        art.join("publication.json"),
        serde_json::to_vec_pretty(&publication).unwrap(),
    )
    .unwrap();
    members.push("artifacts/publication.json".into());
    members.sort();
    let sums = members
        .iter()
        .map(|p| format!("{}  {p}\n", hash(Path::new(OUT).join(p))))
        .collect::<String>();
    fs::write(Path::new(OUT).join("SHA256SUMS"), sums).unwrap();
    fs::write(
        r("publication.json"),
        serde_json::to_vec_pretty(&publication).unwrap(),
    )
    .unwrap();
    println!("Published{}files plusSHA256SUMS;1959exact command streams;3gzip roundtripsPASS; no native/key/witness/proof payload",members.len());
}
