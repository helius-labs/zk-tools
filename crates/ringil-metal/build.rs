//! Compiles the shaders to a metallib for the target when the Metal
//! toolchain is at hand (xcrun), so that the first proof does not compile
//! them on the device: the G2 kernels take a minute from source. Without
//! the toolchain the library compiles from source at run time.
use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    println!("cargo::rerun-if-changed=shaders");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let simulator = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default() == "sim";
    let sdk = match (os.as_str(), simulator) {
        ("macos", _) => "macosx",
        ("ios", false) => "iphoneos",
        ("ios", true) => "iphonesimulator",
        _ => return,
    };
    let source: String = ["field", "curve", "buckets", "quotient"]
        .iter()
        .map(|name| fs::read_to_string(format!("shaders/{name}.metal")).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let all = out.join("ringil.metal");
    fs::write(&all, source).unwrap();
    let air = out.join("ringil.air");
    let lib = out.join("ringil.metallib");
    let compiled = Command::new("xcrun")
        .args(["-sdk", sdk, "metal", "-c", "-O2"])
        .arg(&all)
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
            .is_ok_and(|s| s.success());
    println!("cargo::rustc-check-cfg=cfg(metallib)");
    if compiled {
        println!("cargo::rustc-cfg=metallib");
    } else {
        println!("cargo::warning=no Metal toolchain: the shaders compile from source at run time");
    }
}
