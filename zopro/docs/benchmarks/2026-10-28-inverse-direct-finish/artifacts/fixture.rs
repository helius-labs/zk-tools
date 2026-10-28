use ark_ff::{BigInteger, Field, PrimeField};
use serde_json::{json, Value};
use std::fs;
fn values<F: PrimeField>() -> Value {
    let mut seed = 0x44495246494e4953u64;
    let mut xs = vec![F::ONE, -F::ONE, F::from(2u64), -F::from(2u64)];
    for bit in 0..254 {
        let x = F::from(2u64).pow([bit]);
        xs.extend([x, -x]);
    }
    while xs.len() < 4096 {
        let mut bytes = [0u8; 64];
        for c in bytes.chunks_mut(8) {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            c.copy_from_slice(&seed.to_le_bytes());
        }
        let x = F::from_le_bytes_mod_order(&bytes);
        if x != F::ZERO {
            xs.push(x)
        }
    }
    assert_eq!(xs.len(), 4096);
    json!(xs
        .into_iter()
        .map(|x| {
            let words = |v: F| {
                let bytes = v.into_bigint().to_bytes_le();
                core::array::from_fn::<_, 4, _>(|i| {
                    u64::from_le_bytes(bytes[8 * i..8 * i + 8].try_into().unwrap())
                })
            };
            json!({"input":words(x),"inverse":words(x.inverse().unwrap())})
        })
        .collect::<Vec<_>>())
}
fn main() {
    fs::write(
        "fixture.json",
        serde_json::to_vec_pretty(
            &json!({"fq":values::<ark_bn254::Fq>(),"fr":values::<ark_bn254::Fr>()}),
        )
        .unwrap(),
    )
    .unwrap();
    println!("Generated4096nonzero independent Arkworks inputs and inverses perfield");
}
