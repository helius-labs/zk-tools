use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, fs, path::Path};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/g2-eight-window-colors176";
const W: usize = 8;
const B: usize = 2048;
const TILE: usize = 1600;
const COLORS: usize = 200;
const ROWS: usize = 200;
type Words = [u64; 4];
type Codes = [[i16; W]; 2]; // low, high; execution visits high then low.
type Bits = [[u64; B / 64]; W];
type Tokens = Vec<Vec<[i128; 2]>>;
mod split;
include!("grouping-body.rs");
fn work(g: u64, batches: u64, tails: u64) -> Value {
    let v = 7*g + 5*batches;
    let f = 5*g;
    json!({"V":v,"F":f,"M":8*tails+batches,"S":2*tails,"Fs":4*tails,
        "I":batches,"vector_IFMA_source_proxy":100*v+150*f,
        "full_8lane_groups_or_masked_rows":g,"batch_bridge_calls":batches,"scalar_tail_pairs":tails})
}
mod baseline {
    use super::*;
    include!("baseline-body.rs");
    pub fn run(codes: &[Codes]) -> (Value, Vec<Vec<bool>>) {
        let n=codes.len(); let chunk=n.div_ceil(2);
        let mut streams=Vec::new(); let mut merges=Stats::default();
        let mut phi=0u64; let mut occupied=vec![];
        for w in 0..W {
            let mut parts=vec![];
            for first in (0..n).step_by(chunk) {
                let end=(first+chunk).min(n);
                let mut win=Window::new(B);
                win.feed(codes[first..end].iter().map(|r|r[1][w]));
                phi+=(win.alive.iter().filter(|&&x|x).count()+win.projective.iter().filter(|&&x|x>0).count()) as u64;
                win.feed(codes[first..end].iter().map(|r|r[0][w]));
                let (alive,s)=win.finish();
                assert_eq!(s.stats.generic+s.projective_mixed+s.finish_joins,
                    s.arrivals-alive.iter().filter(|&&v|v).count() as u64);
                parts.push(alive); streams.push(s);
            }
            let mut alive=parts.remove(0);
            for other in parts {merges.merge(&mut alive,&other);}
            occupied.push(alive);
        }
        let sum=|f:fn(&Stream)->u64|streams.iter().map(f).sum::<u64>();
        let g=sum(|s|s.stats.lanes);let batches=sum(|s|s.stats.calls);let tails=sum(|s|s.stats.tails);
        let arrivals=sum(|s|s.arrivals);let fill=sum(|s|s.stats.generic);
        let projective=sum(|s|s.projective_mixed+s.finish_joins);
        let final_occupied=occupied.iter().flatten().filter(|&&v|v).count() as u64;
        assert_eq!(fill+projective+merges.generic,arrivals-final_occupied);
        (json!({"part_count":2,"independent_fill_jobs":16,"arrivals":arrivals,
            "first_assignments":sum(|s|s.first),"fill_affine_pairs":fill,
            "fill_work":work(g,batches,tails),"merge_pairs":merges.generic,
            "merge_work":work(merges.lanes,merges.calls,merges.tails),
            "total_affine_work":work(g+merges.lanes,batches+merges.calls,tails+merges.tails),
            "projective_mixed_additions":sum(|s|s.projective_mixed),
            "finish_projective_affine_joins":sum(|s|s.finish_joins),
            "normalization_points":sum(|s|s.normalize),
            "queued_to_projective":sum(|s|s.queued_to_projective),
            "phi_point_maps":phi,"phi_scalar_Fq_multiplications":2*phi,
            "formal_final_occupied_buckets":final_occupied,
            "generic_join_conservation":fill+projective+merges.generic,
            "affine_bucket_payload_bytes":W*2*B*128,
            "projective_and_normalization_field_costs_in_total_affine_work":false}),occupied)
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Row { index:u16, active:u8, first:u8 }
struct Color { used:Bits, order:Vec<u16> }
struct Tile { rows:Vec<Row>, colors:Vec<Color>, after:Bits, trials:u64, lane_tests:u64 }
fn occupied(bits:&Bits,w:usize,b:usize)->bool {bits[w][b/64]>>(b%64)&1!=0}
fn mark(bits:&mut Bits,w:usize,b:usize) {bits[w][b/64]|=1<<(b%64);}
fn bucket(d:i16)->usize {assert!(d!=0);let b=d.unsigned_abs() as usize-1;assert!(b<B);b}
fn plan_tile(codes:&[Codes],phase:usize,alive:&Bits)->Result<Tile,usize> {
    assert!(codes.len()<=TILE);let mut after=*alive;
    let mut rows=Vec::with_capacity(TILE);
    let mut colors:Vec<Color>=Vec::with_capacity(COLORS);
    let (mut trials,mut lane_tests)=(0,0);
    for (i,row) in codes.iter().enumerate() {
        let (mut active,mut first)=(0u8,0u8);
        for w in 0..W {let d=row[phase][w];if d==0 {continue;}
            let b=bucket(d);if occupied(&after,w,b){active|=1<<w;}else{first|=1<<w;mark(&mut after,w,b);}}
        rows.push(Row{index:i as u16,active,first});
        if active==0 {continue;}
        let mut found=None;
        for (j,c) in colors.iter().enumerate() {
            trials+=1;if c.order.len()==ROWS {continue;}
            let mut conflict=false;
            for w in 0..W {if active>>w&1!=0 {lane_tests+=1;if occupied(&c.used,w,bucket(row[phase][w])) {conflict=true;break;}}}
            if !conflict {found=Some(j);break;}
        }
        let j=match found {Some(j)=>j,None=>{
            if colors.len()==COLORS {return Err(i);}
            colors.push(Color{used:[[0;B/64];W],order:Vec::with_capacity(ROWS)});colors.len()-1
        }};
        let c=&mut colors[j];c.order.push(i as u16);
        for w in 0..W {if active>>w&1!=0 {mark(&mut c.used,w,bucket(row[phase][w]));}}
    }
    Ok(Tile{rows,colors,after,trials,lane_tests})
}
fn blank()->Tokens {vec![vec![[0;2];B];W]}
fn token(i:usize)->i128 {let x=i as i128+1;x*x+17*x+31}
fn add(p:&mut [i128;2],phase:usize,i:usize,d:i16) {p[phase]+=if d<0 {-token(i)}else{token(i)};}
fn reference(codes:&[Codes])->Tokens {
    let mut out=blank();for (i,row) in codes.iter().enumerate(){for phase in 0..2{for w in 0..W{let d=row[phase][w];if d!=0{add(&mut out[w][bucket(d)],phase,i,d);}}}}out
}
#[derive(Default)]
struct Census { arrivals:u64,first:u64,active:u64,vector_rows:u64,bridges:u64,phi:u64,
    tiles:u64,trials:u64,lane_tests:u64,max_colors:usize,max_rows:usize,
    active_hist:[u64;9],color_hist:BTreeMap<usize,u64>,seed_only_rows:u64,
    token_comparisons:u64 }
fn execute(tile:Tile,codes:&[Codes],phase:usize,offset:usize,values:&mut Tokens,c:&mut Census)->Bits {
    c.tiles+=1;c.trials+=tile.trials;c.lane_tests+=tile.lane_tests;
    c.max_colors=c.max_colors.max(tile.colors.len());
    for row in &tile.rows {
        let digits=&codes[row.index as usize][phase];let k=row.active.count_ones();
        c.first+=row.first.count_ones() as u64;c.active+=k as u64;
        c.arrivals+=(row.first.count_ones()+k) as u64;
        if k==0 {c.seed_only_rows+=1;}
        else {c.active_hist[k as usize]+=1;}
        for w in 0..W {if row.first>>w&1!=0 {
            let b=bucket(digits[w]);assert_eq!(values[w][b],[0;2]);
            add(&mut values[w][b],phase,offset+row.index as usize,digits[w]);
        }}
    }
    let mut seen_rows=vec![false;codes.len()];
    for color in &tile.colors {
        assert!(!color.order.is_empty()&&color.order.len()<=ROWS);
        c.bridges+=1;c.vector_rows+=color.order.len() as u64;
        c.max_rows=c.max_rows.max(color.order.len());*c.color_hist.entry(color.order.len()).or_default()+=1;
        let mut seen=[[0;B/64];W];
        for &i in &color.order {
            assert!(!seen_rows[i as usize]);seen_rows[i as usize]=true;
            let row=tile.rows[i as usize];assert_eq!(row.index,i);assert!(row.active!=0);
            for w in 0..W {if row.active>>w&1!=0 {
                let d=codes[i as usize][phase][w];let b=bucket(d);
                assert!(!occupied(&seen,w,b));mark(&mut seen,w,b);
                add(&mut values[w][b],phase,offset+i as usize,d);
            }}
        }
        assert_eq!(seen,color.used);
    }
    for (i,row) in tile.rows.iter().enumerate(){assert_eq!(seen_rows[i],row.active!=0);}
    tile.after
}
fn candidate(codes:&[Codes])->Value {
    let n=codes.len();let chunk=n.div_ceil(16);let mut c=Census::default();
    let mut parts=vec![];let mut formal_parts=vec![];
    for first in (0..n).step_by(chunk) {
        let end=(first+chunk).min(n);let mut values=blank();let mut alive=[[0;B/64];W];
        for phase in [1usize,0] {
            for begin in (first..end).step_by(TILE) {
                let last=(begin+TILE).min(end);
                let tile=plan_tile(&codes[begin..last],phase,&alive).expect("fixed recipe declined; do not rescue");
                alive=execute(tile,&codes[begin..last],phase,begin,&mut values,&mut c);
            }
            if phase==1 {c.phi+=alive.iter().flatten().map(|v|v.count_ones() as u64).sum::<u64>();}
        }
        // Independent direct signed sum for each part, using global row indices.
        let mut direct=blank();for i in first..end {for phase in 0..2 {for w in 0..W {let d=codes[i][phase][w];if d!=0{add(&mut direct[w][bucket(d)],phase,i,d);}}}}
        assert_eq!(values,direct);c.token_comparisons+=(W*B) as u64;
        parts.push(values);formal_parts.push(alive);
    }
    assert_eq!(parts.len(),16);
    let mut combined=parts.remove(0);let mut alive=formal_parts.remove(0);
    let (mut merge_pairs,mut merge_g,mut merge_t,mut merge_calls)=(0u64,0u64,0u64,0u64);
    for (other,bits) in parts.into_iter().zip(formal_parts) {
        for w in 0..W {for first in (0..B).step_by(512) {
            let mut m=0;for b in first..first+512 {
                m+=usize::from(occupied(&alive,w,b)&&occupied(&bits,w,b));
                for phase in 0..2 {combined[w][b][phase]+=other[w][b][phase];}
                if occupied(&bits,w,b){mark(&mut alive,w,b);}
            }
            if m>0 {merge_pairs+=m as u64;merge_g+=(m/8) as u64;merge_t+=(m%8) as u64;merge_calls+=1;}
        }}
    }
    assert_eq!(combined,reference(codes));c.token_comparisons+=(W*B) as u64;
    let final_occupied=alive.iter().flatten().map(|v|v.count_ones() as u64).sum::<u64>();
    assert_eq!(c.active+merge_pairs,c.arrivals-final_occupied);
    assert_eq!(c.active_hist.iter().sum::<u64>(),c.vector_rows);
    let active_hist_pairs=c.active_hist.iter().enumerate().map(|(k,n)|k as u64*n).sum::<u64>();
    assert_eq!(active_hist_pairs,c.active);
    json!({"part_count":16,"independent_fill_jobs":16,"arrivals":c.arrivals,
        "first_assignments":c.first,"fill_affine_pairs":c.active,
        "fill_work":work(c.vector_rows,c.bridges,0),
        "merge_pairs":merge_pairs,"merge_work":work(merge_g,merge_calls,merge_t),
        "total_affine_work":work(c.vector_rows+merge_g,c.bridges+merge_calls,merge_t),
        "inactive_fill_lane_slots":8*c.vector_rows-c.active,
        "fill_lane_utilization":c.active as f64/(8*c.vector_rows) as f64,
        "active_lanes_per_executed_row_histogram":c.active_hist,
        "color_size_histogram":c.color_hist,"tiles":c.tiles,"max_colors_per_tile":c.max_colors,
        "max_rows_per_color":c.max_rows,"color_fit_trials":c.trials,"color_active_lane_tests":c.lane_tests,
        "seed_only_or_zero_rows":c.seed_only_rows,"phi_point_maps":c.phi,
        "phi_scalar_Fq_multiplications":2*c.phi,"formal_final_occupied_buckets":final_occupied,
        "generic_join_conservation":c.active+merge_pairs,"token_bucket_comparisons":c.token_comparisons,
        "projective_mixed_additions":0,"finish_projective_affine_joins":0,"normalization_points":0,
        "affine_bucket_payload_bytes":W*16*B*128,
        "bounded_scheduler_payload_upper_bytes_per_actor":200*(2048+24+200*2)+1600*4+4096,
        "prefix_records_payload_upper_bytes_per_actor":200*640,
        "scheduler_costs_and_masking_in_affine_work":false,
        "equal_x_and_cancellation_modeled":false})
}
fn codes(words:&[Words])->Vec<Codes> {
    words.iter().map(|&word| {let short=split::phi_split(word);
        std::array::from_fn(|phase|std::array::from_fn(|w|{
            let i=2*phase;let d=split::booth(&[short[i],short[i+1]&0x7fff_ffff_ffff_ffff,0,0],w,12);
            if short[i+1]>>63!=0 {-d}else{d}
        }))
    }).collect()
}
fn hash(bytes:&[u8])->String {format!("{:x}",Sha256::digest(bytes))}
fn main() {
    let out=format!("{ROOT}/result.json");assert!(!Path::new(&out).exists());
    assert_eq!(std::mem::size_of::<Row>(),4);assert_eq!(std::mem::size_of::<Bits>(),2048);
    let inputs:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/g2-joint-ring-screen176/inputs.json").unwrap()).unwrap();
    let mut cases=vec![];
    for (name,input) in inputs.as_object().unwrap() {
        let bytes=fs::read(input["file"].as_str().unwrap()).unwrap();assert_eq!(hash(&bytes),input["sha256"].as_str().unwrap());
        assert_eq!(&bytes[..8],b"B2SCAN01");let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
        assert_eq!(bytes.len(),16+160*n);
        assert!(bytes[16..16+128*n].chunks_exact(128).all(|p|p.iter().any(|&v|v!=0)),"identity fixture needs separate model");
        let words=bytes[16+128*n..].chunks_exact(32).map(|b|std::array::from_fn(|i|u64::from_le_bytes(b[8*i..8*i+8].try_into().unwrap()))).collect::<Vec<Words>>();
        let grouped=grouping(&words);let effective=grouped.as_deref().unwrap_or(&words);
        assert_eq!(effective.len(),match name.as_str(){"1x2-B"=>30089,"2x2-B"=>28852,_=>panic!("unexpected fixture")});
        let digits=codes(effective);let (original,occupied)=baseline::run(&digits);let next=candidate(&digits);
        let mut direct_occupied=vec![vec![false;B];W];for row in &digits {for phase in 0..2 {for w in 0..W {if row[phase][w]!=0 {direct_occupied[w][bucket(row[phase][w])]=true;}}}}
        assert_eq!(occupied,direct_occupied);
        for key in ["arrivals","generic_join_conservation","formal_final_occupied_buckets"] {assert_eq!(original[key],next[key],"{key}");}
        let ratio=|key:&str|next["total_affine_work"][key].as_u64().unwrap() as f64/original["total_affine_work"][key].as_u64().unwrap() as f64;
        cases.push(json!({"case":name,"fixture_sha256":hash(&bytes),"raw_rows":n,
            "grouping_selected":grouped.is_some(),"effective_rows":effective.len(),"original":original,"candidate":next,
            "ratios_candidate_original":{"vector_IFMA_source_proxy":ratio("vector_IFMA_source_proxy"),"I":ratio("I")},
            "unchanged_weighted_tree_formal_occupancy_checked":true}));
    }
    let result=json!({"complete":true,"kind":"structural_count_and_signed_token_schedule",
        "cases":cases,"SDK_requests":0,"Gnark_exports":0,"production_changed":false,"adopted":false,
        "native_measurement_selected":false,"goal_achieved":false,"canonical_verified_proof_count":36477,
        "limitations":["No timing, no cryptographic curve/proof oracle, no native mask or exception dispatcher",
            "Scalar projective/normalization work remains separately counted, not priced in vector proxy",
            "Scheduler/broadcast/select/gather/scatter and sign costs are unpriced additions",
            "Payload model is not native allocations, RSS, minimum RAM or mobile qualification"]});
    fs::write(out,serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    for case in result["cases"].as_array().unwrap() {println!("{} rows={} vector_proxy_ratio={} inverse_ratio={} utilization={}",case["case"],case["effective_rows"],case["ratios_candidate_original"]["vector_IFMA_source_proxy"],case["ratios_candidate_original"]["I"],case["candidate"]["fill_lane_utilization"]);}
}
#[cfg(test)] mod tests {
    use super::*;
    #[test]fn decline_preserves_input_and_has_no_partial_commit() {
        let digits=vec![[[1;W];2];TILE];let before=[[0;B/64];W];
        assert!(plan_tile(&digits,1,&before).is_err());assert_eq!(before,[[0;B/64];W]);
        let mut alive=before;for w in 0..W {mark(&mut alive,w,0);}
        let plan=plan_tile(&digits[..200],1,&alive).unwrap();assert_eq!(plan.colors.len(),200);
        assert!(plan_tile(&digits[..201],1,&alive).is_err());
    }
    #[test]fn schedule_boundaries_signs_zeros_and_bucket_reuse() {
        let mut tested=0;
        for n in [0usize,1,7,8,9,199,200,201,1599,1600,1601,3201] {
            for pattern in 0..4 {
                let digits=(0..n).map(|i| std::array::from_fn(|phase|std::array::from_fn(|w|{
                    if pattern==0||(i+3*w+phase)%11==0 {0}else{
                        let b=match pattern {1=>(i*37+w*13+phase)%B,2=>(i+phase)%17,_=>(i/8+phase+w)%B};
                        let d=(b+1) as i16;if (i+w+phase)%3==0 {-d}else{d}
                    }
                }))).collect::<Vec<Codes>>();
                let mut values=blank();let mut alive=[[0;B/64];W];let mut c=Census::default();
                for phase in [1usize,0] {for first in (0..n).step_by(TILE) {
                    let end=(first+TILE).min(n);let tile=plan_tile(&digits[first..end],phase,&alive).unwrap();
                    alive=execute(tile,&digits[first..end],phase,first,&mut values,&mut c);
                }}
                assert_eq!(values,reference(&digits),"n={n} pattern={pattern}");
                let occupied=alive.iter().flatten().map(|v|v.count_ones() as u64).sum::<u64>();
                assert_eq!(c.first,occupied);assert_eq!(c.first+c.active,c.arrivals);
                tested+=1;
            }
        }
        assert_eq!(tested,48);
    }
}
