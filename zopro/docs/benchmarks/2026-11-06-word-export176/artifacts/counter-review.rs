use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
const R: &str = "/home/ubuntu/src/zopro-perf-work/word-export176";
fn main() {
    let a: Value =
        serde_json::from_slice(&fs::read(format!("{R}/analysis.json")).unwrap()).unwrap();
    let mut rows = Vec::new();
    let mut panels = 0;
    for curve in ["G1", "G2"] {
        for n in [200, 352] {
            let mut arms = BTreeMap::new();
            for arm in ["frozen", "OFF", "ON"] {
                let key = format!("stat-{curve}-{n}-w32-{arm}");
                let p = format!("{R}/stat/{key}.csv");
                let bytes = fs::read(&p).unwrap();
                let text = std::str::from_utf8(&bytes).unwrap();
                let mut counts = BTreeMap::new();
                for line in text
                    .lines()
                    .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
                {
                    let f = line.split(';').collect::<Vec<_>>();
                    if ![
                        "cycles:u",
                        "instructions:u",
                        "cache-misses:u",
                        "branch-misses:u",
                    ]
                    .contains(&f[2].trim())
                    {
                        continue;
                    }
                    let event = f[2].trim().to_owned();
                    let count = f[0].trim().parse::<f64>().unwrap();
                    assert!(f[4].trim().parse::<f64>().unwrap() >= 99.9);
                    assert_eq!(
                        count,
                        a["counters"][&key][&event]["count"].as_f64().unwrap()
                    );
                    let expected = a["counters"][&key][&event]["per_wave"].as_f64().unwrap();
                    assert!((count / 16384. - expected).abs() <= 1e-14 * expected.abs().max(1.));
                    assert!(counts.insert(event, count).is_none());
                }
                assert_eq!(counts.len(), 4);
                panels += 1;
                arms.insert(
                    arm,
                    json!({"counts":counts,"csv_sha256":format!("{:x}",Sha256::digest(bytes))}),
                );
            }
            let pct = |event: &str| {
                100. * (arms["ON"]["counts"][event].as_f64().unwrap()
                    / arms["OFF"]["counts"][event].as_f64().unwrap()
                    - 1.)
            };
            rows.push(json!({"curve":curve,"size":n,"ON_vs_OFF_instructions_percent":pct("instructions:u"),"ON_vs_OFF_cycles_percent":pct("cycles:u"),"arms":arms}));
        }
    }
    assert_eq!(panels, 12);
    let out = json!({"review_passed":true,"raw_counter_panels":panels,"all_running_percent_at_least":99.9,"rows":rows,"single_panels_not_confidence_intervals_or_causal_wall_bounds":true});
    fs::write(
        format!("{R}/counter-review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!(
        "PASS12raw counter panels/counts/100%qualification; derived four instruction/cycle changes"
    );
}
