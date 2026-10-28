fn main() {
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
