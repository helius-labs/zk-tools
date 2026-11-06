//! Nonblocking proof-local window actors. No actor waits for future input.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use zopro_bn254::{G1Affine, G1Config, G1Projective};

pub(crate) const CHUNK: usize = 1024;
pub(crate) const WIDTH: usize = 12;
const WINDOWS: usize = 22;

pub(crate) struct Chunk {
    pub start: usize,
    pub words: Vec<[u64; 4]>,
}

struct Inbox<'key> {
    queue: Vec<Option<Arc<Chunk>>>,
    head: usize,
    tail: usize,
    running: bool,
    closed: bool,
    canceled: bool,
    state: Option<Window<'key, G1Config>>,
    result: Option<G1Projective>,
    jobs: usize,
    feeds_before_solver_return: usize,
}

struct Actor<'key> {
    bases: &'key [G1Affine],
    window: usize,
    inbox: Mutex<Inbox<'key>>,
    solver_returned: Arc<AtomicBool>,
}

impl<'key> Actor<'key> {
    fn schedule<'scope>(self: &Arc<Self>, scope: &rayon::Scope<'scope>)
    where
        'key: 'scope,
    {
        let owner = self.clone();
        scope.spawn(move |_| owner.run());
    }

    fn publish<'scope>(self: &Arc<Self>, scope: &rayon::Scope<'scope>, chunk: Arc<Chunk>)
    where
        'key: 'scope,
    {
        let start = {
            let mut inbox = self.inbox.lock().unwrap();
            assert!(!inbox.closed && inbox.tail < inbox.queue.len());
            let tail = inbox.tail;
            inbox.queue[tail] = Some(chunk);
            inbox.tail += 1;
            if inbox.running {
                false
            } else {
                inbox.running = true;
                inbox.jobs += 1;
                true
            }
        };
        if start {
            self.schedule(scope);
        }
    }

    fn close<'scope>(self: &Arc<Self>, scope: &rayon::Scope<'scope>, cancel: bool)
    where
        'key: 'scope,
    {
        let start = {
            let mut inbox = self.inbox.lock().unwrap();
            inbox.closed = true;
            inbox.canceled |= cancel;
            if inbox.running || inbox.result.is_some() {
                false
            } else {
                inbox.running = true;
                inbox.jobs += 1;
                true
            }
        };
        if start {
            self.schedule(scope);
        }
    }

    fn run(&self) {
        let previous = self.inbox.lock().unwrap().state.take();
        let mut state =
            previous.unwrap_or_else(|| Window::new(1 << (WIDTH - 1), batch_size(WIDTH).max(32)));
        loop {
            let chunk = {
                let mut inbox = self.inbox.lock().unwrap();
                if inbox.canceled {
                    for slot in &mut inbox.queue {
                        *slot = None;
                    }
                    inbox.running = false;
                    return;
                }
                if inbox.head < inbox.tail {
                    let head = inbox.head;
                    inbox.head += 1;
                    if !self.solver_returned.load(Ordering::Relaxed) {
                        inbox.feeds_before_solver_return += 1;
                    }
                    inbox.queue[head].take()
                } else if inbox.closed {
                    None
                } else {
                    // State and idle flag transfer under the same mutex.
                    // A publisher cannot launch a successor before state exists.
                    inbox.state = Some(state);
                    inbox.running = false;
                    return;
                }
            };
            if let Some(chunk) = chunk {
                let end = chunk.start + chunk.words.len();
                state.feed(
                    &self.bases[chunk.start..end],
                    &chunk.words,
                    self.window,
                    WIDTH,
                    batch_size(WIDTH).max(32),
                );
            } else {
                let result = crate::msm::weighted_affine(&state.finish());
                let mut inbox = self.inbox.lock().unwrap();
                inbox.result = Some(result);
                inbox.running = false;
                return;
            }
        }
    }
}

pub(crate) struct Stream<'key> {
    actors: Vec<Arc<Actor<'key>>>,
    seen: Vec<bool>,
    len: usize,
    solver_returned: Arc<AtomicBool>,
}

impl<'key> Stream<'key> {
    pub(crate) fn new(bases: &'key [G1Affine]) -> Self {
        assert!((1024..65536).contains(&bases.len()));
        let chunks = bases.len().div_ceil(CHUNK);
        let solver_returned = Arc::new(AtomicBool::new(false));
        let actors = (0..WINDOWS - 1)
            .map(|window| {
                Arc::new(Actor {
                    bases,
                    window,
                    solver_returned: solver_returned.clone(),
                    inbox: Mutex::new(Inbox {
                        queue: vec![None; chunks],
                        head: 0,
                        tail: 0,
                        running: false,
                        closed: false,
                        canceled: false,
                        state: None,
                        result: None,
                        jobs: 0,
                        feeds_before_solver_return: 0,
                    }),
                })
            })
            .collect();
        Self {
            actors,
            seen: vec![false; chunks],
            len: bases.len(),
            solver_returned,
        }
    }

    pub(crate) fn solver_returned(&self) {
        self.solver_returned.store(true, Ordering::Relaxed);
    }

    pub(crate) fn early_feeds(&self) -> usize {
        self.actors
            .iter()
            .map(|a| a.inbox.lock().unwrap().feeds_before_solver_return)
            .sum()
    }

    pub(crate) fn publish<'scope>(
        &mut self,
        scope: &rayon::Scope<'scope>,
        chunk: Arc<Chunk>,
    ) -> Result<(), String>
    where
        'key: 'scope,
    {
        let index = chunk.start / CHUNK;
        let expected = self.len.saturating_sub(chunk.start).min(CHUNK);
        if chunk.start % CHUNK != 0
            || expected == 0
            || chunk.words.len() != expected
            || self.seen.get(index).is_none_or(|seen| *seen)
        {
            return Err("invalid or repeated A chunk".into());
        }
        self.seen[index] = true;
        for actor in &self.actors {
            actor.publish(scope, chunk.clone());
        }
        Ok(())
    }

    pub(crate) fn close<'scope>(&mut self, scope: &rayon::Scope<'scope>) -> Result<(), String>
    where
        'key: 'scope,
    {
        if self.seen.iter().any(|seen| !seen) {
            self.cancel(scope);
            return Err("incomplete A stream".into());
        }
        for actor in &self.actors {
            actor.close(scope, false);
        }
        Ok(())
    }

    pub(crate) fn cancel<'scope>(&mut self, scope: &rayon::Scope<'scope>)
    where
        'key: 'scope,
    {
        for actor in &self.actors {
            actor.close(scope, true);
        }
    }

    pub(crate) fn top(&self, bases: &[G1Affine], words: &[[u64; 4]]) -> G1Projective {
        top_window::<G1Config, true>(bases, words, WINDOWS - 1, WIDTH)
            .expect("eligible original leading window")
    }

    /// Scope completion must precede reading results.
    pub(crate) fn result(&self, top: G1Projective) -> (G1Projective, usize) {
        let mut sums = Vec::with_capacity(WINDOWS);
        let mut jobs = 0;
        for actor in &self.actors {
            let inbox = actor.inbox.lock().unwrap();
            assert!(!inbox.running && inbox.closed && !inbox.canceled);
            sums.push(inbox.result.expect("scope completed every window"));
            jobs += inbox.jobs;
        }
        sums.push(top);
        let mut result = G1Projective::IDENTITY;
        for sum in sums.iter().rev() {
            for _ in 0..WIDTH {
                result = result.double();
            }
            result += sum;
        }
        (result, jobs)
    }
}

pub(crate) fn forecast(n: usize) -> Option<usize> {
    use crate::msm_affine::workspace::{early_window_forecast, vector};
    use core::mem::size_of;
    let chunks = n.div_ceil(CHUNK);
    let actor = size_of::<Actor<'_>>()
        .checked_add(2 * size_of::<usize>())?
        .checked_add(vector::<Option<Arc<Chunk>>>(chunks, false)?)?
        .checked_add(size_of::<Arc<Actor<'_>>>())?;
    let window = early_window_forecast(1 << (WIDTH - 1), batch_size(WIDTH).max(32))?.checked_add(
        crate::msm::workspace_weighted::<G1Config>(1 << (WIDTH - 1))?,
    )?;
    let payload = vector::<[u64; 4]>(n, false)?
        .checked_add(chunks.checked_mul(size_of::<Chunk>() + 2 * size_of::<usize>())?)?;
    // At most one launch per publication and one close launch per actor. Count
    // all of them simultaneously to cover job epilogues after state handoff.
    // Pinned Rayon 1.13 HeapJob stores only its closure (owner Arc + ScopePtr);
    // 1024 bytes per launch is a conservative requested-byte allowance, not an
    // allocator, scheduler-stack, RSS bound or device RAM cap.
    let job = chunks.checked_add(1)?.checked_mul(1024)?;
    payload
        .checked_add((WINDOWS - 1).checked_mul(actor.checked_add(window)?.checked_add(job)?)?)?
        .checked_add(vector::<bool>(chunks, false)?)?
        .checked_add(size_of::<Stream<'_>>() + size_of::<AtomicBool>() + 2 * size_of::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::Seeded;

    fn fixture(n: usize) -> (Vec<G1Affine>, Vec<Fr>) {
        let mut p = G1Projective::IDENTITY;
        let mut points = Vec::new();
        let mut weights = Vec::new();
        for i in 0..n {
            p += G1Affine::GENERATOR;
            if i % 17 == 0 {
                points.push(G1Projective::IDENTITY);
                weights.push(Fr::ZERO);
            } else if i % 3 == 0 {
                points.push(-p);
                weights.push(-Fr::from((i + 1) as u64));
            } else {
                points.push(p);
                weights.push(Fr::from((i + 1) as u64));
            }
        }
        (G1Projective::batch_to_affine(&points), weights)
    }

    #[test]
    fn stream_matches_generator_with_chunk_permutations_and_tails() {
        for workers in [1, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            pool.install(|| {
                let mut rng = Seeded::new(0x6561726c795f6131);
                for n in [1024, 1025, 2051] {
                    let (points, weights) = fixture(n);
                    for kind in 0..4 {
                        let values = (0..n)
                            .map(|i| match kind {
                                0 => Fr::ZERO,
                                1 => -Fr::ONE,
                                2 => rng.fr(),
                                _ => {
                                    Fr::from(2u64).pow(&[[11, 12, 13, 63, 127, 253][i % 6]])
                                        - Fr::ONE
                                }
                            })
                            .collect::<Vec<_>>();
                        let words = values.iter().map(Fr::to_canonical).collect::<Vec<_>>();
                        let expected = G1Affine::GENERATOR
                            * weights
                                .iter()
                                .zip(&values)
                                .fold(Fr::ZERO, |s, (w, v)| s + *w * *v);
                        for reverse in [false, true] {
                            let mut stream = Stream::new(&points);
                            let mut order = (0..n.div_ceil(CHUNK)).collect::<Vec<_>>();
                            if reverse {
                                order.reverse();
                            }
                            let top = rayon::scope(|scope| {
                                for c in order {
                                    let start = c * CHUNK;
                                    stream
                                        .publish(
                                            scope,
                                            Arc::new(Chunk {
                                                start,
                                                words: words[start..(start + CHUNK).min(n)]
                                                    .to_vec(),
                                            }),
                                        )
                                        .unwrap();
                                    rayon::yield_now();
                                }
                                stream.close(scope).unwrap();
                                stream.top(&points, &words)
                            });
                            assert_eq!(stream.result(top).0.to_affine(), expected.to_affine());
                        }
                    }
                }
            });
        }
    }

    #[test]
    fn stream_small_pools_cancel_and_invalid_delivery() {
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            pool.install(|| {
                let (points, _) = fixture(2051);
                let words = vec![Fr::ONE.to_canonical(); points.len()];
                for _ in 0..4 {
                    let mut stream = Stream::new(&points);
                    rayon::scope(|scope| {
                        let chunk = Arc::new(Chunk {
                            start: 0,
                            words: words[..CHUNK].to_vec(),
                        });
                        stream.publish(scope, chunk.clone()).unwrap();
                        assert!(stream.publish(scope, chunk).is_err());
                        assert!(stream.close(scope).is_err());
                    });
                    for actor in &stream.actors {
                        assert!(actor.inbox.lock().unwrap().canceled);
                    }
                }
            });
        }
    }

    #[test]
    fn producer_panic_releases_chunk_owners() {
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            let (points, _) = fixture(1025);
            let chunk = Arc::new(Chunk {
                start: 0,
                words: vec![Fr::ONE.to_canonical(); CHUNK],
            });
            let weak = Arc::downgrade(&chunk);
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                pool.install(|| {
                    let mut stream = Stream::new(&points);
                    rayon::scope(|scope| {
                        stream.publish(scope, chunk.clone()).unwrap();
                        panic!("producer test");
                    });
                })
            }));
            assert!(outcome.is_err());
            drop(chunk);
            assert!(weak.upgrade().is_none());
        }
    }
}
