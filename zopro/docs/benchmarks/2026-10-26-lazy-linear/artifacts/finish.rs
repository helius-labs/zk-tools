use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R: &str = "/tmp/zopro-lazy-linear";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-lazy-linear";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn run(argv: Vec<String>, path: &Path) -> String {
    let out = Command::new(&argv[0]).args(&argv[1..]).output().unwrap();
    save(path.with_extension("command.json"), &json!(argv));
    fs::write(path, &out.stdout).unwrap();
    fs::write(path.with_extension("stderr.log"), out.stderr).unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap()
}
fn main() {
    let r = Path::new(R);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["component_passed"], false);
    assert_eq!(read(r.join("decode/result.json"))["status"], "complete");
    for (p, h) in read(r.join("measurement-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let baseline = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in baseline["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    let old = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let lib = old["libraries"]["zopro_bn254"].as_str().unwrap();
    assert_eq!(hash(lib), old["input_pins"][lib].as_str().unwrap());
    assert_eq!(
        hash(lib),
        "680805fb88bce25c54240d20ca88dd455b80aea1c4c2c496e11ecf66a32200b9"
    );
    let nm = run(
        vec![
            "nm".into(),
            "-S".into(),
            "--defined-only".into(),
            r.join("probe").display().to_string(),
        ],
        &r.join("symbols.log"),
    );
    let mut code = BTreeMap::new();
    for mode in ["Fresh", "Lazy"] {
        let token = format!("Fq256{mode}Kh3_");
        let symbols: Vec<_> = nm
            .lines()
            .filter(|l| l.contains("probe6kernel") && l.contains(&token))
            .collect();
        assert_eq!(symbols.len(), 1);
        let parts: Vec<_> = symbols[0].split_whitespace().collect();
        let name = parts[3];
        let size = u64::from_str_radix(parts[1], 16).unwrap();
        let text = run(
            vec![
                "objdump".into(),
                "-d".into(),
                "-w".into(),
                "--insn-width=16".into(),
                format!("--disassemble={name}"),
                r.join("probe").display().to_string(),
            ],
            &r.join(format!("kernel-{mode}.txt")),
        );
        let mut instructions = 0;
        let mut ifma = 0;
        let mut permutes = 0;
        let mut rsp = 0;
        let mut stack_sub = 0u64;
        for l in text.lines() {
            let fields: Vec<_> = l.split('\t').collect();
            if fields.len() < 3 || !fields[0].trim_end().ends_with(':') {
                continue;
            }
            let asm = fields.last().unwrap().trim();
            if asm.is_empty() {
                continue;
            }
            instructions += 1;
            if asm.starts_with("vpmadd52") {
                ifma += 1;
            }
            if asm.starts_with("vperm") {
                permutes += 1;
            }
            if asm.contains("%rsp") {
                rsp += 1;
            }
            if asm.starts_with("sub") && asm.contains("%rsp") {
                if let Some((_, s)) = asm.split_once("$0x") {
                    if let Some((n, _)) = s.split_once(',') {
                        stack_sub += u64::from_str_radix(n, 16).unwrap();
                    }
                }
            }
        }
        assert!(instructions > 0 && ifma > 0);
        code.insert(mode,json!({"symbol":name,"bytes":size,"static_instructions":instructions,"static_ifma_sites":ifma,"static_vector_permutations":permutes,"rsp_operand_lines":rsp,"literal_stack_subtractions":stack_sub}));
    }
    let frozen_script = fs::read_to_string(r.join("decode/samples-frozen/stdout.log")).unwrap();
    let shared = frozen_script.contains("Fq256FreshKh3_") && !nm.contains("Fq256FrozenKh3_");
    save(
        r.join("codegen.json"),
        &json!({"kernels":code,"frozen_and_fresh_expression_share_linked_symbol":shared,"scope":"Actual timed arithmetic component kernels, not SDK callers. Static sites are not retired counts or costs; literal frames omit pushes/callees/alignment. Compiler may unroll loops."}),
    );
    let stderr = fs::read_to_string(r.join("native/field/stderr.log")).unwrap();
    let count = |key: &str| -> u64 {
        stderr
            .lines()
            .filter_map(|l| l.strip_prefix(key))
            .map(|n| n.parse::<u64>().unwrap())
            .sum()
    };
    let exact = count("LAZY_LINEAR_EXACT_LANES=");
    let chains = count("LAZY_LINEAR_CHAIN_LANES=");
    assert!(exact > 0 && chains > 0);
    let rows = a["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 339);
    let iterations: u64 = rows
        .iter()
        .map(|r| r["job"]["iterations"].as_u64().unwrap())
        .sum();
    assert_eq!(iterations, 396000000);
    let mut profiles = 0u64;
    for (mode, p) in a["profiles"].as_object().unwrap() {
        profiles += p["samples"].as_u64().unwrap();
        let out = Command::new("sudo")
            .args(["-n", "sha256sum"])
            .arg(r.join(format!("profiles/{mode}/perf.data")))
            .output()
            .unwrap();
        assert!(out.status.success());
        assert_eq!(
            String::from_utf8(out.stdout)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap(),
            p["sha256"].as_str().unwrap()
        );
    }
    let close = json!({"status":"closed negative arithmetic component","component_passed":false,"production_sources_unchanged":170,"native_field_tests":37,"new_exact_raw_result_lanes":exact,"new_chained_arkworks_result_lanes":chains,"reduced_radix_model_cases":1600784,"latency_processes":336,"profile_processes":3,"timed_iterations":iterations,"priming_iterations":339*3*512,"separate_reference_iterations":iterations+339*512,"final_u64_comparisons":339*128,"profile_samples":profiles,"lost_samples":0,"sdk_requests":0,"independent_gnark_exports":0,"next_prior_count":6850,"adopted":false,"goal_achieved":false,"new_cycle_profiles":3,"retained_decode_only_repair":true,"next_action":"Do not integrate or rerun this lazy-linear method. Preserve its range proof and negative measurements. Return to actual query work reduction; quotient estimation and fewer correction passes alone did not reduce CPU cost."});
    save(r.join("closeout.json"), &close);
    println!("{}", serde_json::to_string_pretty(&close).unwrap());
}
