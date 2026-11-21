fn main() {
    // Keep ISA-specific implementations out of portable builds. These
    // switches select the same implementations in CI and release builds.
    let ifma_paths = [
        "zopro_booth_digits",
        "zopro_bounded_schedule",
        "zopro_canonical_task_grain",
        "zopro_dense_busy_reset",
        "zopro_direct_chain_inverse",
        "zopro_euler_leading_weights",
        "zopro_euler_pointwise8",
        "zopro_folded_query_reclaim",
        "zopro_g2_horner_buckets",
        "zopro_g2_prefix_records",
        "zopro_indexed_tile_bundles",
        "zopro_live_preparation_charge",
        "zopro_paired_weighted",
        "zopro_phi_grain",
        "zopro_real_kh",
        "zopro_resource_schedule",
        "zopro_small_core_entry",
    ];
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let ifma8 = std::env::var_os("CARGO_FEATURE_IFMA8").is_some();
    let disabled = std::env::var_os("CARGO_CFG_NO_IFMA").is_some();
    let native_ifma = arch == "x86_64"
        && ifma8
        && !disabled
        && ["avx512ifma", "avx512vl", "avx512dq"]
            .iter()
            .all(|needed| features.split(',').any(|feature| feature == *needed));
    for path in ifma_paths {
        println!("cargo::rustc-check-cfg=cfg({path})");
        if native_ifma || (path == "zopro_bounded_schedule" && os == "macos" && arch == "aarch64") {
            println!("cargo::rustc-cfg={path}");
        }
    }
    println!("cargo::rustc-check-cfg=cfg(gpu)");
    println!("cargo::rustc-check-cfg=cfg(gpu_wgpu)");
    println!("cargo::rustc-check-cfg=cfg(gpu_any)");
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
