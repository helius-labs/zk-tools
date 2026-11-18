use serde_json::{Value,json};
use std::{collections::BTreeMap,fs,path::Path};
use sha2::{Sha256,Digest};

const STAGES:[&str;8]=["witness","scalar_preparation","fft","msm","assembly","other_engine","outside_engine","verified"];
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}

fn campaign(root:&Path, phase:&str, blocks:usize, rounds:usize)->Value {
    let out=root.join("remote/output");
    assert_eq!(read(root.join("host-remote/review.json"))["passed"],true);
    let jobs=read(out.join("jobs.json"));
    let mut data:BTreeMap<(String,usize,String),Vec<(usize,[f64;8])>>=BTreeMap::new();
    let mut samples=0;
    for job in jobs.as_array().unwrap() {
        let name=job["name"].as_str().unwrap();
        if !name.starts_with(&format!("{phase}-")){continue;}
        let shape=job["case"]["shape"].as_str().unwrap();
        if phase=="latency" && shape!="1x2"{continue;}
        let w=job["workers"].as_u64().unwrap() as usize;
        let arm=job["arm"].as_str().unwrap();
        let dir=out.join(format!("{phase}-run")).join(name);
        let receipt=read(dir.join("exit.json"));
        assert_eq!(receipt["success"],true);
        let raw=fs::read(dir.join("stdout.log")).unwrap();
        assert_eq!(format!("{:x}",Sha256::digest(&raw)),receipt["stdout_sha256"]);
        let mut sum=[0.;8];
        let mut count=0;
        for line in std::str::from_utf8(&raw).unwrap().lines().filter_map(|l|l.strip_prefix("SAMPLE ")){
            let v:Value=serde_json::from_str(line).unwrap();
            assert_eq!(v["round"],count);
            let ms=|key:&str| {let x=v[key].as_f64().unwrap();assert!(x.is_finite() && x>=0.);x};
            if arm=="AEGLOS" {
                assert_eq!(v["cache_hit"],true);
                let mut a=[0.;8];
                for(i,k)in["witness_ms","scalars_ms","fft_ms","msm_ms","assembly_ms"].iter().enumerate(){a[i]=ms(k);}
                a[5]=ms("engine_ms")-a[..5].iter().sum::<f64>();
                a[6]=ms("verified_ms")-ms("engine_ms");
                assert!(a[5]>=-0.00001 && a[6]>=0.);
                a[7]=ms("verified_ms");
                for(i,v)in a.into_iter().enumerate(){sum[i]+=v;}
            }else{assert_eq!(arm,"ON");sum[7]+=ms("ms");}
            count+=1;
        }
        assert_eq!(count,rounds);
        samples+=count;
        data.entry((shape.into(),w,arm.into())).or_default().push((job["block"].as_u64().unwrap() as usize,sum.map(|s|s/count as f64)));
    }
    let shapes=data.keys().filter(|(_,w,a)|*w==32 && a=="ON").map(|(s,_,_)|s).collect::<Vec<_>>();
    let mut result=Vec::new();
    for s in shapes {
        let get=|w,a:&str|{
            let mut rows=data[&(s.clone(),w,a.to_owned())].clone();
            rows.sort_by_key(|v|v.0);assert_eq!(rows.len(),blocks);
            for(i,(b,_))in rows.iter().enumerate(){assert_eq!(*b,i);}
            rows.iter().map(|v|v.1).collect::<Vec<_>>()
        };
        let a=get(32,"AEGLOS");let b=get(96,"AEGLOS");let z=get(32,"ON");let y=get(96,"ON");
        let avg=|x:&[[f64;8]],i:usize|x.iter().map(|row|row[i]).sum::<f64>()/blocks as f64;
        let stages: BTreeMap<_,_>=STAGES.iter().enumerate().map(|(i,k)|(*k,json!({"32_ms":avg(&a,i),"96_ms":avg(&b,i),"saving_ms":avg(&a,i)-avg(&b,i),"faster_blocks":(0..blocks).filter(|&j|b[j][i]<a[j][i]).count()}))).collect();
        let largest=(0..7).max_by(|&i,&j|(avg(&a,i)-avg(&b,i)).total_cmp(&(avg(&a,j)-avg(&b,j)))).unwrap();
        result.push(json!({"shape":s,"aeglos":stages,"largest_aeglos_saving":STAGES[largest],"zopro_32_ms":avg(&z,7),"zopro_96_ms":avg(&y,7),"zopro_penalty_ms":avg(&y,7)-avg(&z,7),"gap_32_z_minus_a_ms":avg(&z,7)-avg(&a,7),"gap_96_z_minus_a_ms":avg(&y,7)-avg(&b,7)}));
    }
    json!({"samples":samples,"blocks":blocks,"rounds":rounds,"rows":result})
}
fn main(){
    let a=campaign(Path::new("../c7a-scale-profile245"),"latency",3,251);
    let b=campaign(Path::new("../full-c7a244"),"main",9,7);
    assert_eq!(a["samples"],3012);assert_eq!(b["samples"],10332);
    assert_eq!(b["rows"].as_array().unwrap().len(),41);
    fs::write("analysis.json",serde_json::to_vec_pretty(&json!({"primary":a,"catalog":b})).unwrap()).unwrap();
}
