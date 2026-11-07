use std::hint::black_box;
use zopro_bn254::{ifma::Ifma, ifma8::Ifma8, Fq, FqConfig, Fr, FrConfig};

#[no_mangle]
#[inline(never)]
pub fn probe_fq8_mul(a: &Ifma8<FqConfig>, b: &Ifma8<FqConfig>) -> Ifma8<FqConfig> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn probe_fq8_sum2(
    a: &Ifma8<FqConfig>,
    b: &Ifma8<FqConfig>,
    c: &Ifma8<FqConfig>,
    d: &Ifma8<FqConfig>,
) -> Ifma8<FqConfig> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn probe_fr8_mul(a: &Ifma8<FrConfig>, b: &Ifma8<FrConfig>) -> Ifma8<FrConfig> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn probe_fq4_mul(a: &Ifma<FqConfig>, b: &Ifma<FqConfig>) -> Ifma<FqConfig> {
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
            a[0].to_le_bytes(),
            b[0].to_le_bytes(),
            c[0].to_le_bytes(),
            d[0].to_le_bytes()
        ))
    );
}
