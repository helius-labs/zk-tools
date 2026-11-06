use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-aos-gather176";
const P: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g2-aos-gather176";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn h(b: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(b.as_ref()))
}
fn hash(p: impl AsRef<Path>) -> String {
    h(fs::read(p).unwrap())
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        assert!(!e.file_type().unwrap().is_symlink());
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else {
            out.push(e.path());
        }
    }
}
fn gz(source: &Path, destination: &Path) {
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(source)
        .stdout(Stdio::from(fs::File::create(destination).unwrap()))
        .status()
        .unwrap()
        .success());
    let out = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(destination)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, fs::read(source).unwrap());
}
fn main() {
    assert!(!Path::new(P).exists());
    let review = read(r("review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(review["workflow_commands"], 323);
    assert_eq!(review["gate_passed"], false);
    for (p, sha) in read(r("capture-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), sha.as_str().unwrap());
    }
    let a = read(r("analysis.json"));
    let mut report = fs::read_to_string(r("report-head.md")).unwrap();
    report.push_str("| Root / workers | Original ms | Fresh OFF ms | Gather ms | ON/OFF ratio [bounds] | Wins/15 |\n|---|---:|---:|---:|---|---:|\n");
    for shape in ["1x2", "2x2"] {
        for w in [1, 16, 32] {
            let c = format!("{shape}-w{w}");
            let med = |arm: &str| {
                a["medians_ms_per_full_cycle_wave"][format!("{c}/{arm}")]
                    .as_f64()
                    .unwrap()
            };
            let q = &a["comparisons"][format!("{c}/on/off")]["paired"];
            report.push_str(&format!(
                "|{shape} / {w}|{:.6}|{:.6}|{:.6}|{:.6} [{:.6}, {:.6}]|{}|\n",
                med("frozen"),
                med("off"),
                med("on"),
                q["median"].as_f64().unwrap(),
                q["low"].as_f64().unwrap(),
                q["high"].as_f64().unwrap(),
                q["wins"]
            ));
        }
    }
    report.push_str("\nThe predeclared gain requires upper<1 and at least 10/15 wins against both controls for the all-six and two-size 32-worker aggregates. Every ON/control upper must be at most 1.03; every OFF/original interval must fit factor 1.03. Bounds are sorted ranks 4–12/15, nominal 96.484375% marginal coverage conditional on exchangeable blocks, without familywise coverage. No gain floor, exclusions, extensions, changed thresholds, favorable subset or unchanged retry.\n\n| Aggregate / control | Median ratio | Lower / upper | Wins/15 |\n|---|---:|---:|---:|\n");
    for group in ["all6", "logical32_2"] {
        for d in ["frozen", "off"] {
            let q = &a["aggregates"][format!("{group}/on/{d}")]["paired"];
            report.push_str(&format!(
                "|{group} / {d}|{:.6}|{:.6} / {:.6}|{}|\n",
                q["median"].as_f64().unwrap(),
                q["low"].as_f64().unwrap(),
                q["high"].as_f64().unwrap(),
                q["wins"]
            ));
        }
    }
    report.push_str("\n## Generated code and profiles\n\nThe linked common cycle has 815/815/775 instruction sites in original/OFF/ON, with 300 IFMA sites in every arm and 0/0/16 hardware-gather sites. Selected bytes are 4,932/4,932/4,845. These are static counts; fewer sites do not establish cheaper execution.\n\nThe separate counter panels show about 2.63% fewer retired instructions but more cycles. All-worker counts below are per complete cycle wave; they are single diagnostic observations, not paired latency bounds or a causal decomposition.\n\n| Workers | OFF / ON instructions, million | OFF / ON cycles, million | Cycle change |\n|---|---:|---:|---:|\n");
    for w in [16, 32] {
        let get = |arm: &str, event: &str| {
            let name = format!("stat-1x2-w{w}-{arm}");
            let p = a["counters"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["job"]["name"] == name)
                .unwrap();
            p["events"][event]["count"].as_f64().unwrap() / 8.
        };
        let off = get("off", "cycles:u");
        let on = get("on", "cycles:u");
        report.push_str(&format!(
            "|{w}|{:.3} / {:.3}|{:.3} / {:.3}|+{:.2}%|\n",
            get("off", "instructions:u") / 1e6,
            get("on", "instructions:u") / 1e6,
            off / 1e6,
            on / 1e6,
            100. * (on / off - 1.)
        ));
    }
    report.push_str(&fs::read_to_string(r("report-tail.md")).unwrap());
    let old = fs::read_to_string(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-simdmsm-source176/README.md",
    )
    .unwrap();
    let t = old.find("|Shape|Zopro ms|").unwrap();
    let end = old[t..].find("\n\n").unwrap() + t;
    report.push_str(&old[t..end]);
    report.push_str("\n\nMerge54 remains 161.883899 ms without an M5 reference. Primary RSS 132.75 MiB is inherited. Accepted source176/SDK6d83 and the 30,846-proof ledger remain unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.\n\n[Full analysis](artifacts/analysis.json.gz), [independent review](artifacts/review.json), [loader](artifacts/aos_gather.rs), [driver](artifacts/driver.rs), [design](artifacts/design.md), [code counts](artifacts/codegen.json), [source recovery](artifacts/source-audit.json), [exact sources/protocols/command receipts/decoded profiles](artifacts/source-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Four lossless raw profile archives accompany the receipts. Executables, libraries, raw point/index roots, keys, witnesses and proof inputs stay private.\n");
    for (a, b) in [
        ("512KiB", "512 KiB"),
        ("CPU2", "CPU 2"),
        ("CPUs0", "CPUs 0"),
        ("runs100%", "runs 100%"),
        ("with50,774samples", "with 50,774 samples"),
        ("below0.079%", "below 0.079%"),
        ("pass270latency", "pass 270 latency"),
        ("before270latency", "before 270 latency"),
        ("separate fixed16M", "separate fixed 16M"),
        ("comparing317,184component", "comparing 317,184 component"),
        (
            "boundaries,8,192deterministic",
            "boundaries, 8,192 deterministic",
        ),
        ("original35field", "original 35 field"),
        ("all176production", "all 176 production"),
        ("private177-file", "private 177-file"),
        ("has175byte-exact", "has 175 byte-exact"),
        (
            "all64,541primary/66,884grouped",
            "all 64,541 primary / 66,884 grouped",
        ),
        (
            "contain30,089/28,852points",
            "contain 30,089 / 28,852 points",
        ),
        ("All397prior", "All 397 prior"),
        (
            "and3,851,392/3,693,056payload",
            "and 3,851,392 / 3,693,056 payload",
        ),
        (
            "logical4,130,624/4,280,576bytes",
            "logical 4,130,624 / 4,280,576 bytes",
        ),
        (
            "capacities4,194,304/8,388,608bytes",
            "capacities 4,194,304 / 8,388,608 bytes",
        ),
        ("passes782unique", "passes 782 unique"),
        ("/1,909pin", " / 1,909 pin"),
        ("occurrences,323successful", "occurrences, 323 successful"),
        ("commands,296caller", "commands, 296 caller"),
        ("all22paired", "all 22 paired"),
        ("all16failed", "all 16 failed"),
        (
            "totals3,973,383,579measured",
            "totals 3,973,383,579 measured",
        ),
        ("and964,510,416priming", "and 964,510,416 priming"),
        ("local32-worker", "local 32-worker"),
        ("all15blocks", "all 15 blocks"),
    ] {
        report = report.replace(a, b);
    }
    let artifacts = Path::new(P).join("artifacts");
    fs::create_dir_all(&artifacts).unwrap();
    fs::write(Path::new(P).join("README.md"), report).unwrap();
    let mut files = vec![];
    for e in fs::read_dir(R).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_file() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.contains("session") || name.starts_with("checkpoint") {
                continue;
            }
            let ext = e
                .path()
                .extension()
                .map(|x| x.to_string_lossy().to_string());
            if matches!(
                ext.as_deref(),
                Some("rs" | "json" | "md" | "txt" | "log" | "stdout" | "stderr")
            ) {
                files.push(e.path());
            }
        }
    }
    for dir in ["native", "link-runs", "capture", "codegen"] {
        walk(&r(dir), &mut files);
    }
    files.sort();
    let mut archive = BTreeMap::new();
    for p in files {
        let b = fs::read(&p).unwrap();
        let text = String::from_utf8(b.clone()).unwrap();
        assert_eq!(text.as_bytes(), b);
        archive.insert(
            p.strip_prefix(R).unwrap().to_string_lossy().to_string(),
            json!({"sha256":h(&b),"bytes":b.len(),"utf8":text}),
        );
    }
    let bytes = serde_json::to_vec(&archive).unwrap();
    for v in archive.values() {
        let s = v["utf8"].as_str().unwrap();
        assert_eq!(s.len(), v["bytes"].as_u64().unwrap() as usize);
        assert_eq!(h(s), v["sha256"]);
    }
    let temp = r("archive-publication.tmp");
    fs::write(&temp, &bytes).unwrap();
    gz(&temp, &artifacts.join("source-and-receipts.json.gz"));
    fs::remove_file(temp).unwrap();
    gz(&r("analysis.json"), &artifacts.join("analysis.json.gz"));
    for f in [
        "aos_gather.rs",
        "driver.rs",
        "design.md",
        "review.rs",
        "review.json",
        "source-audit.json",
        "publish.rs",
        "codegen.json",
    ] {
        let b = fs::read(r(f)).unwrap();
        fs::write(artifacts.join(f), &b).unwrap();
        assert_eq!(fs::read(artifacts.join(f)).unwrap(), b);
    }
    let mut profiles = vec![];
    for entry in fs::read_dir(r("perf")).unwrap() {
        let p = entry.unwrap().path();
        assert_eq!(p.extension().unwrap(), "data");
        let name = format!("{}.gz", p.file_name().unwrap().to_string_lossy());
        gz(&p, &artifacts.join(&name));
        profiles.push(
            json!({"name":name,"raw_sha256":hash(&p),"raw_bytes":fs::metadata(p).unwrap().len()}),
        );
    }
    assert_eq!(profiles.len(), 4);
    let pubinfo = json!({"UTF8_archive_members":archive.len(),"all_source_receipt_hashes_and_roundtrips_verified":true,"source_receipt_archive_uncompressed_sha256":h(&bytes),"analysis_uncompressed_sha256":hash(r("analysis.json")),"raw_profiles":profiles,"raw_profile_roundtrips_verified":true,"runtime_source_changed":false,"private_roots_keys_witnesses_binaries_libraries_excluded":true,"new_proofs":0,"goal_status":"active/unmet0/8"});
    fs::write(
        artifacts.join("publication.json"),
        serde_json::to_vec_pretty(&pubinfo).unwrap(),
    )
    .unwrap();
    let mut all = vec![];
    walk(Path::new(P), &mut all);
    all.sort();
    let mut sums = String::new();
    for p in &all {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(p),
            p.strip_prefix(P).unwrap().display()
        ));
    }
    fs::write(Path::new(P).join("SHA256SUMS"), sums).unwrap();
    println!("Published {}files/{}exactUTF8members/4losslessprofiles; all roundtrips+checksums prepared; negative trial no adoption",all.len()+1,archive.len());
}
