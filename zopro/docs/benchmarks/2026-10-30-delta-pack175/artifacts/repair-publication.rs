use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/delta-pack175";
fn main(){let p=|s:&str|Path::new(R).join(s);let s=fs::read_to_string(p("publish.rs")).unwrap();let mut t=s.clone();for(a,b)in[
 ("assert_eq!(m,a[\"summaries_ns_per_wave\"][key][arm].as_f64().unwrap());","close(m,a[\"summaries_ns_per_wave\"][key][arm].as_f64().unwrap());"),
 ("assert_eq!(m,q[\"median\"].as_f64().unwrap());","close(m,q[\"median\"].as_f64().unwrap());"),
 ("assert_eq!(v[9],q[\"lower\"].as_f64().unwrap());","close(v[9],q[\"lower\"].as_f64().unwrap());"),
 ("assert_eq!(v[21],q[\"upper\"].as_f64().unwrap());","close(v[21],q[\"upper\"].as_f64().unwrap());")]{assert_eq!(t.matches(a).count(),1);t=t.replace(a,b);}t.push_str("\nfn close(a:f64,b:f64){assert!((a-b).abs()<=1e-12*a.abs().max(b.abs()).max(1.),\"serialized rounding mismatch {a} {b}\");}\n");assert_ne!(t,s);fs::write(p("publish-fixed.rs"),t).unwrap();println!("Original publisher/source/log retained; changed four equality checks only for JSON floating-point round trips, no statistical threshold or observation changes");}
