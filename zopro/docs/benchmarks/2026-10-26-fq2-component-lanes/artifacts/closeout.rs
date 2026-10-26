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
    let root = Path::new("/tmp/zopro-fq2-component-lanes");
    let pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("profile-input-pins-v2.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "{p}");
    }
    for part in ["build-v2", "timing", "profiles-v2"] {
        assert_eq!(
            read(root.join(part).join("result.json"))["status"],
            "complete"
        );
    }
    let manifest = read(root.join("accepted-manifest.json"));
    let accepted = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(accepted.len(), 170);
    for (p, h) in accepted {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
        assert_eq!(
            hash(root.join("accepted-source").join(p)),
            h.as_str().unwrap()
        );
    }
    let nm = Command::new("nm")
        .args(["-S", "-C"])
        .arg(root.join("probe"))
        .output()
        .unwrap();
    assert!(nm.status.success());
    let nm = String::from_utf8(nm.stdout).unwrap();
    let raw = Command::new("nm")
        .args(["-S"])
        .arg(root.join("probe"))
        .output()
        .unwrap();
    assert!(raw.status.success());
    let raw = String::from_utf8(raw.stdout).unwrap();
    let raw_names = raw
        .lines()
        .filter_map(|l| {
            let f = l.split_whitespace().collect::<Vec<_>>();
            (f.len() == 4).then(|| (f[0].to_owned(), f[3].to_owned()))
        })
        .collect::<BTreeMap<_, _>>();
    let mut demangle = BTreeMap::new();
    let mut helper_assembly = String::new();
    let mut helpers = Vec::new();
    for line in nm.lines() {
        let f = line.split_whitespace().collect::<Vec<_>>();
        if f.len() < 4 {
            continue;
        }
        let symbol = f[3..].join(" ");
        if let Some(raw) = raw_names.get(f[0]) {
            demangle.insert(raw.clone(), symbol.clone());
        }
        if symbol.contains("zopro_bn254::ifma") {
            let start = u64::from_str_radix(f[0], 16).unwrap();
            let size = u64::from_str_radix(f[1], 16).unwrap();
            let out = Command::new("objdump")
                .args(["-d", "-C"])
                .arg(format!("--start-address=0x{start:x}"))
                .arg(format!("--stop-address=0x{:x}", start + size))
                .arg(root.join("probe"))
                .output()
                .unwrap();
            assert!(out.status.success());
            let body = String::from_utf8(out.stdout).unwrap();
            let instructions = body
                .lines()
                .filter(|l| {
                    l.split_once(':').is_some_and(|(a, b)| {
                        !a.trim().is_empty()
                            && a.trim().chars().all(|c| c.is_ascii_hexdigit())
                            && b.contains('\t')
                    })
                })
                .collect::<Vec<_>>();
            helpers.push(json!({"symbol":symbol,"size_bytes":size,"instructions":instructions.len(),"ifma_sites":instructions.iter().filter(|l|l.contains("vpmadd52")).count(),"rsp_operand_lines":instructions.iter().filter(|l|l.contains("(%rsp)")).count()}));
            helper_assembly.push_str(&format!("\n{symbol}\n{body}"));
        }
    }
    assert_eq!(helpers.len(), 2);
    fs::write(root.join("helper-codegen.txt"), helper_assembly).unwrap();
    fs::write(root.join("helper-codegen.json"),serde_json::to_vec_pretty(&json!({"helpers":helpers,"scope":"Both actual direct arithmetic array helpers outlined from the outer functions. Outer IFMA/size counts omit these callees and must not be interpreted as reduced total arithmetic."})).unwrap()).unwrap();
    let mut profiles = Vec::new();
    let mut profiled_words = Vec::new();
    let mut total_samples = 0;
    for mode in ["reference", "interleaved"] {
        let capture = if mode == "reference" {
            root.join("profiles/capture-reference")
        } else {
            root.join("profiles-v2/capture-interleaved")
        };
        let exit = read(capture.join("exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(capture.join(format!("{stream}.log"))),
                exit[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
        let output = read(capture.join("stdout.log"));
        assert_eq!(output["mode"], mode);
        assert_eq!(output["pattern"], "affine-expression");
        assert_eq!(output["iterations"], 20000000);
        profiled_words.push(output["canonical_words"].clone());
        let text =
            fs::read_to_string(root.join(format!("profiles-v2/decode-{mode}/stdout.log"))).unwrap();
        let raw =
            fs::read_to_string(root.join(format!("profiles-v2/loss-{mode}/stdout.log"))).unwrap();
        let lost = raw.matches("PERF_RECORD_LOST").count();
        assert_eq!(lost, 0);
        let raw_samples = raw.matches("PERF_RECORD_SAMPLE(").count();
        let mut samples = 0;
        let mut periods = 0u64;
        let mut pids = BTreeSet::new();
        let mut symbols: BTreeMap<String, u64> = BTreeMap::new();
        let mut comms = BTreeSet::new();
        for line in text
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            assert!(fields.len() >= 7, "{line}");
            let (pid, tid) = fields[1].split_once('/').unwrap();
            pids.insert(pid.parse::<u32>().unwrap());
            assert_eq!(pid, tid);
            comms.insert(fields[0].to_owned());
            let period = fields[3].parse::<u64>().unwrap();
            assert_eq!(fields[4], "cycles:u:");
            periods = periods.checked_add(period).unwrap();
            let symbol = demangle
                .get(fields[6])
                .cloned()
                .unwrap_or_else(|| fields[6].to_owned());
            *symbols.entry(symbol).or_default() += period;
            samples += 1;
        }
        assert_eq!(samples, raw_samples);
        assert!(samples > 1000);
        assert_eq!(pids.len(), 1);
        total_samples += samples;
        let mut symbols=symbols.into_iter().map(|(symbol,period)|json!({"symbol":symbol,"period":period,"share":period as f64/periods as f64})).collect::<Vec<_>>();
        symbols.sort_by(|a, b| {
            b["period"]
                .as_u64()
                .unwrap()
                .cmp(&a["period"].as_u64().unwrap())
        });
        profiles.push(json!({"mode":mode,"samples":samples,"raw_sample_events":raw_samples,"lost_event_records":lost,"pids":pids,"comms":comms,"total_period":periods,"ns_per_element":output["ns_per_element"],"symbols":symbols,"captured_stdout_sha256":hash(capture.join("stdout.log"))}));
    }
    assert_eq!(profiled_words[0], profiled_words[1]);
    fs::write(root.join("profile-analysis.json"),serde_json::to_vec_pretty(&json!({"samples":total_samples,"lost_event_records":0,"profiles":profiles,"profile_output_word_comparisons":128,"all_profile_output_words_equal":true,"scope":"Whole component process capture, including initial setup, three primes, output and timer. User-cycle sample periods are not wall-time fractions or causally isolated costs. No repeat of first successful capture."})).unwrap()).unwrap();
    let result = read(root.join("component-result.json"));
    assert_eq!(result["passed"], false);
    assert_eq!(result["measured_processes"], 248);
    let closeout = json!({"status":"complete","adopted":false,"component_gate":false,"all206_content_pins_unchanged":true,"accepted170_files_unchanged":true,"accepted_bn254_sha256":"680805fb88bce25c54240d20ca88dd455b80aea1c4c2c496e11ecf66a32200b9","probe_sha256":hash(root.join("probe")),"native_oracle_tests":4,"operation_input_lane_checks":212992,"complex_result_lane_checks":196608,"norm_result_lane_checks":16384,"corner_result_checks":864,"ordinary_curve_additions":384,"changing_state_short_cases":20,"measured_processes":248,"measured_iterations":248000000u64,"priming_iterations":1488000,"unprofiled_output_word_comparisons":16864,"profile_output_word_comparisons":128,"profiles":2,"samples":total_samples,"lost_event_records":0,"new_sdk_proofs":0,"new_gnark_exports":0,"goal_achieved":false,"conclusion":"Reject this component layout as implemented. All four patterns lose all31pairedblocks, aggregate1.4240752 and affine2.1377621. No SDK trial or production change. Outer code size/IFMA counts omit outlined arithmetic callees; complete per-helper inspection and sampled shares are retained. This does not disprove every interleaved layout."});
    fs::write(
        root.join("closeout.json"),
        format!("{}\n", serde_json::to_string_pretty(&closeout).unwrap()),
    )
    .unwrap();
    println!("Closeout: 248timed processes,2captures{total_samples}samples0lost; all pins and output words verified; no adoption");
}
