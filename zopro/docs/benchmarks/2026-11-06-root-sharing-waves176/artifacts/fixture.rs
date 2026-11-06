use sha2::{Digest, Sha256};
use zopro_bn254::Fq;
pub const ROWS: usize = 256;
pub fn fixture() -> Vec<[Fq; 32]> {
    (0..ROWS)
        .map(|row| {
            std::array::from_fn(|i| {
                if row == 0 {
                    return Fq::from_u64((i + 1) as u64);
                }
                if row == 1 {
                    return -Fq::from_u64((i + 1) as u64);
                }
                let bytes =
                    Sha256::digest(format!("root-sharing/public/row={row}/root={i}").as_bytes());
                let value = Fq::from_be_bytes_mod_order(&bytes);
                if value.is_zero() {
                    Fq::ONE
                } else {
                    value
                }
            })
        })
        .collect()
}
pub fn hash(roots: &[[Fq; 32]]) -> String {
    let mut h = Sha256::new();
    for row in roots {
        for root in row {
            for w in root.0 {
                h.update(w.to_le_bytes());
            }
        }
    }
    format!("{:x}", h.finalize())
}
pub fn accumulate(sum: &mut [u64; 4], value: Fq) {
    for i in 0..4 {
        sum[i] = sum[i].wrapping_add(value.0[i]);
    }
}
