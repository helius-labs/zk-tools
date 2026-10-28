use std::{fs, io::Write};
#[global_allocator]
static ALLOCATOR: zopro_prover::zopro_core::alloc::Allocator =
    zopro_prover::zopro_core::alloc::Allocator;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args_os().collect();
    assert_eq!(a.len(), 3);
    let bytes = fs::read(&a[1])?;
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[2])?;
    let mut out = std::io::BufWriter::new(file);
    zopro_prover::Prover::write_image(&bytes, &mut out)?;
    out.flush()?;
    Ok(())
}
