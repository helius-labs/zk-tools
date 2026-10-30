mod model;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 { return Err("census SOLVER_IMAGE OUTPUT_JSON".into()); }
    let bytes = std::fs::read(&args[1])?;
    let v = model::census(&bytes)?;
    std::fs::write(&args[2], serde_json::to_vec_pretty(&v)?)?;
    println!("AFFINE_CLOSURE wires={} assignments={} affine={} constant={} new_proofs=0",v["wires"],v["assignments"],v["expanded_affine_assignments"],v["expanded_constant_assignments"]);
    Ok(())
}
