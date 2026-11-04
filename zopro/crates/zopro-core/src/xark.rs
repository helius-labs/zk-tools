//! Xark's Groth16 artifacts as a zopro key's parts, read without arkworks:
//! Xark's proving key (`pk.bin`) and verifying key (`vk.bin`), which
//! arkworks' serialization lays out, and the constraint rows the key was
//! made for, in Xark's row file (`r1cs.stream`; `tools/xark` writes it from
//! a circuit's `circuit.xbc`). `pack` makes the key file.
//!
//! The mapping. Xark's assignment z is the constant one, the public inputs
//! in id order, then every other variable in id order (Xark's `VarLayout`).
//! A zopro key of an Xark circuit takes all of z[1..] as its inputs, so its
//! solver program assigns nothing: Xark solves the witness, and the kernel
//! evaluates and checks the rows and proves. The rows are Xark's
//! constraints, then libsnark's input-consistency rows that Xark's QAP
//! appends (A = z_i for each instance variable i, B = C = 0). The vectors:
//! A is the key's a_query and B, B2 its b_g1_query and b_g2_query, each
//! without its points at infinity (the solver's index lists say which
//! variables remain); K is the l_query; Z is the h_query (t^i Z(t) / delta)
//! in bit-reversed order, the order in which zopro's quotient emits the
//! coefficients. The domain is Xark's: n points of arkworks' radix-2 root
//! of unity, with zopro's coset 5 for the quotient. The verifying key is
//! gnark's raw layout of Xark's: alpha from the verifying key, beta and
//! delta (G1) from the proving key, beta2, gamma2, delta2 and the inputs'
//! points from the verifying key, and no commitments.
use std::collections::HashMap;

use rayon::prelude::*;
use zopro_bn254::{CurveConfig, Fq, Fq2, Fr, G1Affine, G1Config, G2Affine, G2Config};

use crate::dump::{Dump, Head};

/// arkworks' flags in the last byte of a coordinate it serializes: y is the
/// larger root (`y > -y`), and the point at infinity.
const NEGATIVE: u8 = 1 << 7;
const INFINITY: u8 = 1 << 6;
const FLAGS: u8 = NEGATIVE | INFINITY;
/// `pk.bin`'s container header (Xark's `KeyFormat`).
const PK_MAGIC: &[u8; 4] = b"XKPK";
/// Xark's row file.
const ROWS_MAGIC: &[u8; 4] = b"XRS1";
/// The coset of zopro's quotient: the multiplicative group's generator.
const COSET: u64 = 5;

/// A cursor over bytes, every read bounds-checked.
struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self.at.checked_add(n).ok_or("a length past the file")?;
        let out = self.bytes.get(self.at..end).ok_or("truncated file")?;
        self.at = end;
        Ok(out)
    }
    fn byte(&mut self) -> Result<u8, String> {
        self.take(1)?
            .first()
            .copied()
            .ok_or_else(|| "truncated file".into())
    }
    fn u64_le(&mut self) -> Result<u64, String> {
        let bytes: [u8; 8] = self.take(8)?.try_into().map_err(|_| "truncated file")?;
        Ok(u64::from_le_bytes(bytes))
    }
    /// An unsigned LEB128 value, as Xark's row file stores its integers.
    fn leb128(&mut self) -> Result<u64, String> {
        let mut value = 0u64;
        let mut shift = 0;
        loop {
            let byte = self.byte()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
            shift += 7;
            if shift > 63 {
                return Err("an integer past 64 bits".into());
            }
        }
    }
    fn count(&mut self) -> Result<usize, String> {
        usize::try_from(self.leb128()?).map_err(|_| "a count past usize".into())
    }
    fn done(&self) -> bool {
        self.at == self.bytes.len()
    }
}

/// How arkworks encoded a key's points.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Encoding {
    /// x alone and a flag for y: 32 bytes a G1 point, 64 a G2 point.
    Compressed,
    /// x then y: 64 and 128 bytes.
    Uncompressed,
}

impl Encoding {
    fn g1(self) -> usize {
        match self {
            Self::Compressed => 32,
            Self::Uncompressed => 64,
        }
    }
    fn g2(self) -> usize {
        2 * self.g1()
    }
}

/// An arkworks base field element, 32 little-endian bytes, and the flags in
/// its last byte.
fn ark_fq(bytes: &[u8]) -> Result<(Fq, u8), &'static str> {
    let bytes: &[u8; 32] = bytes.try_into().map_err(|_| "a 32-byte coordinate")?;
    let mut be = *bytes;
    be.reverse();
    let flags = be[0] & FLAGS;
    be[0] &= !FLAGS;
    let value = Fq::from_be_bytes(&be).ok_or("a coordinate not below the field modulus")?;
    Ok((value, flags))
}

/// An arkworks Fq2 element: c0 then c1, the flags in c1's last byte.
fn ark_fq2(bytes: &[u8]) -> Result<(Fq2, u8), &'static str> {
    let (c0, c1) = bytes.split_at_checked(32).ok_or("a 64-byte coordinate")?;
    let (c0, low) = ark_fq(c0)?;
    if low != 0 {
        return Err("flags inside an Fq2 coordinate");
    }
    let (c1, flags) = ark_fq(c1)?;
    Ok((Fq2::new(c0, c1), flags))
}

/// The y of a compressed point: the root of x^3 + b that the flag names.
/// arkworks' flag is set for the larger root, `y > -y`, which orders Fq2 by
/// c1 and then c0: gnark's order, `is_largest`.
fn root<F: Copy + core::ops::Neg<Output = F>>(
    y: Option<F>,
    flags: u8,
    largest: fn(&F) -> bool,
) -> Result<F, &'static str> {
    let y = y.ok_or("x is not on the curve")?;
    Ok(if largest(&y) == (flags & NEGATIVE != 0) {
        y
    } else {
        -y
    })
}

/// A G1 point in arkworks' encoding: y is `None` when compressed.
fn ark_g1(bytes: &[u8], encoding: Encoding) -> Result<G1Affine, &'static str> {
    let (x, y, flags) = match encoding {
        Encoding::Compressed => {
            let (x, flags) = ark_fq(bytes)?;
            (x, None, flags)
        }
        Encoding::Uncompressed => {
            let (x, y) = bytes.split_at_checked(32).ok_or("a 64-byte point")?;
            let (x, low) = ark_fq(x)?;
            let (y, flags) = ark_fq(y)?;
            if low != 0 {
                return Err("flags on x");
            }
            (x, Some(y), flags)
        }
    };
    if flags & INFINITY != 0 {
        let zero = x.is_zero() && y.is_none_or(|y| y.is_zero()) && flags == INFINITY;
        return infinity(zero, G1Affine::IDENTITY);
    }
    match y {
        None => {
            let y = root((x.square() * x + G1Config::B).sqrt(), flags, Fq::is_largest)?;
            Ok(G1Affine::new(x, y))
        }
        Some(y) => on_curve(G1Affine::new(x, y), G1Affine::is_on_curve),
    }
}

/// A G2 point in arkworks' encoding.
fn ark_g2(bytes: &[u8], encoding: Encoding) -> Result<G2Affine, &'static str> {
    let (x, y, flags) = match encoding {
        Encoding::Compressed => {
            let (x, flags) = ark_fq2(bytes)?;
            (x, None, flags)
        }
        Encoding::Uncompressed => {
            let (x, y) = bytes.split_at_checked(64).ok_or("a 128-byte point")?;
            let (x, low) = ark_fq2(x)?;
            let (y, flags) = ark_fq2(y)?;
            if low != 0 {
                return Err("flags on x");
            }
            (x, Some(y), flags)
        }
    };
    if flags & INFINITY != 0 {
        let zero = x.is_zero() && y.is_none_or(|y| y.is_zero()) && flags == INFINITY;
        return infinity(zero, G2Affine::IDENTITY);
    }
    match y {
        None => {
            let y = root(
                (x.square() * x + G2Config::B).sqrt(),
                flags,
                Fq2::is_largest,
            )?;
            Ok(G2Affine::new(x, y))
        }
        Some(y) => on_curve(G2Affine::new(x, y), G2Affine::is_on_curve),
    }
}

fn on_curve<P>(point: P, is_on_curve: fn(&P) -> bool) -> Result<P, &'static str> {
    if is_on_curve(&point) {
        Ok(point)
    } else {
        Err("a point not on the curve")
    }
}

fn infinity<P>(zero: bool, identity: P) -> Result<P, &'static str> {
    if zero {
        Ok(identity)
    } else {
        Err("a point at infinity with coordinates")
    }
}

/// A vector as arkworks serializes one: a u64 length, then the points of
/// `size` bytes each, decoded on the pool.
fn vector<P: Copy + Send + Sync>(
    cursor: &mut Cursor,
    size: usize,
    identity: P,
    decode: impl Fn(&[u8]) -> Result<P, &'static str> + Sync,
) -> Result<Vec<P>, String> {
    let count = usize::try_from(cursor.u64_le()?).map_err(|_| "a vector past usize")?;
    let bytes = cursor.take(count.checked_mul(size).ok_or("a vector past the file")?)?;
    let mut out = vec![identity; count];
    bytes
        .par_chunks(size)
        .zip(out.par_iter_mut())
        .try_for_each(|(bytes, out)| {
            *out = decode(bytes)?;
            Ok::<(), &'static str>(())
        })?;
    Ok(out)
}

fn g1_vector(cursor: &mut Cursor, encoding: Encoding) -> Result<Vec<G1Affine>, String> {
    vector(cursor, encoding.g1(), G1Affine::IDENTITY, |b| {
        ark_g1(b, encoding)
    })
}

fn g1(cursor: &mut Cursor, encoding: Encoding) -> Result<G1Affine, String> {
    Ok(ark_g1(cursor.take(encoding.g1())?, encoding)?)
}

fn g2(cursor: &mut Cursor, encoding: Encoding) -> Result<G2Affine, String> {
    let point = ark_g2(cursor.take(encoding.g2())?, encoding)?;
    if !point.is_in_subgroup() {
        return Err("a verifying key's G2 point outside the subgroup".into());
    }
    Ok(point)
}

/// arkworks' (and Xark's) Groth16 verifying key.
#[derive(PartialEq, Eq)]
struct VerifyingKey {
    alpha: G1Affine,
    beta2: G2Affine,
    gamma2: G2Affine,
    delta2: G2Affine,
    /// The constant one's point, then each public input's.
    inputs: Vec<G1Affine>,
}

impl VerifyingKey {
    fn read(cursor: &mut Cursor, encoding: Encoding) -> Result<Self, String> {
        Ok(Self {
            alpha: g1(cursor, encoding)?,
            beta2: g2(cursor, encoding)?,
            gamma2: g2(cursor, encoding)?,
            delta2: g2(cursor, encoding)?,
            inputs: g1_vector(cursor, encoding)?,
        })
    }

    /// `vk.bin`: arkworks' verifying key, its points compressed or not.
    fn parse(bytes: &[u8]) -> Result<Self, String> {
        let mut failure = String::new();
        for encoding in [Encoding::Compressed, Encoding::Uncompressed] {
            let mut cursor = Cursor::new(bytes);
            match Self::read(&mut cursor, encoding) {
                Ok(key) if cursor.done() => return Ok(key),
                Ok(_) => failure = "a verifying key with trailing bytes".into(),
                Err(error) => failure = error,
            }
        }
        Err(format!("not an Xark verifying key: {failure}"))
    }
}

/// Xark's proving key: arkworks' layout after Xark's optional header.
struct ProvingKey {
    /// The QAP domain's size when the header states it (Xark's v2 header).
    domain: Option<usize>,
    verifying_key: VerifyingKey,
    beta: G1Affine,
    delta: G1Affine,
    a: Vec<G1Affine>,
    b: Vec<G1Affine>,
    b2: Vec<G2Affine>,
    h: Vec<G1Affine>,
    l: Vec<G1Affine>,
}

impl ProvingKey {
    /// `pk.bin`: `XKPK`, a version (1, or 2 with the domain's size as a
    /// u64), the points' mode (0 compressed, 1 not), then arkworks' key;
    /// or arkworks' compressed key alone, without the header.
    fn parse(bytes: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(bytes);
        let (encoding, domain) = if bytes.get(..4) == Some(PK_MAGIC.as_slice()) {
            cursor.take(4)?;
            let version = cursor.byte()?;
            let encoding = match cursor.byte()? {
                0 => Encoding::Compressed,
                1 => Encoding::Uncompressed,
                mode => return Err(format!("an Xark key of point mode {mode}")),
            };
            let domain = match version {
                1 => None,
                2 => Some(usize::try_from(cursor.u64_le()?).map_err(|_| "a domain past usize")?),
                version => return Err(format!("an Xark key of header version {version}")),
            };
            (encoding, domain)
        } else {
            (Encoding::Compressed, None)
        };
        let verifying_key = VerifyingKey::read(&mut cursor, encoding)?;
        let beta = g1(&mut cursor, encoding)?;
        let delta = g1(&mut cursor, encoding)?;
        let key = Self {
            domain,
            verifying_key,
            beta,
            delta,
            a: g1_vector(&mut cursor, encoding)?,
            b: g1_vector(&mut cursor, encoding)?,
            b2: vector(&mut cursor, encoding.g2(), G2Affine::IDENTITY, |b| {
                ark_g2(b, encoding)
            })?,
            h: g1_vector(&mut cursor, encoding)?,
            l: g1_vector(&mut cursor, encoding)?,
        };
        if !cursor.done() {
            return Err("an Xark proving key with trailing bytes".into());
        }
        Ok(key)
    }
}

/// A coefficient in Xark's row file: signed little-endian bytes (two's
/// complement), reduced mod r.
fn signed_le(bytes: &[u8]) -> Fr {
    let magnitude = Fr::from_le_bytes_mod_order(bytes);
    match bytes.last() {
        Some(last) if last & 0x80 != 0 => {
            magnitude - Fr::from_u64(2).pow(&[8 * bytes.len() as u64])
        }
        _ => magnitude,
    }
}

/// The solver's coefficients: gnark's first five, then each other value
/// once.
struct Coefficients {
    values: Vec<Fr>,
    index: HashMap<[u64; 4], u32>,
}

impl Coefficients {
    fn new() -> Self {
        let mut out = Self {
            values: Vec::new(),
            index: HashMap::new(),
        };
        for value in [
            Fr::ZERO,
            Fr::ONE,
            Fr::from_u64(2),
            -Fr::ONE,
            -Fr::from_u64(2),
        ] {
            out.intern(value);
        }
        out
    }
    fn intern(&mut self, value: Fr) -> u32 {
        let next = self.values.len() as u32;
        let index = *self.index.entry(value.0).or_insert(next);
        if index == next {
            self.values.push(value);
        }
        index
    }
    fn encode(&self) -> Vec<u8> {
        limb_bytes(self.values.iter().map(|value| value.0))
    }
}

/// Xark's row file, read into the solver program's constraint records.
struct Rows {
    /// Public variables (the constant one included): Xark's instance.
    instance: usize,
    /// The assignment's length, the constant one included.
    variables: usize,
    rows: usize,
    /// Each row's record: [row, len(A), len(B), len(C), (coefficient, wire)...].
    records: Vec<u32>,
    coefficients: Coefficients,
}

impl Rows {
    /// `XRS1`, the source's fingerprint, the variables (id, public), the
    /// coefficient pool, the row count, the window and its offsets, and the
    /// rows: three linear combinations each, a constant's pool index and
    /// the terms (variable delta, pool index). Integers are LEB128 but the
    /// offsets (u64 LE).
    fn parse(bytes: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(bytes);
        if cursor.take(4).ok() != Some(ROWS_MAGIC.as_slice()) {
            return Err("not an Xark row file (r1cs.stream)".into());
        }
        let fingerprint = cursor.count()?;
        cursor.take(fingerprint)?;
        // Xark's layout: the constant one at 0, then the public variables
        // in id order, then the others in id order.
        let count = cursor.count()?;
        let mut variables = Vec::with_capacity(count.min(bytes.len()));
        for _ in 0..count {
            let id = u32::try_from(cursor.leb128()?).map_err(|_| "a variable id past u32")?;
            let public = match cursor.byte()? {
                0 => false,
                1 => true,
                _ => return Err("a variable neither public nor not".into()),
            };
            variables.push((id, public));
        }
        variables.sort_unstable_by_key(|&(id, public)| (!public, id));
        let mut position: HashMap<u32, u32> = HashMap::with_capacity(variables.len());
        for (at, &(id, _)) in variables.iter().enumerate() {
            if position.insert(id, at as u32 + 1).is_some() {
                return Err(format!("variable {id} twice in the row file"));
            }
        }
        let instance = 1 + variables.iter().take_while(|(_, public)| *public).count();
        let pool_size = cursor.count()?;
        let mut pool = Vec::with_capacity(pool_size.min(bytes.len()));
        for _ in 0..pool_size {
            let len = cursor.count()?;
            pool.push(signed_le(cursor.take(len)?));
        }
        let rows = cursor.count()?;
        let window = cursor.count()?;
        let windows = cursor.count()?;
        if window == 0 || windows != rows.div_ceil(window) {
            return Err("a row file with a damaged window index".into());
        }
        let mut offsets = Vec::with_capacity(windows.min(bytes.len()));
        for _ in 0..windows {
            offsets.push(cursor.u64_le()?);
        }
        let start = cursor.at;
        let mut coefficients = Coefficients::new();
        let mut records = Vec::new();
        let mut sides: [Vec<(u32, u32)>; 3] = Default::default();
        let mut offsets = offsets.into_iter();
        for row in 0..rows {
            if row % window == 0 && offsets.next() != Some((cursor.at - start) as u64) {
                return Err("a row file whose window index does not match its rows".into());
            }
            for side in &mut sides {
                side.clear();
                let constant = *pool
                    .get(cursor.count()?)
                    .ok_or("a constant outside the pool")?;
                if !constant.is_zero() {
                    side.push((coefficients.intern(constant), 0));
                }
                let mut var = 0u32;
                for _ in 0..cursor.count()? {
                    let delta =
                        u32::try_from(cursor.leb128()?).map_err(|_| "a variable past u32")?;
                    var = var.checked_add(delta).ok_or("a variable past u32")?;
                    let coefficient = *pool
                        .get(cursor.count()?)
                        .ok_or("a coefficient outside the pool")?;
                    let wire = *position.get(&var).ok_or_else(|| {
                        format!("row {row} names variable {var}, which is not declared")
                    })?;
                    if !coefficient.is_zero() {
                        side.push((coefficients.intern(coefficient), wire));
                    }
                }
            }
            records.push(row as u32);
            records.extend(sides.iter().map(|side| side.len() as u32));
            for (coefficient, wire) in sides.iter().flatten() {
                records.extend([*coefficient, *wire]);
            }
        }
        if !cursor.done() {
            return Err("a row file with trailing bytes".into());
        }
        Ok(Self {
            instance,
            variables: variables.len() + 1,
            rows,
            records,
            coefficients,
        })
    }
}

/// A G1 point in the kernel's encoding: each coordinate's Montgomery limbs,
/// little endian.
fn kernel_g1(points: &[G1Affine]) -> Vec<u8> {
    limb_bytes(points.iter().flat_map(|p| [p.x.0, p.y.0]))
}

/// G2 in the kernel's encoding: x.c0, x.c1, y.c0, y.c1.
fn kernel_g2(points: &[G2Affine]) -> Vec<u8> {
    limb_bytes(
        points
            .iter()
            .flat_map(|p| [p.x.c0.0, p.x.c1.0, p.y.c0.0, p.y.c1.0]),
    )
}

/// Each value's Montgomery limbs, little endian: the kernel's encoding of
/// field elements.
fn limb_bytes(values: impl IntoIterator<Item = [u64; 4]>) -> Vec<u8> {
    values
        .into_iter()
        .flatten()
        .flat_map(u64::to_le_bytes)
        .collect()
}

/// The points at `indices`.
fn pick<P: Copy>(points: &[P], indices: &[u32]) -> Vec<P> {
    indices
        .iter()
        .filter_map(|&i| points.get(i as usize).copied())
        .collect()
}

/// gnark's raw G1 encoding: x then y, big endian; zeros at infinity.
fn raw_g1(point: &G1Affine, out: &mut Vec<u8>) {
    for v in [point.x, point.y] {
        out.extend_from_slice(&v.to_be_bytes());
    }
}

/// gnark's raw G2 encoding: x.c1, x.c0, y.c1, y.c0.
fn raw_g2(point: &G2Affine, out: &mut Vec<u8>) {
    for v in [point.x.c1, point.x.c0, point.y.c1, point.y.c0] {
        out.extend_from_slice(&v.to_be_bytes());
    }
}

/// i with its low `bits` bits reversed.
fn bit_reverse(i: usize, bits: u32) -> usize {
    if bits == 0 {
        0
    } else {
        i.reverse_bits() >> (usize::BITS - bits)
    }
}

/// An Xark key's parts, as a dump of them, from its proving key, its
/// verifying key (which must be the proving key's) and the rows it was made
/// for; `metadata` is the application's. A key's input i is Xark's
/// z[i + 1]; its names are `z1`, `z2`, ... The QAP domain has Z's points
/// plus one.
pub fn convert(pk: &[u8], vk: &[u8], rows: &[u8], metadata: &[u8]) -> Result<Dump, String> {
    let key = ProvingKey::parse(pk)?;
    if VerifyingKey::parse(vk)? != key.verifying_key {
        return Err("vk.bin is not the proving key's verifying key".into());
    }
    let rows = Rows::parse(rows)?;
    let (variables, instance) = (rows.variables, rows.instance);
    let witness = variables - instance;
    let shape = [
        ("a_query", key.a.len(), variables),
        ("b_g1_query", key.b.len(), variables),
        ("b_g2_query", key.b2.len(), variables),
        ("l_query", key.l.len(), witness),
        (
            "the verifying key's inputs",
            key.verifying_key.inputs.len(),
            instance,
        ),
    ];
    for (name, points, expected) in shape {
        if points != expected {
            return Err(format!(
                "{name} has {points} points and the rows' circuit needs {expected}: the rows are not the key's"
            ));
        }
    }
    // Xark's domain: the header's, or the h_query's length plus one, as
    // arkworks makes it. zopro's FFT is radix 2: a smooth 3 2^k or 9 2^k
    // domain (`xark setup --domain smooth`) has no zopro key.
    let rows_in_domain = rows.rows + instance;
    let n = key.domain.unwrap_or(key.h.len() + 1);
    if !n.is_power_of_two() || n.trailing_zeros() > Fr::TWO_ADICITY {
        return Err(format!(
            "an Xark key of a {n}-point QAP domain: zopro proves on power-of-two domains only"
        ));
    }
    if key.h.len() + 1 != n && key.h.len() != n {
        return Err(format!(
            "h_query has {} points for a {n}-point domain",
            key.h.len()
        ));
    }
    if n < rows_in_domain {
        return Err(format!("a {n}-point domain for {rows_in_domain} rows"));
    }
    let generator = Fr::root_of_unity(n).ok_or("no root of unity of the domain's size")?;
    // h is of degree n - 2 at most: an n-th point (an h_query of n points)
    // weighs a zero coefficient, and goes.
    let bits = n.trailing_zeros();
    let z: Vec<G1Affine> = (0..n - 1)
        .map(|j| {
            key.h
                .get(bit_reverse(j, bits))
                .copied()
                .ok_or("an h_query shorter than its domain")
        })
        .collect::<Result<_, _>>()?;
    // The variables whose points are not all at infinity.
    let a_indices: Vec<u32> = (0..variables as u32)
        .zip(&key.a)
        .filter(|(_, p)| !p.is_identity())
        .map(|(i, _)| i)
        .collect();
    let b_indices: Vec<u32> = (0..variables as u32)
        .zip(key.b.iter().zip(&key.b2))
        .filter(|(_, (b, b2))| !b.is_identity() || !b2.is_identity())
        .map(|(i, _)| i)
        .collect();
    let a = pick(&key.a, &a_indices);
    let b = pick(&key.b, &b_indices);
    let b2 = pick(&key.b2, &b_indices);
    // The solver program: every variable an input, Xark's rows, then the
    // input-consistency rows.
    let mut program: Vec<u32> = vec![
        2,
        variables as u32,
        variables as u32,
        instance as u32,
        rows_in_domain as u32,
        0,
        a_indices.len() as u32,
        b_indices.len() as u32,
    ];
    program.extend(&a_indices);
    program.extend(&b_indices);
    program.extend(&rows.records);
    for i in 0..instance as u32 {
        program.extend([rows.rows as u32 + i, 1, 0, 0, 1, i]);
    }
    let program: Vec<u8> = program.into_iter().flat_map(u32::to_le_bytes).collect();
    let vk = &key.verifying_key;
    let mut verifying_key = Vec::with_capacity(588 + 64 * vk.inputs.len());
    raw_g1(&vk.alpha, &mut verifying_key);
    raw_g1(&key.beta, &mut verifying_key);
    raw_g2(&vk.beta2, &mut verifying_key);
    raw_g2(&vk.gamma2, &mut verifying_key);
    raw_g1(&key.delta, &mut verifying_key);
    raw_g2(&vk.delta2, &mut verifying_key);
    verifying_key.extend_from_slice(&(vk.inputs.len() as u32).to_be_bytes());
    for point in &vk.inputs {
        raw_g1(point, &mut verifying_key);
    }
    verifying_key.extend_from_slice(&[0; 8]);
    let mut fixed = kernel_g1(&[vk.alpha, key.beta, key.delta]);
    fixed.extend(kernel_g2(&[vk.beta2, vk.delta2]));
    let names = |range: std::ops::Range<usize>| range.map(|i| format!("z{i}")).collect();
    Ok(Dump {
        head: Head {
            public: instance as u32,
            secret: witness as u32,
            constraints: rows_in_domain as u32,
            fixed,
            domain: limb_bytes([generator.0, Fr::from_u64(COSET).0]),
            verifying_key,
        },
        public_names: names(1..instance),
        secret_names: names(instance..variables),
        metadata: metadata.to_vec(),
        vectors: [
            kernel_g1(&a),
            kernel_g1(&b),
            kernel_g1(&key.l),
            kernel_g1(&z),
            kernel_g2(&b2),
        ],
        program,
        coefficients: rows.coefficients.encode(),
    })
}

/// The zopro key file, named `name`, of an Xark key: `convert`, then
/// `artifact::pack`.
pub fn pack(pk: &[u8], vk: &[u8], rows: &[u8], name: &str) -> Result<Vec<u8>, String> {
    crate::artifact::pack(&convert(pk, vk, rows, &[])?.parts(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// arkworks' encoding of a G1 point, as its serializer writes it.
    fn ark_bytes_g1(point: &G1Affine, encoding: Encoding) -> Vec<u8> {
        let le = |v: &Fq| {
            let mut bytes = v.to_be_bytes();
            bytes.reverse();
            bytes
        };
        let (flags, x, y) = if point.is_identity() {
            (INFINITY, Fq::ZERO, Fq::ZERO)
        } else {
            (
                if point.y.is_largest() { NEGATIVE } else { 0 },
                point.x,
                point.y,
            )
        };
        let mut out = le(&x).to_vec();
        if encoding == Encoding::Uncompressed {
            out.extend(le(&y));
        }
        if let Some(last) = out.last_mut() {
            *last |= flags;
        }
        out
    }

    #[test]
    fn g1_points_decode_in_both_encodings() {
        let generator = G1Affine::GENERATOR;
        let points = [
            generator,
            G1Affine::new(generator.x, -generator.y),
            (generator.to_projective() * Fr::from_u64(12345)).to_affine(),
            G1Affine::IDENTITY,
        ];
        for encoding in [Encoding::Compressed, Encoding::Uncompressed] {
            for point in points {
                let bytes = ark_bytes_g1(&point, encoding);
                assert_eq!(ark_g1(&bytes, encoding), Ok(point));
            }
        }
    }

    #[test]
    fn malformed_points_are_refused() {
        let mut off_curve = ark_bytes_g1(&G1Affine::GENERATOR, Encoding::Uncompressed);
        if let Some(byte) = off_curve.first_mut() {
            *byte ^= 1;
        }
        assert!(ark_g1(&off_curve, Encoding::Uncompressed).is_err());
        let mut infinity_with_x = ark_bytes_g1(&G1Affine::IDENTITY, Encoding::Compressed);
        if let Some(byte) = infinity_with_x.first_mut() {
            *byte = 1;
        }
        assert!(ark_g1(&infinity_with_x, Encoding::Compressed).is_err());
        assert!(ark_g1(&[0xff; 32], Encoding::Compressed).is_err());
        assert!(ark_g1(&[0; 31], Encoding::Compressed).is_err());
    }

    #[test]
    fn signed_coefficients_reduce_mod_r() {
        assert_eq!(signed_le(&[]), Fr::ZERO);
        assert_eq!(signed_le(&[5]), Fr::from_u64(5));
        assert_eq!(signed_le(&[0xff]), -Fr::ONE);
        assert_eq!(signed_le(&[0x80, 0x00]), Fr::from_u64(128));
        assert_eq!(signed_le(&[0x00, 0xff]), -Fr::from_u64(256));
    }

    #[test]
    fn leb128_reads_and_bounds() {
        let mut cursor = Cursor::new(&[0x7f, 0x80, 0x01, 0xff]);
        assert_eq!(cursor.leb128(), Ok(127));
        assert_eq!(cursor.leb128(), Ok(128));
        assert!(cursor.leb128().is_err());
        let mut long = Cursor::new(&[0xff; 11]);
        assert!(long.leb128().is_err());
    }

    #[test]
    fn coefficients_start_with_gnarks_five() {
        let mut coefficients = Coefficients::new();
        assert_eq!(coefficients.intern(-Fr::ONE), 3);
        assert_eq!(coefficients.intern(Fr::from_u64(7)), 5);
        assert_eq!(coefficients.intern(Fr::from_u64(7)), 5);
        assert_eq!(coefficients.encode().len(), 6 * 32);
    }

    #[test]
    fn bit_reversal_is_an_involution_that_fixes_the_last_index() {
        for bits in 0..6 {
            let n = 1usize << bits;
            for i in 0..n {
                assert_eq!(bit_reverse(bit_reverse(i, bits), bits), i);
            }
            assert_eq!(bit_reverse(n - 1, bits), n - 1);
        }
    }

    #[test]
    fn what_is_not_an_xark_artifact_is_refused() {
        assert!(ProvingKey::parse(b"XKPK\x03\x00").is_err());
        assert!(ProvingKey::parse(b"XKPK\x02\x07").is_err());
        assert!(VerifyingKey::parse(&[0; 10]).is_err());
        assert!(Rows::parse(b"XRS0").is_err());
    }
}
