use std::{collections::{BTreeMap,BTreeSet},fs,io::Read,os::unix::fs::MetadataExt,path::{Path,PathBuf},process::{Command,Stdio}};
use serde_json::{Value,json}; use sha2::{Digest,Sha256};
fn read(p:impl AsRef<Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String {let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn save(p:impl AsRef<Path>,v:Value){assert!(!p.as_ref().exists(),"exists {}",p.as_ref().display());fs::write(p,serde_json::to_vec_pretty(&v).unwrap()).unwrap();}
fn walk(p:&Path,base:&Path,out:&mut BTreeMap<String,String>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();assert!(!e.file_type().unwrap().is_symlink());if e.file_type().unwrap().is_dir(){walk(&e.path(),base,out)}else{out.insert(e.path().strip_prefix(base).unwrap().to_str().unwrap().into(),hash(e.path()));}}}
fn state()->Value{
    let path=fs::read_to_string("/proc/self/cgroup").unwrap();let cg=path.lines().find_map(|l|l.strip_prefix("0::")).unwrap();
    let base=Path::new("/sys/fs/cgroup").join(cg.trim_start_matches('/')); let mut levels=vec![];
    let mut p=Some(base.as_path());while let Some(x)=p{if !x.starts_with("/sys/fs/cgroup"){break}let mut fields=serde_json::Map::new();for f in["memory.max","memory.swap.max","memory.current","memory.events","memory.peak","cpu.max","cpu.stat","cpuset.cpus.effective"]{fields.insert(f.into(),match fs::read_to_string(x.join(f)){Ok(v)=>json!({"value":v}),Err(e)=>json!({"error":e.to_string()})});}levels.push(json!({"path":x,"fields":fields}));if x==Path::new("/sys/fs/cgroup"){break}p=x.parent();}
    json!({"self_cgroup":path,"mountinfo":fs::read_to_string("/proc/self/mountinfo").unwrap(),"status":fs::read_to_string("/proc/self/status").unwrap(),"ancestors":levels})
}
fn workflow(root:&Path,out:&Path,label:&str,steps:&[Value],pins:&Path){
    let plan=out.join(format!("{label}-plan.json"));save(&plan,json!({"version":1,"source_pins":pins,"steps":steps}));
    let log=fs::File::create(out.join(format!("{label}.log"))).unwrap();
    let s=Command::new(root.join("bin/xtask")).arg("workflow").arg(plan).arg(out.join(format!("{label}-run"))).stdout(Stdio::from(log.try_clone().unwrap())).stderr(Stdio::from(log)).status().unwrap();
    assert!(s.success(),"{label} failed; preserve all partials, do not repeat successful captures");
}
fn verify(root:&Path,j:&Value)->Value {let c=&j["case"];json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[root.join("bin/verify"),root.join(c["image"].as_str().unwrap()),PathBuf::from(c["image_sha256"].as_str().unwrap()),PathBuf::from(format!("{}/0.proof",j["proof_dir"].as_str().unwrap())),PathBuf::from(format!("{}/0.public",j["proof_dir"].as_str().unwrap())),PathBuf::from(c["public_sha256"].as_str().unwrap())],"timeout_seconds":300})}
fn inspect(out:&Path,j:&Value,phase:&str)->Value{
    let name=j["name"].as_str().unwrap();let dir=out.join(format!("{phase}-run/{name}"));let e=read(dir.join("exit.json"));assert_eq!(e["returncode"],0);
    for stream in["stdout","stderr"]{assert_eq!(hash(dir.join(format!("{stream}.log"))),e[format!("{stream}_sha256")].as_str().unwrap());}
    let raw=fs::read_to_string(dir.join("stdout.log")).unwrap();let samples=raw.lines().filter_map(|l|l.strip_prefix("SAMPLE ")).map(|l|serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();
    let sums=raw.lines().filter_map(|l|l.strip_prefix("SUMMARY ")).map(|l|serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();assert_eq!(sums.len(),1);let s=&sums[0];let n=j["rounds"].as_u64().unwrap();let w=j["workers"].as_u64().unwrap();
    assert_eq!(s["rounds"],n);assert_eq!(s["priming"],3);assert_eq!(samples.len(),n as usize);let z=j["engine"]=="zopro";let mut sum=0.;
    for(i,v)in samples.iter().enumerate(){assert_eq!(v["round"],i);let x=v[if z{"ms"}else{"verified_ms"}].as_f64().unwrap();assert!(x.is_finite()&&x>0.);sum+=x;if !z{assert_eq!(v["cache_hit"],true);}}
    if z{assert_eq!(s["workers"],w);assert_eq!(s["placement_before"],s["placement_after"]);let a=&s["placement_before"];assert_eq!(a["mode"],"fixed");let workers=a["worker_rows"].as_array().unwrap();assert_eq!(workers.len(),w as usize);for(i,row)in workers.iter().enumerate(){assert_eq!(row["index"],i);assert_eq!(row["allowed"],json!([i]));}for k in["generated_timed_fixed_scans","generated_timed_key_scans"]{assert_eq!(s[k],0);}if j["case"]["shape"]=="1x2"{assert_eq!(s["generated_fast_parses"],n);}}
    else{assert_eq!(s["threads"],w);assert_eq!(s["key_sha256"],j["case"]["aeglos_key_sha256"]);}
    let proof=Path::new(j["proof_dir"].as_str().unwrap()).join("0.proof");assert_eq!(fs::metadata(&proof).unwrap().len(),164);let ph=hash(&proof);assert_eq!(hash(proof.with_file_name("0.public")),j["case"]["public_sha256"].as_str().unwrap());
    let vd=out.join(format!("{}-verify-run/verify-{name}",if phase=="preflight"{"preflight"}else{"late"}));let ve=read(vd.join("exit.json"));assert_eq!(ve["returncode"],0);for stream in["stdout","stderr"]{assert_eq!(hash(vd.join(format!("{stream}.log"))),ve[format!("{stream}_sha256")].as_str().unwrap());}
    let vr=fs::read_to_string(vd.join("stdout.log")).unwrap();assert!(vr.contains(&format!("VERIFIED proof_sha256={ph} ")));assert!(vr.contains(&format!(" image_sha256={}",j["case"]["image_sha256"].as_str().unwrap())));
    json!({"job":j,"mean_ms":sum/n as f64,"samples":samples,"summary":s,"proof_sha256":ph,"cpu_ms":(s["user_cpu_ms"].as_f64().unwrap()+s["system_cpu_ms"].as_f64().unwrap())/n as f64,"rss_mib":s["whole_process_maxrss_kib"].as_f64().unwrap()/1024.})
}
fn review(root:&Path,out:&Path,mode:&str,stage:&str,pins:&Path){
    workflow(root,out,&format!("independent-{stage}"),&[json!({"kind":"command","name":format!("read-{stage}"),"argv":[root.join("bin/independent").display().to_string(),root.display().to_string(),mode.to_string(),stage.to_string()],"timeout_seconds":300})],pins);
    assert_eq!(read(out.join(format!("independent-{stage}.json")))["passed"],true);
}
fn main(){
    let a=std::env::args().collect::<Vec<_>>();let root=Path::new(&a[1]);let mode=a[2].as_str();
    if mode=="pins"{let mut files=BTreeMap::new();for d in["config","bin","images","requests","keys","lib"]{walk(&root.join(d),root,&mut files);}save(root.join("payload-pins.json"),json!(files));return}
    if mode=="loss"{use std::io::{BufRead,BufReader};let p=&a[3];assert!(Path::new(p).starts_with(root.join("output/perf")));let mut child=Command::new("/usr/bin/perf").args(["script","-D","-i",p]).stdout(Stdio::piped()).spawn().unwrap();let mut lost=vec![];for line in BufReader::new(child.stdout.take().unwrap()).lines(){let line=line.unwrap();if line.contains("PERF_RECORD_LOST"){lost.push(line);}}assert!(child.wait().unwrap().success());println!("{}",json!({"zero_lost":lost.is_empty(),"lost":lost}));return;}
    assert_eq!(mode,"capture");let smoke=false;let cfg=read(root.join("config/config.json"));let payload=read(root.join("payload-pins.json"));for(p,h)in payload.as_object().unwrap(){assert_eq!(hash(root.join(p)),h.as_str().unwrap());}
    let out=root.join(if smoke{"smoke-output"}else{"output"});fs::create_dir(&out).unwrap();let before=state();if !smoke{let fields=&before["ancestors"][0]["fields"];assert_eq!(fields["memory.max"]["value"].as_str().unwrap().trim(),"32212254720");assert_eq!(fields["memory.swap.max"]["value"].as_str().unwrap().trim(),"0");}save(out.join("state-before.json"),before);
    let mut pins=BTreeMap::new();let mut backings=vec![];for family in["sdk","aeglos"]{let mut seen=BTreeSet::new();for b in 0..3{let path=out.join(format!("{family}{b}"));fs::copy(root.join(format!("bin/{family}")),&path).unwrap();let h=hash(&path);assert_eq!(h,cfg[format!("{family}_sha256")].as_str().unwrap());pins.insert(path.display().to_string(),h.clone());let m=fs::metadata(&path).unwrap();assert!(seen.insert((m.dev(),m.ino())));backings.push(json!({"family":family,"backing":b,"path":path,"sha256":h,"dev":m.dev(),"inode":m.ino()}));}}
    for d in["bin","config"]{let mut files=BTreeMap::new();walk(&root.join(d),root,&mut files);for(p,h)in files{pins.insert(root.join(p).display().to_string(),h);}}
    save(out.join("runtime-pins.json"),json!(pins));save(out.join("backings.json"),json!(backings));
    let probe=out.join("probe.data");workflow(root,&out,"observer-probe",&[
json!({"kind":"command","name":"perf-version","argv":["/usr/bin/perf","version"],"timeout_seconds":60}),
json!({"kind":"command","name":"stat-events","argv":["/usr/bin/perf","stat","--no-big-num","-x",";","-e","{cycles:u,instructions:u,cache-misses:u,branch-misses:u}","--","/usr/bin/true"],"timeout_seconds":60}),
json!({"kind":"command","name":"record-events","argv":["/usr/bin/perf","record","--no-buildid-cache","--sample-cpu","--timestamp","-P","--clockid","mono","-e","cycles:u,instructions:u","-c","16000000","-m","512","-o",probe.to_str().unwrap(),"--","/usr/bin/true"],"timeout_seconds":60}),
json!({"kind":"command","name":"event-attributes","argv":["/usr/bin/perf","evlist","-v","-i",probe.to_str().unwrap()],"timeout_seconds":60})],&out.join("runtime-pins.json"));
    let contexts=[("1x2",32,"ON"),("1x2",32,"AEGLOS"),("1x2",96,"ON"),("1x2",96,"AEGLOS"),("merge-54x1",32,"ON"),("merge-54x1",96,"ON")];
    for d in["perf","stat"]{fs::create_dir(out.join(d)).unwrap();}for f in["ctl","ack"]{assert!(Command::new("mkfifo").args(["-m","600"]).arg(out.join(f)).status().unwrap().success());}
    let mut jobs=vec![];let mut steps=vec![];
    let mut add=|shape:&str,w:usize,arm:&str,phase:&str,panel:&str,b:usize|{
        let c=&cfg["cases"][shape];let family=if arm=="AEGLOS"{"aeglos"}else{"sdk"};let engine=if arm=="AEGLOS"{"aeglos"}else{"zopro"};let pre=phase=="preflight";let n=if pre{1}else if shape=="1x2"{251}else{17};
        let name=format!("{phase}-{shape}-w{w}-{arm}-{panel}-b{b:02}");let base=out.join("proofs").join(&name);let pd=if arm=="AEGLOS"{base.join(c["id"].as_str().unwrap())}else{base.clone()};let binary=out.join(format!("{family}{}",b%3));
        let mut env=cfg["environment"].clone();for(k,v)in[("RAYON_NUM_THREADS",w.to_string()),("ZOPRO_BASIS_KEY_DIR",root.join("images").display().to_string()),("ZOPRO_CPU_LAGRANGE_DIR",root.join("cache").display().to_string()),("AEGLOS_CPU_THREADS",w.to_string()),("GOMAXPROCS",w.to_string()),("AEGLOS_BENCH_DISABLE_SHIFTED","0".into()),("AEGLOS_BENCH_MSM_PHASES","0".into()),("ZOPRO_BENCH_MSM_PHASES","0".into()),("ZOPRO_BENCH_PROOFS",base.display().to_string()),("LD_LIBRARY_PATH",root.join("lib").display().to_string())]{env[k]=json!(v);}
        let ctl=out.join("ctl");let ack=out.join("ack");let mut argv=vec![];let mut data=Value::Null;
        if panel=="stat"||panel=="dual"{for k in["ZOPRO_BENCH_PERF_CTL","CROSS_PERF_CTL"]{env[k]=json!(ctl);}for k in["ZOPRO_BENCH_PERF_ACK","CROSS_PERF_ACK"]{env[k]=json!(ack);}argv.push("/usr/bin/perf".to_string());
            if panel=="stat"{let p=out.join("stat").join(format!("{name}.csv"));data=json!(p);argv.extend(["stat".into(),"--no-big-num".into(),"-x".into(),";".into(),"-e".into(),"{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),"-o".into(),p.display().to_string()]);}
            else{let p=out.join("perf").join(format!("{name}.data"));data=json!(p);argv.extend(["record".into(),"--no-buildid-cache".into(),"--sample-cpu".into(),"--timestamp".into(),"-P".into(),"--clockid".into(),"mono".into(),"-e".into(),"cycles:u,instructions:u".into(),"-c".into(),"16000000".into(),"-m".into(),"512".into(),"-o".into(),p.display().to_string()]);}
            argv.extend(["--delay=-1".into(),format!("--control=fifo:{},{}",ctl.display(),ack.display()),"--".into()]);
        }
        argv.extend(["/usr/bin/taskset".to_string(),"-c".into(),format!("0-{}",w-1),binary.display().to_string()]);
        if arm=="AEGLOS"{argv.extend(["aeglos".into(),root.join(c["aeglos_key"].as_str().unwrap()).display().to_string(),root.join(c["request"].as_str().unwrap()).display().to_string(),n.to_string()]);}else{argv.extend([root.join(c["image"].as_str().unwrap()).display().to_string(),root.join(c["request"].as_str().unwrap()).display().to_string(),n.to_string(),pd.display().to_string()]);}
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":300}));jobs.push(json!({"name":name,"case":c,"workers":w,"engine":engine,"arm":arm,"phase":phase,"panel":panel,"preflight":pre,"block":b,"rounds":n,"proof_dir":pd,"binary":binary,"backing":b%3,"data":data}));
    };
    for(s,w,a)in contexts{add(s,w,a,"preflight","plain",0);}
    for b in 0..3{let mut order=contexts.to_vec();order.rotate_left(b);if b%2==1{order.reverse();}for(s,w,a)in order{add(s,w,a,"latency","plain",b);}}
    for b in 0..3{let mut order=contexts.to_vec();order.rotate_left(b);if b%2==1{order.reverse();}for(i,(s,w,a))in order.into_iter().enumerate(){let mut panels=["observer","stat","dual"];panels.rotate_left((b+i)%3);if b%2==1{panels.reverse();}for p in panels{add(s,w,a,"profile",p,b);}}}
    assert_eq!(jobs.len(),78);save(out.join("jobs.json"),json!(jobs));save(out.join("capture-steps.json"),json!(steps));
    let runtime=out.join("runtime-pins.json");review(root,&out,mode,"plan",&runtime);
    let pre=6;let mut ledger:BTreeSet<String>=serde_json::from_value(read(root.join("config/seed.json"))).unwrap();assert_eq!(ledger.len(),cfg["seed_count"].as_u64().unwrap()as usize);let mut rows=vec![];
    workflow(root,&out,"preflight",&steps[..pre],&runtime);workflow(root,&out,"preflight-verify",&jobs[..pre].iter().map(|j|verify(root,j)).collect::<Vec<_>>(),&runtime);
    for j in &jobs[..pre]{let row=inspect(&out,j,"preflight");assert!(ledger.insert(row["proof_sha256"].as_str().unwrap().into()));rows.push(row);}
    save(out.join("preflight-rows.json"),json!(rows));save(out.join("preflight-proof-sha256.json"),json!(ledger));review(root,&out,mode,"preflight",&runtime);
    workflow(root,&out,"latency",&steps[6..24],&runtime);
    workflow(root,&out,"profile",&steps[24..],&runtime);
    workflow(root,&out,"late-verify",&jobs[6..].iter().map(|j|verify(root,j)).collect::<Vec<_>>(),&runtime);
    for j in &jobs[6..]{let row=inspect(&out,j,j["phase"].as_str().unwrap());assert!(ledger.insert(row["proof_sha256"].as_str().unwrap().into()));rows.push(row);}
    let mut decodes=vec![];
    for j in jobs.iter().filter(|j|j["panel"]=="dual"){let name=j["name"].as_str().unwrap();let p=j["data"].as_str().unwrap();for(label,args)in[("events",vec!["/usr/bin/perf","evlist","-v","-i",p]),("decode",vec!["/usr/bin/perf","script","--ns","-i",p,"-F","pid,tid,cpu,time,period,event,ip,sym"]),("metadata",vec!["/usr/bin/perf","script","--header","--show-mmap-events","--show-task-events","--ns","-i",p,"-F","pid,tid,cpu,time,period,event,ip,sym"])]{decodes.push(json!({"kind":"command","name":format!("{label}-{name}"),"argv":args,"timeout_seconds":300}));}
    decodes.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":[root.join("bin/runner").display().to_string(),root.display().to_string(),"loss",p.to_string()],"timeout_seconds":300}));}
    workflow(root,&out,"decode",&decodes,&runtime);
    for(p,h)in payload.as_object().unwrap(){assert_eq!(hash(root.join(p)),h.as_str().unwrap());}for(p,h)in &pins{assert_eq!(hash(p),*h);}
    save(out.join("state-after.json"),state());save(out.join("rows.json"),json!(rows));save(out.join("proof-sha256.json"),json!(ledger));review(root,&out,mode,"finish",&runtime);
    save(out.join("complete.json"),json!({"status":"complete","exports":jobs.len(),"sdk_calls":12696,"canonical_count":ledger.len(),"canonical_sha256":hash(out.join("proof-sha256.json")),"mode":mode}));println!("Complete {} exports / {} SDK calls",jobs.len(),12696);
}
