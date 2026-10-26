//! Reanalyze archived requests. Does not run a prover or create cloud resources.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

fn median(mut v: Vec<f64>) -> f64 {
    assert!(!v.is_empty());
    assert!(v.iter().all(|x| x.is_finite()));
    v.sort_by(f64::total_cmp);
    let n = v.len();
    if n % 2 == 0 {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    } else {
        v[n / 2]
    }
}
fn main() {
    let root = PathBuf::from(std::env::args().nth(1).expect("repository root"));
    let dest = PathBuf::from(std::env::args().nth(2).expect("output JSON"));
    let aeglos_root = PathBuf::from(
        std::env::args()
            .nth(3)
            .expect("pinned Aeglos source checkout root"),
    );
    let base = "zopro/docs/benchmarks/";
    let inputs = root.join(format!("{base}2026-10-26-c7a-architecture-review/inputs"));
    let rows_path =
        format!("{base}2026-10-25-latest-c7a-aeglos/remote-results/output/latency/rows.json");
    let rows: Value = serde_json::from_slice(&fs::read(inputs.join("rows.json")).unwrap()).unwrap();
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 432);
    let shapes = [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
        "merge-54x1",
    ];
    let workers = [16, 32, 64, 96];
    let engines = ["current", "previous", "aeglos-default", "aeglos-windowed"];
    // Validate the complete input matrix, without selecting successful repetitions.
    for shape in shapes {
        for w in workers {
            for engine in engines {
                let selected: Vec<_> = rows
                    .iter()
                    .filter(|r| r["shape"] == shape && r["workers"] == w && r["prover"] == engine)
                    .collect();
                assert_eq!(selected.len(), 3);
                let mut reps: Vec<_> = selected
                    .iter()
                    .map(|r| r["rep"].as_u64().unwrap())
                    .collect();
                reps.sort_unstable();
                assert_eq!(reps, [0, 1, 2]);
                for row in selected {
                    let samples = row["samples"].as_array().unwrap();
                    assert_eq!(samples.len(), if shape == "1x2" { 31 } else { 17 });
                    assert_eq!(row["rounds"].as_u64().unwrap() as usize, samples.len());
                    let times = samples
                        .iter()
                        .map(|s| {
                            s[if engine.starts_with("aeglos") {
                                "verified_ms"
                            } else {
                                "ms"
                            }]
                            .as_f64()
                            .unwrap()
                        })
                        .collect();
                    let computed = median(times);
                    assert!((computed - row["median_ms"].as_f64().unwrap()).abs() < 1e-8);
                }
            }
        }
    }
    let select = |shape: &str, w: u64, engine: &str| -> Vec<&Value> {
        rows.iter()
            .filter(|r| r["shape"] == shape && r["workers"] == w && r["prover"] == engine)
            .collect()
    };
    let latency = |shape: &str, w: u64, engine: &str| {
        median(
            select(shape, w, engine)
                .iter()
                .map(|r| r["median_ms"].as_f64().unwrap())
                .collect(),
        )
    };
    let mut primary = Vec::new();
    for w in workers {
        let selected = select("1x2", w, "aeglos-default");
        let mut stages = BTreeMap::new();
        for key in [
            "verified_ms",
            "witness_ms",
            "fft_ms",
            "msm_ms",
            "assembly_ms",
            "native_bytes",
        ] {
            stages.insert(
                key,
                median(
                    selected
                        .iter()
                        .map(|r| {
                            median(
                                r["samples"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|s| s[key].as_f64().unwrap())
                                    .collect(),
                            )
                        })
                        .collect(),
                ),
            );
        }
        let residuals: Vec<f64> = selected
            .iter()
            .map(|r| {
                median(
                    r["samples"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|s| {
                            let request = s["verified_ms"].as_f64().unwrap();
                            let msm = s["msm_ms"].as_f64().unwrap();
                            assert!(msm >= 0.0 && request >= msm);
                            request - msm
                        })
                        .collect(),
                )
            })
            .collect();
        primary.push(json!({"workers":w,"zopro_request_ms":latency("1x2",w,"current"),"aeglos_stages_median_of_process_medians":stages,"aeglos_request_minus_msm_process_medians_ms":residuals,"aeglos_request_minus_msm_ms":median(residuals)}));
    }
    let mut all_shapes = Vec::new();
    for shape in shapes {
        let best = |engine: &str| {
            workers
                .iter()
                .map(|&w| (w, latency(shape, w, engine)))
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap()
        };
        let (zw, zt) = best("current");
        let (aw, at) = best("aeglos-default");
        let (ww, wt) = best("aeglos-windowed");
        let native = median(
            select(shape, aw, "aeglos-default")
                .iter()
                .map(|r| {
                    median(
                        r["samples"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|s| s["native_bytes"].as_f64().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        );
        all_shapes.push(json!({"shape":shape,"zopro":{"workers":zw,"ms":zt},"aeglos_sorted":{"workers":aw,"ms":at,"native_retained_mib":native/1048576.0},"aeglos_windowed":{"workers":ww,"ms":wt}}));
    }
    let source_pins = format!("{base}2026-10-25-task-census/aeglos-source.json");
    let aeglos: Value =
        serde_json::from_slice(&fs::read(inputs.join("aeglos-source.json")).unwrap()).unwrap();
    let mut input_hashes = BTreeMap::new();
    let mut files = vec![rows_path, source_pins];
    for report in [
        "latest-c7a-aeglos",
        "c7a-warm-profile",
        "worker-groups-c7a",
        "proof-dataflow-c7a",
        "task-census",
    ] {
        files.push(format!("{base}2026-10-25-{report}/README.md"));
    }
    for report in [
        "local-ibs-memory",
        "shift128-g1-component",
        "aws-local-crosshost-aeglos",
    ] {
        files.push(format!("{base}2026-10-23-{report}/README.md"));
    }
    files.push(format!(
        "{base}2026-10-22-universal-indexed-g1-tree/README.md"
    ));
    files.push(format!(
        "{base}2026-10-22-universal-cache-islands/README.md"
    ));
    for file in files {
        let archived = if file.ends_with("/rows.json") {
            inputs.join("rows.json")
        } else if file.ends_with("/aeglos-source.json") {
            inputs.join("aeglos-source.json")
        } else {
            inputs.join(format!(
                "{}.txt",
                Path::new(&file)
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            ))
        };
        input_hashes.insert(
            file.clone(),
            format!("{:x}", Sha256::digest(fs::read(archived).unwrap())),
        );
    }
    // These exact source files were pinned by the previous task-census report.
    for (file, digest) in aeglos["files"].as_object().unwrap() {
        let actual = format!(
            "{:x}",
            Sha256::digest(fs::read(aeglos_root.join(file)).unwrap())
        );
        assert_eq!(actual, digest.as_str().unwrap(), "Aeglos source: {file}");
    }
    let out = json!({"scope":"Reanalysis only of Oct25 167-source C7a comparison; no fresh prover measurements, proofs or cloud resources. Not a benchmark of current 170-source build.","archived_rows_checked":rows.len(),"primary_scaling":primary,"all_shapes":all_shapes,"input_sha256":input_hashes,"aeglos_source_pins":aeglos,"new_sdk_requests":0,"new_gnark_exports":0});
    fs::write(dest, serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("Checked all 432 archived rows and four pinned Aeglos files; wrote reanalysis, no prover executions");
}
