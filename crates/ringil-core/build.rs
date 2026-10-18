fn main() {
    println!("cargo::rustc-check-cfg=cfg(gpu)");
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if std::env::var_os("CARGO_FEATURE_METAL").is_some() && (os == "macos" || os == "ios") {
        println!("cargo::rustc-cfg=gpu");
    }
}
