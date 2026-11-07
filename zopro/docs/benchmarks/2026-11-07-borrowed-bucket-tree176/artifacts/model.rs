use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
use zopro_bn254::Fr;
const R: &str = "/home/ubuntu/src/zopro-perf-work/borrowed-bucket-tree176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const I: &str = "/home/ubuntu/src/zopro-perf-work/g2-queue-scan175/inputs";
const F: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib";
type Word = [u64; 4];
mod split {
    include!("/home/ubuntu/src/zopro-perf-work/partial-windows175/split.rs");
}
mod baseline {
    include!("/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/model.rs");
}
include!("/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/group-function.rs");
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn path(s: &str) -> PathBuf {
    Path::new(R).join(s)
}
fn token(s: &mut u64) -> Fr {
    let mut b = [0; 32];
    for c in b.chunks_mut(8) {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        c.copy_from_slice(&s.to_le_bytes());
    }
    Fr::from_le_bytes_mod_order(&b)
}
fn signed(n: i128) -> Fr {
    let m = n.unsigned_abs();
    let f = Fr::from_canonical([m as u64, (m >> 64) as u64, 0, 0]).unwrap();
    if n < 0 {
        -f
    } else {
        f
    }
}
fn check_split(s: &Word, original: &Word, lambda: Fr) {
    let mut c = [Fr::ZERO; 2];
    for j in 0..2 {
        let magnitude = s[j * 2] as u128 | ((s[j * 2 + 1] & 0x7fff_ffff_ffff_ffff) as u128) << 64;
        assert!(magnitude < 1u128 << 126);
        let value = if s[j * 2 + 1] >> 63 == 0 {
            magnitude as i128
        } else {
            -(magnitude as i128)
        };
        let reconstructed = (0..11).rev().fold(0i128, |a, w| {
            a.checked_mul(4096).unwrap() + split::digit_of(s, j == 1, w, 12) as i128
        });
        assert_eq!(reconstructed, value);
        c[j] = signed(value);
    }
    assert_eq!(c[0] + lambda * c[1], Fr::from_canonical(*original).unwrap());
}
#[derive(Default)]
struct Cost {
    pairs: usize,
    inversions: usize,
    max_pairs: usize,
    max_slots: usize,
    max_tail: usize,
}
impl Cost {
    fn json(&self) -> Value {
        json!({"generic_pairs":self.pairs,"inversion_requests":self.inversions,"largest_level_pairs":self.max_pairs,"largest_compact_buffer_slots":self.max_slots,"largest_scalar_tail_pairs":self.max_tail,"conservative_group_payload_bytes":2*self.max_slots*128+(self.max_pairs/8)*640+self.max_tail*320+self.max_pairs*16+256*128})
    }
}
fn reduce(
    bins: &mut [Vec<Fr>],
    stage: &str,
    band: usize,
    cost: &mut Cost,
    levels: &mut Vec<Value>,
) -> Vec<Option<Fr>> {
    let direct = bins
        .iter()
        .map(|b| {
            if b.is_empty() {
                None
            } else {
                Some(b.iter().copied().fold(Fr::ZERO, |a, b| a + b))
            }
        })
        .collect::<Vec<_>>();
    let mut depth = 0;
    loop {
        let pairs: usize = bins.iter().map(|b| b.len() / 2).sum();
        if pairs == 0 {
            break;
        }
        let slots: usize = bins.iter().map(|b| b.len().div_ceil(2)).sum();
        cost.pairs += pairs;
        cost.inversions += 1;
        cost.max_pairs = cost.max_pairs.max(pairs);
        cost.max_slots = cost.max_slots.max(slots);
        cost.max_tail = cost.max_tail.max(pairs % 8);
        levels.push(json!({"stage":stage,"band":band,"depth":depth,"pairs":pairs,"output_slots":slots,"full_groups":pairs/8,"tail_pairs":pairs%8}));
        for bin in bins.iter_mut() {
            let next = bin
                .chunks(2)
                .map(|p| if p.len() == 2 { p[0] + p[1] } else { p[0] })
                .collect();
            *bin = next;
        }
        depth += 1;
    }
    let roots = bins.iter().map(|b| b.first().copied()).collect::<Vec<_>>();
    assert_eq!(roots, direct);
    roots
}
fn tree(high: &[Vec<Fr>], low: &[Vec<Fr>], lambda: Fr) -> (Vec<Fr>, Cost, Vec<Value>, Vec<Value>) {
    assert_eq!(high.len(), 2048);
    assert_eq!(low.len(), 2048);
    let mut buckets = vec![Fr::ZERO; 2048];
    let mut cost = Cost::default();
    let mut levels = vec![];
    let mut bands = vec![];
    for band in 0..8 {
        let start = band * 256;
        let mut h = high[start..start + 256].to_vec();
        let mut histogram = BTreeMap::<String, usize>::new();
        for j in start..start + 256 {
            *histogram
                .entry(format!("{},{}", high[j].len(), low[j].len()))
                .or_default() += 1;
        }
        let high_roots = reduce(&mut h, "high", band, &mut cost, &mut levels);
        let mut l = low[start..start + 256].to_vec();
        for j in 0..256 {
            if let Some(v) = high_roots[j] {
                l[j].insert(0, v * lambda);
            }
        }
        let low_roots = reduce(&mut l, "low+high-root", band, &mut cost, &mut levels);
        for j in 0..256 {
            buckets[start + j] = low_roots[j].unwrap_or(Fr::ZERO);
        }
        bands.push(
            json!({"band":band,"bucket_count":256,"joint_high_low_population_histogram":histogram}),
        );
    }
    (buckets, cost, levels, bands)
}
fn edge_checks() -> usize {
    let mut checks = 0;
    for h in 0usize..=33 {
        for l in 0usize..=33 {
            let mut bins = vec![(0..h).map(|i| Fr::from((i + 1) as u64)).collect::<Vec<_>>()];
            let mut cost = Cost::default();
            let mut levels = vec![];
            let root = reduce(&mut bins, "edge-high", 0, &mut cost, &mut levels)[0];
            bins[0] = (0..l).map(|i| Fr::from((i + 777) as u64)).collect();
            if let Some(v) = root {
                bins[0].push(-v);
            }
            let result =
                reduce(&mut bins, "edge-low", 0, &mut cost, &mut levels)[0].unwrap_or(Fr::ZERO);
            let expected = (0..l)
                .map(|i| Fr::from((i + 777) as u64))
                .fold(Fr::ZERO, |a, b| a + b)
                - (0..h)
                    .map(|i| Fr::from((i + 1) as u64))
                    .fold(Fr::ZERO, |a, b| a + b);
            assert_eq!(result, expected);
            assert_eq!(cost.pairs, (h + l).saturating_sub(1));
            checks += 1;
        }
    }
    // Structural occupancy must survive synthetic token cancellation.
    let mut bins = vec![vec![Fr::ONE, -Fr::ONE, Fr::ZERO]];
    let mut cost = Cost::default();
    let mut levels = vec![];
    assert_eq!(
        reduce(&mut bins, "cancel", 0, &mut cost, &mut levels),
        vec![Some(Fr::ZERO)]
    );
    assert_eq!(cost.pairs, 2);
    checks + 1
}
fn census(words: &[Word], mapping: Option<&[usize]>, original: &[Word]) -> Value {
    let short = words
        .iter()
        .map(|&s| split::phi_split(s))
        .collect::<Vec<_>>();
    let lambda = split::phi_lambda();
    for (s, o) in short.iter().zip(words) {
        check_split(s, o, lambda);
    }
    let chunk = words.len().div_ceil(2);
    let mut first = None;
    for mut seed in [0xc270_63b8_0d9e_1a45u64, 0x7a51_f3e2_b96d_c008] {
        let original_tokens = (0..original.len())
            .map(|_| token(&mut seed))
            .collect::<Vec<_>>();
        let tokens = if let Some(classes) = mapping {
            let mut t = vec![Fr::ZERO; words.len()];
            for (i, &class) in classes.iter().enumerate() {
                if class != usize::MAX {
                    t[class] += original_tokens[i];
                }
            }
            t
        } else {
            original_tokens.clone()
        };
        let target = original
            .iter()
            .zip(&original_tokens)
            .fold(Fr::ZERO, |a, (s, p)| {
                a + Fr::from_canonical(*s).unwrap() * *p
            });
        let mut weighted = Fr::ZERO;
        let mut rows = vec![];
        let mut total_baseline_inversions = 0;
        let mut total_candidate_inversions = 0;
        let mut total_pairs = 0;
        let mut total_route = 0;
        let mut baseline_pairs = 0;
        for w in 0..11 {
            for part in 0..2 {
                let start = (part * chunk).min(words.len());
                let end = (start + chunk).min(words.len());
                let cap = if w == 10 { 64 } else { 2048 };
                let mut high = vec![vec![]; cap];
                let mut low = vec![vec![]; cap];
                let mut expected = vec![Fr::ZERO; cap];
                let mut model = baseline::Model::new(cap, 200, false);
                let mut arrivals = 0;
                for is_high in [true, false] {
                    let mut feed = vec![];
                    for i in start..end {
                        let d = split::digit_of(&short[i], is_high, w, 12);
                        if d == 0 {
                            continue;
                        }
                        let bucket = d.unsigned_abs() as usize - 1;
                        assert!(bucket < cap);
                        let p = if d < 0 { -tokens[i] } else { tokens[i] };
                        if is_high {
                            high[bucket].push(p);
                        } else {
                            low[bucket].push(p);
                        }
                        expected[bucket] += p;
                        arrivals += 1;
                        feed.push((bucket as u32 | if d < 0 { 1 << 31 } else { 0 }, tokens[i]));
                    }
                    if w < 10 {
                        model.feed(&feed);
                    }
                    if is_high {
                        for p in &mut expected {
                            *p *= lambda;
                        }
                        if w < 10 {
                            model.phi(lambda);
                        }
                    }
                }
                if w < 10 {
                    let (ordinary, c) = model.finish();
                    assert_eq!(ordinary, expected);
                    let (candidate, cost, levels, bands) = tree(&high, &low, lambda);
                    assert_eq!(candidate, expected);
                    let generic_pairs = high
                        .iter()
                        .zip(&low)
                        .map(|(h, l)| (h.len() + l.len()).saturating_sub(1))
                        .sum::<usize>();
                    assert_eq!(cost.pairs, generic_pairs);
                    assert_eq!(
                        c.affine_updates + c.projective_accumulator_adds + c.finish_merges,
                        generic_pairs
                    );
                    total_baseline_inversions += c.batches;
                    total_candidate_inversions += cost.inversions;
                    total_pairs += cost.pairs;
                    baseline_pairs += generic_pairs;
                    total_route += arrivals;
                    rows.push(json!({"window":w,"part":part,"arrivals":arrivals,"baseline":c.json(),"candidate":cost.json(),"levels":levels,"bands":bands,"all_buckets_equal":true,"direct_join_count":generic_pairs}));
                }
                for (b, p) in expected.into_iter().enumerate() {
                    weighted +=
                        p * Fr::from((b + 1) as u64) * Fr::from(2u64).pow(&[(12 * w) as u64]);
                }
            }
        }
        assert_eq!(weighted, target);
        let candidate_inv = total_candidate_inversions;
        let result = json!({"raw_rows":original.len(),"modeled_rows":words.len(),"grouping_admitted":mapping.is_some(),"ordinary_baseline_inversions":total_baseline_inversions,"ordinary_candidate_inversions":candidate_inv,"ordinary_baseline_generic_joins":baseline_pairs,"ordinary_candidate_generic_joins":total_pairs,"route_index_payload_bytes":total_route*4,"route_count_start_cursor_bytes_all10windows":20*(2*2048*3+1)*std::mem::size_of::<usize>(),"native_shape_fixed":{"parts":2,"digit_bits":12,"ordinary_windows":10,"buckets":2048,"band_buckets":256,"unchanged_top_windows":1},"rows":rows,"two_fullwidth_token_streams":true,"whole_original_scalar_dot_product_matches":true,"opportunity_passed":candidate_inv<total_baseline_inversions&&total_pairs<=baseline_pairs});
        if let Some(old) = &first {
            assert_eq!(old, &result);
        } else {
            first = Some(result);
        }
    }
    first.unwrap()
}
fn prepare() {
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let source = load(&manifest);
    let source = source["source_files_sha256"].as_object().unwrap();
    assert_eq!(source.len(), 176);
    let mut pins = BTreeMap::new();
    for (p, h) in source {
        let p = Path::new(Z).join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins.insert(p.display().to_string(), hash(&p));
    }
    for p in [
        path("model.rs"),
        path("design.md"),
        path("model"),
        manifest,
        PathBuf::from(F),
        PathBuf::from("/home/ubuntu/src/zopro-perf-work/partial-windows175/split.rs"),
        PathBuf::from("/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/model.rs"),
        PathBuf::from("/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/lifo-impl.rs"),
        PathBuf::from("/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/group-function.rs"),
        Path::new(I).join("manifest.json"),
    ] {
        pins.insert(p.display().to_string(), hash(&p));
    }
    for f in load(Path::new(I).join("manifest.json"))["fixtures"]
        .as_array()
        .unwrap()
    {
        let p = f["file"].as_str().unwrap();
        assert_eq!(hash(p), f["sha256"].as_str().unwrap());
        pins.insert(p.to_string(), hash(p));
    }
    save(path("pins.json"), &json!(pins));
    save(
        path("plan.json"),
        &json!({"version":1,"source_pins":path("pins.json"),"steps":[{"kind":"command","name":"work-and-token-screen","argv":[path("model"),PathBuf::from("run")],"timeout_seconds":300}]}),
    );
    println!(
        "Prepared {} pins; accepted176 source and two fixture hashes match",
        pins.len()
    );
}
fn run() {
    for (p, h) in load(path("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let edges = edge_checks();
    let mut rows = vec![];
    for f in load(Path::new(I).join("manifest.json"))["fixtures"]
        .as_array()
        .unwrap()
    {
        let b = fs::read(f["file"].as_str().unwrap()).unwrap();
        let n = u64::from_le_bytes(b[8..16].try_into().unwrap()) as usize;
        assert_eq!(n, f["points"].as_u64().unwrap() as usize);
        assert_eq!(b.len(), 16 + 160 * n);
        assert_eq!(
            b[16..16 + 128 * n]
                .chunks_exact(128)
                .filter(|p| p.iter().all(|&v| v == 0))
                .count(),
            0
        );
        let words = b[16 + 128 * n..]
            .chunks_exact(32)
            .map(|s| {
                std::array::from_fn(|i| u64::from_le_bytes(s[i * 8..i * 8 + 8].try_into().unwrap()))
            })
            .collect::<Vec<Word>>();
        let g = groups(&words);
        assert_eq!(g.is_some(), f["shape"] == "2x2");
        let mut row = if let Some((values, classes)) = &g {
            assert_eq!(values.len(), 28852);
            census(values, Some(classes), &words)
        } else {
            assert_eq!(n, 30089);
            census(&words, None, &words)
        };
        row["shape"] = f["shape"].clone();
        rows.push(row);
    }
    let pass = rows.iter().all(|r| r["opportunity_passed"] == true);
    save(
        path("analysis.json"),
        &json!({"schema":1,"mathematical_checks_passed":true,"edge_cases":edges,"fixtures":rows,"necessary_opportunity_gate_passed":pass,"native_selected":pass,"production_changed":false,"adopted":false,"new_timings":0,"new_proofs":0,"proof_ledger_count":31621,"goal_achieved":false,"limits":"Noncancelling Fr token schedule, not native/curve exceptions, time or measured RAM. Unchanged indexed top is outside inversion comparison. Per-group layouts are requested payload, not RSS or minimum RAM."}),
    );
    println!(
        "work/token screen complete; {} edge cases; opportunity={}",
        edges, pass
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("run") => run(),
        _ => panic!("prepare | run"),
    }
}
