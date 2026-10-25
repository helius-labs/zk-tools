//! Verify removal of diagnostic insertions recovers all production call sites.
use std::{fs, path::Path};
fn main() {
    let root = Path::new("/home/ubuntu/src/zk-tools/zopro");
    let overlay = Path::new("/tmp/zopro-phase-cpu/overlay");
    for path in ["crates/zopro-core/src/lib.rs", "crates/zopro-prover/src/lib.rs", "crates/zopro-zolana/src/lib.rs"] {
        let original = fs::read_to_string(root.join(path)).unwrap();
        let candidate = fs::read_to_string(overlay.join(path)).unwrap();
        let mut recovered = candidate.lines().filter(|line| {
            !line.trim_start().starts_with("#[cfg(zopro_phase_cpu)]")
                && *line != "#[cfg(all(zopro_phase_cpu, target_os = \"linux\"))]"
                && *line != "pub mod phase_cpu;"
        }).collect::<Vec<_>>().join("\n");
        recovered.push('\n');
        recovered = recovered.replace("#[cfg(zopro_phase_cpu)] crate::phase_cpu::mark(\"core_cleanup\");", "");
        assert_eq!(recovered, original, "non-insertion change in {path}");
        println!("BYTE_RECOVERY_PASS {path}");
    }
}
