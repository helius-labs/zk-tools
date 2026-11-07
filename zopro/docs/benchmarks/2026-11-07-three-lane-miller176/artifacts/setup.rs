use std::{fs,path::Path};
fn main(){
 let z=Path::new("/home/ubuntu/src/zk-tools/zopro/crates/zopro-bn254/src");let r=Path::new("/home/ubuntu/src/zopro-perf-work/three-lane-miller176");
 let source=fs::read_to_string(z.join("pairing.rs")).unwrap();let start=source.find("use crate::{").unwrap();let end=source.find("/// f^x,").unwrap();let original=&source[start..end];
 let from="use crate::{curve::PSI_X, curve::PSI_Y, Fq, Fq12, Fq2, G1Affine, G2Affine};";let to="use zopro_bn254::{Fq, Fq12, Fq2, G1Affine, G2Affine};";assert_eq!(original.matches(from).count(),1);
 let curve=fs::read_to_string(z.join("curve.rs")).unwrap();let a=curve.find("pub(crate) const PSI_X").unwrap();let b=a+curve[a..].find("impl G2Affine").unwrap();let constants=&curve[a..b];
 let additions="\n// Read-only diagnostic accessors, not part of the copied scalar implementation.\npub fn schedule()-> &'static [i8] { &ATE_LOOP_COUNT }\nimpl G2Prepared { pub fn coefficients(&self)->&[Line] { &self.lines } }\n";
 let helper="\nconst fn fq2(a:&str,b:&str)->Fq2 {Fq2::new(Fq::from_decimal(a),Fq::from_decimal(b))}\n";
 let changed=original.replacen(from,to,1);let mut output=changed.clone();output.push_str(helper);output.push_str(constants);output.push_str(additions);
 let recovered=output.strip_suffix(additions).unwrap().strip_suffix(constants).unwrap().strip_suffix(helper).unwrap().replacen(to,from,1);assert_eq!(recovered,original);
 assert!(!r.join("prepared.rs").exists());fs::write(r.join("prepared.rs"),output).unwrap();fs::write(r.join("copy-review.txt"),"Pairing prefix recovered exactly after reversing import and stripping constant/helper/accessor suffixes. PSI constants copied exactly from curve.rs. No production file changed.\n").unwrap();println!("Private scalar preparation/shared loop copied and original source recovered exactly");
}
