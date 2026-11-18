use std::{collections::BTreeMap,fs,path::Path};
use sha2::{Sha256,Digest};
use serde_json::{json,Value};
fn main(){
 let r=Path::new("../c7a-scale-profile245");let p=r.join("bundle/bin/sdk");let elf=fs::read(p).unwrap();assert_eq!(format!("{:x}",Sha256::digest(&elf)),"d523f49e8548620c4373a1ab4be2712d705709fdd7a590c5ed73e9aff97424e3");
 let word=|i|u64::from_le_bytes(elf[i..i+8].try_into().unwrap());let half=|i|u16::from_le_bytes(elf[i..i+2].try_into().unwrap()) as usize;
 let segments:Vec<_>=(0..half(56)).map(|i|word(32)as usize+i*half(54)).filter(|&i|elf[i..i+4]==[1,0,0,0]&&elf[i+4]&1==1).map(|i|(word(i+8),word(i+16),word(i+32))).collect();
 let mut out=vec![];
 for w in [32,96]{for b in 0..3{
  let label=format!("profile-1x2-w{w}-ON-dual-b{b:02}");let d=r.join("remote/output/decode-run");
  let load=|action:&str|{let dir=d.join(format!("{action}-{label}"));let bytes=fs::read(dir.join("stdout.log")).unwrap();let e:Value=serde_json::from_slice(&fs::read(dir.join("exit.json")).unwrap()).unwrap();assert_eq!(format!("{:x}",Sha256::digest(&bytes)),e["stdout_sha256"]);String::from_utf8(bytes).unwrap()};
  let meta=load("metadata");let maps:Vec<_>=meta.lines().filter(|l|l.contains("PERF_RECORD_MMAP2")&&l.ends_with(&format!("r-xp /var/tmp/c7a-scale-profile245/output/sdk{b}"))).collect();assert_eq!(maps.len(),1);
  let start=maps[0].split(": [0x").nth(1).unwrap();let num=|s:&str|u64::from_str_radix(s.trim_start_matches("0x"),16).unwrap();let (base,tail)=start.split_once('(').unwrap();let (length,tail)=tail.split_once(") @ ").unwrap();let file_offset=num(tail.split_whitespace().next().unwrap());let base=num(base);let length=num(length);
  let mut counts=BTreeMap::<(String,u64),usize>::new();let mut total=BTreeMap::<String,usize>::new();
  for line in load("decode").lines().filter(|l|!l.is_empty()&&!l.starts_with('#')){
    let mut f=line.split_whitespace();let evt=f.nth(4).unwrap().trim_end_matches(':').to_string();let ip=num(f.next().unwrap());let symbol=f.next().unwrap_or("");*total.entry(evt.clone()).or_default()+=1;
    if !symbol.starts_with("crossbeam_epoch::default::with_handle")&&!symbol.starts_with("<crossbeam_epoch::internal::Global>::try_advance"){continue;}
    assert!(ip>=base&&ip<base+length);let ofs=ip-base+file_offset;let seg=segments.iter().find(|(o,_,n)|ofs>=*o&&ofs<*o+*n).unwrap();
    let va=ofs-seg.0+seg.1;
    assert!((0x114c20..0x114e09).contains(&va)||(0x1d9180..0x1d9369).contains(&va)||(0x1089c0..0x108aa4).contains(&va));
    *counts.entry((evt,va)).or_default()+=1;
  }
  let mut panel=vec![];for (evt,all)in total{let mut rows:Vec<_>=counts.iter().filter(|((e,_),_)|*e==evt).map(|((_,va),&n)|(*va,n)).collect();rows.sort_by_key(|v|std::cmp::Reverse(v.1));panel.push(json!({"event":evt,"total":all,"top":rows.iter().take(8).map(|(va,n)|json!({"ip":format!("{va:x}"),"samples":n,"all_sample_share":*n as f64/all as f64})).collect::<Vec<_>>() }));}
  out.push(json!({"workers":w,"block":b,"events":panel}));
 }}
 fs::write("ips.json",serde_json::to_vec_pretty(&out).unwrap()).unwrap();
}
