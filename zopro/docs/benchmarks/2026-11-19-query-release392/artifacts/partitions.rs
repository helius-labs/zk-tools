use std::{cmp::Reverse, env, fs, ops::Range};

fn longest_ready_ranges(ready: &[u8], barrier: u8) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = None;
    for (i, &producer) in ready.iter().enumerate() {
        if producer <= barrier {
            start.get_or_insert(i);
        } else if let Some(first) = start.take() {
            ranges.push(first..i);
        }
    }
    if let Some(first) = start {
        ranges.push(first..ready.len());
    }
    ranges.sort_unstable_by_key(|r| (Reverse(r.len()), r.start));
    ranges
}

fn main() {
    let bytes = fs::read(env::args_os().nth(1).expect("producer map path")).unwrap();
    let (&epochs, ready) = bytes.split_first().expect("producer map header");
    assert!(!ready.is_empty() && epochs > 0 && ready.iter().all(|&e| e <= epochs));
    for barrier in 0..epochs {
        let runs = longest_ready_ranges(ready, barrier);
        let covered: usize = runs.iter().map(Range::len).sum();
        assert_eq!(covered, ready.iter().filter(|&&e| e <= barrier).count());
        for parts in [2, 4] {
            let size = ready.len().div_ceil(parts);
            let existing = ready
                .chunks(size)
                .filter(|chunk| chunk.iter().all(|&e| e <= barrier))
                .count();
            let mut left = size;
            let mut selected = Vec::new();
            for r in runs.iter().take(4) {
                let take = left.min(r.len());
                if take == 0 {
                    break;
                }
                selected.push((r.start, r.start + take));
                left -= take;
            }
            let encoded = selected
                .into_iter()
                .map(|(s, e)| format!("[{s},{e}]"))
                .collect::<Vec<_>>()
                .join(",");
            println!("{{\"barrier\":{barrier},\"parts\":{parts},\"part_size\":{size},\"already_whole\":{existing},\"ready\":{covered},\"runs\":{},\"four_ranges\":[{encoded}],\"shortfall\":{left}}}",runs.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contiguous_runs_are_maximal_ordered_and_include_last_partial_run() {
        let w = [2, 0, 0, 1, 3, 1, 2, 0, 1, 1];
        assert_eq!(longest_ready_ranges(&w, 1), vec![1..4, 7..10, 5..6]);
        assert_eq!(longest_ready_ranges(&w, 0), vec![1..3, 7..8]);
        assert!(longest_ready_ranges(&[4, 4], 0).is_empty());
        assert_eq!(longest_ready_ranges(&[0; 9], 0), vec![0..9]);
    }
}
