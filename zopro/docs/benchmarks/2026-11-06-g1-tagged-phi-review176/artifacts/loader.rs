use zopro_core::{Image,Points,Key,artifact::Artifact};
const ALIGN:usize=64;
fn padding(at: usize) -> usize {
    (ALIGN - at % ALIGN) % ALIGN
}
pub const FORMAT: &str = "zopro-basis-v2";
pub(super) fn is_image(bytes: &[u8]) -> bool {
    [BASIS_MAGIC, EULER_MAGIC, LEGACY_BASIS_MAGIC, LEGACY_EULER_MAGIC]
        .iter().any(|magic| bytes.starts_with(*magic))
}

const EULER_MAGIC: &[u8; 16] = b"zopro-euler-v2\0\0";
const BASIS_MAGIC: &[u8; 16] = b"zopro-basis-v2\0\0";
const LEGACY_EULER_MAGIC: &[u8; 16] = b"zopro-euler-v1\0\0";
const LEGACY_BASIS_MAGIC: &[u8; 16] = b"zopro-basis-v1\0\0";


struct Reader<'a> {
    raw: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        let out = self.raw.get(self.at..self.at.checked_add(len)?)?;
        self.at += len;
        Some(out)
    }

    fn length(&mut self, wide: bool) -> Option<usize> {
        if wide {
            usize::try_from(u64::from_le_bytes(self.take(8)?.try_into().ok()?)).ok()
        } else {
            Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?) as usize)
        }
    }
}

pub fn load_key(image: Image) -> Result<Key, String> {
        use zopro_core::{groth16::PreparedFixedPoints, Key};
        let raw = (*image).as_ref();
        let euler = raw.starts_with(EULER_MAGIC) || raw.starts_with(LEGACY_EULER_MAGIC);
        let wide = raw.starts_with(BASIS_MAGIC) || raw.starts_with(EULER_MAGIC);
        if !is_image(raw) {
            return Err("not a CPU basis image".into());
        }
        let mut reader = Reader { raw, at: 16 };
        let head_len = reader.length(wide).ok_or("basis header")?;
        let head = reader.take(head_len).ok_or("basis head")?;
        let (artifact, end) = Artifact::head(head)?;
        if end != head.len() {
            return Err("basis header tail".into());
        }
        let pad = reader.take(padding(reader.at)).ok_or("basis padding")?;
        if pad.iter().any(|&b| b != 0) {
            return Err("basis nonzero padding".into());
        }
        let meta = reader.take(128).ok_or("basis metadata")?;
        if meta[80..].iter().any(|&b| b != 0) {
            return Err("basis reserved metadata".into());
        }
        let ns = usize::try_from(u64::from_le_bytes(meta[..8].try_into().unwrap()))
            .map_err(|_| "basis slots overflow")?;
        let np = usize::try_from(u64::from_le_bytes(meta[8..16].try_into().unwrap()))
            .map_err(|_| "basis public overflow")?;
        let words = core::array::from_fn(|i| {
            u64::from_le_bytes(meta[16 + i * 8..24 + i * 8].try_into().unwrap())
        });
        zopro_bn254::Fr::from_canonical(words).ok_or("basis scale outside field")?;
        let scale = zopro_bn254::Fr::from_montgomery(words);
        if scale.is_zero() {
            return Err("zero basis scale".into());
        }
        let digest = meta[48..80].try_into().unwrap();
        let [a, b, k, z, b2] = artifact.points.map(|n| n as usize);
        let n = z.checked_add(1).ok_or("basis domain overflow")?;
        if np != artifact.public as usize || !n.is_power_of_two() || n < 8 || b != b2 {
            return Err("basis key dimensions".into());
        }
        let mut range = |points: usize, size: usize| -> Result<(usize, usize), String> {
            let bytes = points.checked_mul(size).ok_or("basis range overflow")?;
            let at = reader.at;
            reader.take(bytes).ok_or("truncated basis points")?;
            Ok((at, bytes))
        };
        let ar = range(a, 64)?;
        let br = range(b, 64)?;
        let kr = range(k.checked_add(ns).ok_or("basis K overflow")?, 64)?;
        let pr = range(n, 64)?;
        let b2r = range(b2, 128)?;
        let public = range(np, 64)?;
        let slot_bytes = reader
            .take(ns.checked_mul(8).ok_or("slot length overflow")?)
            .ok_or("truncated basis slots")?;
        let slots = slot_bytes
            .chunks_exact(8)
            .map(|b| {
                usize::try_from(u64::from_le_bytes(b.try_into().unwrap()))
                    .map_err(|_| "slot index overflow")
            })
            .collect::<Result<Vec<_>, _>>()?;
        let pad = reader
            .take(padding(reader.at))
            .ok_or("basis solver padding")?;
        if pad.iter().any(|&b| b != 0) {
            return Err("basis nonzero solver padding".into());
        }
        let len = reader.length(wide).ok_or("basis solver length")?;
        let solver = reader.take(len).ok_or("basis solver")?;
        if reader.at != raw.len() {
            return Err("basis trailing bytes".into());
        }
        let points = Points::Mapped(&image, [ar, br, (kr.0, k * 64), (pr.0, z * 64), b2r]);
        let build = if euler {Key::build_cpu_euler_basis} else {Key::build_cpu_basis};
        let key = build(
            points,
            artifact.domain,
            &image,
            pr,
            kr,
            public,
            slots,
            scale,
            digest,
            solver,
        )?;

Ok(key)
}
