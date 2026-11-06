use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
use zopro_bn254::Fr;
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn get(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}
fn signed(x: i128) -> Fr {
    let xabs = x.unsigned_abs();
    let f = Fr::from_canonical([xabs as u64, (xabs >> 64) as u64, 0, 0]).unwrap();
    if x < 0 {
        -f
    } else {
        f
    }
}
fn digit(s: &[u64; 4], start: usize, width: usize) -> i16 {
    let mut value = 0i64;
    for bit in 0..width {
        let index = start + bit;
        let b = if index < 256 {
            (s[index / 64] >> (index % 64)) & 1
        } else {
            0
        };
        value += if bit + 1 == width {
            -(b as i64) * (1i64 << bit)
        } else {
            (b as i64) << bit
        };
    }
    if start > 0 {
        let i = start - 1;
        value += ((s[i / 64] >> (i % 64)) & 1) as i64;
    }
    value as i16
}
struct Row {
    start: usize,
    width: usize,
    arrivals: u64,
    high: u64,
    seen: BTreeSet<usize>,
    high_seen: BTreeSet<usize>,
}
fn rows(bits: usize, count: usize) -> Vec<Row> {
    (0..count)
        .map(|i| Row {
            start: i * bits / count,
            width: (i + 1) * bits / count - i * bits / count,
            arrivals: 0,
            high: 0,
            seen: BTreeSet::new(),
            high_seen: BTreeSet::new(),
        })
        .collect()
}
fn put(row: &mut Row, d: i16, high: bool) {
    if d == 0 {
        return;
    }
    let b = d.unsigned_abs() as usize - 1;
    assert!(b < (1 << row.width - 1));
    row.arrivals += 1;
    row.seen.insert(b);
    if high {
        row.high += 1;
        row.high_seen.insert(b);
    }
}
fn dense(n: usize) -> [u64; 4] {
    assert_eq!(n % 256, 0);
    let c = (n / 256) as u64;
    let mut total = 0;
    let mut lanes = 0;
    let mut tails = 0;
    for level in 1..=8 {
        let pairs = level * (256 >> level);
        total += pairs;
        lanes += pairs / 8 * 8;
        tails += pairs % 8;
    }
    assert_eq!((total, lanes, tails), (502, 496, 6));
    [c * total, c * lanes, c * tails, c * 8]
}
fn check_rows(rows: &[Row], a: &Value) {
    assert_eq!(rows.len(), a["rows"].as_array().unwrap().len());
    let mut sums = BTreeMap::<&str, u64>::new();
    for (row, saved) in rows.iter().zip(a["rows"].as_array().unwrap()) {
        let n = 1usize << (row.width - 1);
        let occupied = row.seen.len() as u64;
        for (k, n) in [
            ("start", row.start as u64),
            ("width", row.width as u64),
            ("bucket_capacity", n as u64),
            ("arrivals", row.arrivals),
            ("high_arrivals", row.high),
            ("occupied", occupied),
            ("generic_fill_joins", row.arrivals - occupied),
            ("high_occupied", row.high_seen.len() as u64),
        ] {
            assert_eq!(saved[k], n);
            *sums.entry(k).or_default() += n;
        }
        let d = dense(n);
        for (k, n) in [
            ("dense_affine_joins", d[0]),
            ("dense_full_lane_pairs", d[1]),
            ("dense_scalar_tail_pairs", d[2]),
            ("dense_reduce_invocations", d[3]),
        ] {
            assert_eq!(saved["dense_weighted"][k], n);
            *sums.entry(k).or_default() += n;
        }
    }
    for (target, source) in [
        ("bucket_capacity", "bucket_capacity"),
        ("arrivals", "arrivals"),
        ("high_arrivals", "high_arrivals"),
        ("occupied", "occupied"),
        ("generic_fill_joins", "generic_fill_joins"),
        ("phase_high_occupied", "high_occupied"),
        ("dense_weighted_affine_joins", "dense_affine_joins"),
        ("dense_weighted_full_lane_pairs", "dense_full_lane_pairs"),
        (
            "dense_weighted_scalar_tail_pairs",
            "dense_scalar_tail_pairs",
        ),
        ("dense_weighted_invocations", "dense_reduce_invocations"),
    ] {
        assert_eq!(a[target], sums[source]);
    }
}
fn main() {
    assert!(!Path::new(R).join("review.json").exists());
    let a = read(format!("{R}/analysis.json"));
    let pins = read(format!("{R}/repair-pins.json"));
    let mut checked = 0;
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
        checked += 1;
    }
    let manifest=read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let audit = read(format!("{R}/source-audit.json"));
    let mut source_copies = 0;
    let mut exact = 0;
    for directory in ["source", "source-fixed"] {
        for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
            assert_eq!(
                hash(&fs::read(Path::new(Z).join(p)).unwrap()),
                h.as_str().unwrap()
            );
            let mut b = fs::read(Path::new(R).join(directory).join(p)).unwrap();
            if hash(&b) == h.as_str().unwrap() {
                exact += 1;
            }
            if let Some(suffix) = audit["suffixes"].get(p) {
                let suffix = suffix.as_str().unwrap().as_bytes();
                assert!(b.ends_with(suffix));
                b.truncate(b.len() - suffix.len());
            }
            assert_eq!(hash(&b), h.as_str().unwrap());
            source_copies += 1;
        }
    }
    assert_eq!((source_copies, exact), (352, 348));
    let mut commands = 0;
    let mut failures = 0;
    for (plan, dir) in [("plan.json", "runs"), ("repair-plan.json", "runs-fixed")] {
        let p = read(format!("{R}/{plan}"));
        for step in p["steps"].as_array().unwrap() {
            let n = step["name"].as_str().unwrap();
            let base = format!("{R}/{dir}/{n}");
            let exit = read(format!("{base}/exit.json"));
            let expected = if n == "primary-work-model" { 101 } else { 0 };
            assert_eq!(exit["returncode"], expected);
            assert_eq!(exit["success"], expected == 0);
            if expected != 0 {
                failures += 1;
            }
            for f in ["stdout", "stderr"] {
                assert_eq!(
                    hash(&fs::read(format!("{base}/{f}.log")).unwrap()),
                    exit[format!("{f}_sha256")]
                );
            }
            let cmd = read(format!("{base}/command.json"));
            assert_eq!(
                &cmd["argv"].as_array().unwrap()[4..],
                step["argv"].as_array().unwrap()
            );
            commands += 1;
        }
    }
    assert_eq!((commands, failures), (8, 1));
    let failed = fs::read_to_string(format!("{R}/runs/primary-work-model/stderr.log")).unwrap();
    assert!(failed.contains("30922") && failed.contains("32768"));
    assert_eq!(a["new_solver_calls"], 2);
    assert_eq!(a["new_product_FFT_calls"], 2);
    assert_eq!(a["new_proofs"], 0);
    let b = fs::read(format!("{R}/private-input-cache.bin")).unwrap();
    assert_eq!(&b[..8], b"TAGPHI01");
    assert_eq!(hash(&b), a["cache"]["sha256"]);
    assert_eq!(b.len(), a["cache"]["bytes"].as_u64().unwrap() as usize);
    let (nk, nh) = (get(&b, 8) as usize, get(&b, 16) as usize);
    assert_eq!((nk, nh), (30529, 30922));
    for i in 0..5 {
        assert_eq!(get(&b, 24 + i * 8), a["geometry"][i]);
    }
    assert_eq!(b.len(), 64 + (nk + nh) * 65);
    let lambda = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
    let mut original = rows(255, 20);
    let mut proposed = rows(127, 10);
    let powers = original
        .iter()
        .map(|r| Fr::from(2u64).pow(&[r.start as u64]))
        .collect::<Vec<_>>();
    let mut ids = [0; 2];
    let mut high_nz = [0; 2];
    let mut checks = 0;
    for (i, row) in b[64..].chunks_exact(65).enumerate() {
        let p = usize::from(i >= nk);
        let words = core::array::from_fn(|j| get(row, j * 8));
        let scalar = Fr::from_canonical(words).unwrap();
        let short: [u64; 4] = core::array::from_fn(|j| get(row, 32 + j * 8));
        assert!(row[64] <= 1);
        let id = row[64] != 0;
        ids[p] += id as usize;
        let mut part_values = [0i128; 2];
        for part in 0..2 {
            let j = part * 2;
            let magnitude =
                short[j] as u128 | ((short[j + 1] & 0x7fff_ffff_ffff_ffff) as u128) << 64;
            assert!(magnitude < 1u128 << 126);
            part_values[part] = if short[j + 1] >> 63 == 0 {
                magnitude as i128
            } else {
                -(magnitude as i128)
            };
            let w = [magnitude as u64, (magnitude >> 64) as u64, 0, 0];
            let mut sum = 0i128;
            for r in &mut proposed {
                let mut d = digit(&w, r.start, r.width);
                if part_values[part] < 0 {
                    d = -d;
                }
                sum = sum
                    .checked_add((d as i128).checked_mul(1i128 << r.start).unwrap())
                    .unwrap();
                if !id {
                    put(r, d, part == 1);
                }
            }
            assert_eq!(sum, part_values[part]);
        }
        assert_eq!(
            signed(part_values[0]) + lambda * signed(part_values[1]),
            scalar
        );
        if !id && part_values[1] != 0 {
            high_nz[p] += 1;
        }
        let mut sum = Fr::ZERO;
        for (r, power) in original.iter_mut().zip(&powers) {
            let d = digit(&words, r.start, r.width);
            sum += *power * signed(d as i128);
            if !id {
                put(r, d, false);
            }
        }
        assert_eq!(sum, scalar);
        checks += 1;
    }
    assert_eq!(checks, 61451);
    assert_eq!(a["scalar_rows_checked"], checks);
    for p in 0..2 {
        assert_eq!(a["populations"][p]["identity_points"], ids[p]);
        assert_eq!(
            a["populations"][p]["nonidentity_high_component_nonzero"],
            high_nz[p]
        );
    }
    check_rows(&original, &a["original"]);
    check_rows(&proposed, &a["proposed_tagged"]);
    let o = &a["original"];
    let p = &a["proposed_tagged"];
    let delta = |key: &str| p[key].as_i64().unwrap() - o[key].as_i64().unwrap();
    let beta = p["high_arrivals"].as_i64().unwrap();
    let proxy = 6 * (delta("generic_fill_joins") + delta("dense_weighted_affine_joins")) + beta;
    assert_eq!(a["differences"]["six_mul_join_proxy_delta"], proxy);
    let phase_high = p["phase_high_occupied"].as_i64().unwrap();
    assert_eq!(
        a["differences"]["tag_beta_minus_phase_occupied_transforms"],
        beta - phase_high
    );
    // The historical phase version transforms BOTH affine and projective arrays.
    // Each has at most one entry per occupied high bucket. Count both, never
    // mistake the single-affine occupancy difference for a complete phase cost.
    let old = fs::read_to_string("/home/ubuntu/src/zopro-perf-work/glv-kh-borrowed175/glv_kh.rs")
        .unwrap();
    assert!(old.contains("for p in &mut state.affine {if !p.is_identity(){p.x*=BETA;}}"));
    assert!(old.contains("for p in &mut state.projective {p.x*=BETA;}"));
    let phase_upper = 2 * phase_high;
    let extra = beta - phase_upper;
    assert!(proxy > 0 && extra > 0);
    let baseline_proxy = 6
        * (o["generic_fill_joins"].as_i64().unwrap()
            + o["dense_weighted_affine_joins"].as_i64().unwrap());
    let result = json!({"review_passed":true,"pins_checked":checked,"source_copies_recovered":352,"original_copies_exact":348,"workflow_commands":8,"successful_commands":7,"failed_commands":1,"initial_successful_stages_reused":4,"new_solver_calls_total":2,"new_product_FFT_calls_total":2,"scalar_rows_checked":checks,"all_original_and_component_reconstructions_passed":true,"all_model_rows_and_dense_recurrences_reproduced":true,"private_cache_sha256":hash(&b),"same_geometry_phase_comparison":{"high_occupied":phase_high,"affine_plus_projective_transform_upper_bound":phase_upper,"tag_transform_count":beta,"tag_minus_phase_upper_bound":extra,"not_a_runtime_operation_or_latency_measurement":true},"six_mul_proxy":{"baseline":baseline_proxy,"delta":proxy,"delta_percent":100.*proxy as f64/baseline_proxy as f64,"not_a_wall_time_or_total_CPU_work_bound":true},"decision":"close tagged per-input transform hypothesis without a native algorithm or timing campaign; arithmetic proxy rises and same-geometry phase placement requires far fewer transforms. Inversion/root/collision/recoding/memory/scheduling effects remain unpriced.","new_proofs":0,"goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Independent review PASS {checked}pins/352source copies/8commands/61451scalar rows; proxy+{proxy}; tag-beta minus max2phase upper={extra}; no native algorithm/timing");
}
