use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-ifma-zero-mask");
    let pins = read(root.join("build-pins.json"));
    assert_eq!(pins.as_object().unwrap().len(), 183);
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let result = read(root.join("build/result.json"));
    assert_eq!(result["status"], "complete");
    for step in result["steps"].as_array().unwrap() {
        let dir = root.join("build").join(step.as_str().unwrap());
        let exit = read(dir.join("exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(dir.join(format!("{stream}.log"))),
                exit[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
    }
    let base = read(root.join("accepted-manifest.json"));
    let edits = read(root.join("edits.json"));
    for (name, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(name)),
            h.as_str().unwrap()
        );
    }
    let mut recovery = Vec::new();
    for n in ["ifma.rs", "ifma8.rs"] {
        let path = format!("crates/zopro-bn254/src/{n}");
        let mut text = fs::read_to_string(root.join("candidate").join(&path)).unwrap();
        for edit in edits["ifma.rs"].as_array().unwrap().iter().rev() {
            let from = edit["from"].as_str().unwrap();
            let to = edit["to"].as_str().unwrap();
            let (from, to) = if n == "ifma8.rs" {
                let from = from.replace("256", "512");
                let to = to.replace("256", "512").replace(
                    "                    let m = if cfg!(zopro_unmask_m52) { m } else { _mm512_and_si512(m, mask) };",
                    "                    let m = if cfg!(zopro_unmask_m52) {\n                        m\n                    } else {\n                        _mm512_and_si512(m, mask)\n                    };",
                );
                (from, to)
            } else {
                (from.into(), to.into())
            };
            assert_eq!(text.matches(&to).count(), 1);
            text = text.replace(&to, &from);
        }
        assert_eq!(
            format!("{:x}", Sha256::digest(text.as_bytes())),
            base["source_files_sha256"][&path].as_str().unwrap()
        );
        recovery.push(path);
    }
    let code = read(root.join("codegen.json"));
    assert_eq!(code["field_functions_normalized_equal"], 16);
    assert_eq!(code["total_and_sites_saved"], 0);
    let mut eight = 0;
    for pair in code["pairs"].as_array().unwrap() {
        assert_eq!(pair["and_saved"], 0);
        let symbol = pair["symbol"].as_str().unwrap();
        let a = code["functions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["label"] == "accepted" && v["symbol"] == symbol)
            .unwrap();
        let c = code["functions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["label"] == "candidate" && v["symbol"] == symbol)
            .unwrap();
        for key in [
            "instructions",
            "and_sites",
            "ifma_sites",
            "rsp_operand_lines",
            "literal_stack_subtractions",
            "calls",
        ] {
            assert_eq!(a[key], c[key], "{symbol}/{key}");
        }
        assert_eq!(a["calls"], 0);
        if symbol.ends_with('8') {
            eight += 1;
            assert_eq!(pair["normalized_equal"], true);
        }
    }
    assert_eq!(eight, 12);
    for pair in code["isa_pairs"].as_array().unwrap() {
        assert_eq!(pair["feed_mask52_vs_raw_equal"], true);
    }
    let output = fs::read_to_string(root.join("build/isa-check/stdout.log")).unwrap();
    let (mut raw, mut feed, mut required48) = (0u64, 0u64, 0u64);
    for line in output.lines() {
        let fields = line
            .split_whitespace()
            .filter_map(|w| w.split_once('='))
            .collect::<std::collections::BTreeMap<_, _>>();
        raw += fields["raw_reference_checks"].parse::<u64>().unwrap();
        feed += fields["feed_reference_checks"].parse::<u64>().unwrap();
        required48 += fields["necessary48_counterexamples"]
            .parse::<u64>()
            .unwrap();
    }
    assert_eq!((raw, feed, required48), (100032, 200064, 12));
    for link in read(root.join("links.json")).as_array().unwrap() {
        assert_eq!(
            hash(link["library"].as_str().unwrap()),
            link["library_sha256"].as_str().unwrap()
        );
        assert_eq!(
            hash(root.join(format!("{}-probe", link["label"].as_str().unwrap()))),
            link["probe_sha256"].as_str().unwrap()
        );
    }
    assert_eq!(
        hash("/tmp/zopro-grouping-module/native/candidate-sdk"),
        "03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d"
    );
    let out = json!({"status":"closed_verified; proposed mask removal already optimized in tested arithmetic; no speed experiment","adopted":false,"goal_achieved":false,"raw_isa_lane_checks":raw,"feed_highhalf_reference_checks":feed,"required48bit_mask_counterexamples":required48,"field_wrappers":24,"field_dimensions":"Fq/Fr,R256/R260,4/8lanes,mul/sum2/difference2","normalized_instruction_identical":16,"all_twelve_eight_lane_wrappers_identical":true,"all_24_instruction_AND_IFMA_rsp_and_stack_counts_match":true,"removed_and_sites":0,"fourlane_layout_differences":8,"zero52_feed_toys_identical_both_widths":true,"toy_eightlane_return_still_masks52":true,"last48_masks_preserved":true,"whole_original_field_files_recovered":recovery,"accepted_170_sources_and_sdk_unchanged":true,"build_pins_rehashed":183,"new_benchmarks":0,"new_proofs":0,"new_gnark_exports":0,"new_profiles":0,"new_fullfield_oracles":0,"fullSDK_or_resource_qualification":false,"decision":"No remaining mask-elimination mechanism in tested field kernels. Do not benchmark incidental register/layout changes or adopt a no-work source cleanup for the speed goal.","limitations":"A compiler/ISA screen, not an end-to-end speed measurement or independent complete field proof. Sixteen normalized kernels identical; eight fourlane streams differ despite equal counts. SIMD48mask may lower toblend ratherthanAND. No universal claim for every compiler/target.","owned_job":"23697 terminal/reaped; no AWS"});
    fs::write(
        root.join("closeout.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("CLOSED183pins100032raw+200064feedchecks;24kernels0ANDsavings,12eightlaneidentical;0timings/proofs/adoption");
}
