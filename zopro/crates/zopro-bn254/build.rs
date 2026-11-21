fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let has_feature = |needed| features.split(',').any(|feature| feature == needed);
    let apple_arm64 =
        arch == "aarch64" && std::env::var("CARGO_CFG_TARGET_VENDOR").as_deref() == Ok("apple");
    let x86_adx = arch == "x86_64" && has_feature("bmi2") && has_feature("adx");
    println!("cargo::rustc-check-cfg=cfg(zopro_fast_inverse)");
    if (apple_arm64 || x86_adx)
        && std::env::var_os("CARGO_FEATURE_FAST_INVERSE").is_some()
        && std::env::var_os("CARGO_CFG_NO_ASM").is_none()
    {
        println!("cargo::rustc-cfg=zopro_fast_inverse");
    }

    let native_ifma = arch == "x86_64"
        && std::env::var_os("CARGO_CFG_NO_IFMA").is_none()
        && ["avx512ifma", "avx512vl", "avx512dq"]
            .iter()
            .all(|needed| has_feature(needed));

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
