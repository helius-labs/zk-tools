    pub fn encode(y: Fq) -> Self {
        // p=3 mod4, y=4a+r: y/4 modp = a+r*(p+1)/4 < p.
        // Two fixed masks select T and2T; no table or conditional halving.
        const T: [u64;4] = {
            assert!(FqConfig::P[0]&3==3);
            let p=crate::portable::add4(FqConfig::P,[1,0,0,0]).0;
            [p[0]>>2|p[1]<<62,p[1]>>2|p[2]<<62,p[2]>>2|p[3]<<62,p[3]>>2]
        };
        const TWO_T: [u64;4] = crate::portable::add4(T,T).0;
        let w=y.0;
        let r=w[0]&3;
        let lo=0u64.wrapping_sub(r&1);
        let hi=0u64.wrapping_sub(r>>1);
        let mut carry=0u128;
        let out=core::array::from_fn(|i| {
            let shifted=w[i]>>2 | if i<3 {w[i+1]<<62} else {0};
            let sum=shifted as u128+(T[i]&lo)as u128+(TWO_T[i]&hi)as u128+carry;
            carry=sum>>64;
            sum as u64
        });
        Self(out)
    }
