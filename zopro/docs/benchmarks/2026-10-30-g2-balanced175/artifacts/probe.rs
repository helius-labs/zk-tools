use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    hint::black_box,
    sync::{Arc, Mutex},
    time::Instant,
};
use zopro_bn254::{Fq, Fq2, G2Affine};
#[path = "perf.rs"]
mod perf;
#[global_allocator]
static ALLOCATOR: zopro_core::alloc::Allocator = zopro_core::alloc::Allocator;
fn affinity(cpu: usize) {
    unsafe {
        let mut mask: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut mask);
        libc::CPU_SET(cpu, &mut mask);
        assert_eq!(
            libc::sched_setaffinity(0, std::mem::size_of_val(&mask), &mask),
            0
        );
        let mut got: libc::cpu_set_t = std::mem::zeroed();
        assert_eq!(
            libc::sched_getaffinity(0, std::mem::size_of_val(&got), &mut got),
            0
        );
        for n in 0..libc::CPU_SETSIZE as usize {
            assert_eq!(libc::CPU_ISSET(n, &got), n == cpu);
        }
        assert_eq!(libc::sched_getcpu(), cpu as i32);
    }
}
fn digest(p: G2Affine) -> String {
    let mut h = Sha256::new();
    for f in [p.x.c0, p.x.c1, p.y.c0, p.y.c1] {
        for w in f.to_canonical() {
            h.update(w.to_le_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    assert_eq!(a.len(), 5);
    let workers = a[3].parse::<usize>().unwrap();
    let calls = a[4].parse::<usize>().unwrap();
    assert!([1, 16, 32].contains(&workers) && calls > 0);
    let bytes = fs::read(&a[1]).unwrap();
    let oracle: Value = serde_json::from_slice(&fs::read(&a[2]).unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        oracle["fixture_sha256"]
    );
    assert_eq!(oracle["independent_Arkworks_complete_MSM"], true);
    assert_eq!(&bytes[..8], b"B2SCAN01");
    let n = usize::try_from(u64::from_le_bytes(bytes[8..16].try_into().unwrap())).unwrap();
    assert_eq!(n, oracle["points"]);
    assert_eq!(bytes.len(), 16 + 160 * n);
    let bases = bytes[16..16 + 128 * n]
        .chunks_exact(128)
        .map(|raw| {
            let fields = raw
                .chunks_exact(32)
                .map(|b| Fq::from_be_bytes(b.try_into().unwrap()).expect("canonical Fq"))
                .collect::<Vec<_>>();
            G2Affine::new(
                Fq2::new(fields[1], fields[0]),
                Fq2::new(fields[3], fields[2]),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(bases.len(), n);
    let original = bytes[16 + 128 * n..]
        .chunks_exact(32)
        .map(|b| {
            core::array::from_fn(|i| u64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap()))
        })
        .collect::<Vec<[u64; 4]>>();
    assert_eq!(original.len(), n);
    drop(bytes);
    let before_points = Sha256::digest(
        bases
            .iter()
            .flat_map(|p| [p.x.c0, p.x.c1, p.y.c0, p.y.c1])
            .flat_map(|f: Fq| f.0.into_iter().flat_map(u64::to_le_bytes))
            .collect::<Vec<_>>(),
    );
    let observed = Arc::new(Mutex::new(vec![]));
    let start = observed.clone();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .start_handler(move |i| {
            let cpu = if workers == 1 { 2 } else { i };
            affinity(cpu);
            start
                .lock()
                .unwrap()
                .push(json!({"index":i,"cpu":cpu,"tid":unsafe{libc::syscall(libc::SYS_gettid)}}));
        })
        .build()
        .unwrap();
    pool.broadcast(|_| ());
    let mut worker_rows = observed.lock().unwrap().clone();
    worker_rows.sort_by_key(|r| r["index"].as_u64().unwrap());
    assert_eq!(worker_rows.len(), workers);
    let mut words = original.clone();
    for _ in 0..3 {
        words.copy_from_slice(&original);
        let out =
            pool.install(|| zopro_core::g2_queue_probe(black_box(&bases), black_box(&mut words)));
        assert_eq!(digest(out), oracle["expected_canonical_result_sha256"]);
    }
    let mut control = perf::Control::from_env().unwrap();
    let mut outputs = Vec::with_capacity(calls);
    let mut ns = Vec::with_capacity(calls);
    if let Some(c) = &mut control {
        c.enable().unwrap();
    }
    let before = perf::Usage::now().unwrap();
    let wave = Instant::now();
    for _ in 0..calls {
        let clock = Instant::now();
        words.copy_from_slice(black_box(&original));
        let out = black_box(
            pool.install(|| zopro_core::g2_queue_probe(black_box(&bases), black_box(&mut words))),
        );
        ns.push(clock.elapsed().as_nanos() as u64);
        outputs.push(out);
    }
    let elapsed = wave.elapsed().as_nanos() as u64;
    let after = perf::Usage::now().unwrap();
    if let Some(c) = &mut control {
        c.disable().unwrap();
    }
    for out in outputs {
        assert_eq!(digest(out), oracle["expected_canonical_result_sha256"]);
    }
    pool.broadcast(|ctx| {
        let i = ctx.index();
        assert_eq!(
            unsafe { libc::sched_getcpu() },
            if workers == 1 { 2 } else { i } as i32
        );
    });
    let after_points = Sha256::digest(
        bases
            .iter()
            .flat_map(|p| [p.x.c0, p.x.c1, p.y.c0, p.y.c1])
            .flat_map(|f: Fq| f.0.into_iter().flat_map(u64::to_le_bytes))
            .collect::<Vec<_>>(),
    );
    assert_eq!(before_points, after_points);
    let mut sorted = ns.clone();
    sorted.sort_unstable();
    println!(
        "RESULT {}",
        json!({"pid":std::process::id(),"points":n,"workers":workers,"calls":calls,"primes":3,"query_ns":ns,"median_ms":sorted[sorted.len()/2]as f64/1e6,"loop_elapsed_ns":elapsed,"usage":after.difference(&before),"worker_rows":worker_rows,"input_points_unchanged":true,"all_outputs_match_independent_Arkworks":true,"result_sha256":oracle["expected_canonical_result_sha256"],"reset_copy_inside_clock":true,"query_includes_grouping_and_all_MSM_stages":true,"solver_outside_clock":true,"SDK_requests":0})
    );
}
