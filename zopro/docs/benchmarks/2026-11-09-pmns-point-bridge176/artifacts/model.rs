use num_bigint::{BigInt, Sign};
use serde_json::{json, Value};
use std::{fs, path::Path};

const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/pmns-point-bridge176";
const PARAM: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-23-pmns-field-component/experiment/parameters.json";
const PRIME: &str = "21888242871839275222246405745257275088696311157297823662689037894645226208583";
type Poly = [BigInt; 6];
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn save(name: &str, v: &Value) { let p = format!("{ROOT}/{name}"); assert!(!Path::new(&p).exists()); fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap(); }
fn rem(x: BigInt, p: &BigInt) -> BigInt { let x=x%p; if x.sign()==Sign::Minus { x+p } else { x } }
fn abs(x: &BigInt) -> BigInt { if x.sign()==Sign::Minus { -x } else { x.clone() } }
fn inverse(a: &BigInt, modulus: &BigInt) -> BigInt {
    let (mut old_r,mut r)=(modulus.clone(),rem(a.clone(),modulus));
    let (mut old_t,mut t)=(BigInt::from(0),BigInt::from(1));
    while r!=BigInt::from(0) { let q=&old_r/&r; let nr=&old_r-&q*&r; old_r=r; r=nr; let nt=&old_t-&q*&t; old_t=t; t=nt; }
    assert_eq!(old_r,BigInt::from(1)); let out=rem(old_t,modulus); assert_eq!(rem(a*&out,modulus),BigInt::from(1)); out
}
fn convolution(a: &Poly, b: &Poly) -> Poly {
    let mut raw: [BigInt;11]=core::array::from_fn(|_|BigInt::from(0));
    for i in 0..6 { for j in 0..6 { raw[i+j]+=&a[i]*&b[j]; } }
    for i in (6..11).rev() { let high=raw[i].clone(); raw[i-6]+=high*2u32; }
    core::array::from_fn(|i|raw[i].clone())
}
fn eval(a: &Poly, gamma: &BigInt, p: &BigInt) -> BigInt { a.iter().rev().fold(BigInt::from(0),|v,x|rem(v*gamma+x,p)) }
fn redc(t: &Poly, m: &Poly, n: &Poly, b: &BigInt) -> Poly {
    let low=t.clone().map(|v|rem(v,b)); let q=convolution(n,&low).map(|v|rem(v,b)); let qm=convolution(m,&q);
    core::array::from_fn(|i|{ let value=&t[i]+&qm[i]; assert_eq!(&value%b,BigInt::from(0)); value/b })
}
fn strings(a: &Poly) -> Vec<String> { a.iter().map(ToString::to_string).collect() }

// The exact public input is pinned. Extract complete array-object slices so
// p/gamma are parsed from decimal bytes rather than JSON floating numbers.
fn candidate_objects(text: &str) -> Vec<&str> {
    let key=text.find("\"candidates\"").unwrap(); let start=key+text[key..].find('[').unwrap();
    let mut array=0usize; let mut brace=0usize; let mut quoted=false; let mut escaped=false; let mut begin=0; let mut out=vec![];
    for (offset,&ch) in text.as_bytes()[start..].iter().enumerate() {
        let i=start+offset;
        if quoted { if escaped { escaped=false; } else if ch==b'\\' { escaped=true; } else if ch==b'"' { quoted=false; } continue; }
        match ch { b'"'=>quoted=true, b'['=>array+=1, b']'=>{array-=1;if array==0 { break; }}, b'{'=>{if brace==0 {begin=i;}brace+=1;}, b'}'=>{brace-=1;if brace==0 {out.push(&text[begin..=i]);}}, _=>{} }
    }
    assert_eq!(array,0); assert!(!quoted); out
}
fn decimal(object: &str,key: &str) -> BigInt {
    let token=format!("\"{key}\""); let pos=object.find(&token).unwrap()+token.len(); let rest=object[pos..].trim_start(); let rest=rest.strip_prefix(':').unwrap().trim_start();
    let count=rest.bytes().take_while(|v|v.is_ascii_digit()||*v==b'-').count(); assert!(count>0); BigInt::parse_bytes(rest[..count].as_bytes(),10).unwrap()
}
fn next(s: &mut u64) -> u64 { *s^=*s<<13; *s^=*s>>7; *s^=*s<<17; *s }
fn random_raw(s:&mut u64,p:&BigInt)->BigInt { let x=(0..4).fold(BigInt::from(0),|v,_|(v<<64usize)+BigInt::from(next(s))); x%p }
fn boundary(p: &BigInt) -> Vec<BigInt> {
    let mut out=vec![BigInt::from(0),BigInt::from(1),BigInt::from(2),p-1u32,p-2u32,p/2u32];
    for bits in [52usize,104] { let v=BigInt::from(1u32)<<bits; out.extend([&v-1u32,v.clone(),&v+1u32]); }
    for bits in [156usize,208] { let v=BigInt::from(1u32)<<bits; out.extend([&v-1u32,v]); } assert_eq!(out.len(),16); out
}
fn small_constant(target: &BigInt,m:&Poly,n:&Poly,b:&BigInt,p:&BigInt)->Poly {
    let initial=rem(target*b.pow(5),p); let mut out=core::array::from_fn(|i|if i==0 {initial.clone()} else {BigInt::from(0)});
    for _ in 0..5 { out=redc(&out,m,n,b); } out
}
fn power_two(x:&BigInt)->bool { if x<=&BigInt::from(0) {return false;} let y=x-1u32; (x & &y)==BigInt::from(0) }
fn full_word_products(constants:&[Poly])->u64 { constants.iter().flat_map(|a|a.iter()).filter(|x|**x!=BigInt::from(0)&&!power_two(&abs(x))).count() as u64 }
fn import(raw:&BigInt,constants:&[Poly],bits:usize,rounds:usize,m:&Poly,n:&Poly,b:&BigInt)->Poly {
    let base=BigInt::from(1u32)<<bits; let mut remain=raw.clone(); let mut t:Poly=core::array::from_fn(|_|BigInt::from(0));
    for polynomial in constants { let digit=&remain%&base; remain>>=bits; for j in 0..6 {t[j]+=&digit*&polynomial[j];} }
    assert_eq!(remain,BigInt::from(0)); for _ in 0..rounds {t=redc(&t,m,n,b);} t
}
fn export(a:&Poly,hconstants:&Poly,p:&BigInt,h:&BigInt,b_inv:&BigInt,n52:&BigInt,n48:&BigInt,n256:&BigInt,gamma:&BigInt)->BigInt {
    let t=(0..6).map(|i|&a[i]*&hconstants[i]).sum::<BigInt>(); let mut value=t.clone(); let mut quotient=BigInt::from(0); let mut shift=0usize;
    for bits in [52usize,52,52,52,48] {
        let base=BigInt::from(1u32)<<bits; let inv=if bits==48 {n48} else {n52}; let q=rem(&value*inv,&base); quotient+=&q<<shift; shift+=bits;
        let numerator=value+q*p; assert_eq!(&numerator%&base,BigInt::from(0)); value=numerator/base;
    }
    assert_eq!(shift,256); assert_eq!(quotient,rem(&t*n256,h)); assert_eq!(value,(&t+&quotient*p)/h); assert!(value> -p&&value<p*2u32);
    if value.sign()==Sign::Minus {value+=p;} if value>=*p {value-=p;} assert!(value>=BigInt::from(0)&&value<*p);
    let expected=rem(eval(a,gamma,p)*b_inv*h,p); assert_eq!(value,expected); value
}
fn row_sums(m:&Poly)->Poly { let mut result=core::array::from_fn(|_|BigInt::from(0)); for j in 0..6 {let mut unit=core::array::from_fn(|_|BigInt::from(0)); unit[j]=BigInt::from(1); let column=convolution(m,&unit); for i in 0..6 {result[i]+=abs(&column[i]);}} result }

fn main() {
    assert!(!Path::new(&format!("{ROOT}/analysis.json")).exists());
    // Quote/brace and raw-decimal extraction guards for the new input reader.
    let fixture="{\"candidates\":[{\"gamma\":123456789012345678901234567890,\"note\":\"escaped\\\"{]}\"},{\"gamma\":-7}]}";
    let objects=candidate_objects(fixture); assert_eq!(objects.len(),2); assert_eq!(decimal(objects[0],"gamma").to_string(),"123456789012345678901234567890"); assert_eq!(decimal(objects[1],"gamma"),BigInt::from(-7));
    let raw_input=fs::read_to_string(PARAM).unwrap(); let all=candidate_objects(&raw_input);
    let p=BigInt::parse_bytes(PRIME.as_bytes(),10).unwrap(); let b:BigInt=BigInt::from(1u32)<<52usize; let h:BigInt=BigInt::from(1u32)<<256usize;
    let h_inv=inverse(&h,&p); let b_inv=inverse(&b,&p); let n52=rem(-inverse(&p,&b),&b); let b48:BigInt=BigInt::from(1u32)<<48usize; let n48=rem(-inverse(&p,&b48),&b48); let n256=rem(-inverse(&p,&h),&h);
    let mut configurations=vec![]; let mut parameters=vec![]; let mut constants_out=vec![]; let mut candidate=0u64; let mut imports=0u64; let mut extra_exports=0u64; let mut seed=0x7261646978627269u64;
    for object in all {
        let v:Value=serde_json::from_str(object).unwrap(); if v["field"]!="Fq"||v["c"]!=0||v["d"]!=2 {continue;}
        let id=candidate; candidate+=1; assert_eq!(decimal(object,"p"),p); let gamma=decimal(object,"gamma"); assert_eq!(rem(gamma.pow(6),&p),BigInt::from(2));
        let m:Poly=core::array::from_fn(|i|BigInt::from(v["M"][i].as_u64().unwrap())); let n:Poly=core::array::from_fn(|i|BigInt::from(v["N"][i].as_u64().unwrap())); let rho=BigInt::from(v["rho"].as_u64().unwrap());
        assert_eq!(eval(&m,&gamma,&p),BigInt::from(0)); assert_eq!(convolution(&m,&n).map(|x|rem(x,&b)),core::array::from_fn(|i|if i==0 {&b-1u32}else{BigInt::from(0)}));
        let rows=row_sums(&m); let s=rows.iter().max().unwrap(); assert_eq!(rho,s*2u32); assert!(&rho*8u32<b);
        let hc:Poly=core::array::from_fn(|i|rem(gamma.pow(i as u32)*&h*&h*&b_inv,&p));
        let export_nominal_ifma=2*6*5+5*(1+2*5);
        for bits in 43usize..=52 { for rounds in 1usize..=2 {
            let digits=(254+bits-1)/bits; let mut cs=vec![];
            for i in 0..digits {
                let exponent=(i*bits)as i64-256+52*(rounds as i64+1);
                let target=rem((BigInt::from(1u32)<<(i*bits))*&h_inv*b.pow(rounds as u32+1),&p);
                let direct=if exponent>=0 {let c=BigInt::from(1u32)<<exponent as usize;if c<=rho {Some(c)}else{None}}else{None};
                let c=if let Some(value)=direct {core::array::from_fn(|j|if j==0 {value.clone()}else{BigInt::from(0)})}else{small_constant(&target,&m,&n,&b,&p)};
                assert!(c.iter().all(|x|abs(x)<=rho)); assert_eq!(eval(&c,&gamma,&p),target); cs.push(c);
            }
            let mut bounds:Poly=core::array::from_fn(|_|BigInt::from(0));
            for (i,c) in cs.iter().enumerate() { let width=bits.min(254-i*bits); let max=(BigInt::from(1u32)<<width)-1u32; for j in 0..6 {bounds[j]+=&max*abs(&c[j]);} }
            for _ in 0..rounds {bounds=core::array::from_fn(|j|(&bounds[j]+(&b-1u32)*&rows[j])/&b);}
            let certified=bounds.iter().all(|x|x<=&rho); let coefficient_products=full_word_products(&cs); let nominal=2*coefficient_products+54*rounds as u64;
            let mut outside=0u64; let mut max_seen=BigInt::from(0); let mut inputs=boundary(&p); for _ in 0..256 {inputs.push(random_raw(&mut seed,&p));}
            let mut chained=BigInt::from(1); for iteration in 0..336usize {
                let input=if iteration<272 {inputs[iteration].clone()}else{chained.clone()}; let out=import(&input,&cs,bits,rounds,&m,&n,&b);
                assert_eq!(rem(eval(&out,&gamma,&p)*&h,&p),rem(&input*&b,&p)); let largest=out.iter().map(abs).max().unwrap(); if largest>rho {outside+=1;} if largest>max_seen {max_seen=largest;}
                if certified {assert!(out.iter().all(|x|abs(x)<=rho));} assert!(out.iter().all(|x|abs(x)<=&rho*8u32));
                let back=export(&out,&hc,&p,&h,&b_inv,&n52,&n48,&n256,&gamma); assert_eq!(back,input); chained=rem(back+BigInt::from(next(&mut seed)),&p); imports+=1;
            }
            let record=json!({"candidate":id,"digit_bits":bits,"internal_reductions":rounds,"digits":digits,"nonzero_non_power_two_coefficient_products":coefficient_products,"nominal_import_IFMAs":nominal,"normalized_rho_box_certified":certified,"coefficient_upper_bounds":strings(&bounds),"rho":rho.to_string(),"checked_imports_and_exports":336,"observed_import_outputs_outside_rho":outside,"largest_observed_import_coefficient":max_seen.to_string(),"nominal_export_IFMAs":export_nominal_ifma,"G1_compact_nominal_IFMAs":6*90+4*nominal+2*export_nominal_ifma,"G2_compact_nominal_IFMAs":7*90+5*126+12*nominal+4*export_nominal_ifma,"G1_retained_bucket_optimistic_IFMAs":6*90+2*nominal,"G2_retained_bucket_optimistic_IFMAs":7*90+5*126+6*nominal});
            configurations.push(record); constants_out.push(json!({"candidate":id,"digit_bits":bits,"internal_reductions":rounds,"C":cs.iter().map(strings).collect::<Vec<_>>() }));
        }}
        for index in 0..192usize {
            let a:Poly=core::array::from_fn(|i|if index<64 {if index>>i&1==0 {-(&rho*8u32)}else{&rho*8u32}}else{BigInt::from(next(&mut seed))%(&rho*16u32+1u32)-&rho*8u32});
            export(&a,&hc,&p,&h,&b_inv,&n52,&n48,&n256,&gamma); extra_exports+=1;
        }
        parameters.push(json!({"candidate":id,"p":p.to_string(),"gamma":gamma.to_string(),"M":strings(&m),"N":strings(&n),"rho":rho.to_string(),"row_sums":strings(&rows),"export_constants":strings(&hc),"nominal_export_IFMAs":export_nominal_ifma,"extra_signed_exports":192,"constant_payload_bytes_max_import":6*6*8,"export_constant_payload_bytes":6*5*8}));
    }
    assert_eq!(candidate,6); assert_eq!(configurations.len(),120); assert_eq!(imports,40320); assert_eq!(extra_exports,1152);
    let mut recurrence=vec![]; for prior in 0u64..=64 {let x0=1+2*prior;let x1=1+3*prior;let y=1+prior;let dx0=x0+1;let dx1=x1+1;let dy=y+1;let norm=dx0*dx0+dx1*dx1;let numerator=dy*(dx0+dx1);let out0=1+x0+1;let out1=2+x1+1;let offset0=x0+out0;let offset1=x1+out1;let finish=offset0+offset1;assert_eq!(norm,8+20*prior+13*prior*prior);assert_eq!(numerator,8+14*prior+5*prior*prior);assert_eq!(finish,9+10*prior);assert_eq!(out0,1+2*(prior+1));assert_eq!(out1,1+3*(prior+1));recurrence.push(json!({"prior_unrenormalized_updates":prior,"G2_input_x_bounds":[x0,x1],"G2_input_y_bound":y,"G2_norm_raw_coefficient":norm,"G2_numerator_raw_coefficient":numerator,"G2_finish_raw_coefficient":finish,"G2_output_x_bounds":[out0,out1],"G2_output_y_bound":y+1,"G1_finish_raw_coefficient":4+4*prior}));}
    let parent=read("/home/ubuntu/src/zopro-perf-work/pmns-toeplitz6-feasibility176/analysis.json");let mut geometry=vec![];
    for row in parent["surviving_necessary_geometries"].as_array().unwrap() {let w=BigInt::from(row["w"].as_u64().unwrap());let s=BigInt::from(row["ceil_p_sixth_root"].as_u64().unwrap());let predicate=|c:u32|BigInt::from(4*c)*&w*(&b-1u32)*&s<=&b*&b;geometry.push(json!({"field":row["field"],"lambda":row["lambda"],"w":row["w"],"G1_first_update_certificate_not_excluded":predicate(4),"G2_first_update_certificate_not_excluded":predicate(9),"G2_retained_bucket_second_update_certificate_not_excluded":predicate(41)}));}
    let certified:Vec<_>=configurations.iter().filter(|r|r["normalized_rho_box_certified"]==true).collect(); assert!(!certified.is_empty());
    let best=certified.iter().map(|r|r["nominal_import_IFMAs"].as_u64().unwrap()).min().unwrap(); assert!(6*90+2*best>600); assert!(7*90+5*126+4*best>1450);
    save("constants.json",&json!({"parameters":parameters,"import_configurations":constants_out}));
    save("point-bounds.json",&json!({"units":"rho; all product outputs conditionally assumed <=rho, coefficient add/sub unnormalized","valid_curve_trajectories_not_executed":true,"recurrences":recurrence,"necessary_geometry_bounds":geometry}));
    save("analysis.json",&json!({"passed":true,"existing_Fq_binomial_parameters":6,"new_conversion_configurations":120,"checked_imports":imports,"roundtrip_exports":imports,"additional_signed_exports":extra_exports,"all_normalized_raw_R256_inputs":true,"normalized_bound_certified_rows":certified.len(),"best_certified_nominal_import_IFMAs":best,"nominal_export_IFMAs":115,"configurations":configurations,"G1_accepted_group_nominal_IFMAs":600,"G2_accepted_group_nominal_IFMAs":1450,"G1_best_optimistic_retained_bucket_nominal_IFMAs":540+2*best,"G2_best_optimistic_retained_bucket_nominal_IFMAs":1260+6*best,"G2_ideal_X_reuse_best_nominal_IFMAs":1260+4*best,"fixed_family_one_internal_reduction_nominal_lower_bound":54,"G1_distinct_key_import_reduction_floor":108,"G2_distinct_key_import_reduction_floor":216,"G1_ideal_arithmetic_saving":60,"G2_ideal_arithmetic_saving":190,"fixed_bridge_selected_for_native":false,"product_counts_not_native_instructions_cycles_or_latency":true,"new_timings":0,"new_profiles":0,"new_SDK_requests":0,"new_proofs":0,"canonical_proofs":37023,"production_changed":false,"adopted":false,"goal_complete":false}));
    println!("PMNS bridge PASS:6existingparameters/120configurations/40320imports+41472exports;best normalized nominal import{best};point/bridge work screen closed before native");
}
