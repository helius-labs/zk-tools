use std::{hint::black_box,time::Instant};
use serde_json::json;
use zopro_bn254::{CurveConfig,Fr,G1Config,G2Config,Projective};

// The accepted binary-radix final fold, copied literally from hb_raw/msm_raw.
#[inline(never)]
fn fold<C:CurveConfig>(sums:&[Projective<C>],bits:usize)->Projective<C>{
    let mut result=Projective::<C>::IDENTITY;
    for sum in sums.iter().rev(){
        for _ in 0..bits {result=result.double();}
        result+=sum;
    }
    result
}
fn word(state:&mut u64)->u64{*state^=*state<<13;*state^=*state>>7;*state^=*state<<17;*state}
fn fixture<C:CurveConfig>(bits:usize,windows:usize)->(Vec<Vec<Projective<C>>>,Vec<Projective<C>>){
    let mut state=0xe71b24d03d98a165u64;let mut bank=vec![];let mut oracle=vec![];
    for slot in 0..8{
        let mut weights=vec![];let mut points=vec![];
        for i in 0..windows{
            let w=if slot==0 {Fr::ZERO} else if slot==1 && i%3==0 {Fr::ZERO} else {
                Fr::from_canonical([word(&mut state),word(&mut state),word(&mut state),word(&mut state)&0x1fff_ffff_ffff_ffff]).unwrap()
            };
            let w=if slot==2&&i%2==1{-w}else{w};weights.push(w);points.push(Projective::<C>::generator()*w);
        }
        // Independent scalar dot product with powers; not another Horner fold.
        let scalar:Fr=weights.iter().enumerate().map(|(i,w)|*w*Fr::from(2u64).pow(&[(i*bits)as u64])).fold(Fr::ZERO,|a,b|a+b);
        let expected=Projective::<C>::generator()*scalar;
        assert_eq!(fold(&points,bits).to_affine(),expected.to_affine());
        bank.push(points);oracle.push(expected);
    }
    (bank,oracle)
}
fn measure<C:CurveConfig>(name:&str,bits:usize,windows:usize)->serde_json::Value{
    let(bank,oracle)=fixture::<C>(bits,windows);
    // Timing uses six ordinary/nonzero banks; zero/mixed banks are checks only.
    for i in 0..18 {black_box(fold(black_box(&bank[2+i%6]),black_box(bits)));}
    let mut samples=vec![];
    for block in 0..5{
        let start=Instant::now();let mut result=Projective::<C>::IDENTITY;
        for i in 0..512 {result=black_box(fold(black_box(&bank[2+(i+block)%6]),black_box(bits)));}
        let ns=start.elapsed().as_nanos()as f64/512.;
        assert_eq!(result.to_affine(),oracle[2+(511+block)%6].to_affine());samples.push(ns);
    }
    let mut sorted=samples.clone();sorted.sort_by(f64::total_cmp);
    json!({"curve":name,"bits":bits,"windows":windows,"called_doublings":bits*windows,"ordinary_input_doublings":bits*(windows-1),"add_calls":windows,"folds_per_block":512,"blocks":5,"nanoseconds_per_fold":samples,"median_ns":sorted[2],"input_banks":8,"timed_banks":6,"generator_scalar_oracle_passed":true,"input_payload_bytes":8*windows*std::mem::size_of::<Projective<C>>()})
}
fn main(){
    let rows=vec![measure::<G1Config>("G1",12,22),measure::<G2Config>("G2",12,11),measure::<G1Config>("G1",13,20),measure::<G2Config>("G2",13,10),measure::<G1Config>("G1",14,19),measure::<G2Config>("G2",14,10)];
    println!("{}",json!({"rows":rows,"new_proofs":0,"new_SDK_requests":0,"scope":"single-thread standalone accepted final-fold service on deterministic synthetic projective window sums; no integrated critical-path or candidate speed result","field_oracle":"same accepted field library; independent scalar-power sum, not new field qualification"}));
}
