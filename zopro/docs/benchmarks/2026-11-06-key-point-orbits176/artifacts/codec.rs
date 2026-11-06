use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
use zopro_bn254::{Field,FpConfig,FqConfig,Fq,Fq2,Fr,G1Affine,G2Affine,G1Projective,G2Projective};
pub const ROOT:&str="/home/ubuntu/src/zopro-perf-work/key-point-orbits176";
pub const Z:&str="/home/ubuntu/src/zk-tools/zopro";
pub const BETA:Fq=Fq::from_decimal("21888242871839275220042445260109153167277707414472061641714758635765020556616");
pub const LAMBDA:Fr=Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
pub fn root(p:impl AsRef<Path>)->PathBuf{Path::new(ROOT).join(p)}
pub fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
pub fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
pub fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Point{G1(G1Affine),G2(G2Affine)}
#[derive(Clone,Copy)]pub enum Sum{G1(G1Projective),G2(G2Projective)}
impl Point{
 pub fn zero(self)->bool{match self{Self::G1(p)=>p.is_identity(),Self::G2(p)=>p.is_identity()}}
 pub fn on_curve(self)->bool{match self{Self::G1(p)=>p.is_on_curve(),Self::G2(p)=>p.is_on_curve()}}
 pub fn neg(self)->Self{match self{Self::G1(p)=>Self::G1(-p),Self::G2(p)=>Self::G2(-p)}}
 pub fn phi(self)->Self{match self{Self::G1(mut p)=>{p.x*=BETA;Self::G1(p)},Self::G2(mut p)=>{p.x=p.x.mul_by_fp(&BETA);Self::G2(p)}}}
 pub fn mul(self,s:Fr)->Self{match self{Self::G1(p)=>Self::G1((p*s).to_affine()),Self::G2(p)=>Self::G2((p*s).to_affine())}}
 pub fn key(self)->[u8;129]{let mut out=[0;129];let fields:Vec<Fq>=match self{Self::G1(p)=>{out[0]=1;vec![p.x,p.y]},Self::G2(p)=>{out[0]=2;vec![p.x.c0,p.x.c1,p.y.c0,p.y.c1]}};for(i,f)in fields.iter().enumerate(){out[1+i*32..1+(i+1)*32].copy_from_slice(&f.to_be_bytes());}out}
 pub fn sum(self)->Sum{match self{Self::G1(p)=>Sum::G1(p.to_projective()),Self::G2(p)=>Sum::G2(p.to_projective())}}
 pub fn eigenvalue(self)->Fr{match self{Self::G1(_)=>{let g=Self::G1(G1Affine::GENERATOR);if g.phi()==g.mul(LAMBDA){LAMBDA}else{assert_eq!(g.phi(),g.mul(LAMBDA.square()));LAMBDA.square()}},Self::G2(_)=>LAMBDA}}
}
impl Sum{
 pub fn add_point(&mut self,p:Point){match(self,p){(Self::G1(a),Point::G1(b))=>*a+=b,(Self::G2(a),Point::G2(b))=>*a+=b,_=>panic!("curve mismatch")}}
 pub fn add_sum(&mut self,p:Self){match(self,p){(Self::G1(a),Self::G1(b))=>*a+=b,(Self::G2(a),Self::G2(b))=>*a+=b,_=>panic!("curve mismatch")}}
 pub fn affine(self)->Point{match self{Self::G1(p)=>Point::G1(p.to_affine()),Self::G2(p)=>Point::G2(p.to_affine())}}
}
pub fn points_digest(points:&[Point])->String{let mut h=Sha256::new();for p in points{h.update(p.key());}format!("{:x}",h.finalize())}
fn host_field(b:&[u8])->Fq{assert_eq!(b.len(),32);let w=core::array::from_fn(|i|u64::from_le_bytes(b[8*i..8*i+8].try_into().unwrap()));assert!(w.iter().rev().cmp(FqConfig::P.iter().rev()).is_lt());Fq::from_montgomery(w)}
pub fn image_points(path:&Path,expected:&str)->BTreeMap<String,Vec<Point>>{
 let b=fs::read(path).unwrap();assert_eq!(hash(&b),expected);assert_eq!(&b[..16],b"zopro-euler-v2\0\0");let len=u64::from_le_bytes(b[16..24].try_into().unwrap())as usize;assert!(24+len<=b.len());
 let counts:[usize;5]=core::array::from_fn(|i|u32::from_le_bytes(b[56+4*i..60+4*i].try_into().unwrap())as usize);let mut at=(24+len).div_ceil(64)*64;let ns=u64::from_le_bytes(b[at..at+8].try_into().unwrap())as usize;let np=u64::from_le_bytes(b[at+8..at+16].try_into().unwrap())as usize;assert!(np>0);at+=128;
 let mut out=BTreeMap::<String,Vec<Point>>::new();for(label,n,g2)in[("A",counts[0],false),("B1_unused",counts[1],false),("K",counts[2]+ns,false),("H",counts[3]+1,false),("B",counts[4],true)]{let size=if g2{128}else{64};let end=at.checked_add(n.checked_mul(size).unwrap()).unwrap();assert!(end<=b.len());if label!="B1_unused"{let p=b[at..end].chunks_exact(size).map(|c|{let f=c.chunks_exact(32).map(host_field).collect::<Vec<_>>();let p=if g2{Point::G2(G2Affine::new(Fq2::new(f[0],f[1]),Fq2::new(f[2],f[3])))}else{Point::G1(G1Affine::new(f[0],f[1]))};assert!(p.on_curve());p}).collect();out.insert(label.into(),p);}at=end;}
 let kh=out["K"].iter().chain(&out["H"]).copied().collect();out.insert("KH".into(),kh);out
}
pub fn fixture(v:&Value)->(Vec<Point>,Vec<Fr>){let b=fs::read(v["fixture"].as_str().unwrap()).unwrap();assert_eq!(hash(&b),v["fixture_sha256"].as_str().unwrap());let oracle=read(v["oracle"].as_str().unwrap());assert_eq!(oracle["fixture_sha256"],v["fixture_sha256"]);for k in["all_points_on_curve","all_points_in_prime_subgroup","all_scalars_canonical","independent_Arkworks_complete_MSM"]{assert_eq!(oracle[k],true);}let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(n,oracle["points"]);let g2=v["query"]=="B";assert_eq!(&b[..8],if g2{b"B2SCAN01"}else{b"ACOLMSM1"});let size=if g2{128}else{64};assert_eq!(b.len(),16+n*(size+32));let p=b[16..16+n*size].chunks_exact(size).map(|c|{let f=c.chunks_exact(32).map(|x|Fq::from_be_bytes(x.try_into().unwrap()).unwrap()).collect::<Vec<_>>();let p=if g2{Point::G2(G2Affine::new(Fq2::new(f[1],f[0]),Fq2::new(f[3],f[2])))}else{Point::G1(G1Affine::new(f[0],f[1]))};assert!(p.on_curve());p}).collect();let s=b[16+n*size..].chunks_exact(32).map(|c|Fr::from_canonical(core::array::from_fn(|i|u64::from_le_bytes(c[8*i..8*i+8].try_into().unwrap()))).unwrap()).collect();(p,s)}
pub fn image_cases()->BTreeMap<String,Value>{read(root("inputs.json"))["images"].as_object().unwrap().iter().map(|(k,v)|(k.clone(),v.clone())).collect()}
pub fn policy(s:&[Fr])->Value{use std::collections::HashSet;let n=s.len();let sample=n.min(2048);let stride=n.div_ceil(sample.max(1));let selected=s.iter().step_by(stride.max(1)).take(sample).map(|s|s.to_canonical()).collect::<Vec<_>>();let unique=selected.iter().copied().collect::<HashSet<_>>();let zeros=selected.iter().filter(|s|**s==[0;4]).count();let unique_nonzero=unique.len()-usize::from(zeros!=0);let nonzero=selected.len()-zeros;let percent=if(32768..65536).contains(&n){95}else{80};let full=s.iter().filter(|v|**v!=Fr::ZERO).map(|s|s.to_canonical()).collect::<HashSet<_>>().len();let gate=n>=256&&n<u32::MAX as usize&&unique.len()*200<=selected.len()*199&&!(unique_nonzero==nonzero&&nonzero*100>selected.len()*percent)&&full*100<=n*percent;json!({"sampled":selected.len(),"sample_unique":unique.len(),"sample_zeros":zeros,"percent":percent,"nonzero_full_classes":full,"mathematical_data_gate":gate,"resource_admission_replayed":false,"prepared_key_routing_replayed":false})}
