use std::{fs,path::Path};
fn main(){let root=Path::new("/tmp/zopro-serial-fft-leaves/overlay/crates/zopro-core/src/fft");let path=root.join("pair.rs");let original=fs::read_to_string(&path).unwrap();let start=original.find("fn stored_transform<S: PairStorage, const FORWARD: bool>(").unwrap();let end=original[start..].find("\n// Keep the fixed column scratch").unwrap()+start;let function=&original[start..end];assert_eq!(function.matches("stored_transform::<S, FORWARD>(part, roots, level + 5)").count(),2);let inner=function.replacen("fn stored_transform<","fn transform_inner<",1).replacen("    level: usize,\n) {","    level: usize,\n    cutoff: usize,\n) {",1).replace("dit::<5>(&mut local, roots, level);","dit_bounded(&mut local, roots, level, cutoff);").replace("dif::<5>(&mut local, roots, level);","dif_bounded(&mut local, roots, level, cutoff);").replace("stored_transform::<S, FORWARD>(part, roots, level + 5)","transform_inner::<S, FORWARD>(part, roots, level + 5, cutoff)");assert!(inner.contains("cutoff: usize"));let changed=function.replacen(") {\n", ") {\n    #[cfg(zopro_serial_fft_leaves)]\n    if serial_leaf::enabled() { return serial_leaf::transform::<S, FORWARD>(values, roots, level); }\n",1);let mut source=String::with_capacity(original.len()+256);source.push_str(&original[..start]);source.push_str(&changed);source.push_str(&original[end..]);source.push_str("\n#[cfg(any(zopro_serial_fft_leaves, test))]\nmod serial_leaf;\n");fs::write(&path,source).unwrap();let head=r#"//! Bound nested forks inside existing expanded FFT leaves.
use super::*;

#[cfg(zopro_serial_fft_leaves)]
pub(super) fn enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(||std::env::var("ZOPRO_CPU_SERIAL_FFT_LEAVES").is_ok_and(|v|v=="1"))
}

fn cutoff(vectors: usize, workers: usize) -> usize {
    let budget=workers.max(1).saturating_mul(2);
    let target=(vectors/budget).clamp(64,1024);
    1usize << target.ilog2()
}

pub(super) fn transform<S: PairStorage, const FORWARD: bool>(
    values: &mut [S], roots: &Twiddles, level: usize,
) {
    let cutoff=cutoff(values.len(),rayon::current_num_threads());
    transform_inner::<S,FORWARD>(values,roots,level,cutoff);
}

"#;fs::write(root.join("pair/serial_leaf.rs"),format!("{head}{inner}\n")).unwrap();println!("Original OFF function retained; separate bounded leaf module generated");}
