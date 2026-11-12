use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
    time::Instant,
};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/search-overlap178";
const INPUT: &str = "/home/ubuntu/src/zopro-perf-work/task-interval-c7a176";
type Interval = (u64, u64);
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn u(v: &Value) -> u64 {
    v.as_u64().unwrap()
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
fn raw_path(j: &Value) -> PathBuf {
    Path::new(INPUT)
        .join("remote-results")
        .join(j["proof_dir"].as_str().unwrap().strip_prefix('/').unwrap())
        .join("graphs.json")
}
fn clip((a, b): Interval, (lo, hi): Interval) -> Option<Interval> {
    let (a, b) = (a.max(lo), b.min(hi));
    (a < b).then_some((a, b))
}
#[derive(Default, Debug, PartialEq)]
struct Area {
    wall: [u128; 3],
    search: [u128; 3],
    any_search: [u128; 3],
    pending: u128,
}
fn integrate(pending: &[Interval], searches: &[Interval], bounds: Interval, workers: u64) -> Area {
    let mut events = vec![(bounds.0, 0i64, 0i64), (bounds.1, 0, 0)];
    for (intervals, is_search) in [(pending, false), (searches, true)] {
        for &iv in intervals {
            if let Some((a, b)) = clip(iv, bounds) {
                let (p, s) = if is_search { (0, 1) } else { (1, 0) };
                events.push((a, p, s));
                events.push((b, -p, -s));
            }
        }
    }
    events.sort_unstable();
    let mut out = Area::default();
    let (mut p, mut s, mut last, mut i) = (0i64, 0i64, bounds.0, 0);
    while i < events.len() {
        let t = events[i].0;
        let dt = (t - last) as u128;
        let bin = if p == 0 {
            0
        } else if p < workers as i64 {
            1
        } else {
            2
        };
        out.wall[bin] += dt;
        out.search[bin] += dt * s as u128;
        out.any_search[bin] += dt * u128::from(s > 0);
        out.pending += dt * p as u128;
        while i < events.len() && events[i].0 == t {
            p += events[i].1;
            s += events[i].2;
            i += 1;
        }
        assert!(p >= 0 && s >= 0 && s <= workers as i64);
        last = t;
    }
    assert_eq!((p, s), (0, 0));
    assert_eq!(out.wall.iter().sum::<u128>(), (bounds.1 - bounds.0) as u128);
    assert_eq!(
        out.pending,
        pending
            .iter()
            .filter_map(|&v| clip(v, bounds))
            .map(|(a, b)| (b - a) as u128)
            .sum()
    );
    assert_eq!(
        out.search.iter().sum::<u128>(),
        searches
            .iter()
            .filter_map(|&v| clip(v, bounds))
            .map(|(a, b)| (b - a) as u128)
            .sum()
    );
    out
}
fn selftest() {
    // Independent integer-time oracle: includes ties, empty intervals, clipping,
    // coincident publications, overlapping searches on distinct workers.
    let mut n = 0;
    for a in 0..6 {
        for b in a..7 {
            for c in 0..6 {
                for d in c..7 {
                    let p = vec![(a, b), (c, d), (2, 6)];
                    let s = vec![(0, 4), (3, 7)];
                    let mut expect = Area::default();
                    for t in 1..6 {
                        let pc = p.iter().filter(|&&(a, b)| a <= t && t < b).count();
                        let sc = s.iter().filter(|&&(a, b)| a <= t && t < b).count();
                        let bin = if pc == 0 {
                            0
                        } else if pc < 2 {
                            1
                        } else {
                            2
                        };
                        expect.wall[bin] += 1;
                        expect.search[bin] += sc as u128;
                        expect.any_search[bin] += u128::from(sc > 0);
                        expect.pending += pc as u128;
                    }
                    assert_eq!(integrate(&p, &s, (1, 6), 2), expect);
                    n += 1;
                }
            }
        }
    }
    assert_eq!(n, 729);
    println!("PASS {n} independent discrete-time overlap models");
}
fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn main() {
    let start = Instant::now();
    let r = Path::new(ROOT);
    let input = Path::new(INPUT);
    let jobs = read(input.join("remote-results/output/measure/jobs.json"));
    let jobs = jobs
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["arm"] == "ON" && j["panel"] == "latency")
        .collect::<Vec<_>>();
    assert_eq!(jobs.len(), 18);
    if std::env::args().nth(1).as_deref() == Some("prepare") {
        let mut pins = BTreeMap::new();
        for p in [
            r.join("analyze.rs"),
            r.join("analyze"),
            r.join("design.md"),
            input.join("remote-results/output/measure/jobs.json"),
            input.join("phase-summary.json"),
            input.join("target-analysis.json"),
            input.join("source/vendor/rayon-core/src/registry.rs"),
            input.join("source/vendor/rayon-core/src/task_lifecycle.rs"),
        ]
        .into_iter()
        .chain(jobs.iter().map(|j| raw_path(j)))
        {
            pins.insert(p.display().to_string(), hash(&p));
        }
        save(r.join("pins.json"), &json!(pins));
        save(
            r.join("plan.json"),
            &json!({"version":1,"source_pins":r.join("pins.json"),"steps":[{"kind":"command","name":"analyze-retained-overlap","argv":[r.join("analyze")],"timeout_seconds":300}]}),
        );
        println!("Pinned all18 main ON process inputs; no new capture");
        return;
    }
    selftest();
    let previous = read(input.join("phase-summary.json"));
    let mut rows = vec![];
    let mut graphs = 0;
    for j in jobs {
        let raw = read(raw_path(j));
        assert_eq!(raw.as_array().unwrap().len(), 3);
        let workers = u(&j["workers"]);
        for g in raw.as_array().unwrap() {
            graphs += 1;
            let mut keys = BTreeMap::<(u64, u64), [Vec<Interval>; 2]>::new();
            let mut searches = vec![];
            let mut phases = BTreeMap::new();
            let mut events = 0;
            for t in g["threads"].as_array().unwrap() {
                for key in ["dropped", "late", "outstanding"] {
                    assert_eq!(t[key], 0);
                }
                let mut last_search_end = 0;
                for e in t["events"].as_array().unwrap() {
                    events += 1;
                    let iv = (u(&e[0]), u(&e[1]));
                    assert!(iv.1 >= iv.0);
                    match u(&e[2]) {
                        1 | 3 => keys.entry((u(&e[3]), u(&e[4]))).or_default()
                            [usize::from(e[2] == 3)]
                        .push(iv),
                        5 => {
                            assert_eq!(t["pool"], 1);
                            assert!(iv.0 >= last_search_end);
                            last_search_end = iv.1;
                            searches.push(iv);
                        }
                        7 => {
                            assert!(phases.insert(u(&e[3]), iv).is_none());
                        }
                        _ => panic!("unknown event"),
                    }
                }
            }
            assert_eq!(events, u(&g["check"]["events"]));
            let mut lower = vec![];
            let mut upper = vec![];
            let mut pub_start = vec![];
            let mut matched = 0;
            for pair in keys.values_mut() {
                pair[0].sort_unstable();
                pair[1].sort_unstable();
                assert_eq!(pair[0].len(), pair[1].len());
                for i in 0..pair[0].len() {
                    let ((a, b), (c, d)) = (pair[0][i], pair[1][i]);
                    assert!(a <= b && a <= c && c <= d);
                    if i + 1 < pair[0].len() {
                        assert!(b.max(d) <= pair[0][i + 1].0, "ambiguous generation");
                    }
                    matched += 1;
                    pub_start.push(a);
                    if b < c {
                        lower.push((b, c));
                    }
                    if a < c {
                        upper.push((a, c));
                    }
                }
            }
            assert_eq!(matched, u(&g["check"]["published_jobs"]));
            assert_eq!(keys.len() as u64, u(&g["check"]["job_keys"]));
            for phase in 1..=6 {
                let bounds = phases[&phase];
                assert_eq!(
                    json!([bounds.0, bounds.1]),
                    g["check"]["phases"][phase.to_string()]
                );
                let duration = (bounds.1 - bounds.0) as f64;
                for (name, pending) in [
                    ("after_publication", &lower),
                    ("before_publication", &upper),
                ] {
                    let a = integrate(pending, &searches, bounds, workers);
                    let search_total = a.search.iter().sum::<u128>();
                    assert!(search_total > 0);
                    if name == "after_publication" && phase > 1 {
                        let old = previous["rows"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|x| {
                                x["job"] == j["name"]
                                    && x["round"] == g["round"]
                                    && x["phase"] == phase
                            })
                            .unwrap();
                        assert_eq!(
                            u(&old["matched_publications_in_phase"]),
                            pub_start
                                .iter()
                                .filter(|&&t| bounds.0 <= t && t < bounds.1)
                                .count() as u64
                        );
                        for (key, value) in [
                            (
                                "mean_known_published_not_started_jobs",
                                a.pending as f64 / duration,
                            ),
                            (
                                "fraction_without_known_pending_jobs",
                                a.wall[0] as f64 / duration,
                            ),
                            (
                                "mean_threads_in_steal_invocation",
                                search_total as f64 / duration,
                            ),
                        ] {
                            assert!(
                                (old[key].as_f64().unwrap() - value).abs() < 1e-10,
                                "previous marginal mismatch {key}"
                            );
                        }
                    }
                    rows.push(json!({"job":j["name"],"round":g["round"],"workers":workers,"phase":phase,"proxy":name,"wall_ns":bounds.1-bounds.0,"pending_job_ns":a.pending,"bin_wall_ns":a.wall,"bin_search_thread_ns":a.search,"bin_any_search_ns":a.any_search,"search_thread_ms":search_total as f64/1e6,"search_no_pending_fraction":a.search[0]as f64/search_total as f64,"search_at_least_workers_pending_fraction":a.search[2]as f64/search_total as f64,"no_pending_wall_fraction":a.wall[0]as f64/duration,"search_with_pending_wall_fraction":(a.any_search[1]+a.any_search[2])as f64/duration}));
                }
            }
        }
        println!("checked {}", j["name"]);
    }
    assert_eq!(graphs, 54);
    assert_eq!(rows.len(), 648);
    let mut summary = vec![];
    for workers in [32, 96] {
        for phase in 1..=6 {
            for proxy in ["after_publication", "before_publication"] {
                let group = rows
                    .iter()
                    .filter(|x| {
                        x["workers"] == workers && x["phase"] == phase && x["proxy"] == proxy
                    })
                    .collect::<Vec<_>>();
                assert_eq!(group.len(), 27);
                let mut item = json!({"workers":workers,"phase":phase,"proxy":proxy,"graphs":27});
                for key in [
                    "search_thread_ms",
                    "search_no_pending_fraction",
                    "search_at_least_workers_pending_fraction",
                    "no_pending_wall_fraction",
                    "search_with_pending_wall_fraction",
                ] {
                    let mut v = group
                        .iter()
                        .map(|x| x[key].as_f64().unwrap())
                        .collect::<Vec<_>>();
                    let m = median(&mut v);
                    item[key] = json!({"median":m,"min":v[0],"max":v[26]});
                }
                summary.push(item);
            }
        }
    }
    let out = json!({"scope":"retrospective joint occupancy, all54 historical instrumented graphs; failed observer calibration; not current production or exclusive CPU","models":729,"graphs":54,"prior_marginal_rows_reproduced":270,"bin_order":["no pending jobs","1..workers-1 pending jobs",">=workers pending jobs"],"elapsed_seconds":start.elapsed().as_secs_f64(),"summary":summary,"rows":rows});
    save(r.join("result.json"), &out);
    println!(
        "PASS 54graphs/270prior marginals/648joint rows in {:.3}s",
        start.elapsed().as_secs_f64()
    );
}
