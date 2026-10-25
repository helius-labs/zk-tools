//! The SDKs' proof requests: JSON whose shape zolana's strict parse
//! (`request.ParseStrict`) checks field by field — every field present
//! once, none unknown, strings of 1 to 256 bytes, counts as unsigned 32-bit
//! numbers, no more than 32 levels or 8 MB — and whose values are field
//! elements in hex (`0x` optional, then as big.Int reads base 16: a sign
//! allowed, no underscores) below the scalar field's order.
use std::fmt;

use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use zopro_bn254::Fr;

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

/// The request as a JSON value, its size and depth within bounds and no
/// object's key repeated.
pub fn parse(request: &str) -> Result<Value, Error> {
    if request.len() > MAX_BYTES {
        return Err(Error::Request);
    }
    let mut deserializer = serde_json::Deserializer::from_str(request);
    let value = Unique
        .deserialize(&mut deserializer)
        .and_then(|value| deserializer.end().map(|()| value))
        .map_err(|_| Error::Request)?;
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

/// A JSON value as serde_json reads it, but an object that repeats a key is
/// an error rather than its last value.
struct Unique;

impl<'de> DeserializeSeed<'de> for Unique {
    type Value = Value;

    fn deserialize<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Unique {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JSON value")
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(value.into())
    }

    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(value.into())
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(serde_json::Number::from_f64(value).map_or(Value::Null, Value::Number))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut items = Vec::new();
        while let Some(item) = seq.next_element_seed(Unique)? {
            items.push(item);
        }
        Ok(Value::Array(items))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut object = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if object.contains_key(&key) {
                return Err(de::Error::custom("a repeated key"));
            }
            let value = map.next_value_seed(Unique)?;
            object.insert(key, value);
        }
        Ok(Value::Object(object))
    }
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

/// `s` as zolana's `FromHex` reads it (`0x` optional, then as big.Int reads
/// base 16: an optional sign, hex digits), between 0 and the field's order.
pub fn hex_element(s: &str) -> Result<Fr, Error> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let (negative, digits) = match s.as_bytes().first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
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
    let value = Fr::from_be_bytes(&bytes).ok_or(Error::Request)?;
    if negative && !value.is_zero() {
        return Err(Error::Request);
    }
    Ok(value)
}

/// `value` as a small unsigned integer, when it is one.
pub fn small(value: &Fr) -> Option<u64> {
    let limbs = value.to_canonical();
    if limbs[1..].iter().any(|w| *w != 0) {
        return None;
    }
    Some(limbs[0])
}

pub fn is_zero(value: &Fr) -> bool {
    value.is_zero()
}
