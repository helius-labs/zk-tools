#[cfg(test)]
mod tests {
    use num_bigint::{BigInt, BigUint, Sign};
    use zopro_bn254::{FpConfig, FqConfig, FrConfig};
    const B: i128 = 1i128 << 52;
    fn integer(words: [u64; 4]) -> BigUint {
        BigUint::from_bytes_le(
            &words
                .into_iter()
                .flat_map(u64::to_le_bytes)
                .collect::<Vec<_>>(),
        )
    }
    fn cases(p: &BigUint) -> Vec<[BigUint; 4]> {
        let one = BigUint::from(1u8);
        let twice = p << 1;
        let mut edges = vec![
            BigUint::from(0u8),
            one.clone(),
            p.clone(),
            p - &one,
            p + &one,
            &twice - &one,
            twice,
        ];
        for bit in [48, 51, 52, 103, 104, 155, 156, 207, 208, 252, 253] {
            let x: BigUint = &one << bit;
            edges.extend([&x - &one, x.clone(), x + &one]);
        }
        let mut out = Vec::new();
        for (i, a) in edges.iter().enumerate() {
            for (j, b) in edges.iter().enumerate() {
                out.push([
                    a.clone(),
                    b.clone(),
                    edges[(i * 7 + j) % edges.len()].clone(),
                    edges[(i + j * 11) % edges.len()].clone(),
                ]);
            }
        }
        let mut state = 0x706861736564334du64;
        for round in 0..4096 {
            out.push(core::array::from_fn(|i| {
                let mut bytes = [0u8; 64];
                for c in bytes.chunks_mut(8) {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    c.copy_from_slice(&state.to_le_bytes());
                }
                let x = BigUint::from_bytes_le(&bytes) % p;
                if (round + i) % 2 == 0 {
                    x
                } else {
                    x + p
                }
            }));
        }
        out
    }
    fn digits(x: &BigUint) -> [u64; 5] {
        core::array::from_fn(|j| {
            let v = (x >> (52 * j)) & BigUint::from((1u64 << 52) - 1);
            v.to_u64_digits().first().copied().unwrap_or(0)
        })
    }
    fn value(d: [i128; 5]) -> BigInt {
        d.into_iter()
            .enumerate()
            .map(|(j, c)| BigInt::from(c) << (52 * j))
            .sum()
    }
    fn raw_value<const N: usize>(d: &[[u64; N]; 5], i: usize) -> BigUint {
        d.iter()
            .enumerate()
            .map(|(j, c)| BigUint::from(c[i]) << (52 * j))
            .sum()
    }
    fn exact_redc(mut t: BigInt, p: &BigInt, inv: u64, mixed: bool, bias: bool) -> BigInt {
        for i in 0..5 {
            let radix = BigInt::from(1u8) << if mixed && i == 4 { 48 } else { 52 };
            let digit = ((&t % &radix) + &radix) % &radix;
            let m = (digit * BigInt::from(inv)) % &radix;
            t = (t + m * p) / radix;
        }
        if bias {
            t += p;
        }
        let twice = p << 1;
        if t >= twice {
            t -= twice;
        }
        t
    }
    fn track(v: i128, max: &mut i128) -> i128 {
        *max = (*max).max(v.abs());
        assert!(v.abs() < 32 * B);
        v
    }
    fn model(
        inputs: [[u64; 5]; 4],
        p: [u64; 5],
        inv: u64,
        mixed: bool,
        max: &mut i128,
    ) -> [BigInt; 2] {
        let mut a = [0i128; 5];
        let mut ca = 0i128;
        for j in 0..5 {
            let v = inputs[0][j] as i128 + inputs[1][j] as i128 + ca;
            assert!((0..2 * B).contains(&v));
            a[j] = v & (B - 1);
            ca = v >> 52;
            assert!(ca <= 1);
        }
        assert_eq!(ca, 0);
        assert_eq!(
            value(a),
            value(inputs[0].map(i128::from)) + value(inputs[1].map(i128::from))
        );
        let mut x = [0i128; 6];
        let mut y = [0i128; 6];
        let mut cb = 0i128;
        let mut b = [0i128; 5];
        for i in 0..5 {
            let bv = inputs[2][i] as i128 + inputs[3][i] as i128 + cb;
            assert!((0..2 * B).contains(&bv));
            let bi = bv & (B - 1);
            b[i] = bi;
            cb = bv >> 52;
            assert!(cb <= 1);
            for j in 0..5 {
                let pa = inputs[0][j] as i128 * inputs[2][i] as i128;
                let pb = inputs[1][j] as i128 * inputs[3][i] as i128;
                let pc = a[j] * bi;
                let al = pa & (B - 1);
                let ah = pa >> 52;
                let bl = pb & (B - 1);
                let bh = pb >> 52;
                x[j] = track(x[j] + al - bl, max);
                x[j + 1] = track(x[j + 1] + ah - bh, max);
                y[j] = track(y[j] + (pc & (B - 1)) - al - bl, max);
                y[j + 1] = track(y[j + 1] + (pc >> 52) - ah - bh, max);
            }
            for t in [&mut x, &mut y] {
                let bits = if mixed && i == 4 { 48 } else { 52 };
                let mask = (1i128 << bits) - 1;
                let m = ((t[0] & (B - 1)) * inv as i128) & mask;
                let low = track(((t[0] + mask) >> bits) + ((m * p[0] as i128) >> bits), max);
                for j in 1..5 {
                    let product = m * p[j] as i128;
                    t[j] = track(t[j] + (product & (B - 1)), max);
                    t[j + 1] = track(t[j + 1] + (product >> 52), max);
                }
                if mixed && i == 4 {
                    let mask48 = (1i128 << 48) - 1;
                    *t = [
                        low + ((t[1] & mask48) << 4),
                        (t[1] >> 48) + ((t[2] & mask48) << 4),
                        (t[2] >> 48) + ((t[3] & mask48) << 4),
                        (t[3] >> 48) + ((t[4] & mask48) << 4),
                        (t[4] >> 48) + (t[5] << 4),
                        0,
                    ];
                    for v in t.iter() {
                        assert!(v.abs() < 1i128 << 62);
                    }
                } else {
                    let first = track(t[1] + low, max);
                    *t = [first, t[2], t[3], t[4], t[5], 0];
                }
            }
        }
        assert_eq!(cb, 0);
        assert_eq!(
            value(b),
            value(inputs[2].map(i128::from)) + value(inputs[3].map(i128::from))
        );
        core::array::from_fn(|component| {
            let mut out = if component == 0 { x } else { y };
            for j in 0..5 {
                if component == 0 {
                    out[j] += p[j] as i128;
                }
                assert!(out[j].abs() < 1i128 << 62);
            }
            for j in 0..4 {
                out[j + 1] += out[j] >> 52;
                out[j] &= B - 1;
            }
            let mut result = value(core::array::from_fn(|j| out[j]));
            let twice = value(p.map(i128::from)) << 1;
            if result >= twice {
                result -= twice;
            }
            result
        })
    }
    macro_rules! lane_tests {
        ($name:ident,$vector:path,$n:literal) => {
            mod $name {
                use super::*;
                use $vector as Vector;
                const N:usize=$n;
                #[repr(align(64))]
                struct Aligned([[u64;N];5]);
                fn pack<C:FpConfig,const MIXED:bool>(v:&[BigUint;N])->Vector<C,MIXED>{
                    let d=v.each_ref().map(|v|digits(v));let data=Aligned(core::array::from_fn(|j|core::array::from_fn(|i|d[i][j])));
                    assert_eq!(core::mem::size_of::<Vector<C,MIXED>>(),core::mem::size_of_val(&data.0));
                    // Test-only construction of the private normalized-limb representation.
                    unsafe{core::mem::transmute_copy(&data)}
                }
                fn raw<C:FpConfig,const MIXED:bool>(v:&Vector<C,MIXED>)->[[u64;N];5]{unsafe{core::mem::transmute_copy(v)}}
                fn integer_check<C:FpConfig,const MIXED:bool>(){
                    let p=integer(C::P);let pi=BigInt::from_biguint(Sign::Plus,p.clone());let twice=&p<<1;let mut count=0;let mut max=0i128;
                    for chunk in cases(&p).chunks(N){
                        let values:[[BigUint;N];4]=core::array::from_fn(|j|core::array::from_fn(|i|chunk[i%chunk.len()][j].clone()));let v=values.each_ref().map(pack::<C,MIXED>);
                        let got=Vector::complex_product_streamed(&v[0],&v[1],&v[2],&v[3]);let gd=got.each_ref().map(raw);let previous=Vector::complex_product_shared(&v[0],&v[1],&v[2],&v[3]);assert_eq!(gd,previous.each_ref().map(raw));
                        for i in 0..N {
                            let [a0,a1,b0,b1]=values.each_ref().map(|v|v[i].clone());let real=BigInt::from(&a0*&b0)-BigInt::from(&a1*&b1);let imag=&a0*&b1+&a1*&b0;
                            let inputs=[digits(&a0),digits(&a1),digits(&b0),digits(&b1)];let modeled=model(inputs,digits(&p),C::INV,MIXED,&mut max);
                            for (c,t,bias) in [(0,real,true),(1,BigInt::from(imag),false)] {
                                let exact=exact_redc(t,&pi,C::INV,MIXED,bias);assert_eq!(modeled[c],exact);
                                let actual=raw_value(&gd[c],i);assert!(actual<twice);assert_eq!(BigInt::from(actual),exact);for digit in gd[c]{assert!(digit[i]<1u64<<52);}count+=1;
                            }
                        }
                    }
                    println!("EXACT_STREAMED lanes={N} mixed={MIXED} outputs={count} max_coefficient={max} bound={}",32*B);
                }
                fn chain_check<C:FpConfig,A:ark_ff::PrimeField,const MIXED:bool>(){
                    use ark_ff::Field;
                    let p=integer(C::P);let twice=&p<<1;let inv=A::from(2u64).pow([if MIXED{256}else{260}]).inverse().unwrap();let mut count=0;
                    for chunk in cases(&p).chunks(N){let values:[[BigUint;N];4]=core::array::from_fn(|j|core::array::from_fn(|i|chunk[i%chunk.len()][j].clone()));let v=values.each_ref().map(pack::<C,MIXED>);let algebra=values.each_ref().map(|v|v.each_ref().map(|x|A::from_le_bytes_mod_order(&x.to_bytes_le())*inv));
                        let mut got=[v[0],v[1]];let mut expected=[algebra[0],algebra[1]];
                        for _ in 0..8{
                            got=Vector::complex_product_streamed(&got[0],&got[1],&v[2],&v[3]);for i in 0..N{let [ar,ai]=[expected[0][i],expected[1][i]];expected[0][i]=ar*algebra[2][i]-ai*algebra[3][i];expected[1][i]=ar*algebra[3][i]+ai*algebra[2][i];}
                            for c in 0..2{let gd=raw(&got[c]);let host=got[c].get();for i in 0..N{let bytes=host[i].to_canonical().into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>();assert_eq!(A::from_le_bytes_mod_order(&bytes),expected[c][i]);assert!(raw_value(&gd,i)<twice);for digit in gd{assert!(digit[i]<1u64<<52);}count+=1;}}
                        }
                    }
                    println!("ARK_STREAMED lanes={N} mixed={MIXED} chain_outputs={count}");
                }
                #[test]fn exact_signed_products_and_coefficients(){integer_check::<FqConfig,false>();integer_check::<FqConfig,true>();integer_check::<FrConfig,false>();integer_check::<FrConfig,true>();}
                #[test]fn arkworks_lazy_boundary_and_chains(){chain_check::<FqConfig,ark_ref::Fq,false>();chain_check::<FqConfig,ark_ref::Fq,true>();chain_check::<FrConfig,ark_ref::Fr,false>();chain_check::<FrConfig,ark_ref::Fr,true>();}
            }
        }
    }
    lane_tests!(four, zopro_bn254::ifma::Ifma, 4);
    lane_tests!(eight, zopro_bn254::ifma8::Ifma8, 8);
}
