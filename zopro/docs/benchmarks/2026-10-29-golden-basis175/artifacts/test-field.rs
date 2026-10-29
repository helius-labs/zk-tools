mod golden_basis_tests {
    use super::*;use ark_ff::{Field,PrimeField};type A=ark_ref::Fq;
    fn check<const MIXED:bool>() {
        type C=crate::FqConfig;
        let p=C::P;let twice=crate::portable::add4(p,p).0;
        let edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p,crate::portable::sub4(twice,[1,0,0,0]).0,twice];
        let numeric=|w:[u64;4]|A::from_le_bytes_mod_order(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());
        let inv=A::from(2u64).pow([if MIXED{256}else{260}]).inverse().unwrap();
        let hostinv=A::from(2u64).pow([256]).inverse().unwrap();let limit=split(twice);
        let mut seed=0x4a54524143454f4eu64;let mut checked=0;
        for group in 0..(1296+4096)/8 {
            let raw:[[[u64;4];8];4]=core::array::from_fn(|slot|core::array::from_fn(|lane| {
                let i=group*8+lane;if i<1296 {edges[(i/6usize.pow(slot as u32))%6]}else{
                    let mut bytes=[0u8;64];for b in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;b.copy_from_slice(&seed.to_le_bytes());}
                    let w=Fp::<C>::from_random_bytes(&bytes).0;if seed&1==0{w}else{crate::portable::add4(w,p).0}
                }
            }));
            let v=raw.map(|r|Ifma8::<C,MIXED>::raw(&r));let scalar=raw.map(|r|r.map(|x|numeric(x)*inv));
            let mut x=[v[0],v[1]];let mut expected=[scalar[0],scalar[1]];
            for _ in 0..8 {
                let square=Ifma8::golden_square(&x[0],&x[1]);
                let norm=x[0].difference2(&x[0].add(&x[1]),&x[1],&x[1]);
                let third=x[0].golden_div3();
                let product=Ifma8::golden_product(&x[0],&x[1],&v[2],&v[3]);
                let all=[product[0],product[1],square[0],square[1],norm,third];
                for (slot,result) in all.into_iter().enumerate() {
                    let host=result.get();let digits:[[u64;8];5]=result.0.map(|v|unsafe{core::mem::transmute(v)});
                    for lane in 0..8 {
                        let a=expected[0][lane];let b=expected[1][lane];let c=scalar[2][lane];let d=scalar[3][lane];
                        let wanted=[a*c+b*d,a*d+b*c+b*d,a*a+b*b,b*(a+a+b),a*a+a*b-b*b,a*A::from(3u64).inverse().unwrap()][slot];
                        assert_eq!(numeric(host[lane].0)*hostinv,wanted,"slot={slot} group={group} lane={lane} mixed={MIXED}");
                        assert!((0..5).all(|j|digits[j][lane]<(1u64<<52)));
                        assert!((0..5).rev().map(|j|digits[j][lane]).cmp((0..5).rev().map(|j|limit[j])).is_lt());checked+=1;
                    }
                }
                for lane in 0..8 {let a=expected[0][lane];let b=expected[1][lane];expected[0][lane]=a*scalar[2][lane]+b*scalar[3][lane];expected[1][lane]=a*scalar[3][lane]+b*scalar[2][lane]+b*scalar[3][lane];}
                x=product;
            }
        } eprintln!("GOLDEN_CHECKED_FIELD_OUTPUTS={checked};MIXED={MIXED}");
    }
    #[test]fn golden_fq_r256_arkworks(){check::<true>();}
    #[test]fn golden_fq_r260_arkworks(){check::<false>();}
}
