use super::elf::Elf;
use crate::Result;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
pub struct Sample {
    pub pid: u32,
    pub tid: u32,
    pub period: u64,
    pub ip: u64,
}

pub fn sample(line: &str) -> Result<Option<Sample>> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return Ok(None);
    }
    let f = line.split_whitespace().collect::<Vec<_>>();
    if f.len() < 4 {
        return Err(format!("expected perf pid,tid,period,ip,sym: {line}").into());
    }
    let (pid, tid) = f[0].split_once('/').ok_or("missing sample pid/tid")?;
    Ok(Some(Sample {
        pid: pid.parse()?,
        tid: tid.parse()?,
        period: f[1].parse()?,
        ip: u64::from_str_radix(f[2], 16)?,
    }))
}

pub struct Mapping {
    pub bias: u64,
    pub intervals: Vec<(u64, u64)>,
}
impl Mapping {
    pub fn parse(text: &str, pid: u32, binary: &str, elf: &Elf<'_>) -> Result<Self> {
        let mut intervals = BTreeSet::new();
        let mut biases = BTreeSet::new();
        for line in text.lines() {
            if line.contains("PERF_RECORD_LOST") {
                return Err("lost records in capture metadata".into());
            }
            let Some((_, event)) = line.split_once("PERF_RECORD_MMAP2 ") else {
                continue;
            };
            let (ids, record) = event.split_once(": [").ok_or("malformed MMAP2 ids")?;
            let (mapped_pid, _) = ids.split_once('/').ok_or("malformed MMAP2 pid")?;
            if mapped_pid.parse::<u32>()? != pid {
                continue;
            }
            let (body, tail) = record.split_once("]: ").ok_or("malformed MMAP2 extent")?;
            let (permission, path) = tail.split_once(' ').ok_or("malformed MMAP2 filename")?;
            if !permission.contains('x') || path.trim() != binary {
                continue;
            }
            let (start, body) = body.split_once('(').ok_or("malformed MMAP2 address")?;
            let (size, body) = body.split_once(") @ ").ok_or("malformed MMAP2 length")?;
            let offset = body
                .split_whitespace()
                .next()
                .ok_or("missing MMAP2 file offset")?;
            let hex = |s: &str| u64::from_str_radix(s.trim().trim_start_matches("0x"), 16);
            let start = hex(start)?;
            let size = hex(size)?;
            let offset = hex(offset)?;
            if size == 0 {
                return Err("empty executable mapping".into());
            }
            let end = start.checked_add(size).ok_or("mapping extent overflow")?;
            biases.insert(elf.load_bias(start, offset)?);
            intervals.insert((start, end));
        }
        if biases.len() != 1 || intervals.is_empty() {
            return Err("no unique recorded executable mapping for sample PID".into());
        }
        Ok(Self {
            bias: *biases.first().unwrap(),
            intervals: intervals.into_iter().collect(),
        })
    }
    pub fn address(&self, ip: u64) -> Result<Option<u64>> {
        if self.intervals.iter().any(|&(a, b)| (a..b).contains(&ip)) {
            Ok(Some(
                ip.checked_sub(self.bias)
                    .ok_or("IP below executable load bias")?,
            ))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_format_rejects_metadata_and_keeps_periods_and_threads() {
        let s = sample(" 17/21 1234 00007fab foo").unwrap().unwrap();
        assert_eq!((s.pid, s.tid, s.period, s.ip), (17, 21, 1234, 0x7fab));
        assert!(sample("# metadata").unwrap().is_none());
        assert!(sample("17 1234 7fab foo").is_err());
        assert_eq!(
            sample("17/21 1 7fab std::foo<int, int>")
                .unwrap()
                .unwrap()
                .ip,
            0x7fab
        );
        assert!(sample("17/21 1 zz foo").is_err());
    }
}
