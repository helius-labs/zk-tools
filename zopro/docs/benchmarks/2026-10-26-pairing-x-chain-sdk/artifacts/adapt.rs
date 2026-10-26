use std::{fs,path::Path};
const R:&str="/tmp/zopro-pairing-x-chain-sdk";
fn main(){let old="/tmp/zopro-g1-quarter-current";let mut s=fs::read_to_string(format!("{old}/pipeline.rs")).unwrap().replace(old,R);
 s=s.replace("ZOPRO_CPU_G1_QUARTER_CURRENT","ZOPRO_CPU_PAIRING_X_CHAIN");
 s=s.replace("len(),112","len(),109").replace("len(),36","len(),35");
 s=s.replace("assert_eq!(p.len(),6331);p}","assert_eq!(p.len(),6331);for(path,h)in read(\"/tmp/zopro-g1-quarter-current/proof-inventory.json\").as_object().unwrap(){assert_eq!(hash(path),h.as_str().unwrap());if path.ends_with(\".proof\"){assert!(p.insert(h.as_str().unwrap().into()));}}assert_eq!(p.len(),6562);p}");
 s=s.replace("json!(51)","json!(251)");s=s.replace("physical[\"workers\"]=json!(16);","physical[\"workers\"]=json!(16);physical[\"rounds\"]=json!(151);");
 s=s.replace("\"name\":\"g1_quarter\",\"contains\":[\"g1_quarter\",\"point_domain\"]","\"name\":\"pairing_x_chain\",\"contains\":[\"x_chain\"]");
 s=s.replace("env[\"ZOPRO_G1_QUARTER_TRACE\"]=json!(\"1\");","");
 let start=s.find("let trace=fs::read_to_string").unwrap();let end=s[start..].find("let verified=").unwrap()+start;s.replace_range(start..end,"");s=s.replace("traces.push(json!({\"job\":job,\"selected_batches\":selected}));","traces.push(json!({\"job\":job,\"policy\":if job[\"mode\"]==\"on\"{\"ON flag selects whole final-exp chain\"}else{\"OFF original chain\"},\"per_shape_trace\":false}));");
 s=s.replace("p.len(),6349","p.len(),6580").replace("\"prior_after\":6349","\"prior_after\":6580").replace("7374","10494").replace("\"next_prior_proof_count\":6562","\"next_prior_proof_count\":6793");
 s=s.replace("let rounds=if row[\"phase\"]==\"latency\"{31}else{51};","let rounds=if row[\"phase\"]==\"latency\"{c[\"cases\"].as_array().unwrap().iter().find(|x|x[\"name\"]==row[\"case\"]).unwrap()[\"rounds\"].as_u64().unwrap()}else{251};");
 s=s.replace("/31.).collect()","/r[\"summary\"][\"rounds\"].as_f64().unwrap()).collect()");
 s=s.replace("complete known-algebra rebase study","complete fixed-chain full SDK study");
 fs::write(Path::new(R).join("pipeline.rs"),s).unwrap();
}
