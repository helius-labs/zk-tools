#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{Field as _, PrimeField as _};
    use crate::{Field as _, Fq};

    fn reference(x: Fq) -> ark_ref::Fq {
        let canonical = Fq::from_montgomery(crate::portable::reduce_once(x.0, FqConfig::P));
        ark_ref::Fq::new(ark_ff::BigInt(canonical.to_canonical()))
    }
    fn check(a: [Fq;8], b: [Fq;8]) {
        let result = Ifma8::new(&a).sum_squares(&Ifma8::new(&b));
        for (i,x) in result.get().into_iter().enumerate() {
            assert_eq!(reference(x), reference(a[i]).square()+reference(b[i]).square());
        }
        let digits: [[u64;8];5] = result.0.map(|v| unsafe { core::mem::transmute(v) });
        let two_p=crate::portable::add4(FqConfig::P,FqConfig::P).0;
        for i in 0..8 {
            for d in digits { assert!(d[i] <= MASK); }
            assert!(digits[4][i]<1<<48);
            let w=[digits[0][i]|digits[1][i]<<52, digits[1][i]>>12|digits[2][i]<<40,
                digits[2][i]>>24|digits[3][i]<<28,digits[3][i]>>36|digits[4][i]<<16];
            assert!(crate::portable::less(w,two_p));
        }
    }
    #[test]
    fn symmetric_norm_lazy_edges_and_full_width_arkworks() {
        let p=FqConfig::P;
        let twice=crate::portable::add4(p,p).0;
        let edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p,
            crate::portable::add4(p,[1,0,0,0]).0,crate::portable::sub4(twice,[1,0,0,0]).0];
        for x in edges { for y in edges {
            check([Fq::from_montgomery(x);8],[Fq::from_montgomery(y);8]);
        }}
        let mut state=0x8ac77ce5d51e129bu64;
        let mut random=|| {let mut w=[0;4];for x in &mut w {state^=state<<13;state^=state>>7;state^=state<<17;*x=state;}w[3]&=(1<<62)-1;Fq::from_montgomery(w)};
        for _ in 0..4096 {check(core::array::from_fn(|_|random()),core::array::from_fn(|_|random()));}
    }
    #[test]
    fn symmetric_norm_dependent_lazy_chains_match_arkworks() {
        let a=core::array::from_fn(|i|Fq::from((i*1337+99)as u64));
        let b=core::array::from_fn(|i|Fq::from((i*197+13)as u64));
        let mut x=Ifma8::new(&a);let mut y=Ifma8::new(&b);
        let mut rx=a.map(reference);let mut ry=b.map(reference);
        for _ in 0..512 {
            let next=x.sum_squares(&y);let rnext=core::array::from_fn(|i|rx[i].square()+ry[i].square());
            for (i,v) in next.get().into_iter().enumerate(){assert_eq!(reference(v),rnext[i]);}
            x=y;y=next;rx=ry;ry=rnext;
        }
    }
}
