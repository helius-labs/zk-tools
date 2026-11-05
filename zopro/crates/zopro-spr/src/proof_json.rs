//! The proof as Zolana's prover server returns it: `ar` [x, y], `bs`
//! [[x.c1, x.c0], [y.c1, y.c0]], `krs` [x, y], each coordinate `0x` and 64
//! hex digits, and `provingKeySha256`, the gnark key's sha256.
use serde_json::Value;
use zopro_prover::zopro_core::verify::{Proof, hex_of};

/// The coordinates in `Proof::to_raw`'s order: Ar x, y; Krs x, y; Bs x.c1,
/// x.c0, y.c1, y.c0.
pub fn to_json(proof: &Proof, proving_key_sha256: Option<&str>) -> Value {
    let raw = proof.to_raw();
    let hex = |i: usize| Value::String(format!("0x{}", hex_of(&raw[32 * i..32 * i + 32])));
    let mut json = serde_json::json!({
        "ar": [hex(0), hex(1)],
        "bs": [[hex(4), hex(5)], [hex(6), hex(7)]],
        "krs": [hex(2), hex(3)],
    });
    if let Some(digest) = proving_key_sha256 {
        json["provingKeySha256"] = Value::String(digest.into());
    }
    json
}

/// From `to_json`'s coordinates; the points are checked to be on the curve.
pub fn from_json(json: &Value) -> Result<Proof, &'static str> {
    let coordinates = [
        &json["ar"][0],
        &json["ar"][1],
        &json["krs"][0],
        &json["krs"][1],
        &json["bs"][0][0],
        &json["bs"][0][1],
        &json["bs"][1][0],
        &json["bs"][1][1],
    ];
    let mut raw = Vec::with_capacity(256);
    for value in coordinates {
        let text = value.as_str().ok_or("a coordinate is a string")?;
        let text = text
            .strip_prefix("0x")
            .ok_or("a coordinate starts with 0x")?;
        if text.len() != 64 {
            return Err("a coordinate is 64 hex digits");
        }
        for i in 0..32 {
            raw.push(u8::from_str_radix(&text[2 * i..2 * i + 2], 16).map_err(|_| "not hex")?);
        }
    }
    Proof::from_raw(&raw)
}
