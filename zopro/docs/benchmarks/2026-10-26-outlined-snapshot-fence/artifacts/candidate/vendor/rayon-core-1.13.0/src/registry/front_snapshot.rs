//! Preserve the first scalar probe, then share a fence across bounded tiles.
use super::*;

impl WorkerThread {
    #[inline(never)]
    pub(super) fn steal_front_snapshot_tiles(&self) -> Option<JobRef> {
        debug_assert!(self.local_deque_is_empty());
        let thread_infos = &self.registry.thread_infos.as_slice();
        let num_threads = thread_infos.len();
        if num_threads <= 1 { return None; }
        loop {
            let mut retry = false;
            let start = self.rng.next_usize(num_threads);
            let mut victims = (start..num_threads).chain(0..start).filter(|&i| i != self.index);
            // Keep the common successful-first-victim path as a scalar probe.
            if let Some(index) = victims.next() {
                match thread_infos[index].stealer.steal() {
                    Steal::Success(job) => return Some(job),
                    Steal::Retry => retry = true,
                    Steal::Empty => {}
                }
            }
            while let Some(index) = victims.next() {
                let mut tile = [&thread_infos[index].stealer; 8];
                let mut len = 1;
                while len < tile.len() {
                    match victims.next() {
                        Some(next) => { tile[len] = &thread_infos[next].stealer; len += 1; }
                        None => break,
                    }
                }
                match Stealer::steal_one_from_front_snapshots(&tile[..len]) {
                    Steal::Success(job) => return Some(job),
                    Steal::Retry => retry = true,
                    Steal::Empty => {}
                }
            }
            // Exactly as the original scan: a race causes another complete
            // pass with a fresh random start, rather than declaring idle.
            if !retry { return None; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_samples_explicit_mode_before_worker_execution() {
        let expected = std::env::var("ZOPRO_CPU_FRONT_SNAPSHOT_FENCE").is_ok_and(|v| v == "1");
        let pool = crate::ThreadPoolBuilder::new().num_threads(4).build().unwrap();
        pool.install(|| assert_eq!(Registry::current().front_snapshot_fence, expected));
    }
}
