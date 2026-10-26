use std::hint::black_box;
use zopro_bn254::{Fq, FqConfig, Fr, FrConfig};
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r256_mul4(
    a: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    b: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    c: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    d: &zopro_bn254::ifma::Ifma<FqConfig, true>,
) -> zopro_bn254::ifma::Ifma<FqConfig, true> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r256_sum24(
    a: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    b: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    c: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    d: &zopro_bn254::ifma::Ifma<FqConfig, true>,
) -> zopro_bn254::ifma::Ifma<FqConfig, true> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r256_difference24(
    a: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    b: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    c: &zopro_bn254::ifma::Ifma<FqConfig, true>,
    d: &zopro_bn254::ifma::Ifma<FqConfig, true>,
) -> zopro_bn254::ifma::Ifma<FqConfig, true> {
    a.difference2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r256_mul8(
    a: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    b: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    c: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    d: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
) -> zopro_bn254::ifma8::Ifma8<FqConfig, true> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r256_sum28(
    a: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    b: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    c: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    d: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
) -> zopro_bn254::ifma8::Ifma8<FqConfig, true> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r256_difference28(
    a: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    b: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    c: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
    d: &zopro_bn254::ifma8::Ifma8<FqConfig, true>,
) -> zopro_bn254::ifma8::Ifma8<FqConfig, true> {
    a.difference2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r260_mul4(
    a: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    b: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    c: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    d: &zopro_bn254::ifma::Ifma<FqConfig, false>,
) -> zopro_bn254::ifma::Ifma<FqConfig, false> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r260_sum24(
    a: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    b: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    c: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    d: &zopro_bn254::ifma::Ifma<FqConfig, false>,
) -> zopro_bn254::ifma::Ifma<FqConfig, false> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r260_difference24(
    a: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    b: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    c: &zopro_bn254::ifma::Ifma<FqConfig, false>,
    d: &zopro_bn254::ifma::Ifma<FqConfig, false>,
) -> zopro_bn254::ifma::Ifma<FqConfig, false> {
    a.difference2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r260_mul8(
    a: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    b: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    c: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    d: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
) -> zopro_bn254::ifma8::Ifma8<FqConfig, false> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r260_sum28(
    a: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    b: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    c: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    d: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
) -> zopro_bn254::ifma8::Ifma8<FqConfig, false> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fq_r260_difference28(
    a: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    b: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    c: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
    d: &zopro_bn254::ifma8::Ifma8<FqConfig, false>,
) -> zopro_bn254::ifma8::Ifma8<FqConfig, false> {
    a.difference2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r256_mul4(
    a: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    b: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    c: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    d: &zopro_bn254::ifma::Ifma<FrConfig, true>,
) -> zopro_bn254::ifma::Ifma<FrConfig, true> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r256_sum24(
    a: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    b: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    c: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    d: &zopro_bn254::ifma::Ifma<FrConfig, true>,
) -> zopro_bn254::ifma::Ifma<FrConfig, true> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r256_difference24(
    a: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    b: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    c: &zopro_bn254::ifma::Ifma<FrConfig, true>,
    d: &zopro_bn254::ifma::Ifma<FrConfig, true>,
) -> zopro_bn254::ifma::Ifma<FrConfig, true> {
    a.difference2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r256_mul8(
    a: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    b: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    c: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    d: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
) -> zopro_bn254::ifma8::Ifma8<FrConfig, true> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r256_sum28(
    a: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    b: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    c: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    d: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
) -> zopro_bn254::ifma8::Ifma8<FrConfig, true> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r256_difference28(
    a: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    b: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    c: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
    d: &zopro_bn254::ifma8::Ifma8<FrConfig, true>,
) -> zopro_bn254::ifma8::Ifma8<FrConfig, true> {
    a.difference2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r260_mul4(
    a: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    b: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    c: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    d: &zopro_bn254::ifma::Ifma<FrConfig, false>,
) -> zopro_bn254::ifma::Ifma<FrConfig, false> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r260_sum24(
    a: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    b: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    c: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    d: &zopro_bn254::ifma::Ifma<FrConfig, false>,
) -> zopro_bn254::ifma::Ifma<FrConfig, false> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r260_difference24(
    a: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    b: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    c: &zopro_bn254::ifma::Ifma<FrConfig, false>,
    d: &zopro_bn254::ifma::Ifma<FrConfig, false>,
) -> zopro_bn254::ifma::Ifma<FrConfig, false> {
    a.difference2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r260_mul8(
    a: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    b: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    c: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    d: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
) -> zopro_bn254::ifma8::Ifma8<FrConfig, false> {
    a.mul(b)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r260_sum28(
    a: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    b: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    c: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    d: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
) -> zopro_bn254::ifma8::Ifma8<FrConfig, false> {
    a.sum2(b, c, d)
}
#[no_mangle]
#[inline(never)]
pub fn audit_fr_r260_difference28(
    a: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    b: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    c: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
    d: &zopro_bn254::ifma8::Ifma8<FrConfig, false>,
) -> zopro_bn254::ifma8::Ifma8<FrConfig, false> {
    a.difference2(b, c, d)
}
fn main() {
    {
        let v = <zopro_bn254::ifma::Ifma<FqConfig, true>>::new(&[Fq::from_u64(black_box(7)); 4]);
        black_box(audit_fq_r256_mul4(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FqConfig, true>>::new(&[Fq::from_u64(black_box(7)); 4]);
        black_box(audit_fq_r256_sum24(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FqConfig, true>>::new(&[Fq::from_u64(black_box(7)); 4]);
        black_box(audit_fq_r256_difference24(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FqConfig, true>>::new(&[Fq::from_u64(black_box(7)); 8]);
        black_box(audit_fq_r256_mul8(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FqConfig, true>>::new(&[Fq::from_u64(black_box(7)); 8]);
        black_box(audit_fq_r256_sum28(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FqConfig, true>>::new(&[Fq::from_u64(black_box(7)); 8]);
        black_box(audit_fq_r256_difference28(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FqConfig, false>>::new(&[Fq::from_u64(black_box(7)); 4]);
        black_box(audit_fq_r260_mul4(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FqConfig, false>>::new(&[Fq::from_u64(black_box(7)); 4]);
        black_box(audit_fq_r260_sum24(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FqConfig, false>>::new(&[Fq::from_u64(black_box(7)); 4]);
        black_box(audit_fq_r260_difference24(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FqConfig, false>>::new(&[Fq::from_u64(black_box(7)); 8]);
        black_box(audit_fq_r260_mul8(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FqConfig, false>>::new(&[Fq::from_u64(black_box(7)); 8]);
        black_box(audit_fq_r260_sum28(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FqConfig, false>>::new(&[Fq::from_u64(black_box(7)); 8]);
        black_box(audit_fq_r260_difference28(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FrConfig, true>>::new(&[Fr::from_u64(black_box(7)); 4]);
        black_box(audit_fr_r256_mul4(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FrConfig, true>>::new(&[Fr::from_u64(black_box(7)); 4]);
        black_box(audit_fr_r256_sum24(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FrConfig, true>>::new(&[Fr::from_u64(black_box(7)); 4]);
        black_box(audit_fr_r256_difference24(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FrConfig, true>>::new(&[Fr::from_u64(black_box(7)); 8]);
        black_box(audit_fr_r256_mul8(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FrConfig, true>>::new(&[Fr::from_u64(black_box(7)); 8]);
        black_box(audit_fr_r256_sum28(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FrConfig, true>>::new(&[Fr::from_u64(black_box(7)); 8]);
        black_box(audit_fr_r256_difference28(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FrConfig, false>>::new(&[Fr::from_u64(black_box(7)); 4]);
        black_box(audit_fr_r260_mul4(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FrConfig, false>>::new(&[Fr::from_u64(black_box(7)); 4]);
        black_box(audit_fr_r260_sum24(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma::Ifma<FrConfig, false>>::new(&[Fr::from_u64(black_box(7)); 4]);
        black_box(audit_fr_r260_difference24(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FrConfig, false>>::new(&[Fr::from_u64(black_box(7)); 8]);
        black_box(audit_fr_r260_mul8(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FrConfig, false>>::new(&[Fr::from_u64(black_box(7)); 8]);
        black_box(audit_fr_r260_sum28(&v, &v, &v, &v));
    }
    {
        let v = <zopro_bn254::ifma8::Ifma8<FrConfig, false>>::new(&[Fr::from_u64(black_box(7)); 8]);
        black_box(audit_fr_r260_difference28(&v, &v, &v, &v));
    }
}
