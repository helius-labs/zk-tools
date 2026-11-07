use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, fs, path::{Path, PathBuf}};
use zopro_bn254::Fr;

const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/nonbinary-radix176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const INPUT: &str = "/home/ubuntu/src/zopro-perf-work/g2-queue-scan175/inputs";
const MANIFEST: &str = "docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json";
const FIELD: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib";
mod split { include!("/home/ubuntu/src/zopro-perf-work/partial-windows175/split.rs"); }
type Words = [u64; 4];
fn path(s: &str) -> PathBuf { Path::new(ROOT).join(s) }
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn save(p: impl AsRef<Path>, v: &Value) { assert!(!p.as_ref().exists()); fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap(); }
fn hash(p: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(fs::read(p).unwrap())) }
fn signed(x: i128) -> Fr {
    let a = x.unsigned_abs();
    let f = Fr::from_canonical([a as u64, (a >> 64) as u64, 0, 0]).unwrap();
    if x < 0 { -f } else { f }
}
fn components(s: Words) -> [i128; 2] {
    std::array::from_fn(|j| {
        let i = j * 2;
        let m = s[i] as u128 | ((s[i + 1] & 0x7fff_ffff_ffff_ffff) as u128) << 64;
        assert!(m < 1u128 << 126);
        if s[i + 1] >> 63 == 0 { m as i128 } else { -(m as i128) }
    })
}
fn recode(x: i128) -> [i16; 11] {
    let mut m = x.unsigned_abs();
    let sign = if x < 0 { -1i128 } else { 1 };
    let d = std::array::from_fn(|_| {
        let rem = (m % 3072) as i128;
        let d = if rem >= 1536 { rem - 3072 } else { rem };
        m = ((m as i128 - d) / 3072) as u128;
        i16::try_from(sign * d).unwrap()
    });
    assert_eq!(m, 0);
    assert_eq!(reconstruct(&d, 3072), x);
    d
}
fn reference(mut x: i128) -> [i16; 11] {
    let d = std::array::from_fn(|_| {
        let r = x.rem_euclid(3072);
        let d = if r >= 1536 { r - 3072 } else { r };
        x = (x - d) / 3072;
        i16::try_from(d).unwrap()
    });
    assert_eq!(x, 0);
    d
}
fn reconstruct(d: &[i16; 11], radix: i128) -> i128 {
    d.iter().rev().fold(0i128, |a, &v| a.checked_mul(radix).unwrap().checked_add(v as i128).unwrap())
}
fn grouping(words: &[Words]) -> Option<Vec<Words>> {
    if words.len() < 256 { return None; }
    let n = words.len();
    let sample = n.min(2048);
    let stride = n.div_ceil(sample);
    let observed = n.div_ceil(stride).min(sample);
    let unique = words.iter().step_by(stride).take(sample).collect::<BTreeSet<_>>();
    if unique.len() * 200 > observed * 199 { return None; }
    let percent = if (32768..65536).contains(&n) { 95 } else { 80 };
    let zeros = words.iter().step_by(stride).take(sample).filter(|s| **s == [0;4]).count();
    let nonzero = observed - zeros;
    if unique.len() - usize::from(zeros != 0) == nonzero && nonzero * 100 > observed * percent { return None; }
    let mut map = BTreeSet::new();
    let mut values = Vec::new();
    for &w in words { if w != [0;4] && map.insert(w) { values.push(w); } }
    if values.len() * 100 > n * percent { None } else { Some(values) }
}
struct Window { cap: usize, arrivals: u64, occupied: [Vec<bool>; 2], high: [Vec<bool>; 2] }
impl Window {
    fn new(cap: usize) -> Self {
        Self { cap, arrivals: 0, occupied: std::array::from_fn(|_| vec![false; cap]), high: std::array::from_fn(|_| vec![false; cap]) }
    }
    fn add(&mut self, d: i16, p: usize, high: bool) {
        if d == 0 { return; }
        let b = d.unsigned_abs() as usize - 1;
        assert!(b < self.cap);
        self.arrivals += 1;
        self.occupied[p][b] = true;
        if high { self.high[p][b] = true; }
    }
    fn row(&self) -> Value {
        let counts = self.occupied.each_ref().map(|v| v.iter().filter(|&&b| b).count() as u64);
        let union = (0..self.cap).filter(|&i| self.occupied[0][i] || self.occupied[1][i]).count() as u64;
        let merge = (0..self.cap).filter(|&i| self.occupied[0][i] && self.occupied[1][i]).count() as u64;
        let fill = self.arrivals - counts.iter().sum::<u64>();
        assert_eq!(fill + merge, self.arrivals - union);
        let high = self.high.iter().map(|v| v.iter().filter(|&&b| b).count() as u64).sum::<u64>();
        let weighted = self.cap.div_ceil(256) as u64 * dense(self.cap.min(256));
        let occupied = (0..self.cap).map(|i| self.occupied[0][i] || self.occupied[1][i]).collect::<Vec<_>>();
        let structural_weighted = occupied.chunks(256).map(structural_weighted).sum::<u64>();
        json!({"capacity": self.cap, "arrivals": self.arrivals, "occupied": union,
            "part_occupied": counts, "ideal_fill_joins": fill, "ideal_merge_joins": merge,
            "ideal_accumulation_joins": fill+merge, "high_part_occupied": high,
            "beta_Fq_products": 2*high, "dense_weighted_joins": weighted,
            "total_ideal_generic_joins": fill+merge+weighted,
            "structural_weighted_joins": structural_weighted,
            "total_structural_generic_joins": fill+merge+structural_weighted})
    }
}
fn dense(n: usize) -> u64 {
    assert!(n.is_power_of_two());
    let mut joins = 0;
    let mut size = n;
    while size > 1 { joins += size as u64 - 1; size /= 2; }
    joins
}
fn structural_weighted(occupied: &[bool]) -> u64 {
    assert!(occupied.len().is_power_of_two());
    let mut lists = vec![occupied.to_vec()];
    let mut joins = 0;
    while lists[0].len() > 1 {
        let copied = lists[0].iter().step_by(2).copied().collect();
        for list in &mut lists {
            *list = list.chunks_exact(2).map(|p| { joins += u64::from(p[0] && p[1]); p[0] || p[1] }).collect();
        }
        lists.push(copied);
    }
    joins
}
fn summarize(w: &[Window]) -> Value {
    let rows = w.iter().map(Window::row).collect::<Vec<_>>();
    let mut out = json!({"windows": rows});
    for key in ["capacity", "arrivals", "occupied", "ideal_fill_joins", "ideal_merge_joins", "ideal_accumulation_joins", "high_part_occupied", "beta_Fq_products", "dense_weighted_joins", "total_ideal_generic_joins", "structural_weighted_joins", "total_structural_generic_joins"] {
        out[key] = rows.iter().map(|r| r[key].as_u64().unwrap()).sum::<u64>().into();
    }
    out
}
fn edge_checks() -> usize {
    let mut values = vec![0i128, 1, -1, (1i128<<126)-1, -((1i128<<126)-1)];
    let mut power = 1i128;
    for w in 0..11 {
        for center in [power, power.checked_mul(1536).unwrap_or(i128::MAX)] {
            for delta in -1..=1 { for sign in [-1,1] { if let Some(v) = center.checked_add(delta).and_then(|v| v.checked_mul(sign)) { if v.unsigned_abs() < 1u128<<126 { values.push(v); } } } }
        }
        if w != 10 { power = power.checked_mul(3072).unwrap(); }
    }
    let mut state = 0x5a9bc217d33e9981u128;
    for _ in 0..8192 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let x = (state & ((1u128<<126)-1)) as i128;
        values.push(if state>>127 != 0 { -x } else { x });
    }
    for &x in &values {
        let d = recode(x);
        assert!(d.iter().all(|&v| v.unsigned_abs() <= 1536));
        assert_eq!(reconstruct(&reference(x), 3072), x);
    }
    // Exhaust every identity pattern of the small weighted subtrees and
    // separately check the dense identity-free formula at all used sizes.
    for n in [1usize, 2, 4, 8] {
        for mask in 0..1usize<<n {
            let active = (0..n).map(|i| mask>>i&1 != 0).collect::<Vec<_>>();
            assert!(structural_weighted(&active) <= dense(n));
        }
    }
    for n in [1,2,4,8,16,32,64,128,256] { assert_eq!(structural_weighted(&vec![true;n]),dense(n)); }
    values.len()
}
fn prepare() {
    let manifest_path = Path::new(Z).join(MANIFEST);
    let manifest = read(&manifest_path);
    let source = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(source.len(), 176);
    let mut pins = BTreeMap::new();
    for (p,h) in source { let p = Path::new(Z).join(p); assert_eq!(hash(&p),h.as_str().unwrap()); pins.insert(p.display().to_string(),hash(&p)); }
    for p in [path("model.rs"),path("design.md"),path("model"),manifest_path,PathBuf::from(FIELD),PathBuf::from("/home/ubuntu/src/zopro-perf-work/partial-windows175/split.rs"),Path::new(INPUT).join("manifest.json")] { pins.insert(p.display().to_string(),hash(&p)); }
    for f in read(Path::new(INPUT).join("manifest.json"))["fixtures"].as_array().unwrap() { let p = f["file"].as_str().unwrap(); assert_eq!(hash(p),f["sha256"].as_str().unwrap()); pins.insert(p.to_string(),hash(p)); }
    save(path("pins.json"), &json!(pins));
    save(path("plan.json"), &json!({"version":1,"source_pins":path("pins.json"),"steps":[{"kind":"command","name":"exact-radix-work-screen","argv":[path("model"),PathBuf::from("run")],"timeout_seconds":300}]}));
    println!("Prepared {} pins; production176 and both fixture hashes match",pins.len());
}
fn run() {
    for(p,h)in read(path("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let edges = edge_checks();
    let lambda = split::phi_lambda();
    let mut rows = Vec::new();
    for f in read(Path::new(INPUT).join("manifest.json"))["fixtures"].as_array().unwrap() {
        let bytes = fs::read(f["file"].as_str().unwrap()).unwrap();
        let n = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
        assert_eq!(n,f["points"].as_u64().unwrap() as usize);
        assert_eq!(bytes.len(),16+160*n);
        let identities = bytes[16..16+128*n].chunks_exact(128).filter(|p| p.iter().all(|&v|v==0)).count();
        assert_eq!(identities,0);
        let original = bytes[16+128*n..].chunks_exact(32).map(|s| std::array::from_fn(|i| u64::from_le_bytes(s[i*8..i*8+8].try_into().unwrap()))).collect::<Vec<Words>>();
        let grouped = grouping(&original);
        let admitted = grouped.is_some();
        let words = grouped.as_deref().unwrap_or(&original);
        let expected = if f["shape"] == "1x2" {30089}else{28852};
        assert_eq!(words.len(),expected);
        assert_eq!(admitted,f["shape"] == "2x2");
        let mut baseline = (0..11).map(|w|Window::new(if w==10 {64}else{2048})).collect::<Vec<_>>();
        let mut candidate = (0..11).map(|_|Window::new(1536)).collect::<Vec<_>>();
        let chunk = words.len().div_ceil(2);
        for (i,&s) in words.iter().enumerate() {
            let scalar = Fr::from_canonical(s).unwrap();
            let phi = split::phi_split(s);
            let c = components(phi);
            assert_eq!(signed(c[0])+lambda*signed(c[1]),scalar);
            for j in 0..2 {
                let d = recode(c[j]);
                assert_eq!(reconstruct(&reference(c[j]),3072),c[j]);
                let b = std::array::from_fn(|w|split::digit_of(&phi,j!=0,w,12));
                assert_eq!(reconstruct(&b,4096),c[j]);
                for w in 0..11 { baseline[w].add(b[w],i/chunk,j!=0);candidate[w].add(d[w],i/chunk,j!=0); }
            }
        }
        let b=summarize(&baseline);let c=summarize(&candidate);
        let ratio=c["total_structural_generic_joins"].as_u64().unwrap() as f64/b["total_structural_generic_joins"].as_u64().unwrap() as f64;
        rows.push(json!({"shape":f["shape"],"raw_rows":n,"grouping_admitted":admitted,"modeled_rows":words.len(),"original_identity_rows":identities,"grouped_point_identities_unknown":admitted,"baseline":b,"candidate":c,"generic_join_ratio":ratio,"literal_i16_codec_extra_bytes":12*words.len(),"interwindow_horner":{"baseline_doubles":120,"candidate_doubles":110,"candidate_added_projective_adds":10}}));
    }
    let pass=rows.iter().all(|r|r["generic_join_ratio"].as_f64().unwrap()<1.0);
    save(path("analysis.json"),&json!({"schema":1,"mathematical_checks_passed":true,"edge_integers_checked":edges,"fixture_rows":rows,"necessary_opportunity_check":pass,"runtime_candidate":false,"new_proofs":0,"new_timings":0,"production_changed":false,"proof_ledger_count":31387,"goal_achieved":false}));
    println!("{} edge integers; both exact fixture/component reconstructions pass; opportunity={}",edges,pass);
}
fn main() { match std::env::args().nth(1).as_deref() { Some("prepare")=>prepare(),Some("run")=>run(),_=>panic!("prepare | run") } }
