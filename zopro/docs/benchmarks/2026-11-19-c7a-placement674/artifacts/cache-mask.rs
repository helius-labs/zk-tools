//! Select physical cores using Linux affinity and last-level cache sharing.
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fs;
use std::os::unix::process::CommandExt;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn cpu_list(raw: &str) -> Result<Vec<usize>> {
    let mut cpus = BTreeSet::new();
    for part in raw.trim().split(',') {
        let (lo, hi) = part.split_once('-').unwrap_or((part, part));
        let (lo, hi) = (lo.parse::<usize>()?, hi.parse::<usize>()?);
        if lo > hi {
            return Err("descending CPU range".into());
        }
        cpus.extend(lo..=hi);
    }
    Ok(cpus.into_iter().collect())
}

fn cache_domains() -> Result<Vec<Vec<usize>>> {
    let status = fs::read_to_string("/proc/self/status")?;
    let allowed = status
        .lines()
        .find_map(|l| l.strip_prefix("Cpus_allowed_list:"))
        .ok_or("missing inherited CPU mask")?;
    let mut seen_cores = BTreeSet::new();
    let mut domains = BTreeMap::<Vec<usize>, Vec<usize>>::new();
    for cpu in cpu_list(allowed)? {
        let root = format!("/sys/devices/system/cpu/cpu{cpu}");
        let siblings = cpu_list(&fs::read_to_string(format!(
            "{root}/topology/thread_siblings_list"
        ))?)?;
        if !seen_cores.insert(siblings) {
            continue;
        }
        let mut l3 = None;
        for index in fs::read_dir(format!("{root}/cache"))? {
            let index = index?.path();
            if index
                .file_name()
                .and_then(|x| x.to_str())
                .is_some_and(|s| s.starts_with("index"))
                && fs::read_to_string(index.join("level"))?.trim() == "3"
            {
                l3 = Some(cpu_list(&fs::read_to_string(
                    index.join("shared_cpu_list"),
                )?)?);
                break;
            }
        }
        domains
            .entry(l3.ok_or("missing L3 sharing topology")?)
            .or_default()
            .push(cpu);
    }
    Ok(domains.into_values().collect())
}

fn select(domains: &[Vec<usize>], policy: &str, width: usize, block: usize) -> Result<Vec<usize>> {
    let mut order = Vec::new();
    match policy {
        "compact" => order.extend(domains.iter().flatten().copied()),
        "spread" => {
            for lane in 0..domains.iter().map(Vec::len).max().unwrap_or(0) {
                order.extend(domains.iter().filter_map(|g| g.get(lane)).copied());
            }
        }
        _ => return Err("expected compact or spread".into()),
    }
    if width == 0 || width > order.len() {
        return Err("invalid physical core count".into());
    }
    let offset = block.checked_mul(width).ok_or("block offset overflow")? % order.len();
    order.rotate_left(offset);
    order.truncate(width);
    order.sort_unstable();
    Ok(order)
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let policy = args.next().ok_or("missing policy")?;
    let width = args.next().ok_or("missing width")?.parse()?;
    let block = args.next().ok_or("missing block")?.parse()?;
    let program = args.next().ok_or("missing target executable")?;
    let domains = cache_domains()?;
    let mask = select(&domains, &policy, width, block)?;
    let cpus = mask
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",");
    eprintln!("L3_DOMAINS {domains:?}");
    eprintln!("PLACEMENT {policy} {width} {block} {cpus}");
    Err(Command::new("/usr/bin/taskset")
        .args(["-c", &cpus, &program])
        .args(args)
        .exec()
        .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c7a_windows_cover_every_core_in_three_blocks() {
        let domains: Vec<Vec<usize>> = (0..12).map(|d| (8 * d..8 * d + 8).collect()).collect();
        for (policy, expected_domains) in [("compact", 4), ("spread", 12)] {
            let mut seen = BTreeSet::new();
            for b in 0..3 {
                let mask = select(&domains, policy, 32, b).unwrap();
                assert_eq!(
                    mask.iter().map(|id| id / 8).collect::<BTreeSet<_>>().len(),
                    expected_domains
                );
                assert_eq!(mask.len(), 32);
                for id in mask {
                    assert!(seen.insert(id));
                }
            }
            assert_eq!(seen.len(), 96);
        }
    }

    #[test]
    fn uneven_domains_and_partial_masks_are_valid() {
        let domains = vec![vec![2, 6, 8], vec![10], vec![15, 20]];
        assert_eq!(select(&domains, "spread", 3, 0).unwrap(), [2, 10, 15]);
        assert_eq!(select(&domains, "spread", 3, 1).unwrap(), [6, 8, 20]);
        assert!(select(&domains, "compact", 7, 0).is_err());
        assert!(select(&domains, "spread", 0, 0).is_err());
        assert_eq!(cpu_list("2-4,6,9-10").unwrap(), [2, 3, 4, 6, 9, 10]);
    }
}
