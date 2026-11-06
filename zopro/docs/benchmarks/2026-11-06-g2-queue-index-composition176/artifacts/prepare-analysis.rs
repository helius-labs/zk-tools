use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-queue-index-composition176";
const P:&str="/home/ubuntu/src/zopro-perf-work/g2-contained-index176";
fn change(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replacen(a,b,1);}
fn main(){
 let mut s=fs::read_to_string(format!("{P}/analyze.rs")).unwrap().replace(P,R);
 change(&mut s,"const ARMS: [&str; 3] = [\"FROZEN\", \"OFF\", \"ON\"];","const ARMS: [&str; 4] = [\"ORIGINAL\", \"DRAIN\", \"PACK\", \"BOTH\"];");
 change(&mut s,"if j[\"variant\"] == \"ON\" && j[\"case\"][\"query\"] == \"B\"", "if [\"PACK\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap()) && j[\"case\"][\"query\"] == \"B\"");
 change(&mut s,"    assert_eq!(v[\"query\"], j[\"actual_query\"]);","    assert_eq!(v[\"query\"], j[\"actual_query\"]);\n    assert_eq!(v[\"composition\"][\"pack_enabled\"], [\"PACK\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap()));\n    assert_eq!(v[\"composition\"][\"drain_enabled\"], [\"DRAIN\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap()));");
 let a=s.find("    let audit = read(r(\"source-audit.json\"));").unwrap();let b=s.find("    let mut commands = 0;").unwrap();
 let audit=r#"
    let audit=read(r("source-audit.json"));
    let parent=read("/home/ubuntu/src/zopro-perf-work/g2-contained-index176/source-audit.json");
    let mut originals_exact=0;
    for(p,h)in m["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(Path::new(Z).join(p)),h.as_str().unwrap());if hash(r("source").join(p))==h.as_str().unwrap(){originals_exact+=1;}}
    assert_eq!(originals_exact,170);
    let mut recovered_parent=0;let mut recovered_originals=0;
    for(p,h)in audit["parent_source_pins"].as_object().unwrap(){let rel=Path::new(p).strip_prefix("/home/ubuntu/src/zopro-perf-work/g2-contained-index176/source").unwrap();let mut bytes=fs::read(r("source").join(rel)).unwrap();let relstr=rel.to_str().unwrap();for e in audit["edits"].as_array().unwrap().iter().rev().filter(|e|e["file"]==relstr){let mut text=String::from_utf8(bytes).unwrap();if let Some(t)=e["suffix"].as_str(){assert!(text.ends_with(t));text.truncate(text.len()-t.len());}else{let new=e["new"].as_str().unwrap();assert_eq!(text.matches(new).count(),1);text=text.replacen(new,e["old"].as_str().unwrap(),1);}bytes=text.into_bytes();}assert_eq!(format!("{:x}",Sha256::digest(&bytes)),h.as_str().unwrap());recovered_parent+=1;if let Some(t)=parent["suffixes"][relstr].as_str(){let mut text=String::from_utf8(bytes).unwrap();assert!(text.ends_with(t));text.truncate(text.len()-t.len());bytes=text.into_bytes();}if let Some(original)=m["source_files_sha256"][relstr].as_str(){assert_eq!(format!("{:x}",Sha256::digest(&bytes)),original);recovered_originals+=1;}}
    assert_eq!(recovered_parent,178);assert_eq!(recovered_originals,176);
    for(_,v)in parent["clones"].as_object().unwrap(){let mut recovered=v["candidate"].as_str().unwrap().to_owned();for pair in v["edits"].as_array().unwrap().iter().rev(){let from=pair[1].as_str().unwrap();assert_eq!(recovered.matches(from).count(),1);recovered=recovered.replacen(from,pair[0].as_str().unwrap(),1);}assert_eq!(recovered,v["original"]);}
"#;
 s.replace_range(a..b,audit);
 let a=s.find("    let mut ratios = BTreeMap").unwrap();let b=s.find("    save(r(\"symbols.json\")").unwrap();
 let ratios=r#"
    const PAIRS:[(&str,&str);6]=[("DRAIN","ORIGINAL"),("PACK","ORIGINAL"),("BOTH","ORIGINAL"),("PACK","DRAIN"),("BOTH","DRAIN"),("BOTH","PACK")];
    let mut ratios=BTreeMap::<(String,String),Vec<f64>>::new();
    for c in cases.as_array().unwrap(){let name=c["name"].as_str().unwrap();let mut medians=BTreeMap::new();let mut rss=BTreeMap::new();let mut service=BTreeMap::new();for arm in ARMS{let key=(name.to_string(),arm.to_string());assert_eq!(times[&key].len(),15);medians.insert(arm,median(&times[&key])/1e6);rss.insert(arm,maxrss[&key]);service.insert(arm,median(&cpu[&key]));}let mut pairs=BTreeMap::new();for(num,den)in PAIRS{let values=times[&(name.into(),num.into())].iter().zip(&times[&(name.into(),den.into())]).map(|(a,b)|a/b).collect::<Vec<_>>();let mut stat=stats(&values);let selected=c["query"]=="A"||(num=="BOTH"&&["ORIGINAL","DRAIN"].contains(&den));let pass=if c["query"]=="A"{stat["low"].as_f64().unwrap()>=1./1.03&&stat["high"].as_f64().unwrap()<=1.03}else{stat["high"].as_f64().unwrap()<=1.03};if selected&&!pass{failed.push(format!("{name}/{num}/{den}/{}",if c["query"]=="A"{"unchanged-A-stability"}else{"nonregression"}));}if num=="BOTH"&&["ORIGINAL","DRAIN"].contains(&den)&&name=="1x2-B-w32"&&(stat["high"].as_f64().unwrap()>=1.||stat["wins"].as_u64().unwrap()<10){failed.push(format!("{name}/{num}/{den}/primary-gain"));}stat["declared_check_passed"]=json!(pass);stat["selection_gate"]=json!(selected);ratios.insert((name.into(),format!("{num}/{den}")),values);pairs.insert(format!("{num}/{den}"),stat);}rows.push(json!({"case":name,"median_ms":medians,"max_process_RSS_MiB":rss,"median_CPU_ms_per_query":service,"comparisons":pairs}));}
    let mut aggregates=BTreeMap::new();for(num,den)in PAIRS{let values=(0..15).map(|b|{let a=ratios[&("1x2-B-w32".into(),format!("{num}/{den}"))][b];let c=ratios[&("2x2-B-w32".into(),format!("{num}/{den}"))][b];((a.ln()+c.ln())/2.).exp()}).collect::<Vec<_>>();let stat=stats(&values);if num=="BOTH"&&["ORIGINAL","DRAIN"].contains(&den)&&(stat["high"].as_f64().unwrap()>=1.||stat["wins"].as_u64().unwrap()<10){failed.push(format!("B32-two-shape/{num}/{den}/gain"));}aggregates.insert(format!("{num}/{den}"),stat);}
    assert_eq!(all.len(),456);assert_eq!(commands,481);assert_eq!(total,15688);assert_eq!(reused,0);
"#;
 s.replace_range(a..b,ratios);
 change(&mut s,"necessary same-executable component selection only; FROZEN descriptive; full SDK required before adoption","necessary same-executable four-arm composition screen; full SDK required before adoption");
 change(&mut s,"\"native_tests\":128,\"new_native_tests\":3,\"snapshot_files\":178","\"native_tests_each_drain_mode\":130,\"native_drain_modes\":2,\"new_native_tests\":0,\"reused_contained_tests\":3,\"reused_queue_tests\":2,\"snapshot_files\":180");
 change(&mut s,"\"caller_rows\":348,\"new_caller_processes\":341,\"reused_preflights\":7","\"caller_rows\":456,\"new_caller_processes\":456,\"reused_preflights\":0");
 assert!(!std::path::Path::new(R).join("analyze.rs").exists());fs::write(format!("{R}/analyze.rs"),s).unwrap();println!("Prepared four-arm analysis; execution requires all477capture commands terminal.");
}
