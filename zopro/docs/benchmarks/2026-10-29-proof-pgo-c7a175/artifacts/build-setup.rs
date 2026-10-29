const TRIPLE: &str = "x86_64-unknown-linux-gnu";
const LLVM:&str="/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata";
fn environment(m: &Value, label: &str) -> Value {
    let mut env = m["environment"].clone();
    env["RUSTFLAGS"] = json!(fs::read_to_string(r(format!("{label}-flags.txt"))).unwrap());
    env["CARGO_TARGET_DIR"] = json!(r(format!("target-{label}")));
    env["CARGO_BUILD_JOBS"] = json!("16");
    env["RAYON_NUM_THREADS"] = json!("4");
    env["LLVM_PROFILE_FILE"] = json!(r("build-raw/%p-%m.profraw"));
    env
}
fn steps(m: &Value, label: &str, tests: bool) -> Vec<Value> {
    let env = environment(m, label);
    let cargo = r("source/Cargo.toml");
    let mut steps = vec![];
    if tests {
        for (name, package) in [("field", "zopro-bn254"), ("core", "zopro-core")] {
            steps.push(json!({"kind":"command","name":format!("{label}-{name}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","--target",TRIPLE,"-p",package,"--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        }
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","--target",TRIPLE,"-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
    }
    steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","--target",TRIPLE,"-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
    steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
    let mut flags: Vec<String> = ["-O", "-C", "lto=fat", "-C", "codegen-units=1"]
        .iter()
        .map(|x| (*x).into())
        .collect();
    flags.extend(
        fs::read_to_string(r(format!("{label}-flags.txt")))
            .unwrap()
            .split_whitespace()
            .map(String::from),
    );
    let driver = if label == "generate" {
        r("train-driver.rs")
    } else {
        Path::new(Z).join("crates/zopro-bench/examples/cpu-sdk.rs")
    };
    steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":driver,"additional_sources":if label=="generate"{vec![]}else{vec![Path::new(Z).join("crates/zopro-bench/examples/support/perf.rs")]},"rustc_flags":flags,"timeout_seconds":900}));
    steps
}
include!("build-setup-body.rs");
