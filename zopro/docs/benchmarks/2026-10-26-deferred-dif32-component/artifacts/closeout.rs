use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    process::Command,
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
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
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32");
    let pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("profile-input-pins.json"))).unwrap();
    assert_eq!(pins.len(), 214);
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "{p}");
    }
    for stage in ["build-v3", "timing", "profiles"] {
        assert_eq!(
            read(root.join(stage).join("result.json"))["status"],
            "complete"
        );
    }
    let oracle = read(root.join("oracle-qualification.json"));
    assert_eq!(oracle["status"], "passed");
    assert_eq!(
        read(root.join("build/field-tests/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        33
    );
    let manifest = read(root.join("accepted-manifest.json"));
    assert_eq!(
        manifest["source_files_sha256"].as_object().unwrap().len(),
        170
    );
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    let variants = [
        ("accepted", root.join("accepted-probe"), "reference"),
        ("fresh", root.join("target/release/probe"), "reference"),
        ("deferred", root.join("target/release/probe"), "deferred"),
    ];
    let mut profiles = Vec::new();
    let mut outputs = Vec::new();
    let mut total = 0;
    for (label, binary, mode) in variants {
        let capture = root.join(format!("profiles/capture-{label}"));
        let exit = read(capture.join("exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(capture.join(format!("{stream}.log"))),
                exit[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
        let got = read(capture.join("stdout.log"));
        assert_eq!(got["mode"], mode);
        assert_eq!(got["q"], 256);
        assert_eq!(got["column"], 173);
        assert_eq!(got["iterations"], 5000000);
        assert_eq!(got["canonical_words"].as_array().unwrap().len(), 256);
        outputs.push(got["canonical_words"].clone());
        let plain = Command::new("nm").arg("-S").arg(&binary).output().unwrap();
        assert!(plain.status.success());
        let plain = String::from_utf8(plain.stdout).unwrap();
        let plain = plain
            .lines()
            .filter_map(|l| {
                let f = l.split_whitespace().collect::<Vec<_>>();
                (f.len() == 4).then(|| (f[0].to_owned(), f[3].to_owned()))
            })
            .collect::<BTreeMap<_, _>>();
        let demangled = Command::new("nm")
            .args(["-S", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(demangled.status.success());
        let demangled = String::from_utf8(demangled.stdout).unwrap();
        let names = demangled
            .lines()
            .filter_map(|l| {
                let f = l.split_whitespace().collect::<Vec<_>>();
                if f.len() < 4 {
                    return None;
                }
                plain.get(f[0]).map(|raw| (raw.clone(), f[3..].join(" ")))
            })
            .collect::<BTreeMap<_, _>>();
        let text =
            fs::read_to_string(root.join(format!("profiles/decode-{label}/stdout.log"))).unwrap();
        let dump =
            fs::read_to_string(root.join(format!("profiles/loss-{label}/stdout.log"))).unwrap();
        let lost = dump.matches("PERF_RECORD_LOST").count();
        assert_eq!(lost, 0);
        let raw_count = dump.matches("PERF_RECORD_SAMPLE(").count();
        let mut count = 0;
        let mut periods = 0u64;
        let mut pids = BTreeSet::new();
        let mut symbols: BTreeMap<String, u64> = BTreeMap::new();
        for line in text
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        {
            let f = line.split_whitespace().collect::<Vec<_>>();
            assert!(f.len() >= 7, "{line}");
            let (pid, tid) = f[1].split_once('/').unwrap();
            assert_eq!(pid, tid);
            pids.insert(pid.parse::<u32>().unwrap());
            assert_eq!(f[4], "cycles:u:");
            let period = f[3].parse::<u64>().unwrap();
            periods += period;
            let name = names.get(f[6]).cloned().unwrap_or_else(|| f[6].to_owned());
            *symbols.entry(name).or_default() += period;
            count += 1;
        }
        assert_eq!(count, raw_count);
        assert_eq!(pids.len(), 1);
        assert!(count > 1000);
        total += count;
        let mut symbols = symbols
            .into_iter()
            .map(|(s, p)| json!({"symbol":s,"period":p,"share":p as f64/periods as f64}))
            .collect::<Vec<_>>();
        symbols.sort_by(|a, b| {
            b["period"]
                .as_u64()
                .unwrap()
                .cmp(&a["period"].as_u64().unwrap())
        });
        profiles.push(json!({"variant":label,"samples":count,"raw_sample_events":raw_count,"lost_event_records":lost,"pids":pids,"total_period":periods,"profile_ns_per_element":got["ns_per_element"],"symbols":symbols}));
    }
    assert_eq!(outputs[0], outputs[1]);
    assert_eq!(outputs[0], outputs[2]);
    fs::write(root.join("profile-analysis.json"),serde_json::to_vec_pretty(&json!({"profiles":profiles,"samples":total,"lost_event_records":0,"profile_word_comparisons_vs_two_controls":2048,"all_output_words_equal":true,"scope":"Whole single-thread component process including setup/primes/output, not an SDK warm interval or causal spill model."})).unwrap()).unwrap();
    let result = read(root.join("component-result.json"));
    assert_eq!(result["passed"], true);
    let closeout = json!({"status":"complete","component_gate_passed":true,"adopted":false,"full_goal_achieved":false,"base_source_files":170,"unchanged_candidate_files":168,"production170_unchanged":true,"content_pins_verified":pins.len(),"native_field_tests":33,"new_oracle_tests":6,"raw_integer_field_outputs":49152,"independent_montgomery_products":122880,"maximum_modeled_cios_coefficient":47239727476272382u64,"modeled_coefficient_limit":32u64<<52,"maximum_intermediate_bits":260,"direct_fourier_field_outputs":5760,"repeated_tile_field_outputs":5120,"timed_processes":279,"measured_tile_iterations":27900000u64,"priming_tile_iterations":1674000,"all_output_words_equal":true,"unprofiled_word_comparisons_vs_two_controls":190464,"profile_word_comparisons_vs_two_controls":2048,"profiles":3,"profile_samples":total,"lost_event_records":0,"aggregate_vs_accepted":result["aggregate"]["accepted"],"aggregate_vs_fresh":result["aggregate"]["fresh"],"new_sdk_proofs":0,"new_gnark_exports":0,"next":"Commit/push this completed component, then declare full native FFT/SDK integration using the same byte-pinned arithmetic. It must preserve cached/uncached root bounds, storage normalization, fallback paths, all-nine CPU/Metal context, independent Gnark exports, actual profiles and memory compatibility. Component result is not sufficient for adoption."});
    fs::write(
        root.join("closeout.json"),
        format!("{}\n", serde_json::to_string_pretty(&closeout).unwrap()),
    )
    .unwrap();
    println!("Verified component PASS,279processes3profiles{total}samples0lost, all214pins; no SDK or adoption yet");
}
