//! Compiles the shaders to metallibs for the target when the Metal
//! toolchain is at hand (xcrun), so that a phone does not compile them at
//! its first key load: from source that took seconds for the G1 kernels and
//! minutes for G2. Two libraries, both with the G2 kernels: the default
//! optimization level and the size level, which a GPU whose MSM disagreed
//! with the CPU retries with. Without the toolchain the library compiles
//! from source at run time.
use std::{env, fs, path::Path, path::PathBuf, process::Command};

fn compile(
    sdk: &str,
    version_flag: &str,
    source: &Path,
    optimize: &str,
    out: &Path,
    name: &str,
) -> bool {
    let air = out.join(format!("{name}.air"));
    let lib = out.join(format!("{name}.metallib"));
    Command::new("xcrun")
        // Metal 3.0 and the oldest OS ringil supports: the SDK's newest
        // language version would not load on an older iPhone.
        .args([
            "-sdk",
            sdk,
            "metal",
            "-c",
            "-std=metal3.0",
            version_flag,
            optimize,
        ])
        .arg(source)
        .arg("-o")
        .arg(&air)
        .status()
        .is_ok_and(|s| s.success())
        && Command::new("xcrun")
            .args(["-sdk", sdk, "metallib"])
            .arg(&air)
            .arg("-o")
            .arg(&lib)
            .status()
            .is_ok_and(|s| s.success())
}

fn main() {
    println!("cargo::rerun-if-changed=shaders");
    println!("cargo::rustc-check-cfg=cfg(metallib)");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let simulator = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default() == "sim";
    let (sdk, version_flag) = match (os.as_str(), simulator) {
        ("macos", _) => ("macosx", "-mmacosx-version-min=13.0"),
        ("ios", false) => ("iphoneos", "-mios-version-min=16.0"),
        ("ios", true) => ("iphonesimulator", "-mios-simulator-version-min=16.0"),
        _ => return,
    };
    let mut source: String = ["field", "curve", "buckets", "quotient"]
        .iter()
        .map(|name| fs::read_to_string(format!("shaders/{name}.metal")).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    source.push_str("\nBUCKET_KERNELS(fp2_t, g2)\n");
    let all = out.join("ringil.metal");
    fs::write(&all, source).unwrap();
    let compiled = compile(sdk, version_flag, &all, "-O2", &out, "ringil")
        && compile(sdk, version_flag, &all, "-Os", &out, "ringil-size");
    if compiled {
        println!("cargo::rustc-cfg=metallib");
    } else {
        println!("cargo::warning=no Metal toolchain: the shaders compile from source at run time");
    }
}
