//! Private-deque availability hints. Queue ownership remains authoritative.
#[cfg(zopro_ready_loom)]
use loom::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(zopro_ready_loom))]
use std::sync::atomic::{AtomicUsize, Ordering};

const BITS: usize = usize::BITS as usize;

#[derive(Debug)]
pub(super) struct ReadyVictims {
    words: Box<[AtomicUsize]>,
    workers: usize,
}

impl ReadyVictims {
    pub(super) fn new(workers: usize) -> Self {
        assert!(workers > 0);
        Self {
            words: (0..workers.div_ceil(BITS))
                .map(|_| AtomicUsize::new(0))
                .collect(),
            workers,
        }
    }

    /// Called only by this deque's owner, after pushing and before notifying sleep.
    #[inline]
    pub(super) fn publish(&self, worker: usize) {
        assert!(worker < self.workers);
        self.words[worker / BITS].fetch_or(1usize << (worker % BITS), Ordering::Release);
    }

    /// Called only by this deque's owner, after it observes its own deque empty.
    #[inline]
    pub(super) fn clear(&self, worker: usize) {
        assert!(worker < self.workers);
        self.words[worker / BITS].fetch_and(!(1usize << (worker % BITS)), Ordering::Release);
    }

    #[inline]
    pub(super) fn from(&self, start: usize) -> Victims<'_> {
        assert!(start < self.workers);
        Victims {
            ready: self,
            start,
            cursor: start,
            end: self.workers,
            wrapped: false,
            remaining: 0,
            base: 0,
        }
    }
}

#[derive(Debug)]
pub(super) struct Victims<'a> {
    ready: &'a ReadyVictims,
    start: usize,
    cursor: usize,
    end: usize,
    wrapped: bool,
    remaining: usize,
    base: usize,
}

impl Iterator for Victims<'_> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        loop {
            if self.remaining != 0 {
                let bit = self.remaining.trailing_zeros() as usize;
                self.remaining &= self.remaining - 1;
                return Some(self.base + bit);
            }
            if self.cursor == self.end {
                if self.wrapped {
                    return None;
                }
                self.wrapped = true;
                self.cursor = 0;
                self.end = self.start;
                continue;
            }
            self.base = self.cursor / BITS * BITS;
            let high = self.end.min(self.base + BITS);
            let lower_mask = usize::MAX << (self.cursor - self.base);
            let upper_mask = if high - self.base == BITS {
                usize::MAX
            } else {
                (1usize << (high - self.base)) - 1
            };
            self.remaining = self.ready.words[self.base / BITS].load(Ordering::Acquire)
                & lower_mask
                & upper_mask;
            self.cursor = high;
        }
    }
}

#[cfg(all(test, not(zopro_ready_loom)))]
mod ready_tests {
    use super::*;

    #[test]
    fn every_start_matches_cyclic_reference_across_words() {
        for workers in [1, 2, 3, 31, 32, 33, 63, 64, 65, 96, 127, 128, 129, 257, 1001] {
            let ready = ReadyVictims::new(workers);
            for pattern in 0..6 {
                let selected = (0..workers)
                    .map(|i| match pattern {
                        0 => false,
                        1 => true,
                        2 => i % 2 == 0,
                        3 => i % 7 == 3,
                        4 => i == workers - 1,
                        _ => i == 0 || i == workers / 2,
                    })
                    .collect::<Vec<_>>();
                for (i, &set) in selected.iter().enumerate() {
                    if set { ready.publish(i); } else { ready.clear(i); }
                }
                for start in 0..workers {
                    let expected = (start..workers).chain(0..start)
                        .filter(|&i| selected[i]).collect::<Vec<_>>();
                    assert_eq!(ready.from(start).collect::<Vec<_>>(), expected);
                }
            }
            assert_eq!(ready.words.len(), workers.div_ceil(BITS));
        }
    }

    #[test]
    fn owner_transitions_preserve_other_owners_and_padding_bits() {
        for workers in [1, 65, 96, 129] {
            let ready = ReadyVictims::new(workers);
            for i in 0..workers { ready.publish(i); }
            for i in 0..workers {
                ready.clear(i);
                assert_eq!(ready.from(0).count(), workers - i - 1);
            }
            assert!(ready.from(workers - 1).next().is_none());
            ready.publish(workers - 1);
            assert_eq!(ready.from(0).collect::<Vec<_>>(), vec![workers - 1]);
        }
    }
}
