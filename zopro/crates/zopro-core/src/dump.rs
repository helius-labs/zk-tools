//! A dump: a gnark proving key's parts as `tools/gnark` writes them, one
//! file each in a directory, for `pack` and the benchmarks. `head` is a
//! 16-byte magic, the public (the constant one included) and secret input
//! counts, the constraint count and the verifying key's length (u32s,
//! little endian), the fixed points and the FFT domain in the kernel's
//! encoding, and gnark's raw verifying key; `a`, `b`, `k`, `z` and `b2` are
//! the vectors in the kernel's encoding; `program` and `coefficients` the
//! solver's; `public-names` and `secret-names` one name per line; and
//! `metadata`, when there is one, the application's.
use std::{fs, io, path::Path};

use crate::{artifact::Parts, groth16::FixedPoints};

const MAGIC: &[u8; 16] = b"zopro-dump\0\0\0\0\0\0";

/// A dump's head.
pub struct Head {
    pub public: u32,
    pub secret: u32,
    pub constraints: u32,
    /// The kernel's encoding, 448 bytes.
    pub fixed: Vec<u8>,
    /// The kernel's encoding, 64 bytes.
    pub domain: Vec<u8>,
    /// gnark's raw verifying key.
    pub verifying_key: Vec<u8>,
}

impl Head {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 544 || &bytes[..16] != MAGIC {
            return Err("not a dump head");
        }
        let word = |i: usize| u32::from_le_bytes(bytes[16 + 4 * i..20 + 4 * i].try_into().unwrap());
        let verifying_key = bytes
            .get(544..544 + word(3) as usize)
            .ok_or("truncated dump head")?;
        Ok(Self {
            public: word(0),
            secret: word(1),
            constraints: word(2),
            fixed: bytes[32..480].to_vec(),
            domain: bytes[480..544].to_vec(),
            verifying_key: verifying_key.to_vec(),
        })
    }

    pub fn read(dir: &Path) -> io::Result<Self> {
        Self::parse(&fs::read(dir.join("head"))?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn fixed_points(&self) -> FixedPoints {
        FixedPoints::decode(&self.fixed).expect("448 bytes")
    }
}

/// Everything `pack` takes from a dump.
pub struct Dump {
    pub head: Head,
    pub vectors: [Vec<u8>; 5],
    pub program: Vec<u8>,
    pub coefficients: Vec<u8>,
    pub public_names: Vec<String>,
    pub secret_names: Vec<String>,
    pub metadata: Vec<u8>,
}

impl Dump {
    pub fn read(dir: &Path) -> io::Result<Self> {
        let read = |name: &str| fs::read(dir.join(name));
        let lines = |name: &str| -> io::Result<Vec<String>> {
            Ok(String::from_utf8_lossy(&read(name)?)
                .lines()
                .map(str::to_owned)
                .collect())
        };
        let metadata = match read("metadata") {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e),
        };
        Ok(Self {
            head: Head::read(dir)?,
            vectors: [read("a")?, read("b")?, read("k")?, read("z")?, read("b2")?],
            program: read("program")?,
            coefficients: read("coefficients")?,
            public_names: lines("public-names")?,
            secret_names: lines("secret-names")?,
            metadata,
        })
    }

    /// The parts of a key named `name`.
    pub fn parts<'a>(&'a self, name: &'a str) -> Parts<'a> {
        let [a, b, k, z, b2] = &self.vectors;
        Parts {
            name,
            metadata: &self.metadata,
            public: self.head.public,
            secret: self.head.secret,
            constraints: self.head.constraints,
            fixed: &self.head.fixed,
            domain: &self.head.domain,
            verifying_key: &self.head.verifying_key,
            public_names: &self.public_names,
            secret_names: &self.secret_names,
            a,
            b,
            k,
            z,
            b2,
            program: &self.program,
            coefficients: &self.coefficients,
        }
    }
}
