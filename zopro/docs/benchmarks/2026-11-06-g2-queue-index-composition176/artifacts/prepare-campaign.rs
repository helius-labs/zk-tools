use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-queue-index-composition176";
const P:&str="/home/ubuntu/src/zopro-perf-work/g2-contained-index176";
fn change(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replacen(a,b,1);}
fn main(){
let old=fs::read_to_string(format!("{P}/campaign.rs")).unwrap();
let mut prefix=old[..old.find("fn prepare() {").unwrap()].to_string().replace(P,R);
change(&mut prefix,"const ARMS: [&str; 3] = [\"FROZEN\", \"OFF\", \"ON\"];","const ARMS: [&str; 4] = [\"ORIGINAL\", \"DRAIN\", \"PACK\", \"BOTH\"];");
prefix=prefix.replace("j[\"variant\"] == \"ON\"", "[\"PACK\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap())");
change(&mut prefix,"    assert_eq!(v[\"query\"], j[\"actual_query\"]);","    assert_eq!(v[\"query\"], j[\"actual_query\"]);\n    assert_eq!(v[\"composition\"][\"pack_enabled\"], [\"PACK\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap()));\n    assert_eq!(v[\"composition\"][\"drain_enabled\"], [\"DRAIN\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap()));");
change(&mut prefix,"    env[\"RAYON_NUM_THREADS\"] = json!(w.to_string());","    env[\"RAYON_NUM_THREADS\"] = json!(w.to_string());\n    env[\"ZOPRO_CPU_QUERY_DRAIN\"] = json!(if [\"DRAIN\",\"BOTH\"].contains(&arm){\"1\"}else{\"0\"});\n    env[\"ZOPRO_CPU_QUERY_PACK\"] = json!(if [\"PACK\",\"BOTH\"].contains(&arm){\"1\"}else{\"0\"});");
change(&mut prefix,"if arm == \"ON\" && case[\"query\"] == \"B\"", "if [\"PACK\",\"BOTH\"].contains(&arm) && case[\"query\"] == \"B\"");
let prepare=r#"
fn prepare(){
 assert!(!r("capture-plan.json").exists());assert_eq!(read(r("native/result.json"))["status"],"complete");
 for tag in["OFF","ON"]{let t=fs::read_to_string(r(format!("native/core-drain-{tag}/stdout.log"))).unwrap();assert!(t.contains("test result: ok. 130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));}
 let link=fs::read_to_string(r("native/driver-build/stderr.log")).unwrap();let line=link.lines().find(|l|l.contains("Running")&&l.contains("--crate-name zopro_queue_index_composition")).unwrap();
 for f in["-C opt-level=3","-C lto=fat","-C codegen-units=1","-C target-cpu=native","--cfg zopro_contained_index","--cfg zopro_queue_drain","--cfg zopro_queue_index_composition"]{assert!(line.contains(f),"{f}");}
 assert!(!r("candidate-bin").exists());let new=r("target/release/zopro-queue-index-composition");fs::copy(&new,r("candidate-bin")).unwrap();let mut ids=serde_json::Map::new();for arm in ARMS{ids.insert(arm.into(),json!({"binary":r("candidate-bin"),"binary_sha256":hash(&new),"actual_rustc":line}));}let ids=Value::Object(ids);save(r("identities.json"),&ids);
 let fixtures=read(r("fixtures.json"));let mut cases=vec![];for shape in["1x2","2x2"]{for w in[1,16,32]{let mut c=fixtures[format!("{shape}-B")].clone();c["name"]=json!(format!("{shape}-B-w{w}"));c["workers"]=json!(w);cases.push(c);}}let mut a=fixtures["1x2-A"].clone();a["name"]=json!("1x2-A-w32");a["workers"]=json!(32);cases.push(a);save(r("cases.json"),&json!(cases));
 let mut env=read(r("environment.json"));for k in["RUSTFLAGS","RUSTC","CARGO_BUILD_JOBS","CARGO_TARGET_DIR"]{env.as_object_mut().unwrap().remove(k);}
 let mut jobs=vec![];let mut steps=vec![];for c in &cases{for arm in ARMS{add(c,arm,0,"preflight",&env,&ids,&mut jobs,&mut steps,None);}}steps.push(command("preflight-check",vec![r("campaign").display().to_string(),"preflight".into()]));
 for b in 0..15{let mut order=(0..7).collect::<Vec<_>>();order.rotate_left(b%7);if b%2==1{order.reverse();}let mut arms=ARMS;arms.rotate_left(b%4);if b%2==1{arms.reverse();}for i in order{for arm in arms{add(&cases[i],arm,b,"latency",&env,&ids,&mut jobs,&mut steps,None);}}}
 for d in["stat","perf"]{fs::create_dir(r(d)).unwrap();}for n in["ctl","ack"]{assert!(Command::new("mkfifo").arg(r(n)).status().unwrap().success());}
 let c=cases.iter().find(|c|c["name"]=="1x2-B-w32").unwrap();for panel in["stat","cycles"]{for arm in ARMS{add(c,arm,0,panel,&env,&ids,&mut jobs,&mut steps,None);}}
 for j in jobs.iter().filter(|j|j["panel"]=="cycles"){let n=j["name"].as_str().unwrap();let d=j["data"].as_str().unwrap();steps.push(command(&format!("own-{n}"),vec!["sudo".into(),"-n".into(),r("campaign").display().to_string(),"own".into(),d.into()]));for meta in[false,true]{let mut argv=vec!["perf".into(),"script".into(),"--header".into(),"--ns".into(),"-i".into(),d.into(),"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()];if meta{argv.extend(["--show-mmap-events".into(),"--show-task-events".into(),"--show-lost-events".into()]);}steps.push(command(&format!("{}-{n}",if meta{"metadata"}else{"decode"}),argv));}steps.push(command(&format!("events-{n}"),vec!["perf".into(),"evlist".into(),"-v".into(),"-i".into(),d.into()]));steps.push(command(&format!("loss-{n}"),vec![r("campaign").display().to_string(),"loss".into(),d.into()]));}
 assert_eq!(jobs.len(),456);assert_eq!(steps.len(),477);save(r("jobs.json"),&json!(jobs));
 let mut pins=BTreeMap::new();for d in["source","driver","native"]{walk(&r(d),&mut pins);}for f in["setup.rs","finish-setup.rs","setup-first-failure.txt","startup-command-correction.txt","design.md","history-audit.md","protocol.md","source-audit.json","driver-audit.json","build-plan.json","build-pins.json","prepare-campaign.rs","campaign.rs","campaign","environment.json","fixtures.json","cases.json","jobs.json","identities.json","candidate-bin"]{pins.insert(r(f).display().to_string(),hash(r(f)));}for c in &cases{for k in["fixture","oracle"]{let p=c[k].as_str().unwrap();pins.insert(p.into(),hash(p));}}
 save(r("capture-pins.json"),&json!(pins));save(r("capture-plan.json"),&json!({"version":1,"source_pins":r("capture-pins.json"),"steps":steps}));println!("Prepared477capturecommands/456freshcallers/420latencies then8PMU/profile;onebinary/fourarms. Native130tests perdrainmode and optimizedargv checked. No capture started.");
}
fn preflight(){let jobs=read(r("jobs.json"));let mut count=0;for j in jobs.as_array().unwrap().iter().filter(|j|j["panel"]=="preflight"){caller(j);count+=1;}assert_eq!(count,28);println!("All28freshpreflights match independentwholeMSMoracles,actualtoproutes and bothprocess selectors; pointsunchanged.");}
"#;
let tail=&old[old.find("fn own(data: &str)").unwrap()..];
let output=format!("{prefix}{prepare}{tail}");assert!(!std::path::Path::new(R).join("campaign.rs").exists());fs::write(format!("{R}/campaign.rs"),output).unwrap();println!("Generated four-arm campaign; preparation awaits terminal native workflow.");
}
