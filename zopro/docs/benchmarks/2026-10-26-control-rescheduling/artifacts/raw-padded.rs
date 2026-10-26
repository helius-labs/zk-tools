//! Decode only this experiment's pinned little-endian perf tracepoint layout.
//! Task names, syscall arguments and addresses are never emitted.
use serde_json::json;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{BufWriter, Read, Seek, SeekFrom, Write},
};
fn u16at(b: &[u8], n: usize) -> u16 {
    u16::from_le_bytes(b[n..n + 2].try_into().unwrap())
}
fn u32at(b: &[u8], n: usize) -> u32 {
    u32::from_le_bytes(b[n..n + 4].try_into().unwrap())
}
fn u64at(b: &[u8], n: usize) -> u64 {
    u64::from_le_bytes(b[n..n + 8].try_into().unwrap())
}
fn state(x: u64) -> String {
    let mut s = String::new();
    for (i, c) in "SDTtXZPI".chars().enumerate() {
        if x & (1 << i) != 0 {
            if !s.is_empty() {
                s.push('|');
            }
            s.push(c);
        }
    }
    if s.is_empty() {
        s.push('R');
    }
    assert_eq!(x & !0x1ff, 0, "unknown scheduler state bits");
    if x & 0x100 != 0 {
        s.push('+');
    }
    s
}
fn sample(
    b: &[u8],
    attrs: &BTreeMap<u64, (u32, u64, u64)>,
) -> Option<(u32, u32, u32, u64, String)> {
    let id = u64at(b, 8);
    let &(ty, config, mask) = attrs.get(&id).expect("known event identifier");
    if ty != 2 {
        assert_eq!(mask, 0x10087);
        return None;
    }
    assert_eq!(mask, 0x10587, "exact recorded tracepoint sample layout");
    assert!(b.len() >= 60);
    let pid = u32at(b, 24);
    let tid = u32at(b, 28);
    let ns = u64at(b, 32);
    let cpu = u32at(b, 40);
    let len = u32at(b, 56) as usize;
    assert!(60 + len <= b.len());
    let raw = &b[60..60 + len];
    assert_eq!(u16at(raw, 0) as u64, config);
    let kind = match config {
        330 => {
            // perf pads RAW so its u32 length plus payload aligns to eight bytes.
            assert!(raw.len() == 64 || raw.len() == 68);
            format!(
                "sched:sched_switch: prev_pid={} prev_state={} ==> next_pid={}",
                u32at(raw, 24),
                state(u64at(raw, 32)),
                u32at(raw, 56)
            )
        }
        289 => "syscalls:sys_enter_sched_yield: ".into(),
        288 => "syscalls:sys_exit_sched_yield: ".into(),
        439 => "syscalls:sys_enter_futex: ".into(),
        438 => "syscalls:sys_exit_futex: ".into(),
        _ => panic!("undeclared tracepoint"),
    };
    Some((pid, tid, cpu, ns, kind))
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4, "raw PERF_DATA NUMERIC_EVENTS METADATA");
    let mut f = File::open(&a[1]).unwrap();
    let mut header = [0u8; 104];
    f.read_exact(&mut header).unwrap();
    assert_eq!(&header[..8], b"PERFILE2");
    let stride = u64at(&header, 16);
    let attr_off = u64at(&header, 24);
    let attr_len = u64at(&header, 32);
    let data_off = u64at(&header, 40);
    let data_len = u64at(&header, 48);
    assert!(stride >= 112 && stride <= 512 && attr_len % stride == 0);
    let mut attrs = BTreeMap::new();
    let mut configs = Vec::new();
    for i in 0..attr_len / stride {
        f.seek(SeekFrom::Start(attr_off + i * stride)).unwrap();
        let mut b = vec![0; stride as usize];
        f.read_exact(&mut b).unwrap();
        let ty = u32at(&b, 0);
        let size = u32at(&b, 4);
        assert!(size >= 96);
        let config = u64at(&b, 8);
        let mask = u64at(&b, 24);
        assert_eq!(u32at(&b, 92), 1, "CLOCK_MONOTONIC");
        let ids_off = u64at(&b, b.len() - 16);
        let ids_len = u64at(&b, b.len() - 8);
        assert_eq!(ids_len % 8, 0);
        f.seek(SeekFrom::Start(ids_off)).unwrap();
        let mut ids = vec![0; ids_len as usize];
        f.read_exact(&mut ids).unwrap();
        for chunk in ids.chunks_exact(8) {
            assert!(attrs.insert(u64at(chunk, 0), (ty, config, mask)).is_none());
        }
        configs.push(json!({"type":ty,"config":config,"sample_mask":mask}));
    }
    assert_eq!(configs.len(), 6);
    f.seek(SeekFrom::Start(data_off)).unwrap();
    let mut out = BufWriter::new(File::create(&a[2]).unwrap());
    let mut records = 0u64;
    let mut samples = 0u64;
    let mut lost = 0u64;
    let mut pos = 0;
    while pos < data_len {
        let mut b = vec![0u8; 8];
        f.read_exact(&mut b).unwrap();
        let ty = u32at(&b, 0);
        let size = u16at(&b, 6) as usize;
        assert!(size >= 8 && pos + size as u64 <= data_len);
        b.resize(size, 0);
        f.read_exact(&mut b[8..]).unwrap();
        match ty {
            9 => {
                if let Some((pid, tid, cpu, ns, kind)) = sample(&b, &attrs) {
                    writeln!(
                        out,
                        "{pid}/{tid} [{cpu:03}] {}.{:09}: {kind}",
                        ns / 1_000_000_000,
                        ns % 1_000_000_000
                    )
                    .unwrap();
                    samples += 1;
                }
            }
            2 => lost += u64at(&b, 16),
            13 => lost += u64at(&b, 8),
            _ => {}
        }
        pos += size as u64;
        records += 1;
    }
    out.flush().unwrap();
    fs::write(&a[3],serde_json::to_vec_pretty(&json!({"records":records,"tracepoint_samples":samples,"lost_samples":lost,"sample_layout_and_event_ids_checked":true,"clock":"CLOCK_MONOTONIC","configs":configs,"sensitive_payloads_emitted":false})).unwrap()).unwrap();
    println!("{samples} tracepoint samples; {lost} lost samples");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scheduler_state_retains_preemption() {
        assert_eq!(state(0), "R");
        assert_eq!(state(0x100), "R+");
        assert_eq!(state(1), "S");
        assert_eq!(state(0x101), "S+");
    }
    #[test]
    fn exact_sample_layout_and_pid() {
        let mut b = vec![0u8; 124];
        let put32 = |b: &mut [u8], n, v: u32| b[n..n + 4].copy_from_slice(&v.to_le_bytes());
        let put64 = |b: &mut [u8], n, v: u64| b[n..n + 8].copy_from_slice(&v.to_le_bytes());
        put64(&mut b, 8, 12);
        put32(&mut b, 24, 42);
        put32(&mut b, 28, 43);
        put64(&mut b, 32, 1234567890);
        put32(&mut b, 40, 2);
        put32(&mut b, 56, 64);
        b[60..62].copy_from_slice(&330u16.to_le_bytes());
        put32(&mut b, 84, 43);
        put64(&mut b, 92, 0x100);
        put32(&mut b, 116, 0);
        let e = sample(&b, &BTreeMap::from([(12, (2, 330, 0x10587))])).unwrap();
        assert_eq!((e.0, e.1, e.2, e.3), (42, 43, 2, 1234567890));
        assert!(e.4.contains("prev_state=R+"));
        assert!(e.4.contains("next_pid=0"));
    }
    #[test]
    #[should_panic(expected = "exact recorded tracepoint sample layout")]
    fn unexpected_sample_layout_rejected() {
        let mut b = vec![0u8; 16];
        b[8..16].copy_from_slice(&12u64.to_le_bytes());
        sample(&b, &BTreeMap::from([(12, (2, 330, 0x10087))]));
    }
}
