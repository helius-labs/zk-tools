fn main() {
    // Select the measured native IFMA paths.
    println!("cargo::rustc-check-cfg=cfg(zopro_phi_grain)");
    println!("cargo::rustc-check-cfg=cfg(zopro_dense_busy_reset)");
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let ifma8 = std::env::var_os("CARGO_FEATURE_IFMA8").is_some();
    let disabled = std::env::var_os("CARGO_CFG_NO_IFMA").is_some();
    if arch == "x86_64"
        && ifma8
        && !disabled
        && ["avx512ifma", "avx512vl", "avx512dq"]
            .iter()
            .all(|needed| features.split(',').any(|feature| feature == *needed))
    {
        println!("cargo::rustc-cfg=zopro_dense_busy_reset");
        println!("cargo::rustc-cfg=zopro_phi_grain");
    }
    println!("cargo::rustc-check-cfg=cfg(gpu)");
    println!("cargo::rustc-check-cfg=cfg(gpu_wgpu)");
    println!("cargo::rustc-check-cfg=cfg(gpu_any)");
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let apple = os == "macos" || os == "ios";
    let metal = std::env::var_os("CARGO_FEATURE_METAL").is_some() && apple;
    let wgpu = std::env::var_os("CARGO_FEATURE_WGPU").is_some() && !metal;
    if metal {
        println!("cargo::rustc-cfg=gpu");
    }
    if wgpu {
        println!("cargo::rustc-cfg=gpu_wgpu");
    }
    if metal || wgpu {
        println!("cargo::rustc-cfg=gpu_any");
    }
}
