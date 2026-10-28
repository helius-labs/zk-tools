//! Decompression of gnark's compressed BN254 points on the thread pool. A key
//! file stores every point compressed, and reading it costs a square root per
//! point, which the single-threaded Go wasm takes seconds to do for a wide
//! key. The bridge hands the compressed points here and gets gnark's raw
//! encoding back, which it decodes without square roots.
//!
//! gnark's compressed encoding is the big-endian x coordinate with two flag
//! bits in the first byte: `10` for the lexicographically smaller y, `11` for
//! the larger, `01` for the point at infinity (the rest zero). A G2 point
//! stores x as `A1 | A0`. The raw encoding is x then y, each coordinate in the
//! same order, and the point at infinity is all zero.
use zopro_bn254::{CurveConfig, Fq, Fq2, G1Affine, G1Config, G2Affine, G2Config};
use rayon::prelude::*;
use wasm_bindgen::prelude::*;

const MASK: u8 = 0b11 << 6;
const SMALLEST: u8 = 0b10 << 6;
const LARGEST: u8 = 0b11 << 6;
const INFINITY: u8 = 0b01 << 6;

/// `compressed` G1 points (32 bytes each) as raw points (64 bytes each).
#[wasm_bindgen]
pub fn decompress_g1(compressed: &[u8]) -> Result<Vec<u8>, JsError> {
    g1(compressed).map_err(JsError::new)
}

/// `compressed` G2 points (64 bytes each) as raw points (128 bytes each).
#[wasm_bindgen]
pub fn decompress_g2(compressed: &[u8]) -> Result<Vec<u8>, JsError> {
    g2(compressed).map_err(JsError::new)
}

pub(crate) fn g1(compressed: &[u8]) -> Result<Vec<u8>, &'static str> {
    decompress(compressed, 32, 64, g1_point)
}

pub(crate) fn g2(compressed: &[u8]) -> Result<Vec<u8>, &'static str> {
    decompress(compressed, 64, 128, g2_point)
}

fn decompress(
    compressed: &[u8],
    compressed_size: usize,
    raw_size: usize,
    point: fn(&[u8], &mut [u8]) -> Result<(), &'static str>,
) -> Result<Vec<u8>, &'static str> {
    if !compressed.len().is_multiple_of(compressed_size) {
        return Err("compressed points of a partial length");
    }
    let mut raw = vec![0u8; compressed.len() / compressed_size * raw_size];
    compressed
        .par_chunks(compressed_size)
        .zip(raw.par_chunks_mut(raw_size))
        .try_for_each(|(compressed, raw)| point(compressed, raw))?;
    Ok(raw)
}

/// Whether `value` is larger than its negation: above (q - 1) / 2.
pub(crate) fn largest(value: &Fq) -> bool {
    value.is_largest()
}

/// As gnark orders E2: by the second coordinate, by the first when it is zero.
pub(crate) fn largest2(value: &Fq2) -> bool {
    value.is_largest()
}

/// The canonical big-endian field element in `bytes`.
pub(crate) fn coordinate(bytes: &[u8]) -> Result<Fq, &'static str> {
    let bytes: &[u8; 32] = bytes.try_into().map_err(|_| "a 32-byte coordinate")?;
    Fq::from_be_bytes(bytes).ok_or("coordinate not below the field modulus")
}

fn write(out: &mut [u8], value: &Fq) {
    out.copy_from_slice(&value.to_be_bytes());
}

/// The flags of a compressed point, or `None` for the point at infinity,
/// which must be zero otherwise.
fn flags(compressed: &[u8]) -> Result<Option<u8>, &'static str> {
    let mask = compressed[0] & MASK;
    if mask == INFINITY {
        if compressed[0] & !MASK != 0 || compressed[1..].iter().any(|&byte| byte != 0) {
            return Err("point at infinity with coordinates");
        }
        return Ok(None);
    }
    if mask != SMALLEST && mask != LARGEST {
        return Err("not a compressed point");
    }
    Ok(Some(mask))
}

/// A compressed G1 point's coordinates, `None` for the point at infinity.
fn g1_coordinates(compressed: &[u8]) -> Result<Option<(Fq, Fq)>, &'static str> {
    let Some(mask) = flags(compressed)? else {
        return Ok(None);
    };
    let mut x_bytes = [0u8; 32];
    x_bytes.copy_from_slice(compressed);
    x_bytes[0] &= !MASK;
    let x = coordinate(&x_bytes)?;
    let mut y = (x.square() * x + G1Config::B)
        .sqrt()
        .ok_or("x is not on the curve")?;
    if largest(&y) != (mask == LARGEST) {
        y = -y;
    }
    Ok(Some((x, y)))
}

/// A compressed G2 point's coordinates, `None` for the point at infinity.
fn g2_coordinates(compressed: &[u8]) -> Result<Option<(Fq2, Fq2)>, &'static str> {
    let Some(mask) = flags(compressed)? else {
        return Ok(None);
    };
    let mut a1 = [0u8; 32];
    a1.copy_from_slice(&compressed[..32]);
    a1[0] &= !MASK;
    let x = Fq2::new(coordinate(&compressed[32..])?, coordinate(&a1)?);
    let mut y = (x.square() * x + G2Config::B)
        .sqrt()
        .ok_or("x is not on the twist")?;
    if largest2(&y) != (mask == LARGEST) {
        y = -y;
    }
    Ok(Some((x, y)))
}

fn g1_point(compressed: &[u8], raw: &mut [u8]) -> Result<(), &'static str> {
    match g1_coordinates(compressed)? {
        None => raw.fill(0),
        Some((x, y)) => {
            write(&mut raw[..32], &x);
            write(&mut raw[32..], &y);
        }
    }
    Ok(())
}

fn g2_point(compressed: &[u8], raw: &mut [u8]) -> Result<(), &'static str> {
    match g2_coordinates(compressed)? {
        None => raw.fill(0),
        Some((x, y)) => {
            write(&mut raw[..32], &x.c1);
            write(&mut raw[32..64], &x.c0);
            write(&mut raw[64..96], &y.c1);
            write(&mut raw[96..], &y.c0);
        }
    }
    Ok(())
}

/// `compressed` G1 points as the kernel's points, with no byte encoding
/// between: a key's load holds each vector once.
pub(crate) fn g1_affine(compressed: &[u8]) -> Result<Vec<G1Affine>, &'static str> {
    affine(compressed, 32, G1Affine::IDENTITY, |c| {
        Ok(g1_coordinates(c)?.map_or(G1Affine::IDENTITY, |(x, y)| G1Affine::new(x, y)))
    })
}

/// `compressed` G2 points as the kernel's points.
pub(crate) fn g2_affine(compressed: &[u8]) -> Result<Vec<G2Affine>, &'static str> {
    affine(compressed, 64, G2Affine::IDENTITY, |c| {
        Ok(g2_coordinates(c)?.map_or(G2Affine::IDENTITY, |(x, y)| G2Affine::new(x, y)))
    })
}

/// Into a vector allocated once: collecting a fallible parallel iterator
/// goes through per-thread lists, the vector twice over at its end.
fn affine<P: Copy + Send + Sync>(
    compressed: &[u8],
    size: usize,
    identity: P,
    point: impl Fn(&[u8]) -> Result<P, &'static str> + Sync,
) -> Result<Vec<P>, &'static str> {
    if !compressed.len().is_multiple_of(size) {
        return Err("compressed points of a partial length");
    }
    let mut out = vec![identity; compressed.len() / size];
    compressed
        .par_chunks(size)
        .zip(out.par_iter_mut())
        .try_for_each(|(compressed, out)| {
            *out = point(compressed)?;
            Ok(())
        })?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vectors {
        include!("decompress_vectors.rs");
    }

    fn hex(text: &str) -> Vec<u8> {
        (0..text.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn g1_points_decompress_as_gnark_encodes_them() {
        let compressed: Vec<u8> = vectors::G1.iter().flat_map(|(c, _)| hex(c)).collect();
        let raw: Vec<u8> = vectors::G1.iter().flat_map(|(_, r)| hex(r)).collect();
        assert_eq!(g1(&compressed).unwrap(), raw);
    }

    #[test]
    fn g2_points_decompress_as_gnark_encodes_them() {
        let compressed: Vec<u8> = vectors::G2.iter().flat_map(|(c, _)| hex(c)).collect();
        let raw: Vec<u8> = vectors::G2.iter().flat_map(|(_, r)| hex(r)).collect();
        assert_eq!(g2(&compressed).unwrap(), raw);
    }

    #[test]
    fn points_decompress_as_the_kernel_decodes_gnark_raw_ones() {
        let compressed: Vec<u8> = vectors::G1.iter().flat_map(|(c, _)| hex(c)).collect();
        let raw = g1(&compressed).unwrap();
        let points = g1_affine(&compressed).unwrap();
        assert_eq!(points.len(), raw.len() / 64);
        for (point, raw) in points.iter().zip(raw.chunks(64)) {
            assert_eq!(point.x.to_be_bytes(), raw[..32]);
            assert_eq!(point.y.to_be_bytes(), raw[32..]);
        }
        let compressed: Vec<u8> = vectors::G2.iter().flat_map(|(c, _)| hex(c)).collect();
        let raw = g2(&compressed).unwrap();
        let points = g2_affine(&compressed).unwrap();
        for (point, raw) in points.iter().zip(raw.chunks(128)) {
            assert_eq!(point.x.c1.to_be_bytes(), raw[..32]);
            assert_eq!(point.x.c0.to_be_bytes(), raw[32..64]);
            assert_eq!(point.y.c1.to_be_bytes(), raw[64..96]);
            assert_eq!(point.y.c0.to_be_bytes(), raw[96..]);
        }
    }

    #[test]
    fn malformed_points_are_refused() {
        let mut off_curve = hex(vectors::G1[0].0);
        off_curve[31] ^= 1;
        assert!(g1(&off_curve).is_err());
        let mut uncompressed = hex(vectors::G1[0].0);
        uncompressed[0] &= !MASK;
        assert!(g1(&uncompressed).is_err());
        let mut infinity_with_x = hex(vectors::G1[8].0);
        infinity_with_x[5] = 1;
        assert!(g1(&infinity_with_x).is_err());
        assert!(g1(&[0; 31]).is_err());
        assert!(g2(&hex(vectors::G1[0].0)).is_err());
    }
}
