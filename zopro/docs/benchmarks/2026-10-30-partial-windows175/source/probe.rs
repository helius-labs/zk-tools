use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, fs};
use zopro_bn254::{Fq, Fq2, Fr, G2Affine};
mod split;
type Word = [u64; 4];
type Digits = [i16; 22];
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/g2-queue-scan175";

// Same zero-aware admission and stable class order as the accepted policy.
fn groups(words:&[Word])->Option<(Vec<Word>,Vec<usize>)>{let n=words.len();if n<256{return None}let sample=n.min(2048);let stride=n.div_ceil(sample);let seen=words.iter().step_by(stride).take(sample).collect::<BTreeSet<_>>();let observed=n.div_ceil(stride).min(sample);if seen.len()*200>observed*199{return None}let percent=if(32768..65536).contains(&n){95}else{80};let zeros=words.iter().step_by(stride).take(sample).filter(|s|**s==[0;4]).count();let nonzero=observed-zeros;if seen.len()-usize::from(zeros!=0)==nonzero&&nonzero*100>observed*percent{return None}let mut map=BTreeMap::new();let mut values=vec![];let mut classes=vec![usize::MAX;n];for(i,&v)in words.iter().enumerate(){if v==[0;4]{continue}let class=*map.entry(v).or_insert_with(||{values.push(v);values.len()-1});classes[i]=class;}if values.len()*100>n*percent{None}else{Some((values,classes))}}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Pair { left: usize, right: usize, mask: u32 }
#[derive(Default)]
struct Plan {
    masks: Vec<u32>, pairs: Vec<Pair>, comparisons: usize,
    column_comparisons: usize, eligible: usize, tiles: usize,
    peak_candidates: usize, peak_candidate_capacity: usize, largest_tile: usize,
    tails: Vec<usize>, full_pairs: usize, partial_pairs: usize,
    full_removed: usize, partial_removed: usize,
    matched_histogram: BTreeMap<usize, usize>,
}
fn matches(a: &Digits, b: &Digits) -> u32 {
    (0..22).fold(0, |m,c| m | (u32::from(a[c]!=0 && a[c]==b[c])<<c))
}
fn plan(rows: &[Digits], active: &[bool], parts: usize) -> Plan {
    assert_eq!(rows.len(),active.len()); assert!(parts>0);
    let mut p=Plan { masks:vec![0;rows.len()], ..Plan::default() };
    let chunk=rows.len().div_ceil(parts);
    for part in 0..parts {
        let start=(part*chunk).min(rows.len()); let end=(start+chunk).min(rows.len());
        for first in (start..end).step_by(64) {
            let last=(first+64).min(end); let n=last-first;
            p.tiles+=1; p.largest_tile=p.largest_tile.max(n);
            if n<64 {p.tails.push(n);}
            let mut candidates=Vec::new();
            for i in first..last { if !active[i] {continue;}
                for j in i+1..last { if !active[j] {continue;}
                    p.comparisons+=1; p.column_comparisons+=22;
                    let mask=matches(&rows[i],&rows[j]);
                    if mask.count_ones()>=2 {candidates.push(Pair{left:i,right:j,mask});}
                }
            }
            p.eligible+=candidates.len(); p.peak_candidates=p.peak_candidates.max(candidates.len());
            p.peak_candidate_capacity=p.peak_candidate_capacity.max(candidates.capacity());
            candidates.sort_unstable_by_key(|x| (std::cmp::Reverse(x.mask.count_ones()),x.left,x.right));
            let mut used=[false;64]; let before=p.pairs.len();
            for pair in candidates {
                if used[pair.left-first]||used[pair.right-first] {continue;}
                used[pair.left-first]=true;used[pair.right-first]=true;
                p.masks[pair.left]=pair.mask;p.masks[pair.right]=pair.mask;
                let k=pair.mask.count_ones() as usize;
                *p.matched_histogram.entry(k).or_default()+=1;
                if rows[pair.left]==rows[pair.right] {p.full_pairs+=1;p.full_removed+=k;}
                else {p.partial_pairs+=1;p.partial_removed+=k;}
                for c in 0..22 {
                    let common=if pair.mask>>c&1!=0 {rows[pair.left][c]} else {0};
                    let left=if pair.mask>>c&1!=0 {0} else {rows[pair.left][c]};
                    let right=if pair.mask>>c&1!=0 {0} else {rows[pair.right][c]};
                    assert_eq!(left as i32+common as i32,rows[pair.left][c] as i32);
                    assert_eq!(right as i32+common as i32,rows[pair.right][c] as i32);
                }
                assert_eq!(pair.left/chunk,pair.right/chunk);
                p.pairs.push(pair);
            }
            assert!(p.pairs.len()-before<=n/2);
        }
    }
    assert_eq!(p.full_pairs+p.partial_pairs,p.pairs.len());p
}
fn scalar(w:&Word)->Fr {Fr::from_le_bytes_mod_order(&w.iter().flat_map(|v|v.to_le_bytes()).collect::<Vec<_>>())}
fn signed(v:i128)->Fr {let x=Fr::from_le_bytes_mod_order(&v.unsigned_abs().to_le_bytes());if v<0 {-x}else{x}}
fn reconstruct(d:&Digits,high:bool)->i128 {
    (0..11).map(|w| (d[usize::from(high)*11+w] as i128)*(1i128<<(12*w))).sum()
}
fn digits(words:&[Word])->Vec<Digits> {
    let lambda=split::phi_lambda();
    words.iter().map(|w|{
        let short=split::phi_split(*w);
        let d=core::array::from_fn(|c|split::digit_of(&short,c>=11,c%11,12));
        for high in [false,true] {
            let i=usize::from(high)*2;
            let mag=short[i] as u128|((short[i+1]&0x7fff_ffff_ffff_ffff)as u128)<<64;
            assert!(mag<1u128<<126);
            let component=if short[i+1]>>63!=0 {-(mag as i128)}else{mag as i128};
            assert_eq!(reconstruct(&d,high),component);
        }
        assert_eq!(signed(reconstruct(&d,false))+lambda*signed(reconstruct(&d,true)),scalar(w));
        d
    }).collect()
}
fn token(seed:&mut u64)->Fr {
    let mut b=[0;32];for chunk in b.chunks_exact_mut(8){*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}
    Fr::from_le_bytes_mod_order(&b)
}
fn feed(sums:&mut [Fr], counts:&mut [usize], d:i16, value:Fr, signed_bins:bool) {
    if d==0 {return;}
    let half=if signed_bins{sums.len()/2}else{sums.len()};
    let b=d.unsigned_abs() as usize-1;assert!(b<half);
    let slot=b+if signed_bins && d<0 {half}else{0};
    sums[slot]+=if !signed_bins && d<0 {-value}else{value};counts[slot]+=1;
}
fn oracle(rows:&[Digits],active:&[bool],tokens:&[Fr],p:&Plan,parts:usize)->(Value,Fr) {
    assert_eq!(rows.len(),tokens.len());
    let aux=p.pairs.iter().map(|x|tokens[x.left]+tokens[x.right]).collect::<Vec<_>>();
    let chunk=rows.len().div_ceil(parts);let lambda=split::phi_lambda();
    let mut baseline_arrivals=0usize;let mut after_arrivals=0usize;
    let mut baseline_proxy=0usize;let mut after_proxy=0usize;let mut weighted=Fr::ZERO;
    let mut checked_bins=0;let mut occupied_bins=0;
    for part in 0..parts {
        let first=(part*chunk).min(rows.len());let last=(first+chunk).min(rows.len());
        for c in 0..22 {
            let top=c%11==10; let half=if top{64}else{2048};let size=if top{2*half}else{half};
            let mut expected=vec![Fr::ZERO;size];let mut got=vec![Fr::ZERO;size];
            let mut before=vec![0;size];let mut after=vec![0;size];
            for i in first..last {if active[i] {
                feed(&mut expected,&mut before,rows[i][c],tokens[i],top);
                if p.masks[i]>>c&1==0 {feed(&mut got,&mut after,rows[i][c],tokens[i],top);}
            }}
            for (a,pair) in p.pairs.iter().enumerate() {
                if (first..last).contains(&pair.left) && pair.mask>>c&1!=0 {
                    assert!((first..last).contains(&pair.right));
                    feed(&mut got,&mut after,rows[pair.left][c],aux[a],top);
                }
            }
            assert_eq!(expected,got,"every bucket token sum");checked_bins+=size;
            for (b,a) in before.iter().zip(&after) {assert_eq!(*b!=0,*a!=0);}
            let nb=before.iter().sum::<usize>();let na=after.iter().sum::<usize>();
            let occupied=before.iter().filter(|&&n|n!=0).count();occupied_bins+=occupied;
            baseline_arrivals+=nb;after_arrivals+=na;
            baseline_proxy+=nb-occupied;after_proxy+=na-occupied;
            let mut sum=Fr::ZERO;
            for b in 0..half {
                let value=if top{got[b]-got[half+b]}else{got[b]};
                sum+=Fr::from((b+1)as u64)*value;
            }
            if c>=11 {sum*=lambda;}
            weighted+=sum*Fr::from(2u64).pow(&[(12*(c%11))as u64]);
        }
    }
    let removed=p.full_removed+p.partial_removed;
    assert_eq!(baseline_arrivals-after_arrivals,removed);
    assert_eq!(baseline_proxy-after_proxy,removed);
    assert!(removed>=2*p.pairs.len());
    (json!({"baseline_arrivals":baseline_arrivals,"after_arrivals":after_arrivals,
        "removed_arrivals":removed,"baseline_generic_bucket_tree_additions":baseline_proxy,
        "after_generic_bucket_tree_additions":after_proxy,"charged_auxiliary_point_additions":p.pairs.len(),
        "generic_net_additions_saved":removed-p.pairs.len(),
        "generic_net_fraction":(removed-p.pairs.len())as f64/baseline_proxy.max(1)as f64,
        "bins_checked_per_token_stream":checked_bins,"structurally_occupied_bins":occupied_bins}),weighted)
}
fn read(path:&str)->Value {serde_json::from_slice(&fs::read(path).unwrap()).unwrap()}
fn census(shape:&str)->Value {
    let path=format!("{ROOT}/inputs/{shape}.b2q");let b=fs::read(&path).unwrap();
    let receipt=read(&format!("{ROOT}/oracle-fixed-runs/oracle-{shape}/stdout.log"));
    let hash=format!("{:x}",Sha256::digest(&b));assert_eq!(hash,receipt["fixture_sha256"]);
    assert_eq!(receipt["independent_Arkworks_complete_MSM"],true);assert_eq!(&b[..8],b"B2SCAN01");
    let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;
    assert_eq!(n,receipt["points"]);assert_eq!(b.len(),16+160*n);
    let live=b[16..16+128*n].chunks_exact(128).map(|raw|{
        let fields=raw.chunks_exact(32).map(|v|Fq::from_be_bytes(v.try_into().unwrap()).expect("canonical field")).collect::<Vec<_>>();
        let point=G2Affine::new(Fq2::new(fields[1],fields[0]),Fq2::new(fields[3],fields[2]));
        assert!(point.is_on_curve());!point.is_identity()
    }).collect::<Vec<_>>();
    let original=b[16+128*n..].chunks_exact(32).map(|v|core::array::from_fn(|i|u64::from_le_bytes(v[i*8..i*8+8].try_into().unwrap()))).collect::<Vec<Word>>();
    for w in &original {assert_eq!(scalar(w).to_canonical(),*w);}
    drop(b);
    let grouping=groups(&original);
    let words=grouping.as_ref().map_or_else(||original.clone(),|g|g.0.clone());
    let active=if let Some((_,mapping))=&grouping {
        let mut out=vec![false;words.len()];for (&class,&live) in mapping.iter().zip(&live) {if class!=usize::MAX {out[class]|=live;}}out
    }else{live.clone()};
    assert!((1024..65536).contains(&words.len()));
    let rows=digits(&words);let p=plan(&rows,&active,2);let mut first=None;
    for tag in 0..2 {
        let mut seed=0x7061727469616c75u64^tag;
        let raw_tokens=live.iter().map(|&live|{let t=token(&mut seed);if live{t}else{Fr::ZERO}}).collect::<Vec<_>>();
        let tokens=if let Some((_,mapping))=&grouping {
            let mut out=vec![Fr::ZERO;words.len()];for (&class,&value) in mapping.iter().zip(&raw_tokens) {if class!=usize::MAX{out[class]+=value;}}out
        }else{raw_tokens.clone()};
        let target=original.iter().zip(&raw_tokens).map(|(w,&t)|scalar(w)*t).fold(Fr::ZERO,|a,b|a+b);
        assert_eq!(words.iter().zip(&tokens).map(|(w,&t)|scalar(w)*t).fold(Fr::ZERO,|a,b|a+b),target);
        let (counts,weighted)=oracle(&rows,&active,&tokens,&p,2);assert_eq!(weighted,target);
        if let Some(old)=&first {assert_eq!(old,&counts);}else{first=Some(counts);}
    }
    let counts=first.unwrap();
    json!({"shape":shape,"fixture_sha256":hash,"original_rows":n,"post_group_rows":words.len(),
        "grouping_admitted":grouping.is_some(),"input_identity_rows":live.iter().filter(|&&v|!v).count(),
        "structurally_inactive_post_group_rows":active.iter().filter(|&&v|!v).count(),
        "groups_may_cancel_on_real_curve":grouping.is_some(),"width":12,"components":2,"point_parts":2,
        "tile_rows":64,"tiles":p.tiles,"largest_tile":p.largest_tile,"tail_tiles":p.tails,
        "pair_comparisons":p.comparisons,"signed_column_comparisons":p.column_comparisons,
        "eligible_pairs":p.eligible,"selected_pairs":p.pairs.len(),"whole_duplicate_pairs":p.full_pairs,
        "genuine_partial_pairs":p.partial_pairs,"whole_duplicate_removed_arrivals":p.full_removed,
        "genuine_partial_removed_arrivals":p.partial_removed,
        "genuine_partial_generic_net_additions_saved":p.partial_removed-p.partial_pairs,
        "shared_columns_histogram":p.matched_histogram,"peak_tile_candidate_records":p.peak_candidates,
        "peak_count_helper_candidate_capacity":p.peak_candidate_capacity,
        "count_helper_candidate_record_bytes":std::mem::size_of::<Pair>(),
        "hypothetical_runtime_extra_requested_payload_bytes":4*words.len()+140*p.pairs.len(),
        "payload_excludes_capacity_rounding_headers_tile_normalization_allocator_stacks_existing_workspace":true,
        "every_rewrite_column_conserved_exactly":true,"every_Phi_Booth_reconstruction_checked":true,
        "every_bin_and_weighted_sum_checked_with_two_Fr_token_streams":true,
        "actual_curve_exceptions_inversions_and_latency_unmodeled":true,"counts":counts})
}
fn main() {
    let rows=["1x2","2x2"].map(census);
    let pass=rows.iter().all(|v|v["counts"]["generic_net_additions_saved"].as_u64().unwrap()>0 && v["genuine_partial_pairs"].as_u64().unwrap()>0);
    for row in &rows {println!("ROW {}",row);}
    println!("COMPLETE {}",json!({"necessary_opportunity_gate":pass,"algebra_oracles":true,
        "production_changed":false,"performance_claim":false,"new_solver_calls":0,"SDK_requests":0,
        "new_independent_exports":0,"proof_ledger":25231,"goal_achieved":false}));
}

#[cfg(test)] mod tests {
    use super::*;
    fn check(rows:Vec<Digits>,parts:usize)->Plan {
        let active=vec![true;rows.len()];let p=plan(&rows,&active,parts);
        for tag in 0..2 {
            let mut seed=0x7465737473^tag;let tokens=(0..rows.len()).map(|_|token(&mut seed)).collect::<Vec<_>>();
            let target=rows.iter().zip(&tokens).map(|(d,&t)|(signed(reconstruct(d,false))+split::phi_lambda()*signed(reconstruct(d,true)))*t).fold(Fr::ZERO,|a,b|a+b);
            assert_eq!(oracle(&rows,&active,&tokens,&p,parts).1,target);
        }p
    }
    #[test] fn partial_negative_zero_and_full_duplicates() {
        let mut a=[0;22];a[0]=-2048;a[1]=2048;a[10]=63;a[11]=-17;a[21]=-64;
        let mut b=a;b[2]=7;let mut c=a;c[11]=9;
        let p=check(vec![a,a,b,c,[0;22]],1);
        assert_eq!(p.full_pairs,1);assert_eq!(p.partial_pairs,1);assert_eq!(p.pairs[0].left,0);assert_eq!(p.pairs[0].right,1);
    }
    #[test] fn empty_singleton_odd_tails_and_partition_boundaries() {
        for n in [0,1,2,3,63,64,65,127,128,129] {for parts in [1,2,3] {
            let mut row=[0;22];row[0]=3;row[13]=-4;let p=check(vec![row;n],parts);
            let chunk=n.div_ceil(parts);for pair in &p.pairs {assert_eq!(pair.left/chunk,pair.right/chunk);}
        }}
        let mut rows=vec![[0;22];128];rows[63][0]=7;rows[63][11]=9;rows[64]=rows[63];
        assert_eq!(check(rows,2).pairs.len(),0);
    }
    #[test] fn distinct_rows_single_match_and_inactive_points_do_not_merge() {
        let mut a=[0;22];a[0]=7;a[2]=8;let mut b=a;b[2]=9;
        assert!(check(vec![a,b],1).pairs.is_empty());
        assert!(plan(&[a,a],&[true,false],1).pairs.is_empty());
    }
    #[test] fn signed_phi_digit_reconstruction_edges_and_deterministic_scalars() {
        let mut words=vec![Fr::ZERO.to_canonical(),Fr::ONE.to_canonical(),(-Fr::ONE).to_canonical()];
        let mut seed=1597;for _ in 0..8192 {words.push(token(&mut seed).to_canonical());}
        assert_eq!(digits(&words).len(),8195);
        for v in [0u128,1,(1u128<<126)-1,1u128<<120,(1u128<<120)-1] {for negative in [false,true] {
            let value=v|if negative{1u128<<127}else{0};let short=[value as u64,(value>>64)as u64,value as u64,(value>>64)as u64];
            let d=core::array::from_fn(|c|split::digit_of(&short,c>=11,c%11,12));
            for high in [false,true] {assert_eq!(reconstruct(&d,high),if negative{-(v as i128)}else{v as i128});}
        }}
    }
}
