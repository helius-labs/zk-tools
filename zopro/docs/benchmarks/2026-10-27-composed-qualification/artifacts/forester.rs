use std::{fs, path::Path, sync::Arc, time::Instant};
use zopro_bn254::Fr;
#[global_allocator]
static ALLOCATOR: zopro_prover::zopro_core::alloc::Allocator =
    zopro_prover::zopro_core::alloc::Allocator;
fn event(stage: &str, mut v: serde_json::Value) {
    let mut u = unsafe { std::mem::zeroed::<libc::rusage>() };
    assert_eq!(unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut u) }, 0);
    v["stage"] = stage.into();
    v["process_peak_rss_kib"] = u.ru_maxrss.into();
    println!("EVENT {v}");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4);
    let out = Path::new(&a[3]);
    fs::create_dir_all(out).unwrap();
    let start = Instant::now();
    event("load_begin", serde_json::json!({"image":a[1]}));
    let f = fs::File::open(&a[1]).unwrap();
    let map = Arc::new(unsafe { memmap2::Mmap::map(&f) }.unwrap());
    drop(f);
    let mut p = zopro_prover::Prover::load_image(map).unwrap();
    event(
        "loaded",
        serde_json::json!({"seconds":start.elapsed().as_secs_f64(),"public":p.public_inputs(),"inputs":p.names().len()}),
    );
    let inputs: Vec<_> = (0..2)
        .map(|i| fs::read(Path::new(&a[2]).join(format!("{i}.witness"))).unwrap())
        .collect();
    for round in 0..2 {
        let b = &inputs[round];
        let word = |i| u32::from_be_bytes(b[i..i + 4].try_into().unwrap()) as usize;
        assert_eq!(word(0), p.public_inputs());
        assert_eq!(word(0) + word(4), word(8));
        assert_eq!(word(8), p.names().len());
        assert_eq!(b.len(), 12 + 32 * word(8));
        let t = Instant::now();
        let w: Vec<Fr> = b[12..]
            .chunks_exact(32)
            .map(|v| Fr::from_be_bytes(v.try_into().unwrap()).unwrap())
            .collect();
        let decoded = t.elapsed().as_secs_f64();
        let proved = p.prove(&w).unwrap();
        let proof = proved.proof.to_gnark();
        let public = proved.public_witness();
        assert_eq!(proof.len(), 164);
        let seconds = t.elapsed().as_secs_f64();
        fs::write(out.join(format!("{round}.proof")), proof).unwrap();
        fs::write(out.join(format!("{round}.public")), public).unwrap();
        event(
            "proof",
            serde_json::json!({"round":round,"cold":round==0,"variant":round,"verified_seconds":seconds,"decode_seconds":decoded,"prove_ms":proved.prove_ms,"verify_ms":proved.verify_ms}),
        );
    }
    event("complete", serde_json::json!({"proofs":2}));
}
