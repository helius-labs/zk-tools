//! Exact host interpreter of a new local-tile schedule; no SIMD or clocks.
use num_bigint::{BigInt, BigUint};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/blocked-pair52-model176";
const B: i128 = 1 << 52;
const M: u64 = (1 << 52) - 1;
const PRIMES: [&str; 2] = [
    "21888242871839275222246405745257275088696311157297823662689037894645226208583",
    "21888242871839275222246405745257275088548364400416034343698204186575808495617",
];
type Digits = [u64; 5];
fn u(n: u64) -> BigUint { BigUint::from(n) }
fn big(n: &BigUint) -> BigInt { BigInt::from(n.clone()) }
fn val(t: &[i128]) -> BigInt { t.iter().rev().fold(BigInt::from(0), |v, x| (v << 52usize) + BigInt::from(*x)) }
fn words(n: &BigUint) -> Digits {
    let mut v = n.clone();
    std::array::from_fn(|_| { let x = (&v & u(M)).to_u64_digits().first().copied().unwrap_or(0); v >>= 52usize; x })
}
fn pos(n: BigInt, p: &BigInt) -> BigInt { ((n % p) + p) % p }
fn inverse(n: &BigUint, p: &BigUint) -> BigUint {
    let (mut r, mut s) = (big(p), big(n));
    let (mut x, mut y) = (BigInt::from(0), BigInt::from(1));
    while s != BigInt::from(0) { let q = &r / &s; (r, s) = (s.clone(), r - &q * s); (x, y) = (y.clone(), x - q * y); }
    assert_eq!(r, BigInt::from(1)); pos(x, &big(p)).to_biguint().unwrap()
}
fn ifma(a: u64, b: u64) -> (i128, i128) { assert!(a <= M && b <= M); let p = a as i128 * b as i128; (p & (B - 1), p >> 52) }
fn signed_pair(x: i128, y: i128, radix: i128) -> (i128, i128) {
    assert!((-radix..radix).contains(&x) && (-radix..radix).contains(&y));
    let a = x.abs(); let oriented = if x < 0 { -y } else { y };
    let p = a * oriented.rem_euclid(radix);
    (p % radix, p / radix - if oriented < 0 { a } else { 0 })
}
#[derive(Default)] struct Counts { products: u64, rows: u64, raw: u64, reduction: u64, tiles: u64, neg: u64, endpoints: u64, max: i128, mixed_max: i128, digest: Sha256 }
fn observe(n: i128, c: &mut Counts) { assert!(n.abs() < 64 * B); assert!(i64::try_from(n).is_ok()); c.max = c.max.max(n.abs()); }
fn add(t: &mut [i128; 7], k: usize, n: i128, c: &mut Counts) { t[k] += n; observe(t[k], c); }
fn raw(t: &mut [i128; 7], k: usize, a: u64, b: u64, c: &mut Counts) {
    let (l, h) = ifma(a,b); add(t,k,l,c); add(t,k+1,h,c); c.raw += 2;
}
fn tile(t: &mut [i128; 7], start: usize, a: &Digits, b: &Digits, c: &mut Counts) {
    let i = start; let j = start + 1;
    let (l0,h0) = ifma(a[i],b[i]); let (l1,h1) = ifma(a[j],b[j]);
    let dx = a[i] as i128 - a[j] as i128;
    let dy = b[j] as i128 - b[i] as i128;
    let (lp,hp) = signed_pair(dx,dy,B);
    assert_eq!(lp + B*hp, dx*dy);
    c.neg += u64::from(dx*dy < 0);
    let cross_l = l0 + l1 + lp; let cross_h = h0 + h1 + hp;
    observe(cross_l,c); observe(cross_h,c);
    assert_eq!(cross_l + B*cross_h, a[i] as i128*b[j] as i128 + a[j] as i128*b[i] as i128);
    add(t,start,l0,c); add(t,start+1,h0,c);
    add(t,start+1,cross_l,c); add(t,start+2,cross_h,c);
    add(t,start+2,l1,c); add(t,start+3,h1,c);
    c.raw += 6; c.tiles += 1;
}
// Bound propagation is deliberately independent of field values and signs.
#[derive(Clone, Copy, Debug)] struct Range { lo:i128, hi:i128 }
impl Range { fn plus(self, other:Self)->Self { Self{lo:self.lo+other.lo,hi:self.hi+other.hi} } }
fn bound_screen(fused: bool, mixed: bool)->Value {
    let mut t=[Range{lo:0,hi:0};7]; let mut maximum=0i128; let mut trace=Vec::new();
    let digit=Range{lo:0,hi:B-1}; let signed=Range{lo:1-B,hi:B-1};
    let mut put=|t:&mut [Range;7],k:usize,x:Range| { t[k]=t[k].plus(x); maximum=maximum.max(t[k].lo.abs()).max(t[k].hi.abs()); };
    let times=if fused{2}else{1};
    for start in [0usize,2] {
        for _ in 0..times {
            put(&mut t,start,digit); put(&mut t,start+1,digit);
            put(&mut t,start+1,digit.plus(digit).plus(digit));
            put(&mut t,start+2,digit.plus(digit).plus(signed));
            put(&mut t,start+2,digit); put(&mut t,start+3,digit);
            for row in start..start+2 { for a in 0..5 { if a==start||a==start+1 {continue;} let k=a+row-start; put(&mut t,k,digit); put(&mut t,k+1,digit); } }
        }
        for row in start..start+2 {
            let ceil=Range{lo:(t[0].lo+B-1).div_euclid(B),hi:(t[0].hi+B-1).div_euclid(B)};
            for j in 1..5 { put(&mut t,j,digit); put(&mut t,j+1,digit); }
            put(&mut t,1,ceil.plus(digit));
            t=[t[1],t[2],t[3],t[4],t[5],t[6],Range{lo:0,hi:0}];
            trace.push(json!({"row":row,"coefficient_ranges":t.map(|v|[v.lo.to_string(),v.hi.to_string()])}));
        }
    }
    for _ in 0..times { for a in 0..5 {put(&mut t,a,digit);put(&mut t,a+1,digit);} }
    let bits=if mixed{48}else{52}; let radix=1i128<<bits;
    let ceil=Range{lo:(t[0].lo+radix-1).div_euclid(radix),hi:(t[0].hi+radix-1).div_euclid(radix)};
    for j in 1..5{put(&mut t,j,digit);put(&mut t,j+1,digit);}
    // Uniform high(m*p0/B) <B. Mixed high((16m)*p0/B) <B too.
    let carry=ceil.plus(digit);
    let mut result=[t[1],t[2],t[3],t[4],t[5]];
    if mixed { for v in &mut result{v.lo*=16;v.hi*=16;} }
    result[0]=result[0].plus(carry);
    let mut final_max=maximum;
    for i in 0..4 {
        final_max=final_max.max(result[i].lo.abs()).max(result[i].hi.abs());
        let carry=Range{lo:result[i].lo.div_euclid(B),hi:result[i].hi.div_euclid(B)};
        result[i]=digit; result[i+1]=result[i+1].plus(carry);
    }
    for v in result {final_max=final_max.max(v.lo.abs()).max(v.hi.abs());}
    assert!(maximum<64*B && final_max<1024*B && final_max<i64::MAX as i128);
    json!({"fused":fused,"mixed":mixed,"prepacking_abs_bound_inclusive":maximum.to_string(),"all_intermediate_abs_bound_inclusive":final_max.to_string(),"signed64":true,"rows":trace})
}
fn execute(inputs: &[BigUint;4], p: &BigUint, mixed:bool, op:usize, c:&mut Counts)->BigUint {
    let two=p*u(2); for n in inputs {assert!(n<=&two);c.endpoints+=u64::from(n==&two);}
    let a=words(&inputs[0]);let b=words(&inputs[1]);let cc=if op==2{&two-&inputs[2]}else{inputs[2].clone()};let d=words(&inputs[3]);let cc=words(&cc);
    let products=if op==0{vec![(a,b)]}else{vec![(a,b),(cc,d)]};
    let pd=words(p); let inv=(u(1)<<52usize)-inverse(&(p% (u(1)<<52usize)),&(u(1)<<52usize));let inv=inv.to_u64_digits()[0];
    let mut t=[0i128;7]; let mut reference=BigInt::from(0);
    let mut qs=Vec::new();
    for start in [0usize,2] {
        for (a,b) in &products {
            tile(&mut t,start,a,b,c);
            for row in start..start+2 {for j in 0..5 {if j==start||j==start+1 {continue;}raw(&mut t,j+row-start,a[j],b[row],c);}}
        }
        // Independent whole integers insert both rows, without tile or IFMA decomposition.
        for (a,b) in &products { let av=val(&a.map(|x|x as i128)); reference += av*BigInt::from(b[start] as u128 + ((b[start+1] as u128)<<52)); }
        assert_eq!(val(&t),reference);
        for row in start..start+2 {
            let q=((t[0] as u64 as u128*inv as u128)&M as u128)as u64;
            let low=pos(reference.clone(),&BigInt::from(B)).to_biguint().unwrap().to_u64_digits().first().copied().unwrap_or(0);
            assert_eq!(q,((low as u128*inv as u128)&M as u128)as u64);
            let ceil=(t[0]+B-1).div_euclid(B);let (_,h0)=ifma(q,pd[0]); let carry=ceil+h0;observe(carry,c);
            for j in 1..5{let(l,h)=ifma(q,pd[j]);add(&mut t,j,l,c);add(&mut t,j+1,h,c);}
            add(&mut t,1,carry,c);
            t=[t[1],t[2],t[3],t[4],t[5],t[6],0];
            let sum=reference+BigInt::from(q)*big(p); assert_eq!(&sum%BigInt::from(B),BigInt::from(0));reference=sum>>52usize;
            assert_eq!(val(&t),reference,"row {row}");c.rows+=1;c.reduction+=10;qs.push(q);
        }
    }
    for (a,b) in &products {for j in 0..5{raw(&mut t,j,a[j],b[4],c);}reference+=val(&a.map(|x|x as i128))*BigInt::from(b[4]);}
    assert_eq!(val(&t),reference);
    let bits=if mixed{48usize}else{52};let radix=1i128<<bits;
    let q=((t[0] as u64 as u128*inv as u128)&((radix-1)as u128))as u64;
    let low=pos(reference.clone(),&BigInt::from(radix)).to_biguint().unwrap().to_u64_digits().first().copied().unwrap_or(0);
    assert_eq!(q,((low as u128*inv as u128)&((radix-1)as u128))as u64);
    let ceil=(t[0]+radix-1).div_euclid(radix);let (_,h0)=ifma(if mixed{q<<4}else{q},pd[0]);let carry=ceil+h0;
    for j in 1..5{let(l,h)=ifma(q,pd[j]);add(&mut t,j,l,c);add(&mut t,j+1,h,c);}
    let mut out=std::array::from_fn::<_,5,_>(|i|t[i+1]);if mixed{for x in &mut out{*x*=16;assert!(x.abs()<1024*B);c.mixed_max=c.mixed_max.max(x.abs());}}
    out[0]+=carry; for j in 0..4{out[j+1]+=out[j].div_euclid(B);out[j]=out[j].rem_euclid(B);}
    assert!(out.iter().all(|x|*x>=0&&*x<B));
    let sum=reference+BigInt::from(q)*big(p);assert_eq!(&sum%BigInt::from(radix),BigInt::from(0));reference=sum>>bits;
    assert_eq!(val(&out),reference);c.rows+=1;c.reduction+=10;qs.push(q);
    let raw=reference.to_biguint().unwrap();
    let integer=&inputs[0]*&inputs[1]+if op==0{u(0)}else{(&products[1].0.iter().rev().fold(u(0),|x,d|(x<<52usize)+u(*d)))*&inputs[3]};
    let r=u(1)<<if mixed{256usize}else{260};let pinv=inverse(p,&r);let m=((&r-(&integer*&pinv)%&r)%&r);
    assert_eq!(raw,(&integer+&m*p)/&r);
    let mut numerical_q=u(0);for(i,q)in qs.iter().enumerate(){numerical_q+=u(*q)<<(52*i);}assert_eq!(numerical_q,m);
    let mut out=raw;if op!=0&&mixed&&out>=two{out-=&two;}assert!(out<two);
    let ordinary=(&inputs[0]*&inputs[1])%p;
    let ordinary=match op{0=>ordinary,1=>(ordinary+&inputs[2]*&inputs[3])%p,_=>(ordinary+p-(&inputs[2]*&inputs[3])%p)%p};
    assert_eq!(&out%p,(ordinary*inverse(&(&r%p),p))%p);
    c.products+=1;let bytes=out.to_bytes_le();c.digest.update((bytes.len()as u64).to_le_bytes());c.digest.update(bytes);out
}
fn main(){
    assert!(!Path::new(&format!("{ROOT}/result.json")).exists());
    let mut small=0u64;for bits in 1..=4 {let r=1i128<<bits;for a in 0..r{for b in 0..r{for c in 0..r{for d in 0..r{let(l,h)=signed_pair(a-b,d-c,r);assert_eq!(a*c+b*d+l+r*h,a*d+b*c);small+=1;}}}}}
    let bounds:Vec<_>=[false,true].into_iter().flat_map(|f|[false,true].into_iter().map(move|m|bound_screen(f,m))).collect();
    let mut panels=Vec::new();
    for (tag,decimal) in ["Fq","Fr"].into_iter().zip(PRIMES){let p=BigUint::parse_bytes(decimal.as_bytes(),10).unwrap();let two=&p*u(2);let range=&two+u(1);
        let mut edges=BTreeSet::from([u(0),u(1),u(2),&p-u(1),p.clone(),&p+u(1),&two-u(1),two.clone()]);
        for bit in[26usize,48,51,52,53,104,105,156,157,208,209,253,254]{let a=u(1)<<bit;for n in[&a-u(1),a.clone(),&a+u(1)]{if n<=two{edges.insert(n);}}}
        let edges:Vec<_>=edges.into_iter().collect();
        for mixed in[true,false]{let mut c=Counts::default();let mut contexts=[0u64;3];
            for i in 0..edges.len(){for j in 0..edges.len(){let xs=[edges[i].clone(),edges[j].clone(),edges[(i+11)%edges.len()].clone(),edges[(j+19)%edges.len()].clone()];for op in 0..3{execute(&xs,&p,mixed,op,&mut c);contexts[op]+=1;}}}
            for case in 0u64..4096{let xs=std::array::from_fn(|slot|{let mut h=Sha256::new();h.update(b"blocked-pair52-model176/full-width");h.update(tag.as_bytes());h.update(case.to_le_bytes());h.update((slot as u64).to_le_bytes());BigUint::from_bytes_le(&h.finalize())%&range});for op in 0..3{execute(&xs,&p,mixed,op,&mut c);contexts[op]+=1;}}
            for op in 0..3{let mut xs=[&two-u(1),p.clone(),u(1),u(2)];for step in 0..1024{let next=execute(&xs,&p,mixed,op,&mut c);xs=[xs[1].clone(),xs[2].clone(),xs[3].clone(),next];contexts[op]+=1;assert_eq!(c.rows,c.products*5);assert!(step<1024);}}
            let expected_raw=contexts[0]*46+(contexts[1]+contexts[2])*92;assert_eq!(c.raw,expected_raw);assert_eq!(c.reduction,c.products*50);assert_eq!(c.tiles,contexts[0]*2+(contexts[1]+contexts[2])*4);
            panels.push(json!({"field":tag,"radix_bits":if mixed{256}else{260},"boundary_values":edges.len(),"boundary_tuples":edges.len()*edges.len(),"random_tuples":4096,"chain_steps_per_operation":1024,"operations":contexts,"products":c.products,"rows":c.rows,"raw_IFMA_proxy":c.raw,"reduction_IFMA_proxy":c.reduction,"tiles":c.tiles,"negative_pair_products":c.neg,"inclusive_two_p_input_occurrences":c.endpoints,"max_observed_prepacking_abs":c.max.to_string(),"max_observed_mixed_shift_abs":c.mixed_max.to_string(),"output_digest":format!("{:x}",c.digest.finalize())}));
        }
    }
    let result=json!({"status":"pass","exhaustive_small_radix_tiles":small,"panels":panels,"intervals":bounds,"array_coefficients":7,"baseline_CIOS_coefficients":6,"prior_full_pair_coefficients":10,"ordinary_IFMA_proxy":96,"fused_IFMA_proxy":142,"original_IFMA_proxy":[100,150],"prior_full_pair_IFMA_proxy":[80,110],"native_executed":false,"timings":0,"proofs":0,"production_changed":false,"goal_complete":false});
    fs::write(format!("{ROOT}/result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("Exact blocked-tile model PASS; {} small-radix tiles; no native or timings",small);
}
