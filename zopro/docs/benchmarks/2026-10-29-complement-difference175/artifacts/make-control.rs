use std::{fs,path::Path};const R:&str="/home/ubuntu/src/zopro-perf-work/complement-difference175";
fn main(){let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-product-sub175/control.rs").unwrap().replace("g2-product-sub175","complement-difference175");s=s.replace("assert_eq!(model[\"new_exact_REDC_cases\"], 21568);\n    assert_eq!(model[\"new_normalization_cases\"], 8324);","assert_eq!(model[\"new_exact_REDC_cases\"], 30084);");
let start=s.find("    for f in [\n        \"setup.rs\"").unwrap();let end=start+s[start..].find("    pin(&mut pins, X);").unwrap();s.replace_range(start..end,r#"    for f in ["setup.rs","setup","field-original-pins.json","bounds.md","design.md","kernel.rs","test-field.rs","src/bin/probe.rs","perf.rs","Cargo.toml","Cargo.lock","src/bin/model.rs","target/release/model","model.json","model.log","model.stderr.log","native-tests.log","frozen-build.log","native-build.log","frozen-probe","target/release/probe","control.rs","control","make-control.rs","make-control","frozen-assembly.txt","candidate-assembly.txt"] { pin(&mut pins,r(f)); }
"#);
let start=s.find("    for pattern in [\"chain\", \"affine\"] {").unwrap();let end=start+s[start..].find("    for (name, data) in profiles {").unwrap();s.replace_range(start..end,r#"    for pattern in ["chain","affine"] {
        for label in ["fresh","candidate"] {
            let name=format!("{pattern}-{label}");let dir=r("profiles").join(&name);fs::create_dir_all(&dir).unwrap();
            let ctl=dir.join("ctl").display().to_string();let ack=dir.join("ack").display().to_string();
            assert!(std::process::Command::new("mkfifo").args([&ctl,&ack]).status().unwrap().success());
            let data=dir.join("perf.data").display().to_string();let(bin,mode)=invoke(label);
            let argv=vec!["sudo".into(),"-n".into(),"env".into(),format!("ZOPRO_BENCH_PERF_CTL={ctl}"),format!("ZOPRO_BENCH_PERF_ACK={ack}"),"perf".into(),"record".into(),"--no-buildid-cache".into(),"-e".into(),"cycles:u".into(),"-F".into(),"199".into(),"-o".into(),data.clone(),"--delay=-1".into(),format!("--control=fifo:{ctl},{ack}"),"--".into(),"taskset".into(),"-c".into(),"2".into(),bin,mode.into(),pattern.into(),"1000000".into()];
            steps.push(step(format!("profile-{name}"),argv));profiles.push((name,data));
        }
    }
"#);s=s.replace("whole-process including scalar oracle; distinguish vector/scalar symbols","FIFO-gated native arithmetic interval; setup/primes/scalar oracle excluded").replace("all12caseupper","all8caseupper").replace("wholeprocess; scalar oracle included","FIFO-gated native arithmetic interval; scalar oracle excluded").replace("complete G2 fused product/sub component","complete complemented fused difference component");fs::write(Path::new(R).join("control.rs"),s).unwrap();}
