//! The SDKs' proof requests: JSON whose shape the Go prover checked field by
//! field — every field present, none unknown, strings of 1 to 256 bytes,
//! counts as unsigned 32-bit numbers, no more than 32 levels or 8 MB — and
//! whose values are field elements in hex (`0x` optional, as big.Int reads
//! base 16: a sign allowed, no underscores) below the scalar field's order.
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use serde_json::Value;

/// The Go prover's errors, kept apart for the SDKs' messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// `invalid proof request`: the JSON's shape or a value.
    Request,
    /// `unsupported proof request`: a circuit type this prover has no key for.
    Unsupported,
    /// `request does not match proving system`: the request's shape is not
    /// the key's.
    Shape,
    /// The proof itself: the solver, the kernel, or verification.
    Prove(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request => write!(f, "invalid proof request"),
            Self::Unsupported => write!(f, "unsupported proof request"),
            Self::Shape => write!(f, "request does not match proving system"),
            Self::Prove(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Self::Prove(e.into())
    }
}

const MAX_BYTES: usize = 8 << 20;
const MAX_DEPTH: usize = 32;

/// The request as a JSON value, its size and depth within bounds.
pub fn parse(request: &str) -> Result<Value, Error> {
    if request.len() > MAX_BYTES {
        return Err(Error::Request);
    }
    let value: Value = serde_json::from_str(request).map_err(|_| Error::Request)?;
    fn depth(value: &Value) -> usize {
        match value {
            Value::Object(map) => 1 + map.values().map(depth).max().unwrap_or(0),
            Value::Array(items) => 1 + items.iter().map(depth).max().unwrap_or(0),
            _ => 0,
        }
    }
    if depth(&value) > MAX_DEPTH || !value.is_object() {
        return Err(Error::Request);
    }
    Ok(value)
}

/// An object with exactly `fields`.
pub fn object<'a>(
    value: &'a Value,
    fields: &[&str],
) -> Result<&'a serde_json::Map<String, Value>, Error> {
    let map = value.as_object().ok_or(Error::Request)?;
    if map.len() != fields.len() || !fields.iter().all(|f| map.contains_key(*f)) {
        return Err(Error::Request);
    }
    Ok(map)
}

/// A string of 1 to 256 bytes.
pub fn string(value: &Value) -> Result<&str, Error> {
    match value.as_str() {
        Some(s) if !s.is_empty() && s.len() <= 256 => Ok(s),
        _ => Err(Error::Request),
    }
}

/// A count: a JSON number that is an unsigned 32-bit integer.
pub fn count(value: &Value) -> Result<u32, Error> {
    value
        .as_u64()
        .and_then(|n| u32::try_from(n).ok())
        .ok_or(Error::Request)
}

/// A field element in hex.
pub fn element(value: &Value) -> Result<Fr, Error> {
    hex_element(string(value)?)
}

/// Field elements in hex.
pub fn elements(value: &Value) -> Result<Vec<Fr>, Error> {
    value
        .as_array()
        .ok_or(Error::Request)?
        .iter()
        .map(element)
        .collect()
}

/// `s` as big.Int reads it in base 16 (an optional sign, `0x` optional,
/// hex digits), between 0 and the field's order.
pub fn hex_element(s: &str) -> Result<Fr, Error> {
    let (negative, digits) = match s.as_bytes().first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    let digits = digits.strip_prefix("0x").unwrap_or(digits);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(Error::Request);
    }
    let digits = digits.trim_start_matches('0');
    if digits.len() > 64 {
        return Err(Error::Request);
    }
    let mut bytes = [0u8; 32];
    for (i, digit) in digits.bytes().rev().enumerate() {
        let nibble = (digit as char).to_digit(16).unwrap() as u8;
        bytes[31 - i / 2] |= nibble << (4 * (i % 2));
    }
    let value = Fr::from_be_bytes_mod_order(&bytes);
    if value.into_bigint().to_bytes_be() != bytes {
        return Err(Error::Request);
    }
    if negative && !value.is_zero() {
        return Err(Error::Request);
    }
    Ok(value)
}

/// `value` as a small unsigned integer, when it is one.
pub fn small(value: &Fr) -> Option<u64> {
    let bigint = value.into_bigint();
    if bigint.0[1..].iter().any(|w| *w != 0) {
        return None;
    }
    Some(bigint.0[0])
}

pub fn is_zero(value: &Fr) -> bool {
    value.is_zero()
}

trait IsZero {
    fn is_zero(&self) -> bool;
}

impl IsZero for Fr {
    fn is_zero(&self) -> bool {
        *self == Fr::from(0u64)
    }
}
