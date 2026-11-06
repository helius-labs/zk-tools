fn check<C: FpConfig, A: ark_ff::PrimeField<BigInt=ark_ff::BigInt<4>>, const M:bool>() {
    use ark_ff::{BigInteger,Field};
    let twice=crate::portable::add4(C::P,C::P).0;
    let vals=[[0;4],[1,0,0,0],crate::portable::sub4(C::P,[1,0,0,0]).0,C::P,crate::portable::add4(C::P,[1,0,0,0]).0,crate::portable::sub4(twice,[1,0,0,0]).0,twice];
    let bytes=|w:[u64;4]| w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>();
    let inverse=A::from(2u64).pow([if M{256}else{260}]).inverse().unwrap();
    let host_inverse=A::from(2u64).pow([256]).inverse().unwrap();
    let mut seed=0x391745b2e674ce9du64;
    let cases=2401+4096;
    for group in 0..(cases+7)/8 {
        let input:[[[u64;4];8];4]=core::array::from_fn(|slot|core::array::from_fn(|lane|{
            let i=group*8+lane;
            if i<2401{vals[(i/7usize.pow(slot as u32))%7]}else if i>=cases{[0;4]}else{
                let mut b=[0u8;64];for chunk in b.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}
                let w=A::from_le_bytes_mod_order(&b).into_bigint().0;
                if seed&1!=0{crate::portable::add4(w,C::P).0}else{w}
            }
        }));
        let v=input.map(|w|Ifma8::<C,M>::raw(&w));
        let field=input.map(|w|w.map(|a|A::from_le_bytes_mod_order(&bytes(a))*inverse));
        for op in 0..3 {
            let mut state=v[0];let mut expected=field[0];
            for _ in 0..8 {
                let retained=if op==0{state.retained::<false>(&v[1],&v[2],&v[3])}else if op==1{state.retained::<true>(&v[1],&v[2],&v[3])}else{state.retained_difference2(&v[1],&v[2],&v[3])};
                state=if op==0{state.mul(&v[1])}else if op==1{state.sum2(&v[1],&v[2],&v[3])}else{state.difference2(&v[1],&v[2],&v[3])};
                let got:[[u64;8];5]=state.0.map(|v|unsafe{core::mem::transmute(v)});
                let reference:[[u64;8];5]=retained.0.map(|v|unsafe{core::mem::transmute(v)});
                assert_eq!(got,reference,"exact lazy representative op={op}");
                let host=state.get();
                for i in 0..8 {
                    expected[i]*=field[1][i];
                    if op==1{expected[i]+=field[2][i]*field[3][i];}
                    if op==2{expected[i]-=field[2][i]*field[3][i];}
                    assert_eq!(A::from_le_bytes_mod_order(&bytes(host[i].0))*host_inverse,expected[i]);
                    let w=core::array::from_fn::<_,5,_>(|j|got[j][i]);
                    assert!(w.iter().all(|v|*v<1<<52));assert!(w[4]<1<<48);
                    let raw=[w[0]|w[1]<<52,w[1]>>12|w[2]<<40,w[2]>>24|w[3]<<28,w[3]>>36|w[4]<<16];
                    assert!(crate::portable::less(raw,twice));
                }
            }
        }
    }
    eprintln!("SPLIT_REDC_NATIVE_LANES_PER_CONTEXT={}",((cases+7)/8)*8*8*3);
}
#[test] fn fq_mixed(){check::<crate::FqConfig,ark_ref::Fq,true>();}
#[test] fn fq_uniform(){check::<crate::FqConfig,ark_ref::Fq,false>();}
#[test] fn fr_mixed(){check::<crate::FrConfig,ark_ref::Fr,true>();}
#[test] fn fr_uniform(){check::<crate::FrConfig,ark_ref::Fr,false>();}
