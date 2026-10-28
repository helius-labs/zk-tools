use std::{
    fs,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
#[global_allocator]
static ALLOCATOR: zopro_prover::zopro_core::alloc::Allocator =
    zopro_prover::zopro_core::alloc::Allocator;
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4);
    let image = &a[1];
    let request = fs::read_to_string(&a[2]).unwrap();
    let out = Path::new(&a[3]);
    fs::create_dir_all(out).unwrap();
    let f = fs::File::open(image).unwrap();
    let map = Arc::new(unsafe { memmap2::Mmap::map(&f) }.unwrap());
    drop(f);
    let mut sdk =
        zopro_zolana::Prover::new(zopro_prover::Prover::load_image(map).unwrap()).unwrap();
    for _ in 0..3 {
        std::hint::black_box(sdk.prove(&request).unwrap());
    }
    let tids = || {
        let mut v: Vec<u32> = fs::read_dir("/proc/self/task")
            .unwrap()
            .map(|e| e.unwrap().file_name().to_str().unwrap().parse().unwrap())
            .collect();
        v.sort_unstable();
        v
    };
    let before = tids();
    assert_eq!(before.len(), rayon::current_num_threads() + 1);
    let barrier = Path::new(&std::env::var("ZOPRO_PRESSURE_PAUSE").unwrap()).to_owned();
    fs::write(barrier.join("ready"), b"ready").unwrap();
    let start = Instant::now();
    while !barrier.join("go").exists() {
        assert!(
            start.elapsed() < Duration::from_secs(90),
            "pressure barrier timeout"
        );
        std::thread::sleep(Duration::from_millis(5));
    }
    let mut times = Vec::new();
    for round in 0..2 {
        let t = Instant::now();
        let p = sdk.prove(&request).unwrap();
        assert_eq!(p.proof.len(), 164);
        assert_eq!(p.public_inputs.len(), 44);
        times.push(t.elapsed().as_secs_f64() * 1000.);
        fs::write(out.join(format!("{round}.proof")), p.proof).unwrap();
        fs::write(out.join(format!("{round}.public")), p.public_inputs).unwrap();
    }
    let after = tids();
    assert_eq!(before, after);
    println!(
        "COMPLETE {}",
        serde_json::json!({"priming":3,"rounds":2,"SDK_requests":5,"post_ms":times,"workers":rayon::current_num_threads(),"before_tids":before,"after_tids":after})
    );
}
