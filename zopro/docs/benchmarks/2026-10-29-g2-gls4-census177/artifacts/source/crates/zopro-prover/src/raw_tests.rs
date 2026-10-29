use super::*;

fn fixed_head() -> Vec<u8> {
    let mut head = vec![0u8; 564];
    head[..16].copy_from_slice(b"zopro-key\0\0\0\0\0\0\0");
    head[16..20].copy_from_slice(&3u32.to_le_bytes());
    head[20..24].copy_from_slice(&1u32.to_le_bytes());
    head
}

#[test]
fn rejects_overflowing_and_truncated_lengths() {
    for length in [u64::MAX, u32::MAX as u64 + 1, 1] {
        let mut raw = MAGIC.to_vec();
        raw.extend_from_slice(&length.to_le_bytes());
        assert!(parse(&raw).is_none());
    }
    let mut head = fixed_head();
    head.extend_from_slice(&[0; 48]);
    let mut raw = Vec::new();
    write_head(&mut raw, &head).unwrap();
    raw.extend_from_slice(&u64::MAX.to_le_bytes());
    assert!(parse(&raw).is_none());
    raw.truncate(raw.len() - 8);
    write_tail(&mut raw, &[]).unwrap();
    assert!(parse(&raw).is_some());
    raw.push(0);
    assert!(parse(&raw).is_none());
}

#[cfg(all(unix, target_pointer_width = "64"))]
#[test]
fn sparse_images_cross_four_gib_without_truncation() {
    use std::{fs::OpenOptions, io::{Seek, SeekFrom}};
    let large = (1u64 << 32) + 17;
    for large_header in [false, true] {
        let path = std::env::temp_dir().join(format!("zopro-wide-image-{}-{}-{}",
            std::process::id(), large_header,
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let mut file = OpenOptions::new().read(true).write(true).create_new(true).open(&path).unwrap();
        // Unlink while open: even a failed assertion cannot leave a sparse artifact.
        std::fs::remove_file(&path).unwrap();
        let head_len = 564 + 48 + if large_header { large } else { 0 };
        file.write_all(MAGIC).unwrap();
        file.write_all(&head_len.to_le_bytes()).unwrap();
        file.write_all(&fixed_head()).unwrap();
        file.write_all(&0u64.to_le_bytes()).unwrap(); // name
        file.write_all(&(if large_header { large } else { 0 }).to_le_bytes()).unwrap();
        if large_header { file.seek(SeekFrom::Current(large as i64)).unwrap(); }
        file.write_all(&[0; 32]).unwrap(); // remaining four blobs
        let aligned = (24 + head_len).next_multiple_of(64);
        file.seek(SeekFrom::Start(aligned)).unwrap();
        let solver_len = if large_header { 0 } else { large };
        file.write_all(&solver_len.to_le_bytes()).unwrap();
        file.set_len(aligned + 8 + solver_len).unwrap();
        // SAFETY: the file stays open and is never modified while mapped.
        let map = unsafe { memmap2::Mmap::map(&file) }.unwrap();
        let (artifact, ranges, solver) = parse(&map).unwrap();
        assert!(ranges.iter().all(|&(at, len)| at == aligned as usize && len == 0));
        assert_eq!(artifact.metadata.len() as u64, if large_header { large } else { 0 });
        assert_eq!(solver.len() as u64, solver_len);
        assert!(parse(&map[..map.len() - 1]).is_none());
        struct Counter { total: usize, prefix: Vec<u8> }
        impl Write for Counter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                if self.total == 0 { self.prefix.extend_from_slice(bytes); }
                self.total += bytes.len();
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> { Ok(()) }
        }
        let mut counter = Counter { total: 0, prefix: Vec::new() };
        write_tail(&mut counter, solver).unwrap();
        assert_eq!(counter.prefix, solver_len.to_le_bytes());
        assert_eq!(counter.total as u64, 8 + solver_len);
    }
}
