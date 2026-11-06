use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-two-pending-count176";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: &str, v: &Value) {
    fs::write(format!("{R}/{p}"), serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
#[derive(Default)]
struct Counts {
    arrivals: u64,
    first: u64,
    singles: u64,
    triples: u64,
    queued: u64,
    drained: u64,
    spilled: u64,
    flushes: u64,
    projective_initial: u64,
    projective_add: u64,
    max_pending: usize,
    max_queue: usize,
    max_ready: usize,
    ordinary: BTreeMap<usize, u64>,
    triple: BTreeMap<usize, u64>,
}
struct Engine {
    two: bool,
    batch: usize,
    cap: usize,
    occupied: Vec<bool>,
    pending: Vec<Option<i64>>,
    pending_count: usize,
    busy: Vec<bool>,
    ready: Vec<(usize, i64)>,
    queue: Vec<(usize, i64)>,
    affine: Vec<i128>,
    projective: Vec<i128>,
    projective_seen: Vec<bool>,
    expected: Vec<i128>,
    c: Counts,
}
impl Engine {
    fn new(half: usize, batch: usize, two: bool) -> Self {
        assert!(batch >= 16);
        Self {
            two,
            batch,
            cap: if two { batch / 16 * 8 } else { batch },
            occupied: vec![false; half],
            pending: vec![None; half],
            pending_count: 0,
            busy: vec![false; half],
            ready: Vec::new(),
            queue: Vec::new(),
            affine: vec![0; half],
            projective: vec![0; half],
            projective_seen: vec![false; half],
            expected: vec![0; half],
            c: Counts::default(),
        }
    }
    fn add(&mut self, b: usize, t: i64) {
        assert!(!self.busy[b]);
        if !self.occupied[b] {
            self.occupied[b] = true;
            self.affine[b] = t as i128;
            self.c.first += 1;
            return;
        }
        let value = if self.two {
            if let Some(first) = self.pending[b].take() {
                self.pending_count -= 1;
                first.checked_add(t).unwrap()
            } else {
                self.pending[b] = Some(t);
                self.pending_count += 1;
                self.c.max_pending = self.c.max_pending.max(self.pending_count);
                return;
            }
        } else {
            t
        };
        self.busy[b] = true;
        self.ready.push((b, value));
        self.c.max_ready = self.c.max_ready.max(self.ready.len());
    }
    fn execute(&mut self) {
        if self.ready.is_empty() {
            return;
        }
        let n = self.ready.len();
        assert!(n <= self.cap);
        if self.two {
            self.c.triples += n as u64;
            *self.c.triple.entry(n).or_default() += 1;
        } else {
            self.c.singles += n as u64;
            *self.c.ordinary.entry(n).or_default() += 1;
        }
        for (b, t) in self.ready.drain(..) {
            assert!(self.busy[b]);
            self.busy[b] = false;
            self.affine[b] += t as i128;
        }
    }
    fn drain(&mut self) {
        while let Some(&(b, t)) = self.queue.last() {
            if self.busy[b] {
                break;
            }
            self.queue.pop();
            self.c.drained += 1;
            self.add(b, t);
            if self.ready.len() == self.cap {
                self.execute();
            }
        }
    }
    fn flush_queue(&mut self) {
        if self.queue.is_empty() {
            return;
        }
        self.c.flushes += 1;
        for &(b, t) in &self.queue {
            self.c.spilled += 1;
            self.projective[b] += t as i128;
            if self.projective_seen[b] {
                self.c.projective_add += 1;
            } else {
                self.projective_seen[b] = true;
                self.c.projective_initial += 1;
            }
        }
        self.queue.clear();
    }
    fn feed(&mut self, b: usize, t: i64) {
        self.c.arrivals += 1;
        self.expected[b] += t as i128;
        if self.busy[b] {
            self.c.queued += 1;
            self.queue.push((b, t));
            self.c.max_queue = self.c.max_queue.max(self.queue.len());
            if self.queue.len() == self.batch - 1 {
                self.flush_queue();
            }
            return;
        }
        self.add(b, t);
        if self.ready.len() == self.cap {
            self.execute();
            self.drain();
        }
    }
    fn finish(mut self) -> Value {
        self.execute();
        if self.two {
            while !self.queue.is_empty() {
                self.drain();
                self.execute();
            }
            let mut n = 0usize;
            for (b, p) in self.pending.iter_mut().enumerate() {
                if let Some(t) = p.take() {
                    self.affine[b] += t as i128;
                    self.c.singles += 1;
                    n += 1;
                    if n == self.batch {
                        *self.c.ordinary.entry(n).or_default() += 1;
                        n = 0;
                    }
                }
            }
            if n > 0 {
                *self.c.ordinary.entry(n).or_default() += 1;
            }
            self.pending_count = 0;
        } else {
            self.flush_queue();
        }
        assert!(
            self.ready.is_empty()
                && self.queue.is_empty()
                && self.pending.iter().all(Option::is_none)
                && self.busy.iter().all(|v| !*v)
        );
        for b in 0..self.expected.len() {
            assert_eq!(self.affine[b] + self.projective[b], self.expected[b]);
        }
        let c = &self.c;
        assert_eq!(c.arrivals, c.first + c.singles + 2 * c.triples + c.spilled);
        assert_eq!(c.queued, c.drained + c.spilled);
        assert_eq!(c.spilled, c.projective_initial + c.projective_add);
        assert_eq!(
            c.singles,
            c.ordinary.iter().map(|(n, c)| *n as u64 * c).sum::<u64>()
        );
        assert_eq!(
            c.triples,
            c.triple.iter().map(|(n, c)| *n as u64 * c).sum::<u64>()
        );
        let ordinary_batches: u64 = c.ordinary.values().sum();
        let triple_batches: u64 = c.triple.values().sum();
        let ordinary_tail: u64 = c.ordinary.iter().map(|(n, c)| (*n % 8) as u64 * c).sum();
        let triple_tail: u64 = c.triple.iter().map(|(n, c)| (*n % 8) as u64 * c).sum();
        json!({"two_arrival":self.two,"batch":self.batch,"ready_capacity":self.cap,"arrivals":c.arrivals,"first_writes":c.first,"ordinary_affine_updates":c.singles,"triple_updates":c.triples,"logical_affine_updates":c.singles+2*c.triples,"queue_arrivals":c.queued,"queue_drained":c.drained,"projective_arrivals":c.spilled,"projective_initializations":c.projective_initial,"projective_accumulator_adds":c.projective_add,"queue_flushes":c.flushes,"ordinary_batches":ordinary_batches,"triple_batches":triple_batches,"inverse_stages_model":ordinary_batches+triple_batches,"ordinary_batch_histogram":c.ordinary,"triple_batch_histogram":c.triple,"ordinary_scalar_tail_updates":ordinary_tail,"triple_scalar_tail_updates":triple_tail,"max_pending_slots":c.max_pending,"max_queue_slots":c.max_queue,"max_ready_slots":c.max_ready,"additional_pending_payload_forecast_bytes":if self.two{self.expected.len()*16}else{0},"generic_base_field_M_site_proxy":6*c.singles+13*c.triples,"integer_group_output_conservation_passed":true})
    }
}
fn bit(s: &[u64; 4], n: usize) -> u64 {
    if n < 256 {
        (s[n / 64] >> (n % 64)) & 1
    } else {
        0
    }
}
fn digit(s: &[u64; 4], start: usize, width: usize) -> i16 {
    if start == 0 {
        let v = s[0] & ((1u64 << width) - 1);
        return (v as i64 - ((v >> (width - 1)) << width) as i64) as i16;
    }
    let off = start - 1;
    let word = off / 64;
    let shift = off % 64;
    let mut v = if word < 4 { s[word] >> shift } else { 0 };
    if shift + width + 1 > 64 && word < 3 {
        v |= s[word + 1] << (64 - shift);
    }
    v &= (1u64 << (width + 1)) - 1;
    (((v + 1) >> 1) as i64 - ((v >> width) << width) as i64) as i16
}
fn add_signed(out: &mut [u64; 5], d: i16, start: usize) {
    let mut t = [0u64; 5];
    let j = start / 64;
    let sh = start % 64;
    let v = d.unsigned_abs() as u64;
    t[j] = v << sh;
    if sh != 0 && j + 1 < 5 {
        t[j + 1] = v >> (64 - sh);
    }
    if d >= 0 {
        let mut carry = 0u128;
        for i in 0..5 {
            let v = out[i] as u128 + t[i] as u128 + carry;
            out[i] = v as u64;
            carry = v >> 64;
        }
    } else {
        let mut borrow = false;
        for i in 0..5 {
            let (a, b) = out[i].overflowing_sub(t[i]);
            let (c, d) = a.overflowing_sub(u64::from(borrow));
            out[i] = c;
            borrow = b || d;
        }
    }
}
fn small_checks() -> usize {
    let mut cases = 0;
    for half in [1, 8, 32] {
        for batch in [16, 32] {
            for pattern in 0..5 {
                for two in [false, true] {
                    let mut e = Engine::new(half, batch, two);
                    for i in 0..511 {
                        let b = match pattern {
                            0 => 0,
                            1 => i % half,
                            2 => (i / 3) % half,
                            3 => (i * 17 + i / 7) % half,
                            _ => (i * i + 3 * i) % half,
                        };
                        let t = (i + 1) as i64 * if i % 3 == 0 { -1 } else { 1 };
                        e.feed(b, t);
                    }
                    let _ = e.finish();
                    cases += 1;
                }
            }
        }
    }
    cases
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("prepare") {
        assert!(!Path::new(R).join("pins.json").exists());
        let mut pins = BTreeMap::new();
        for p in [
            format!("{R}/model.rs"),
            format!("{R}/design.md"),
            format!("{P}/private-input-cache.bin"),
            format!("{P}/analysis.json"),
            format!("{P}/review.json"),
            format!("{Z}/crates/zopro-core/src/msm.rs"),
            format!("{Z}/crates/zopro-core/src/msm_affine/borrowed_pending.rs"),
            format!("{Z}/crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs"),
            format!("{Z}/docs/benchmarks/2026-10-28-mixed-sdk/README.md"),
            format!("{Z}/docs/benchmarks/2026-10-28-triple-callers/README.md"),
            format!("{Z}/docs/benchmarks/2026-11-06-triple-h-native176/README.md"),
        ] {
            pins.insert(p.clone(), hash(&p));
        }
        save("pins.json", &json!(pins));
        save(
            "plan.json",
            &json!({"version":1,"source_pins":format!("{R}/pins.json"),"environment":{},"steps":[{"kind":"command","name":"two-arrival-count-model","argv":[format!("{R}/model")],"timeout_seconds":120}]}),
        );
        println!("Prepared one model execution; cache reused, no solver/native/proof commands");
        return;
    }
    assert!(!Path::new(R).join("analysis.json").exists());
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let old = read(format!("{P}/analysis.json"));
    assert_eq!(read(format!("{P}/review.json"))["review_passed"], true);
    let path = format!("{P}/private-input-cache.bin");
    assert_eq!(hash(&path), old["cache"]["sha256"]);
    let bytes = fs::read(&path).unwrap();
    assert_eq!(&bytes[..8], b"TAGPHI01");
    let u64at = |at| u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap());
    let nk = u64at(8) as usize;
    let nh = u64at(16) as usize;
    assert_eq!(nk, 30529);
    assert_eq!(nh, 30922);
    assert_eq!(bytes.len(), 64 + 65 * (nk + nh));
    let tests = small_checks();
    let mut baseline = Vec::new();
    let mut candidate = Vec::new();
    let mut digit_checks = 0u64;
    for w in 0..20 {
        let start = w * 255 / 20;
        let width = (w + 1) * 255 / 20 - start;
        let half = 1usize << (width - 1);
        let batch = if width == 12 { 200 } else { 352 };
        let (mut b, mut c) = (
            Engine::new(half, batch, false),
            Engine::new(half, batch, true),
        );
        for i in 0..nk + nh {
            let at = 64 + i * 65;
            let words = core::array::from_fn(|j| u64at(at + j * 8));
            let d = digit(&words, start, width);
            let unsigned = (0..width).map(|j| bit(&words, start + j) << j).sum::<u64>();
            let expected = unsigned as i64
                + if start == 0 {
                    0
                } else {
                    bit(&words, start - 1) as i64
                }
                - (bit(&words, start + width - 1) << width) as i64;
            assert_eq!(d as i64, expected);
            digit_checks += 1;
            assert!(d.unsigned_abs() as usize <= half);
            let identity = bytes[at + 64];
            assert!(identity <= 1);
            if d != 0 && identity == 0 {
                let bucket = d.unsigned_abs() as usize - 1;
                let t = (i + 1) as i64 * if d < 0 { -1 } else { 1 };
                b.feed(bucket, t);
                c.feed(bucket, t);
            }
        }
        baseline
            .push(json!({"window":w,"start":start,"width":width,"half":half,"counts":b.finish()}));
        candidate
            .push(json!({"window":w,"start":start,"width":width,"half":half,"counts":c.finish()}));
    }
    for i in 0..nk + nh {
        let at = 64 + i * 65;
        let words = core::array::from_fn(|j| u64at(at + j * 8));
        let mut sum = [0u64; 5];
        for w in 0..20 {
            let start = w * 255 / 20;
            let width = (w + 1) * 255 / 20 - start;
            add_signed(&mut sum, digit(&words, start, width), start);
        }
        assert_eq!(&sum[..4], &words);
        assert_eq!(sum[4], 0);
    }
    let sum = |rows: &Vec<Value>| {
        let keys = [
            "arrivals",
            "first_writes",
            "ordinary_affine_updates",
            "triple_updates",
            "logical_affine_updates",
            "queue_arrivals",
            "queue_drained",
            "projective_arrivals",
            "projective_initializations",
            "projective_accumulator_adds",
            "queue_flushes",
            "ordinary_batches",
            "triple_batches",
            "inverse_stages_model",
            "ordinary_scalar_tail_updates",
            "triple_scalar_tail_updates",
            "additional_pending_payload_forecast_bytes",
            "generic_base_field_M_site_proxy",
        ];
        keys.into_iter()
            .map(|k| {
                (
                    k.to_owned(),
                    rows.iter()
                        .map(|r| r["counts"][k].as_u64().unwrap())
                        .sum::<u64>(),
                )
            })
            .collect::<BTreeMap<_, _>>()
    };
    let base = sum(&baseline);
    let proposed = sum(&candidate);
    assert_eq!(base["arrivals"], proposed["arrivals"]);
    assert_eq!(base["first_writes"], proposed["first_writes"]);
    let out = json!({"complete":true,"mode":"finite generic signed integer-group state-machine model, not curve execution","cache_source_sha256":old["cache"]["sha256"],"input_rows":nk+nh,"digit_checks":digit_checks,"scalar_reconstructions":nk+nh,"small_state_machine_cases":tests,"baseline":base,"two_arrival":proposed,"baseline_windows":baseline,"two_arrival_windows":candidate,"limitations":"Actual input identities skipped; no coordinates/equal-X/H-zero/inverse cancellation observations. Finite-group occupancy assumed. M-site proxy excludes field reduction shape, inverses, scalar tails/exception service, packing/gather/copy, projective normalization, admission and scheduling. Pending payload is16B per bucket across20 windows, not measured RAM; ready/scratch/resource coexistence also must be charged. No native candidate selected from this screen alone.","new_solver_or_FFT_calls":0,"new_timing_observations":0,"new_native_curve_tests":0,"new_SDK_requests":0,"new_proofs":0,"production_changed":false,"adopted":false,"goal_status":"active/unmet0/8"});
    save("analysis.json", &out);
    println!("Count screen complete: {} digit checks, {} scalar reconstructions, {tests} integer-group cases; inverse stages {}->{}, projective arrivals {}->{}, M proxy {}->{}; NOT timing",digit_checks,nk+nh,base["inverse_stages_model"],proposed["inverse_stages_model"],base["projective_arrivals"],proposed["projective_arrivals"],base["generic_base_field_M_site_proxy"],proposed["generic_base_field_M_site_proxy"]);
}
