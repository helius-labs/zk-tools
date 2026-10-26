use std::{hint::black_box, time::Instant};
use zopro_bn254::{ifma8::Ifma8, Fq, FqConfig};

type V = Ifma8<FqConfig, true>;
type Complex = [V; 2];

#[inline(always)]
fn ordinary(a: &Complex, b: &Complex) -> Complex {
    [
        a[0].difference2(&b[0], &a[1], &b[1]),
        a[0].sum2(&b[1], &a[1], &b[0]),
    ]
}

#[inline(always)]
fn multiply<const STRATEGY: u8>(a: &Complex, b: &Complex) -> Complex {
    #[cfg(zopro_phased_complex_products)]
    {
        if STRATEGY == 1 {
            return V::complex_product_shared(&a[0], &a[1], &b[0], &b[1]);
        }
        if STRATEGY == 2 {
            return V::complex_product_phased(&a[0], &a[1], &b[0], &b[1]);
        }
    }
    ordinary(a, b)
}

fn vector(seed: u64) -> V {
    let mut state = black_box(seed);
    V::new(&core::array::from_fn(|_| {
        let mut bytes = [0u8; 64];
        for chunk in bytes.chunks_mut(8) {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            chunk.copy_from_slice(&state.to_le_bytes());
        }
        Fq::from_random_bytes(&bytes)
    }))
}

#[no_mangle]
#[inline(never)]
pub fn complex_old_codegen(a: &Complex, b: &Complex) -> Complex {
    ordinary(a, b)
}

#[cfg(zopro_phased_complex_products)]
#[no_mangle]
#[inline(never)]
pub fn complex_shared_codegen(a: &Complex, b: &Complex) -> Complex {
    V::complex_product_shared(&a[0], &a[1], &b[0], &b[1])
}

#[cfg(zopro_phased_complex_products)]
#[no_mangle]
#[inline(never)]
pub fn complex_phased_codegen(a: &Complex, b: &Complex) -> Complex {
    V::complex_product_phased(&a[0], &a[1], &b[0], &b[1])
}
#[inline(never)]
fn run<const STRATEGY: u8>(pattern: &str, iterations: usize) -> (u128, Vec<V>) {
    let b = [vector(701), vector(907)];
    let factor = vector(211);
    let ax = [vector(307), vector(401)];
    let ay = [vector(503), vector(601)];
    let mut z = [vector(101), vector(103)];
    let mut streams = [
        [vector(1001), vector(1003)],
        [vector(1101), vector(1103)],
        [vector(1201), vector(1203)],
        [vector(1301), vector(1303)],
    ];
    let started = Instant::now();
    match pattern {
        "chain" => {
            for _ in 0..iterations {
                z = multiply::<STRATEGY>(&z, &b);
            }
        }
        "streams4" => {
            for _ in 0..iterations {
                for stream in &mut streams {
                    *stream = multiply::<STRATEGY>(stream, &b);
                }
            }
        }
        "scaled" => {
            for _ in 0..iterations {
                z = multiply::<STRATEGY>(&z, &b);
                z = [z[0].mul(&factor), z[1].mul(&factor)];
            }
        }
        "affine-expression" => {
            for _ in 0..iterations {
                let mut slope = multiply::<STRATEGY>(&z, &b);
                slope = [slope[0].mul(&factor), slope[1].mul(&factor)];
                let cross = slope[0].mul(&slope[1]);
                let square = [
                    slope[0].add(&slope[1]).mul(&slope[0].sub(&slope[1])),
                    cross.add(&cross),
                ];
                let dx = [ax[0].sub(&square[0]), ax[1].sub(&square[1])];
                z = multiply::<STRATEGY>(&slope, &dx);
                z = [z[0].sub(&ay[0]), z[1].sub(&ay[1])];
            }
        }
        _ => unreachable!(),
    }
    black_box((&z, &streams));
    let elapsed = started.elapsed().as_nanos();
    let result = if pattern == "streams4" {
        streams.into_iter().flatten().collect()
    } else {
        z.to_vec()
    };
    (elapsed, black_box(result))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "codegen" {
        let a = [vector(101), vector(103)];
        let b = [vector(701), vector(907)];
        black_box(complex_old_codegen(&a, &b));
        #[cfg(zopro_phased_complex_products)]
        {
            black_box(complex_shared_codegen(&a, &b));
            black_box(complex_phased_codegen(&a, &b));
        }
        return Ok(());
    }
    if args.len() != 4 {
        return Err("usage: component old|shared|phased PATTERN ITERATIONS".into());
    }
    let strategy: u8 = match args[1].as_str() {
        "old" => 0,
        "shared" if cfg!(zopro_phased_complex_products) => 1,
        "phased" if cfg!(zopro_phased_complex_products) => 2,
        _ => return Err("unsupported mode".into()),
    };
    let pattern = args[2].as_str();
    if !["chain", "streams4", "scaled", "affine-expression"].contains(&pattern) {
        return Err("unsupported pattern".into());
    }
    let iterations: usize = args[3].parse()?;
    if iterations == 0 {
        return Err("iterations must be positive".into());
    }
    for _ in 0..3 {
        black_box(match strategy {
            0 => run::<0>(pattern, 2000),
            1 => run::<1>(pattern, 2000),
            2 => run::<2>(pattern, 2000),
            _ => unreachable!(),
        });
    }
    let (elapsed_ns, result) = match strategy {
        0 => run::<0>(pattern, iterations),
        1 => run::<1>(pattern, iterations),
        2 => run::<2>(pattern, iterations),
        _ => unreachable!(),
    };
    let words: Vec<u64> = result
        .into_iter()
        .flat_map(|v| v.get())
        .flat_map(|v| v.to_canonical())
        .collect();
    let streams = if pattern == "streams4" { 4. } else { 1. };
    println!("ROW {{\"pattern\":\"{pattern}\",\"strategy\":{strategy},\"iterations\":{iterations},\"primes\":3,\"elapsed_ns\":{elapsed_ns},\"ns_per_lane\":{},\"output_words\":{words:?}}}", elapsed_ns as f64 / (iterations as f64 * 8. * streams));
    Ok(())
}
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
            let x = &one << bit;
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
        t: [i128; 10],
        p: [u64; 5],
        inv: u64,
        mixed: bool,
        bias: bool,
        max: &mut i128,
    ) -> BigInt {
        let mut t = t;
        for i in 0..5 {
            let bits = if mixed && i == 4 { 48 } else { 52 };
            let mask = (1i128 << bits) - 1;
            let m = ((t[i] & (B - 1)) * inv as i128) & mask;
            for j in 0..5 {
                let product = m * p[j] as i128;
                t[i + j] = track(t[i + j] + (product & (B - 1)), max);
                t[i + j + 1] = track(t[i + j + 1] + (product >> 52), max);
            }
            if bits == 52 {
                t[i + 1] = track(t[i + 1] + (t[i] >> 52), max);
            }
        }
        let mut out: [i128; 5] = if mixed {
            for j in 4..9 {
                t[j + 1] = track(t[j + 1] + (t[j] >> 52), max);
                t[j] &= B - 1;
            }
            core::array::from_fn(|j| {
                if j == 4 {
                    (t[8] >> 48) + (t[9] << 4)
                } else {
                    (t[j + 4] >> 48) | ((t[j + 5] << 4) & (B - 1))
                }
            })
        } else {
            core::array::from_fn(|j| t[j + 5])
        };
        for j in 0..5 {
            if bias {
                out[j] += p[j] as i128;
            }
            assert!(out[j].abs() < 1i128 << 62);
        }
        for j in 0..4 {
            out[j + 1] += out[j] >> 52;
            out[j] &= B - 1;
        }
        let mut result = value(out);
        let modulus = value(p.map(|x| x as i128)) << 1;
        if result >= modulus {
            result -= modulus;
        }
        result
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
                        let got=Vector::complex_product_phased(&v[0],&v[1],&v[2],&v[3]);let gd=got.each_ref().map(raw);
                        for i in 0..N {
                            let [a0,a1,b0,b1]=values.each_ref().map(|v|v[i].clone());let real=BigInt::from(&a0*&b0)-BigInt::from(&a1*&b1);let imag=&a0*&b1+&a1*&b0;
                            let inputs=[digits(&a0),digits(&a1),digits(&b0),digits(&b1)];let mut d=[0i128;10];let mut s=[0i128;10];
                            for j in 0..5{for k in 0..5{let a=inputs[0][j]as i128*inputs[2][k]as i128;let b=inputs[1][j]as i128*inputs[3][k]as i128;
                                d[j+k]=track(d[j+k]+(a&(B-1))-(b&(B-1)),&mut max);d[j+k+1]=track(d[j+k+1]+(a>>52)-(b>>52),&mut max);
                                s[j+k]=track(s[j+k]-(a&(B-1))-(b&(B-1)),&mut max);s[j+k+1]=track(s[j+k+1]-(a>>52)-(b>>52),&mut max);
                            }}
                            let sums=[digits(&(&a0+&a1)),digits(&(&b0+&b1))];for j in 0..5{for k in 0..5{let c=sums[0][j]as i128*sums[1][k]as i128;s[j+k]=track(s[j+k]+(c&(B-1)),&mut max);s[j+k+1]=track(s[j+k+1]+(c>>52),&mut max);}}
                            for (c,t,coeff,bias) in [(0,real,d,true),(1,BigInt::from(imag),s,false)] {
                                let exact=exact_redc(t,&pi,C::INV,MIXED,bias);assert_eq!(model(coeff,digits(&p),C::INV,MIXED,bias,&mut max),exact);
                                let actual=raw_value(&gd[c],i);assert!(actual<twice);assert_eq!(BigInt::from(actual),exact);for digit in gd[c]{assert!(digit[i]<1u64<<52);}count+=1;
                            }
                        }
                    }
                    println!("EXACT_PHASED lanes={N} mixed={MIXED} outputs={count} max_coefficient={max} bound={}",32*B);
                }
                fn chain_check<C:FpConfig,A:ark_ff::PrimeField,const MIXED:bool>(){
                    use ark_ff::Field;
                    let p=integer(C::P);let twice=&p<<1;let inv=A::from(2u64).pow([if MIXED{256}else{260}]).inverse().unwrap();let mut count=0;
                    for chunk in cases(&p).chunks(N){let values:[[BigUint;N];4]=core::array::from_fn(|j|core::array::from_fn(|i|chunk[i%chunk.len()][j].clone()));let v=values.each_ref().map(pack::<C,MIXED>);let algebra=values.each_ref().map(|v|v.each_ref().map(|x|A::from_le_bytes_mod_order(&x.to_bytes_le())*inv));
                        let mut got=[v[0],v[1]];let mut expected=[algebra[0],algebra[1]];
                        for _ in 0..8{
                            got=Vector::complex_product_phased(&got[0],&got[1],&v[2],&v[3]);for i in 0..N{let [ar,ai]=[expected[0][i],expected[1][i]];expected[0][i]=ar*algebra[2][i]-ai*algebra[3][i];expected[1][i]=ar*algebra[3][i]+ai*algebra[2][i];}
                            for c in 0..2{let gd=raw(&got[c]);let host=got[c].get();for i in 0..N{let bytes=host[i].to_canonical().into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>();assert_eq!(A::from_le_bytes_mod_order(&bytes),expected[c][i]);assert!(raw_value(&gd,i)<twice);for digit in gd{assert!(digit[i]<1u64<<52);}count+=1;}}
                        }
                    }
                    println!("ARK_PHASED lanes={N} mixed={MIXED} chain_outputs={count}");
                }
                #[test]fn exact_signed_products_and_coefficients(){integer_check::<FqConfig,false>();integer_check::<FqConfig,true>();integer_check::<FrConfig,false>();integer_check::<FrConfig,true>();}
                #[test]fn arkworks_lazy_boundary_and_chains(){chain_check::<FqConfig,ark_ref::Fq,false>();chain_check::<FqConfig,ark_ref::Fq,true>();chain_check::<FrConfig,ark_ref::Fr,false>();chain_check::<FrConfig,ark_ref::Fr,true>();}
            }
        }
    }
    lane_tests!(four, zopro_bn254::ifma::Ifma, 4);
    lane_tests!(eight, zopro_bn254::ifma8::Ifma8, 8);
}
