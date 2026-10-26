use crate::Result;

#[derive(Debug, Clone, Copy)]
pub struct Load {
    pub offset: u64,
    pub address: u64,
    pub file_size: u64,
    pub memory_size: u64,
}

pub struct Elf<'a> {
    bytes: &'a [u8],
    loads: Vec<Load>,
}

fn number(bytes: &[u8], offset: usize, width: usize) -> Result<u64> {
    let end = offset.checked_add(width).ok_or("ELF offset overflow")?;
    let slice = bytes.get(offset..end).ok_or("truncated ELF header")?;
    let mut value = 0;
    for (i, &b) in slice.iter().enumerate() {
        value |= u64::from(b) << (8 * i);
    }
    Ok(value)
}

impl<'a> Elf<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        if bytes.len() < 64 || &bytes[..4] != b"\x7fELF" || bytes[4..7] != [2, 1, 1] {
            return Err("expected ELF64 little-endian version 1".into());
        }
        if ![2, 3].contains(&number(bytes, 16, 2)?) || number(bytes, 18, 2)? != 62 {
            return Err("expected x86-64 executable or PIE".into());
        }
        let offset: usize = number(bytes, 32, 8)?.try_into()?;
        let stride: usize = number(bytes, 54, 2)?.try_into()?;
        let count: usize = number(bytes, 56, 2)?.try_into()?;
        if stride < 56 || count == 0 || count == 65535 {
            return Err("unsupported ELF program-header table".into());
        }
        let mut loads = Vec::new();
        for i in 0..count {
            let p = offset
                .checked_add(i.checked_mul(stride).ok_or("ELF table overflow")?)
                .ok_or("ELF table overflow")?;
            if number(bytes, p, 4)? != 1 || number(bytes, p + 4, 4)? & 1 == 0 {
                continue;
            }
            let load = Load {
                offset: number(bytes, p + 8, 8)?,
                address: number(bytes, p + 16, 8)?,
                file_size: number(bytes, p + 32, 8)?,
                memory_size: number(bytes, p + 40, 8)?,
            };
            if load.file_size > load.memory_size
                || load
                    .offset
                    .checked_add(load.file_size)
                    .is_none_or(|end| end > bytes.len() as u64)
                || load.address.checked_add(load.memory_size).is_none()
                || load.offset % 4096 != load.address % 4096
            {
                return Err("invalid executable PT_LOAD bounds/alignment".into());
            }
            loads.push(load);
        }
        if loads.is_empty() {
            return Err("ELF has no executable PT_LOAD".into());
        }
        Ok(Self { bytes, loads })
    }

    pub fn load_bias(&self, start: u64, file_offset: u64) -> Result<u64> {
        let mut biases = Vec::new();
        for load in &self.loads {
            let page_start = load.offset & !4095;
            let end = load
                .offset
                .checked_add(load.memory_size)
                .ok_or("ELF extent overflow")?;
            let page_end = end.checked_add(4095).ok_or("ELF page overflow")? & !4095;
            if (page_start..page_end).contains(&file_offset) {
                let virtual_address =
                    i128::from(load.address) + i128::from(file_offset) - i128::from(load.offset);
                let bias: u64 = (i128::from(start) - virtual_address).try_into()?;
                biases.push(bias);
            }
        }
        biases.sort_unstable();
        biases.dedup();
        if biases.len() != 1 {
            return Err("mapping has no unique executable PT_LOAD bias".into());
        }
        Ok(biases[0])
    }

    pub fn bind(&self, address: u64, actual: &[u8]) -> Result<()> {
        if actual.is_empty() || actual.len() > 15 {
            return Err("invalid x86 instruction byte length".into());
        }
        for load in &self.loads {
            if let Some(delta) = address.checked_sub(load.address) {
                if delta
                    .checked_add(actual.len() as u64)
                    .is_some_and(|end| end <= load.file_size)
                {
                    let offset: usize = load
                        .offset
                        .checked_add(delta)
                        .ok_or("ELF offset overflow")?
                        .try_into()?;
                    if self.bytes.get(offset..offset + actual.len()) == Some(actual) {
                        return Ok(());
                    }
                    return Err(format!("instruction bytes differ from ELF at {address:x}").into());
                }
            }
        }
        Err(format!("instruction outside file-backed executable segment: {address:x}").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn image() -> Vec<u8> {
        let mut b = vec![0; 0x4000];
        b[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
        for (p, v, n) in [
            (16, 3u64, 2),
            (18, 62, 2),
            (32, 64, 8),
            (54, 56, 2),
            (56, 1, 2),
            (64, 1, 4),
            (68, 5, 4),
            (72, 0x1200, 8),
            (80, 0x2200, 8),
            (96, 0x1000, 8),
            (104, 0x1000, 8),
        ] {
            b[p..p + n].copy_from_slice(&v.to_le_bytes()[..n]);
        }
        b[0x1345..0x1348].copy_from_slice(&[0x48, 0x89, 0xd8]);
        b
    }
    #[test]
    fn pie_file_and_virtual_offsets_are_distinct() {
        let b = image();
        let e = Elf::parse(&b).unwrap();
        assert_eq!(e.load_bias(0x70002000, 0x1000).unwrap(), 0x70000000);
        e.bind(0x2345, &[0x48, 0x89, 0xd8]).unwrap();
        assert!(e.bind(0x1345, &[0x48, 0x89, 0xd8]).is_err());
        assert!(e.bind(0x2345, &[0x48, 0x89, 0xd9]).is_err());
        assert!(e.bind(0x3200, &[0]).is_err());
        assert!(e.load_bias(0x70000000, 0x9000).is_err());
    }
    #[test]
    fn malformed_and_non_x86_images_are_rejected() {
        assert!(Elf::parse(&image()[..63]).is_err());
        for (p, v) in [(4, 1), (5, 2), (18, 183), (54, 0), (56, 0)] {
            let mut b = image();
            b[p] = v;
            assert!(Elf::parse(&b).is_err());
        }
        let mut b = image();
        b[32..40].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(Elf::parse(&b).is_err());
        let mut b = image();
        b[96..104].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(Elf::parse(&b).is_err());
    }
}
