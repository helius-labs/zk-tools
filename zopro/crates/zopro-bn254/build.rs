fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let native_ifma = arch == "x86_64"
        && std::env::var_os("CARGO_CFG_NO_IFMA").is_none()
        && ["avx512ifma", "avx512vl", "avx512dq"]
            .iter()
            .all(|needed| features.split(',').any(|feature| feature == *needed));

    for path in [
        "zopro_cios_cancel_carry",
        "zopro_gcd_balanced_runs",
        "zopro_horizontal_limb_sum",
    ] {
        println!("cargo::rustc-check-cfg=cfg({path})");
        if native_ifma {
            println!("cargo::rustc-cfg={path}");
        }
    }
}
