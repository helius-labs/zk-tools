use std::hint::black_box;
use zopro_bn254::{ifma::Ifma, ifma8::Ifma8, Fq, FqConfig, Fr, FrConfig};

#[no_mangle]
#[inline(never)]
pub fn probe_fq8_mul(
    a: &Ifma8<FqConfig, true>,
    b: &Ifma8<FqConfig, true>,
) -> Ifma8<FqConfig, true> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn probe_fq8_sum2(
    a: &Ifma8<FqConfig, true>,
    b: &Ifma8<FqConfig, true>,
    c: &Ifma8<FqConfig, true>,
    d: &Ifma8<FqConfig, true>,
) -> Ifma8<FqConfig, true> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn probe_fr8_mul(
    a: &Ifma8<FrConfig, true>,
    b: &Ifma8<FrConfig, true>,
) -> Ifma8<FrConfig, true> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn probe_fq4_mul(a: &Ifma<FqConfig, true>, b: &Ifma<FqConfig, true>) -> Ifma<FqConfig, true> {
    a.mul(b)
}

fn main() {
    let seed = std::env::args()
        .nth(1)
        .unwrap_or("17".into())
        .parse::<u64>()
        .unwrap();
    let fq = Ifma8::new(&[Fq::from(seed); 8]);
    let fr = Ifma8::new(&[Fr::from(seed); 8]);
    let fq4 = Ifma::new(&[Fq::from(seed); 4]);
    let a = probe_fq8_mul(black_box(&fq), black_box(&fq)).get();
    let b = probe_fq8_sum2(
        black_box(&fq),
        black_box(&fq),
        black_box(&fq),
        black_box(&fq),
    )
    .get();
    let c = probe_fr8_mul(black_box(&fr), black_box(&fr)).get();
    let d = probe_fq4_mul(black_box(&fq4), black_box(&fq4)).get();
    println!(
        "{:?}",
        black_box((
            a[0].to_be_bytes(),
            b[0].to_be_bytes(),
            c[0].to_be_bytes(),
            d[0].to_be_bytes()
        ))
    );
}
