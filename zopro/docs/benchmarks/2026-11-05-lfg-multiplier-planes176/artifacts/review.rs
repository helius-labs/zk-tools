#[path="split.rs"]mod split;
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf},collections::{BTreeSet,BTreeMap}};
const R:&str="/home/ubuntu/src/zopro-perf-work/lfg-multiplier-planes176";fn r(s:impl AsRef<Path>)->PathBuf{Path::new(R).join(s)}fn load(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn candidate(w:[u64;4],bits:usize)->Vec<(i64,i64)>{let mut out=Vec::new();let mut carry=0;for col in 0..bits.div_ceil(13){let mut t=carry;for b in 0..13{let bit=col*13+b;if bit<256&&w[bit/64]>>(bit%64)&1!=0{t+=1i64<<b;}}if col+1==bits.div_ceil(13){out.push((t,1));continue;}let residue=t.rem_euclid(8192);let abs=residue.min(8192-residue);if abs==0{out.push((0,1));carry=t/8192;continue;}let mut odd=abs;while odd%2==0{odd/=2;}let m=if odd%8==1||odd%8==7{1}else{3};let bucket=if m==1{abs}else{let z=(residue*2731)%8192;z.min(8192-z)};let signed=if(m*bucket)%8192==residue{bucket}else{-bucket};carry=(t-m*signed)/8192;out.push((signed,m));}out}
// Counts via closed occupancy formulas; no symbolic point tree or field operations.
fn weighted(flags: &[bool]) -> [u64; 3] {
    let (mut generic, mut adds, mut doubles) = (0, 0, 0);
    let mut active_groups = Vec::new();
    for (index, group) in flags.chunks(256).enumerate() {
        let active = group.iter().filter(|x| **x).count() as u64;
        generic += active.saturating_sub(1);
        if active != 0 {
            active_groups.push(index);
            doubles += group.len().ilog2() as u64;
        }
        let mut half = 1;
        while half < group.len() {
            let selected = (0..group.len()).step_by(2*half)
                .filter(|&i| group[i..i+half].iter().any(|x| *x)).count() as u64;
            generic += selected.saturating_sub(1);
            adds += u64::from(selected != 0);
            half *= 2;
        }
    }
    if let Some(&last) = active_groups.last() {
        adds += 2 * (active_groups.len() as u64 - 1) + last as u64;
        doubles += 8 * u64::from(last != 0);
    }
    [generic, adds, doubles]
}
fn counts(words: &[[u64;4]], ids: &[bool], g2: bool) -> [u64; 5] {
    let parts = if g2 {2} else {1};
    let columns = if g2 {10} else {20};
    let components = words.iter().map(|&w| if g2 {
        let s = split::phi_split(w);
        vec![([s[2],s[3]&0x7fff_ffff_ffff_ffff,0,0],s[3]>>63!=0),
             ([s[0],s[1]&0x7fff_ffff_ffff_ffff,0,0],s[1]>>63!=0)]
    } else {vec![(w,false)]}).collect::<Vec<_>>();
    let codes = components.iter().map(|row| row.iter().map(|(w,_)|
        candidate(*w,if g2 {126} else {254})).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut total = [0u64;5]; // fill, weight, merges, projective additions, doublings
    let mut live_columns = Vec::new();
    for column in 0..columns {
        let top = column+1 == columns;
        let capacity = if top {if g2 {1024} else {256}} else {4096};
        let mut live_planes = Vec::new();
        for multiplier in [1,3] {
            if top && multiplier == 3 {continue;}
            let mut union = BTreeSet::new();
            for part in 0..parts {
                let lo = part * words.len().div_ceil(parts);
                let hi = ((part+1)*words.len().div_ceil(parts)).min(words.len());
                let mut ordinary = Vec::new();
                let mut part_union = BTreeSet::new();
                for comp in 0..if g2 {2} else {1} {
                    let mut signed_top = Vec::new();
                    for i in lo..hi {
                        if ids[i] {continue;}
                        let (mut digit,m) = codes[i][comp][column];
                        if m != multiplier || digit == 0 {continue;}
                        if components[i][comp].1 {digit = -digit;}
                        let index = digit.unsigned_abs() as usize - 1;
                        assert!(index < capacity);
                        if top {signed_top.push(index + usize::from(digit<0)*capacity);}
                        else {ordinary.push(index);}
                    }
                    if top {
                        signed_top.sort_unstable();
                        total[0] += signed_top.len() as u64;
                        signed_top.dedup();
                        total[0] -= signed_top.len() as u64;
                        let pos = signed_top.iter().copied().filter(|&i| i<capacity).collect::<BTreeSet<_>>();
                        let neg = signed_top.iter().copied().filter(|&i| i>=capacity).map(|i| i-capacity).collect::<BTreeSet<_>>();
                        total[2] += pos.intersection(&neg).count() as u64;
                        let component = pos.union(&neg).copied().collect::<BTreeSet<_>>();
                        total[2] += part_union.intersection(&component).count() as u64;
                        part_union.extend(component);
                    }
                }
                if !top {
                    ordinary.sort_unstable();
                    total[0] += ordinary.len() as u64;
                    ordinary.dedup();
                    total[0] -= ordinary.len() as u64;
                    part_union.extend(ordinary);
                }
                total[2] += union.intersection(&part_union).count() as u64;
                union.extend(part_union);
            }
            let flags = (0..capacity).map(|i|union.contains(&i)).collect::<Vec<_>>();
            let [generic,adds,doubles] = weighted(&flags);
            total[1] += generic;
            total[3] += adds;
            total[4] += doubles;
            let live = !union.is_empty();
            if multiplier==3 && live {total[3]+=1; total[4]+=1;}
            live_planes.push(live);
        }
        if live_planes.len()==2 && live_planes[0] && live_planes[1] {total[3]+=1;}
        live_columns.push(live_planes.into_iter().any(|v|v));
    }
    let active = live_columns.iter().filter(|v|**v).count();
    if active != 0 {
        total[3] += active as u64 - 1;
        total[4] += 13 * live_columns.iter().rposition(|v|*v).unwrap() as u64;
    }
    total
}
fn main() {
    let pins=load(r("pins.json"));
    for (p,h) in pins.as_object().unwrap() {assert_eq!(hash(p),h.as_str().unwrap());}
    assert_eq!(load(r("runs/result.json"))["status"],"complete");
    for step in load(r("plan.json"))["steps"].as_array().unwrap() {
        let n=step["name"].as_str().unwrap();
        let e=load(r(format!("runs/{n}/exit.json")));
        assert_eq!(e["returncode"],0); assert_eq!(e["success"],true);
        for stream in ["stdout","stderr"] {assert_eq!(hash(r(format!("runs/{n}/{stream}.log"))),e[format!("{stream}_sha256")]);}
    }
    let tests=load(r("runs/plane-tests/tests.json"));
    assert_eq!(tests["passed_tests"].as_array().unwrap().len(),3);
    assert_eq!(tests["unfiltered"],true);
    let manifest=load("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    for (p,h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),h.as_str().unwrap());
    }
    let a=load(r("analysis.json")); let inputs=load(r("inputs.json"));
    let prior=load("/home/ubuntu/src/zopro-perf-work/lfg-tiled-count176/analysis.json");
    let mut evidence=BTreeMap::new(); let mut bypass=0;
    for row in a["rows"].as_array().unwrap() {
        let name=row["case"].as_str().unwrap();
        if row["global_grouping_admitted"]==true {assert_eq!(row["candidate_selected"],false);bypass+=1;continue;}
        let input=&inputs[name]; let bytes=fs::read(input["file"].as_str().unwrap()).unwrap();
        let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
        let g2=input["query"]=="B"; let mut words=Vec::new(); let mut ids=Vec::new();
        assert_eq!(bytes.len(),16+n*if g2 {160} else {33});
        for i in 0..n {
            let offset=if g2 {16+128*n+32*i} else {16+33*i};
            words.push(std::array::from_fn(|j|u64::from_le_bytes(bytes[offset+8*j..offset+8*j+8].try_into().unwrap())));
            ids.push(if g2 {bytes[16+128*i..16+128*(i+1)].iter().all(|b|*b==0)} else {bytes[offset+32]!=0});
        }
        let c=counts(&words,&ids,g2);
        for (i,key) in ["fill_generic_pairs","weighted_generic_pairs","merge_generic_pairs","projective_combines","projective_doublings"].into_iter().enumerate() {assert_eq!(row["late_multiplier_planes"][key],c[i],"{name}/{key}");}
        let old=prior["rows"].as_array().unwrap().iter().find(|v|v["case"]==name).unwrap();
        assert_eq!(row["baseline"],old["baseline"]);
        let net=row["baseline"]["generic_pairs_excluding_formation"].as_i64().unwrap()-(c[0]+c[1]+c[2])as i64;
        assert_eq!(row["net_generic_pairs_saved"],net);
        assert_eq!(row["candidate_selected"],g2); assert_eq!(row["G1_diagnostic_only"],!g2);
        assert_eq!(row["input_triples"],0); assert_eq!(row["code_payload_bytes_if_u16_20"],40*n);
        if g2 {assert!(net>0);assert_eq!(a["primary_G2_necessary_count_screen_passed"],true);}
        evidence.insert(name.to_string(),c);
    }
    assert_eq!(bypass,2); assert_eq!(evidence.len(),2);
    let v=json!({"passed":true,"pin_entries":pins.as_object().unwrap().len(),"successful_commands":4,"new_tests":3,"production_originals_exact":176,"independent_sorted_bucket_and_closed_tree_counts":evidence,"baseline_reused_exact_not_rerun":true,"global_grouping_bypasses":bypass,"primary_G2_necessary_count_screen_passed":true,"complete_curve_resource_and_caller_qualification_pending":true,"new_proofs":0,"proof_ledger":29461,"adopted":false});
    fs::write(r("review.json"),serde_json::to_vec_pretty(&v).unwrap()).unwrap(); println!("{v}");
}
