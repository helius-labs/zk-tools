use ark_ec::{AffineRepr,CurveGroup};
use ark_ff::{BigInt,Field};
use serde_json::json;
use sha2::{Digest,Sha256};
use std::{fs,path::Path};
fn word(bytes:&[u8],at:&mut usize)->u64{let n=u64::from_le_bytes(bytes[*at..*at+8].try_into().unwrap());*at+=8;n}
fn point(bytes:&[u8],at:&mut usize)->ark_bn254::G2Affine{
    let mut f=||{let raw=[word(bytes,at),word(bytes,at),word(bytes,at),word(bytes,at)];let value=ark_bn254::Fq::new_unchecked(BigInt(raw));assert_eq!(value.0.0,raw);value};
    let x=ark_bn254::Fq2::new(f(),f());let y=ark_bn254::Fq2::new(f(),f());
    let p=if x==ark_bn254::Fq2::ZERO&&y==ark_bn254::Fq2::ZERO{ark_bn254::G2Affine::identity()}else{ark_bn254::G2Affine::new_unchecked(x,y)};assert!(p.is_on_curve());p
}
fn main(){
    let directory=std::env::args().nth(1).unwrap();assert!(std::env::var_os("ZOPRO_G2_CAPTURE_DIR").is_none());
    let mut files=fs::read_dir(&directory).unwrap().map(|e|e.unwrap().path()).filter(|p|p.extension().and_then(|s|s.to_str())==Some("g2r")).collect::<Vec<_>>();files.sort();assert!(!files.is_empty()&&files.len()<=40);
    let(mut pairs,mut bytes_total)=(0usize,0usize);
    for path in &files{let bytes=fs::read(path).unwrap();let mut fixture=zopro_core::g2_replay::Fixture::decode(&bytes).unwrap();let mut at=8;let mode=word(&bytes,&mut at);let buckets=word(&bytes,&mut at);let n=word(&bytes,&mut at)as usize;
        assert_eq!(n,fixture.len());assert_eq!(mode!=0,fixture.borrowed());assert_eq!(buckets as usize,fixture.original_bucket_count());
        let mut negative=0;let mut indices=std::collections::BTreeSet::new();
        for _ in 0..n{let encoded=u32::from_le_bytes(bytes[at..at+4].try_into().unwrap());at+=4;assert!(indices.insert(encoded&0x7fff_ffff));let a=point(&bytes,&mut at);let b=point(&bytes,&mut at);let expected=point(&bytes,&mut at);let b=if encoded>>31!=0{negative+=1;-b}else{b};assert_eq!((a.into_group()+b).into_affine(),expected,"independent signed addition");}
        assert_eq!(at,bytes.len());assert_eq!(negative,fixture.negative());
        fixture.with_runner(|r|{r.run(1);r.check();r.run(3);r.check();});
        pairs+=n;bytes_total+=bytes.len();
        println!("FIXTURE {}",json!({"file":path.file_name().unwrap().to_str().unwrap(),"sha256":format!("{:x}",Sha256::digest(&bytes)),"bytes":bytes.len(),"pairs":n,"borrowed":mode!=0,"original_buckets":buckets,"negative":negative,"full_groups":n/8,"tail_points":n%8,"independent_Arkworks_additions":n,"replayed_reset_calls":4,"matched_captured_outputs":true}));
    }
    assert!(bytes_total<=7_947_520);println!("AUDIT {}",json!({"files":files.len(),"pairs":pairs,"serialized_bytes":bytes_total,"all_Arkworks_oracles_passed":true,"all_replays_passed":true,"timings_collected":false}));
    assert!(Path::new(&directory).is_dir());
}
