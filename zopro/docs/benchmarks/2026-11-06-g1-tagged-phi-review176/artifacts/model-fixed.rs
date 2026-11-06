use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, sync::Arc};
use zopro_bn254::Fr;
#[global_allocator]
static ALLOCATOR: zopro_core::alloc::Allocator = zopro_core::alloc::Allocator;
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176";
mod loader {
    include!("loader.rs");
}
#[derive(Clone, Copy)]
struct Span {
    start: usize,
    width: usize,
}
include!("digit.rs");
fn cover(bits: usize, count: usize) -> Vec<Span> {
    (0..count)
        .map(|i| Span {
            start: i * bits / count,
            width: (i + 1) * bits / count - i * bits / count,
        })
        .collect()
}
fn bit_digit(s: &[u64; 4], span: Span) -> i16 {
    let bit = |n: usize| {
        if n < 256 {
            (s[n / 64] >> (n % 64)) & 1
        } else {
            0
        }
    };
    let value = (0..span.width)
        .map(|i| bit(span.start + i) << i)
        .sum::<u64>();
    let carry = if span.start == 0 {
        0
    } else {
        bit(span.start - 1)
    };
    (value as i64 + carry as i64 - ((bit(span.start + span.width - 1) << span.width) as i64)) as i16
}
fn short_value(s: [u64; 4], part: usize) -> i128 {
    let i = part * 2;
    let magnitude = s[i] as u128 | ((s[i + 1] & 0x7fff_ffff_ffff_ffff) as u128) << 64;
    assert!(magnitude < 1u128 << 126);
    if s[i + 1] >> 63 != 0 {
        -(magnitude as i128)
    } else {
        magnitude as i128
    }
}
fn fr_signed(x: i128) -> Fr {
    let a = x.unsigned_abs();
    let f = Fr::from_canonical([a as u64, (a >> 64) as u64, 0, 0]).unwrap();
    if x < 0 {
        -f
    } else {
        f
    }
}
struct Window {
    span: Span,
    arrivals: u64,
    high: u64,
    buckets: Vec<bool>,
    high_buckets: Vec<bool>,
}
impl Window {
    fn new(span: Span) -> Self {
        let n = 1usize << (span.width - 1);
        Self {
            span,
            arrivals: 0,
            high: 0,
            buckets: vec![false; n],
            high_buckets: vec![false; n],
        }
    }
    fn add(&mut self, d: i16, high: bool) {
        if d == 0 {
            return;
        }
        let b = d.unsigned_abs() as usize - 1;
        assert!(b < self.buckets.len());
        self.buckets[b] = true;
        self.arrivals += 1;
        if high {
            self.high += 1;
            self.high_buckets[b] = true;
        }
    }
    fn row(&self) -> Value {
        let occupied = self.buckets.iter().filter(|&&x| x).count() as u64;
        json!({"start":self.span.start,"width":self.span.width,"bucket_capacity":self.buckets.len(),"arrivals":self.arrivals,"high_arrivals":self.high,"occupied":occupied,"generic_fill_joins":self.arrivals-occupied,"high_occupied":self.high_buckets.iter().filter(|&&x|x).count(),"dense_weighted":dense_weighted(self.buckets.len())})
    }
}
fn dense_weighted(n: usize) -> Value {
    assert!(n % 256 == 0);
    let chunks = n / 256;
    let mut counts = vec![256usize];
    let mut joins = 0;
    let mut full = 0;
    let mut tails = 0;
    let mut layers = 0;
    while counts[0] > 1 {
        let next = counts[0] / 2;
        let pairs = counts.iter().map(|&c| c / 2).sum::<usize>();
        joins += pairs;
        full += pairs / 8 * 8;
        tails += pairs % 8;
        for c in &mut counts {
            *c = (*c).div_ceil(2);
        }
        counts.push(next);
        layers += 1;
    }
    assert_eq!((joins, full, tails, layers), (502, 496, 6, 8));
    json!({"chunks256":chunks,"dense_affine_joins":chunks*joins,"dense_full_lane_pairs":chunks*full,"dense_scalar_tail_pairs":chunks*tails,"dense_reduce_invocations":chunks*layers,"dense_vector_groups":chunks*full/8,"excludes_projective_horner_and_partial_combination":true})
}
fn sum(rows: &[Value], name: &str) -> u64 {
    rows.iter().map(|r| r[name].as_u64().unwrap()).sum()
}
fn aggregate(windows: &[Window]) -> Value {
    let rows = windows.iter().map(Window::row).collect::<Vec<_>>();
    let dense = |name: &str| {
        rows.iter()
            .map(|r| r["dense_weighted"][name].as_u64().unwrap())
            .sum::<u64>()
    };
    json!({"window_count":rows.len(),"bucket_capacity":sum(&rows,"bucket_capacity"),"arrivals":sum(&rows,"arrivals"),"high_arrivals":sum(&rows,"high_arrivals"),"occupied":sum(&rows,"occupied"),"generic_fill_joins":sum(&rows,"generic_fill_joins"),"phase_high_occupied":sum(&rows,"high_occupied"),"dense_weighted_affine_joins":dense("dense_affine_joins"),"dense_weighted_invocations":dense("dense_reduce_invocations"),"dense_weighted_full_lane_pairs":dense("dense_full_lane_pairs"),"dense_weighted_scalar_tail_pairs":dense("dense_scalar_tail_pairs"),"rows":rows})
}
fn main() {
    assert!(!std::path::Path::new(R).join("analysis.json").exists());
    let cases: Value =
        serde_json::from_slice(&fs::read(format!("{R}/cases.json")).unwrap()).unwrap();
    let c = &cases[0];
    let file = fs::File::open(c["image"].as_str().unwrap()).unwrap();
    let map = unsafe { memmap2::Mmap::map(&file).unwrap() };
    assert_eq!(format!("{:x}", Sha256::digest(&map[..])), c["image_sha256"]);
    let image: zopro_core::Image = Arc::new(map);
    let key = loader::load_key(image).unwrap();
    let witness = fs::read(format!("{R}/witness.bin")).unwrap();
    let witnessinfo: Value =
        serde_json::from_slice(&fs::read(format!("{R}/witness-info.json")).unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&witness)),
        witnessinfo["witness_sha256"]
    );
    let inputs = key.tagged_phi_inputs(&witness).unwrap();
    assert_eq!(inputs.geometry[0], 32768);
    assert_eq!(inputs.k.len(), 30529);
    assert!(inputs.h.len() <= 32768);
    assert_eq!(inputs.h.len(), inputs.h_identity.len());
    let mut original = cover(255, 20)
        .into_iter()
        .map(Window::new)
        .collect::<Vec<_>>();
    let mut proposed = cover(127, 10)
        .into_iter()
        .map(Window::new)
        .collect::<Vec<_>>();
    let lambda = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
    let weights = original
        .iter()
        .map(|w| Fr::from(2u64).pow(&[w.span.start as u64]))
        .collect::<Vec<_>>();
    let mut cache = b"TAGPHI01".to_vec();
    for n in [inputs.k.len(), inputs.h.len()]
        .into_iter()
        .chain(inputs.geometry)
    {
        cache.extend_from_slice(&(n as u64).to_le_bytes());
    }
    let mut populations = Vec::new();
    let mut checks = 0;
    for (name, values, identity) in [
        ("K", &inputs.k, &inputs.k_identity),
        ("H", &inputs.h, &inputs.h_identity),
    ] {
        assert_eq!(values.len(), identity.len());
        let mut high_nz = 0;
        let mut id_n = 0;
        for (s, &id) in values.iter().zip(identity) {
            let scalar = Fr::from_canonical(*s).unwrap();
            let short = zopro_core::tagged_phi_split(*s);
            let parts = [short_value(short, 0), short_value(short, 1)];
            assert_eq!(fr_signed(parts[0]) + lambda * fr_signed(parts[1]), scalar);
            let mut reconstructed = Fr::ZERO;
            for (w, power) in original.iter_mut().zip(&weights) {
                let d = digit_at(s, w.span);
                assert_eq!(d, bit_digit(s, w.span));
                reconstructed += *power
                    * if d < 0 {
                        -Fr::from(d.unsigned_abs() as u64)
                    } else {
                        Fr::from(d as u64)
                    };
                if !id {
                    w.add(d, false);
                }
            }
            assert_eq!(reconstructed, scalar);
            for part in 0..2 {
                let magnitude = parts[part].unsigned_abs();
                let words = [magnitude as u64, (magnitude >> 64) as u64, 0, 0];
                let mut reconstructed = 0i128;
                for w in &mut proposed {
                    let d = digit_at(&words, w.span);
                    assert_eq!(d, bit_digit(&words, w.span));
                    let signed = if parts[part] < 0 { -d } else { d };
                    let term = (signed as i128).checked_mul(1i128 << w.span.start).unwrap();
                    reconstructed = reconstructed.checked_add(term).unwrap();
                    if !id {
                        w.add(signed, part == 1);
                    }
                }
                assert_eq!(reconstructed, parts[part]);
            }
            if !id && parts[1] != 0 {
                high_nz += 1;
            }
            if id {
                id_n += 1;
            }
            for w in s.into_iter().chain(short.iter()) {
                cache.extend_from_slice(&w.to_le_bytes());
            }
            cache.push(id as u8);
            checks += 1;
        }
        populations.push(json!({"query":name,"rows":values.len(),"identity_points":id_n,"nonidentity_high_component_nonzero":high_nz}));
    }
    assert_eq!(cache.len(), 64 + (inputs.k.len() + inputs.h.len()) * 65);
    assert!(!std::path::Path::new(R)
        .join("private-input-cache.bin")
        .exists());
    fs::write(format!("{R}/private-input-cache.bin"), &cache).unwrap();
    let orig = aggregate(&original);
    let prop = aggregate(&proposed);
    let o = |name: &str| orig[name].as_i64().unwrap();
    let p = |name: &str| prop[name].as_i64().unwrap();
    let fill = p("generic_fill_joins") - o("generic_fill_joins");
    let weighted = p("dense_weighted_affine_joins") - o("dense_weighted_affine_joins");
    let beta = p("high_arrivals");
    let proxy = 6 * (fill + weighted) + beta;
    let output = json!({"source":"exact accepted primary Euler-v2 canonical K/H inputs; one existing solve+product FFT","geometry":inputs.geometry,"populations":populations,"scalar_rows_checked":checks,"cache":{"path":format!("{R}/private-input-cache.bin"),"bytes":cache.len(),"sha256":format!("{:x}",Sha256::digest(&cache)),"public_payload_included":false},"original":orig,"proposed_tagged":prop,"differences":{"generic_fill_joins":fill,"dense_weighted_affine_joins":weighted,"dense_weighted_invocations":p("dense_weighted_invocations")-o("dense_weighted_invocations"),"new_beta_multiplications":beta,"tag_beta_minus_phase_occupied_transforms":beta-p("phase_high_occupied"),"six_mul_join_proxy_delta":proxy},"model_limits":"Generic finite joins and dense weighted geometry, not actual exception/queue/field-operation counts or wall-time bounds. Proxy uses6multiplication-sites per generic join and one per input transform; excludes scalar inversions, tail/projective/Horner/recoding/admission/memory/scheduling costs. Proposed runtime is NOT implemented.","new_solver_calls":2,"new_product_FFT_calls":2,"new_SDK_proof_requests":0,"new_proofs":0,"accepted_source_changed":false,"native_runtime_candidate":false,"goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/analysis.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!("Checked{checks}actual canonical/decomposition/digit rows; arrivals{}->{}; weighted joins delta{weighted}; beta{beta};6M proxy delta{proxy}; NO runtime candidate/performance claim",o("arrivals"),p("arrivals"));
}
