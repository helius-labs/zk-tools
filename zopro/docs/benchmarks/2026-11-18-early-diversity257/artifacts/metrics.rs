use serde_json::{json, Value};
use std::fs;

fn read(path: &str) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn median(mut x: [f64; 3]) -> f64 {
    assert!(x.iter().all(|v| v.is_finite() && *v > 0.0));
    x.sort_by(f64::total_cmp);
    x[1]
}

fn ratios(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] / b[i])
}

fn main() {
    assert_eq!(read("host-remote/review.json")["passed"], true);
    let rows = read("host-rows.json");
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 234);
    let mut cells = Vec::new();
    for (shape, m5) in [
        ("1x2", Some((37., 24.))),
        ("2x2", Some((59., 30.))),
        ("4x2", Some((109., 48.))),
        ("8x2", Some((189., 74.))),
        ("12x2", Some((284., 101.))),
        ("16x2", Some((335., 124.))),
        ("24x2", Some((528., 176.))),
        ("merge-24x1", Some((507., 175.))),
        ("merge-54x1", None),
    ] {
        for width in [32, 96] {
            let group: Vec<_> = rows
                .iter()
                .filter(|v| v["shape"] == shape && v["workers"] == width)
                .collect();
            let series = |arm: &str, field: &str| -> [f64; 3] {
                std::array::from_fn(|block| {
                    let mut it = group.iter().filter(|v| {
                        v["arm"] == arm && v["phase"] == "latency" && v["block"] == block
                    });
                    let v = it.next().unwrap()[field].as_f64().unwrap();
                    assert!(it.next().is_none());
                    v
                })
            };
            let rss = |arm: &str| {
                group
                    .iter()
                    .filter(|v| v["arm"] == arm)
                    .map(|v| v["lifetime_RSS_MiB"].as_f64().unwrap())
                    .reduce(f64::max)
                    .unwrap()
            };
            let time = |arm| series(arm, "mean_ms");
            let cpu = |arm| series(arm, "CPU_ms_per_request");
            let (on, off, reference) = (time("ON"), time("OFF"), time("REF"));
            let calibration = median(ratios(off, reference));
            let gains = ratios(on, off);
            let vs_ref = ratios(on, reference);
            let cpu_delta = median(ratios(cpu("ON"), cpu("OFF")));
            let cpu_vs_ref = median(ratios(cpu("ON"), cpu("REF")));
            cells.push(json!({
                "shape":shape, "workers":width, "wall_ms_blocks":{"ON":on,"OFF":off,"REF":reference},
                "wall_ms_median":{"ON":median(on),"OFF":median(off),"REF":median(reference)},
                "cpu_ms_median":{"ON":median(cpu("ON")),"OFF":median(cpu("OFF")),"REF":median(cpu("REF"))},
                "max_process_RSS_MiB":{"ON":rss("ON"),"OFF":rss("OFF"),"REF":rss("REF")},
                "paired_OFF_over_REF":calibration, "paired_ON_over_OFF":gains,"paired_ON_over_REF":vs_ref,
                "control_calibrated":(1./1.03..=1.03).contains(&calibration),
                "cpu_ON_over_OFF":cpu_delta, "cpu_ON_over_REF":cpu_vs_ref,
                "all_wall_wins":gains.iter().chain(vs_ref.iter()).all(|v|*v<1.),
                "rss_guard":rss("ON")<=rss("REF")*1.10+8.,
                "latency_guard":median(gains)<=1.03,
                "M5_CPU_over_ON":m5.map(|(c,_)|c/median(on)),
                "M5_Metal_over_ON":m5.map(|(_,m)|m/median(on)),
            }));
        }
    }
    println!("{}", serde_json::to_string_pretty(&cells).unwrap());
}
