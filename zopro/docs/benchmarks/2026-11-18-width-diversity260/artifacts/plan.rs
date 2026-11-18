use std::{collections::BTreeMap,fs};
use serde_json::{json,Value};
use sha2::{Sha256,Digest};
const R:&str="/var/tmp/width-diversity260";
fn write(n:&str,v:Value){ fs::write(format!("bundle/config/{n}.json"),serde_json::to_vec_pretty(&v).unwrap()).unwrap(); }
fn main(){
 let p:Value=serde_json::from_slice(&fs::read("../g2-interior256/bundle/config/pre.json").unwrap()).unwrap();
 let s=&p["steps"][0];let env=s["environment"].as_object().unwrap().iter().map(|(k,v)|(k.clone(),v.as_str().unwrap().replace("/var/tmp/g2-interior256",R))).collect::<BTreeMap<_,_>>();
 let case:Value=serde_json::from_slice(&fs::read("bundle/config/case.json").unwrap()).unwrap();
 let arms=[("accepted","accepted",96,0,0),("parent","parent",96,32,0),("neither","sdk",96,0,0),("width","sdk",96,32,0),("cert","sdk",96,0,1),("both","sdk",96,32,1),("both32","sdk",32,32,1),("aeglos","aeglos",96,0,0)];
 let mut pins=BTreeMap::new();for n in ["runner","reader","accepted","parent","sdk","aeglos","verify","xtask"]{let h=format!("{:x}",Sha256::digest(fs::read(format!("bundle/bin/{n}")).unwrap()));pins.insert(format!("{R}/bin/{n}"),h.clone());if ["accepted","parent","sdk","aeglos"].contains(&n){for b in 0..7{pins.insert(format!("{R}/output/{n}{b}"),h.clone());}}}
 write("pins",json!(pins));let mut jobs=vec![];
 for step in 0..8{
  let pre=step==0;let b=if pre{0}else{step-1};let stage=if pre{"pre".into()}else{format!("b{b}")};let n=if pre{1}else{151};let mut order=arms;order.rotate_left(b);if !pre&&b%2==1{order.reverse();}
  let mut captures=vec![];let mut reviews=vec![];
  for(arm,family,w,width,cert)in order{
   let name=format!("{stage}-{arm}");let backing=format!("{R}/output/{family}{b}");let base=format!("{R}/output/proofs/{name}");let ae=family=="aeglos";let proof=if ae{format!("{base}/{}",case["id"].as_str().unwrap())}else{base.clone()};
   let mut e=env.clone(); for(k,v)in [("RAYON_NUM_THREADS",w.to_string()),("AEGLOS_CPU_THREADS",w.to_string()),("GOMAXPROCS",w.to_string()),("ZOPRO_BENCH_FRONTEND_WORKERS",width.to_string()),("ZOPRO_BENCH_EARLY_DIVERSITY",cert.to_string()),("ZOPRO_BENCH_PROOFS",base.clone())]{e.insert(k.into(),v);}
   let mut argv=vec!["/usr/bin/taskset".into(),"-c".into(),format!("0-{}",w-1),backing.clone()];let image=format!("{R}/{}",case["image"].as_str().unwrap());let request=format!("{R}/{}",case["request"].as_str().unwrap());
   if ae{argv.extend(["aeglos".into(),format!("{R}/{}",case["aeglos_key"].as_str().unwrap()),request,n.to_string()]);}else{argv.extend([image.clone(),request,n.to_string(),base]);}
   captures.push(json!({"kind":"command","name":name,"argv":argv,"environment":e,"timeout_seconds":300}));
   reviews.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":[format!("{R}/bin/verify"),image,case["image_sha256"],format!("{proof}/0.proof"),format!("{proof}/0.public"),case["public_sha256"]],"timeout_seconds":300}));
   jobs.push(json!({"name":name,"arm":arm,"family":family,"bin":backing,"w":w,"width":width,"cert":cert,"pre":pre,"block":b,"proof":proof,"n":n}));
  }
  for(label,steps)in [(stage.clone(),captures),(format!("{stage}-review"),reviews)]{write(&label,json!({"version":1,"source_pins":format!("{R}/output/pins.json"),"steps":steps}));}
 }
 assert_eq!(jobs.len(),64);write("jobs",json!(jobs));
}
