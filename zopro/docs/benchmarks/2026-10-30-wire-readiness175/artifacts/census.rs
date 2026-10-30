use std::{fs, sync::Arc};
mod model;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args_os().collect();
    if a.len() != 4 {
        return Err("census export IMAGE OUT or analyze SOLVER_IMAGE OUT".into());
    }
    match a[1].to_str().unwrap() {
        "export" => {
            let file = fs::File::open(&a[2])?;
            let map = unsafe { memmap2::Mmap::map(&file)? };
            let p = zopro_prover::Prover::load_image(Arc::new(map))?;
            let bytes = p.solver_image().ok_or("missing solver image")?;
            fs::write(&a[3], &bytes)?;
            println!("EXPORTED_SOLVER bytes={} new_proofs=0", bytes.len());
        }
        "analyze" => {
            let b = fs::read(&a[2])?;
            let v = model::census(&b)?;
            fs::write(&a[3], serde_json::to_vec_pretty(&v)?)?;
            println!(
                "STRUCTURAL_CENSUS ops={} wires={} new_proofs=0 new_timings=0",
                v["ops"], v["wires"]
            );
        }
        _ => return Err("unknown mode".into()),
    }
    Ok(())
}
