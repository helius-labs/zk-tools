use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    sync::{Arc, Mutex},
    time::Instant,
};
use zopro_core::early_a_trial::Output;
#[path = "perf.rs"]
mod perf;
#[global_allocator]
static ALLOCATOR: zopro_core::alloc::Allocator = zopro_core::alloc::Allocator;
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn point_hash(o: &Output) -> String {
    let p = o.point.to_affine();
    let mut h = Sha256::new();
    for f in [p.x, p.y] {
        for w in f.to_canonical() {
            h.update(w.to_le_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
fn meta(o: &Output) -> Value {
    json!({"streamed":o.streamed,"grouped_fallback":o.grouped_fallback,"admission_fallback":o.admission_fallback,"published_chunks":o.published_chunks,"early_chunks":o.early_chunks,"early_values":o.early_values,"jobs":o.jobs,"feeds_before_solver_return":o.feeds_before_solver_return,"forecast_bytes":o.forecast_bytes})
}
fn pin(cpu: usize) {
    unsafe {
        let mut mask: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut mask);
        libc::CPU_SET(cpu, &mut mask);
        assert_eq!(
            libc::sched_setaffinity(0, std::mem::size_of_val(&mask), &mask),
            0
        );
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(
        args.len(),
        7,
        "driver CASES SHAPE WORKERS MODE ROUNDS ORDER"
    );
    let cases = read(&args[1]);
    let shape = &args[2];
    let workers = args[3].parse::<usize>().unwrap();
    assert!([1, 4, 16, 32].contains(&workers));
    let mode = &args[4];
    let rounds = args[5].parse::<usize>().unwrap();
    let order = args[6].parse::<usize>().unwrap();
    assert!(rounds > 0);
    let case = cases
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["shape"] == shape.as_str())
        .unwrap();
    let image_path = case["image"].as_str().unwrap();
    let image = fs::read(image_path).unwrap();
    assert_eq!(hash(&image), case["image_sha256"]);
    let request = fs::read_to_string(case["request"].as_str().unwrap()).unwrap();
    let observed = Arc::new(Mutex::new(Vec::new()));
    let rows = observed.clone();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .stack_size(512 * 1024)
        .start_handler(move |i| {
            let cpu = if workers == 1 { 2 } else { i };
            pin(cpu);
            rows.lock()
                .unwrap()
                .push(json!({"worker":i,"cpu":cpu,"tid":unsafe{libc::syscall(libc::SYS_gettid)}}));
        })
        .build()
        .unwrap();
    pool.broadcast(|_| ());
    let sdk = pool.install(|| {
        zopro_zolana::Prover::new(zopro_prover::Prover::load_image(Arc::new(image)).unwrap())
            .unwrap()
    });
    let witness = pool.install(|| sdk.witness(&request).unwrap());
    let bytes = witness
        .iter()
        .flat_map(|f| f.0.iter().flat_map(|w| w.to_le_bytes()))
        .collect::<Vec<_>>();
    drop(witness);
    let prepared = pool.install(|| sdk.prover().prepare_early_a());
    if mode == "decline" {
        assert!(prepared.is_none());
        println!(
            "CALLER {}",
            json!({"shape":shape,"workers":workers,"preparation_declined":true,"new_proofs":0})
        );
        return;
    }
    let prepared = prepared.expect("eligible prepared primary solver→A caller");
    assert_eq!(
        shape, "1x2",
        "only primary oracle is pinned for this screen"
    );
    let oracle = read(
        "/home/ubuntu/src/zopro-perf-work/coalesce-query176/consumers/oracle-1x2-A/stdout.log",
    );
    assert_eq!(oracle["independent_Arkworks_complete_MSM"], true);
    let check = |o: &Output| assert_eq!(point_hash(o), oracle["expected_canonical_result_sha256"]);
    let run = |on| pool.install(|| prepared.run(&bytes, on)).unwrap();
    if mode == "preflight" {
        let mut outputs = vec![];
        for _ in 0..2 {
            for on in [false, true] {
                let out = run(on);
                check(&out);
                if on {
                    assert!(out.streamed && !out.admission_fallback && !out.grouped_fallback);
                }
                outputs.push(meta(&out));
            }
        }
        assert!(pool.install(|| prepared.run(&[], true)).is_err());
        std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", "0");
        let denied = run(true);
        check(&denied);
        assert!(!denied.streamed && denied.admission_fallback);
        std::env::remove_var("ZOPRO_CPU_PREPARE_MAX_BYTES");
        std::env::set_var("ZOPRO_CPU_TOP_TREE", "0");
        let unsupported = run(true);
        check(&unsupported);
        assert!(!unsupported.streamed);
        std::env::remove_var("ZOPRO_CPU_TOP_TREE");
        println!(
            "CALLER {}",
            json!({"shape":shape,"workers":workers,"mode":mode,"outputs":outputs,"bad_witness_rejected":true,"admission_fallback_checked":true,"runtime_fallback_checked":true,"all_results_match_independent_A_MSM":true,"new_proofs":0})
        );
        return;
    }
    assert!(["mixed", "off", "on"].contains(&mode.as_str()));
    for _ in 0..3 {
        for on in [false, true] {
            check(&run(on));
        }
    }
    let count = rounds * if mode == "mixed" { 2 } else { 1 };
    let mut outputs = Vec::with_capacity(count);
    let mut samples = Vec::with_capacity(count);
    let mut control = perf::Control::from_env().unwrap();
    if let Some(c) = control.as_mut() {
        c.enable().unwrap();
    }
    let before = perf::Usage::now().unwrap();
    let start = Instant::now();
    for i in 0..rounds {
        for k in 0..if mode == "mixed" { 2 } else { 1 } {
            let on = match mode.as_str() {
                "on" => true,
                "off" => false,
                _ => (i + order + k) % 2 == 1,
            };
            let clock = Instant::now();
            let out = std::hint::black_box(run(on));
            samples.push(json!({"on":on,"ns":clock.elapsed().as_nanos()as u64}));
            outputs.push(out);
        }
    }
    let elapsed = start.elapsed().as_nanos() as u64;
    let after = perf::Usage::now().unwrap();
    if let Some(c) = control.as_mut() {
        c.disable().unwrap();
    }
    for o in &outputs {
        check(o);
    }
    for on in [false, true] {
        check(&run(on));
    }
    let metadata = outputs.iter().map(meta).collect::<Vec<_>>();
    let mut worker_rows = observed.lock().unwrap().clone();
    worker_rows.sort_by_key(|r| r["worker"].as_u64().unwrap());
    pool.broadcast(|ctx| {
        assert_eq!(
            unsafe { libc::sched_getcpu() },
            if workers == 1 { 2 } else { ctx.index() } as i32
        )
    });
    println!(
        "CALLER {}",
        json!({"shape":shape,"workers":workers,"mode":mode,"order":order,"pid":std::process::id(),"rounds":rounds,"samples":samples,"metadata":metadata,"elapsed_ns":elapsed,"priming_calls":6,"postcheck_calls":2,"all_results_match_independent_A_MSM":true,"whole_solver_A_chain_only":true,"image_path":image_path,"image_sha256":case["image_sha256"],"request_sha256":hash(request.as_bytes()),"worker_rows":worker_rows,"usage":after.difference(&before),"new_proofs":0})
    );
}
