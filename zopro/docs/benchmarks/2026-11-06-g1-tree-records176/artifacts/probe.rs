//! Private complete binary callers and focused qualification, never production.
use super::*;
use std::{hint::black_box, mem::size_of};
use zopro_bn254::{G1Affine, G1Config, G1Projective};

#[derive(Default)]
struct Work {
    points: Vec<LimbPoint<Fq>>,
    counts: Vec<usize>,
    output: Vec<G1Affine>,
    scratch: Scratch<G1Config>,
}
impl Work {
    fn bytes(&self) -> [usize; 5] {
        [
            self.points.capacity() * size_of::<LimbPoint<Fq>>()
                + self.counts.capacity() * size_of::<usize>()
                + self.output.capacity() * size_of::<G1Affine>(),
            self.scratch.pairs.capacity() * size_of::<usize>(),
            self.scratch.groups.capacity() * size_of::<Group<Fq>>(),
            self.scratch.prefix_records.capacity() * size_of::<[FqBatch; 2]>(),
            size_of::<Self>(),
        ]
    }
    fn reduce(&mut self, bases: &[G1Affine], order: &[u32], starts: &[usize], counts: &[usize]) {
        self.points.clear();
        self.points.reserve_exact(order.len());
        for &index in order {
            self.points.push(limbs::<G1Config>(bases[index as usize]));
        }
        self.counts.clear();
        self.counts.extend_from_slice(counts);
        while self.counts.iter().any(|&n| n > 1) {
            reduce_layer::<G1Config>(
                &mut self.points,
                starts,
                &mut self.counts,
                &mut self.scratch,
            );
        }
        self.output.clear();
        for (&start, &n) in starts.iter().zip(&self.counts) {
            self.output.push(if n == 0 {
                G1Affine::IDENTITY
            } else {
                affine::<G1Config>(self.points[start])
            });
        }
    }
}
pub struct TreeCaller {
    bases: Vec<G1Affine>,
    order: Vec<u32>,
    starts: Vec<usize>,
    counts: Vec<usize>,
    expected: Vec<G1Affine>,
    work: Work,
    last_bytes: [usize; 5],
}
impl TreeCaller {
    pub fn new(counts: &[usize]) -> Self {
        let n = counts
            .iter()
            .try_fold(0usize, |a, &b| a.checked_add(b))
            .unwrap();
        let values: Vec<_> = (0..n)
            .map(|i| Fr::from((1u64 << 48) * (i as u64 + 1) + 1800000054 + i as u64 * 137))
            .collect();
        let g = G1Projective::generator();
        let bases =
            G1Projective::batch_to_affine(&values.iter().map(|s| g * *s).collect::<Vec<_>>());
        Self::from_bases(bases, counts)
    }
    pub fn from_words(words: &[[u64; 4]], counts: &[usize]) -> Self {
        assert_eq!(
            words.len(),
            counts
                .iter()
                .try_fold(0usize, |a, &b| a.checked_add(b))
                .unwrap()
        );
        let g = G1Projective::generator();
        let bases = G1Projective::batch_to_affine(
            &words
                .iter()
                .map(|w| g * Fr::from_canonical(*w).unwrap())
                .collect::<Vec<_>>(),
        );
        Self::from_bases(bases, counts)
    }
    fn from_bases(bases: Vec<G1Affine>, counts: &[usize]) -> Self {
        assert!(bases.len() <= u32::MAX as usize);
        let n = bases.len();
        let mut order: Vec<u32> = (0..n as u32).collect();
        let mut state = 0x6a09e667f3bcc909u64;
        for i in (1..n).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            order.swap(i, state as usize % (i + 1));
        }
        let mut starts = Vec::new();
        let mut offset = 0;
        let mut expected = Vec::new();
        for &count in counts {
            starts.push(offset);
            let mut sum = G1Projective::IDENTITY;
            for &i in &order[offset..offset + count] {
                sum += bases[i as usize];
            }
            expected.push(sum.to_affine());
            offset += count;
        }
        assert_eq!(offset, n);
        Self {
            bases,
            order,
            starts,
            counts: counts.to_vec(),
            expected,
            work: Work::default(),
            last_bytes: [0; 5],
        }
    }
    pub fn run(&mut self, fresh: bool, rounds: usize) {
        for _ in 0..rounds {
            let s = black_box(&mut *self);
            if fresh {
                let mut w = Work::default();
                w.reduce(&s.bases, &s.order, &s.starts, &s.counts);
                s.last_bytes = w.bytes();
                s.last_bytes[0] += s.work.output.capacity() * size_of::<G1Affine>();
                s.last_bytes[4] += size_of::<Work>();
                std::mem::swap(&mut s.work.output, &mut w.output);
            } else {
                s.work.reduce(&s.bases, &s.order, &s.starts, &s.counts);
                s.last_bytes = s.work.bytes();
            }
            black_box(&s.work.output);
        }
    }
    pub fn check(&self) {
        assert_eq!(self.work.output, self.expected);
    }
    pub fn order(&self) -> &[u32] {
        &self.order
    }
    pub fn scratch_bytes(&self) -> [usize; 5] {
        self.last_bytes
    }
    pub fn coordinates(&self) -> Vec<[[u64; 4]; 2]> {
        self.work
            .output
            .iter()
            .map(|p| [p.x.to_canonical(), p.y.to_canonical()])
            .collect()
    }
    pub fn weighted_words(words: &[[u64; 4]]) -> [[u64; 4]; 2] {
        assert!(words.chunks(256).all(|c| c.len().is_power_of_two()));
        let g = G1Projective::generator();
        let bases = G1Projective::batch_to_affine(
            &words
                .iter()
                .map(|w| g * Fr::from_canonical(*w).unwrap())
                .collect::<Vec<_>>(),
        );
        let p = weighted_affine::<G1Config>(&bases).to_affine();
        [p.x.to_canonical(), p.y.to_canonical()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pool(n: usize) -> Vec<G1Affine> {
        let g = G1Projective::generator();
        let mut p = g * Fr::from(0x918f62a97u64);
        let raw: Vec<_> = (0..n)
            .map(|_| {
                p += g;
                p
            })
            .collect();
        G1Projective::batch_to_affine(&raw)
    }
    fn storage(s: &Scratch<G1Config>, max_groups: usize) {
        if Fq::TREE_PREFIX {
            assert_eq!(s.groups.capacity(), 0);
            assert!(s.prefix_records.capacity() <= max_groups);
            assert!(
                s.prefix_records.capacity() * size_of::<[FqBatch; 2]>()
                    <= max_groups * size_of::<Group<Fq>>()
            );
        } else {
            assert_eq!(s.prefix_records.capacity(), 0);
        }
    }
    #[test]
    fn full_vectors_partial_tails_and_exact_retained_capacity() {
        let bases = pool(8300);
        let mut cases = 0;
        for vector_inverse in [false, true] {
            let mut s = Scratch::<G1Config> {
                vector_inverse,
                ..Default::default()
            };
            let mut max_groups = 0;
            for m in [
                0usize, 1, 2, 3, 7, 8, 9, 15, 16, 17, 23, 24, 25, 31, 32, 33, 47, 48, 49, 63, 64,
                65, 127, 128, 129, 255, 256, 257, 511, 512, 513, 1023, 2048, 4096, 257, 31, 17, 1,
                0,
            ] {
                for tail in [0, 1] {
                    for pattern in 0..5 {
                        let counts = [2 * m + tail, 0, 1, 2, 3, 5, 7];
                        let starts: Vec<_> = counts
                            .iter()
                            .scan(0, |a, n| {
                                let old = *a;
                                *a += n;
                                Some(old)
                            })
                            .collect();
                        let len = counts.iter().sum();
                        let input: Vec<G1Affine> = (0..len)
                            .map(|i| match pattern {
                                0 => bases[i],
                                1 => {
                                    if i % 3 == 0 {
                                        G1Affine::IDENTITY
                                    } else {
                                        bases[i]
                                    }
                                }
                                2 => bases[i / 2],
                                3 => {
                                    if i % 2 == 0 {
                                        bases[i / 2]
                                    } else {
                                        -bases[i / 2]
                                    }
                                }
                                _ => {
                                    if i % 4 == 0 {
                                        G1Affine::IDENTITY
                                    } else if i % 4 == 1 {
                                        -bases[i / 2]
                                    } else {
                                        bases[i / 2]
                                    }
                                }
                            })
                            .collect();
                        let mut points: Vec<_> =
                            input.iter().copied().map(limbs::<G1Config>).collect();
                        let mut actual = counts;
                        reduce_layer(&mut points, &starts, &mut actual, &mut s);
                        max_groups = max_groups.max(s.pairs.len() / LANES);
                        storage(&s, max_groups);
                        for ((&start, &n), &got) in starts.iter().zip(&counts).zip(&actual) {
                            assert_eq!(got, n.div_ceil(2));
                            for i in 0..got {
                                let p = if 2 * i + 1 < n {
                                    (input[start + 2 * i] + input[start + 2 * i + 1]).to_affine()
                                } else {
                                    input[start + 2 * i]
                                };
                                assert_eq!(affine::<G1Config>(points[start + i]), p);
                            }
                        }
                        cases += 1;
                    }
                }
            }
        }
        eprintln!(
            "G1_TREE exact_pairs_tails_cases={cases} prefix_enabled={} lanes={LANES}",
            Fq::TREE_PREFIX
        );
    }
    #[test]
    fn weighted_copied_even_identity_and_original_forecasts() {
        let g = G1Projective::generator();
        let mut cases = 0;
        for inverse in [false, true] {
            let mut s = Scratch::<G1Config> {
                vector_inverse: inverse,
                ..Default::default()
            };
            let mut max_groups = 0;
            for n in [1usize, 2, 4, 8, 16, 32, 64, 128, 256, 128, 16, 1] {
                for pattern in 0..5 {
                    let scalars: Vec<_> = (0..n)
                        .map(|i| match pattern {
                            0 => Fr::ZERO,
                            1 => Fr::ONE,
                            2 => {
                                if i % 2 == 0 {
                                    Fr::ONE
                                } else {
                                    -Fr::ONE
                                }
                            }
                            3 => Fr::from((i * 37 + 19) as u64),
                            _ => {
                                if i % 5 == 0 {
                                    Fr::ZERO
                                } else {
                                    Fr::from((i / 2 + 3) as u64)
                                }
                            }
                        })
                        .collect();
                    let bases = G1Projective::batch_to_affine(
                        &scalars.iter().map(|a| g * *a).collect::<Vec<_>>(),
                    );
                    let mut tree = vec![LimbPoint::<Fq>::infinity(); 2 * n - 1];
                    for (out, p) in tree.iter_mut().zip(bases) {
                        *out = limbs::<G1Config>(p);
                    }
                    let want = g * scalars
                        .iter()
                        .enumerate()
                        .fold(Fr::ZERO, |a, (i, &v)| a + Fr::from((i + 1) as u64) * v);
                    assert_eq!(
                        weighted_sum::<G1Config>(&mut tree, n, &mut s).to_affine(),
                        want.to_affine()
                    );
                    max_groups = max_groups.max(n / (2 * LANES));
                    storage(&s, max_groups);
                    cases += 1;
                }
            }
        }
        #[cfg(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            let bases = pool(8200);
            let mut s = Scratch::<G1Config>::default();
            let mut max_groups = 0;
            let mut max_n = 0;
            for n in [
                0usize, 1, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 257, 511, 2048, 8192, 128, 16,
                0,
            ] {
                let mut points: Vec<_> =
                    bases[..n].iter().copied().map(limbs::<G1Config>).collect();
                let mut counts = [n];
                reduce_layer(&mut points, &[0], &mut counts, &mut s);
                max_groups = max_groups.max(s.pairs.len() / LANES);
                max_n = max_n.max(n);
                storage(&s, max_groups);
                let requested = max_n * size_of::<LimbPoint<Fq>>()
                    + s.pairs.capacity() * size_of::<usize>()
                    + s.groups.capacity() * size_of::<Group<Fq>>()
                    + s.prefix_records.capacity() * size_of::<[FqBatch; 2]>();
                assert!(requested <= workspace_grouping::<G1Config>(max_n, 1).unwrap());
                assert!(requested <= workspace_grouping_linear::<G1Config>(max_n, 1).unwrap());
            }
        }
        eprintln!("G1_TREE weighted_identity_cases={cases} existing_forecasts_unchanged");
    }
    #[test]
    fn grouped_and_weighted_public_paths_in_small_stack_rayon_pools() {
        let bases = pool(4096);
        let scalar0 = Fr::from(0x918f62a97u64);
        let g = G1Projective::generator();
        for workers in [1, 4, 16, 32] {
            let p = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            p.install(|| {
                assert_eq!(rayon::current_num_threads(), workers);
                for ng in [0usize, 1, 255, 256, 257, 513] {
                    let counts: Vec<_> = (0..ng).map(|i| (i * 7 + 3) % 13).collect();
                    let mut starts = vec![0];
                    for &n in &counts {
                        starts.push(starts.last().unwrap() + n);
                    }
                    let total = *starts.last().unwrap();
                    let order: Vec<_> = (0..total)
                        .map(|i| ((i * 37 + 11) % bases.len()) as u32)
                        .collect();
                    let want: Vec<_> = counts
                        .iter()
                        .enumerate()
                        .map(|(j, &n)| {
                            let s = order[starts[j]..starts[j] + n]
                                .iter()
                                .fold(Fr::ZERO, |s, &i| s + scalar0 + Fr::from(i as u64 + 1));
                            (g * s).to_affine()
                        })
                        .collect();
                    assert_eq!(group_affine(&bases, &order, &starts, &counts), want);
                    if ng > 0 {
                        assert_eq!(reduce_indexed_buckets(&bases, &order, &starts), want);
                    }
                }
                for n in [
                    0usize, 1, 2, 4, 8, 16, 32, 64, 128, 256, 257, 258, 260, 264, 272, 288, 320,
                    384, 512, 1024, 2048, 4096,
                ] {
                    let want = (0..n).fold(Fr::ZERO, |a, i| {
                        a + Fr::from((i + 1) as u64) * (scalar0 + Fr::from(i as u64 + 1))
                    });
                    assert_eq!(
                        weighted_affine::<G1Config>(&bases[..n]).to_affine(),
                        (g * want).to_affine()
                    );
                }
            });
            eprintln!("G1_TREE rayon_workers={workers} stack_bytes=524288 public_group+indexed+weighted_pass");
        }
    }
    #[test]
    fn fixed_stack_workers_reuse_complete_scratch_through_exceptions() {
        use std::sync::Arc;
        let bases = Arc::new(pool(4096));
        for workers in [1, 4, 16, 32] {
            let mut jobs = Vec::new();
            for _ in 0..workers {
                let bases = bases.clone();
                jobs.push(
                    std::thread::Builder::new()
                        .stack_size(512 * 1024)
                        .spawn(move || {
                            let mut s = Scratch::<G1Config>::default();
                            let mut max_groups = 0;
                            for inverse in [false, true] {
                                s.vector_inverse = inverse;
                                for n in [16usize, 2048, 17, 511, 7, 256, 0] {
                                    for pattern in 0..3 {
                                        let input: Vec<_> = (0..n)
                                            .map(|i| {
                                                if pattern == 1 && i % 3 == 0 {
                                                    G1Affine::IDENTITY
                                                } else if pattern == 2 && i % 2 != 0 {
                                                    -bases[i / 2]
                                                } else if pattern == 2 {
                                                    bases[i / 2]
                                                } else {
                                                    bases[i]
                                                }
                                            })
                                            .collect();
                                        let mut sum = G1Projective::IDENTITY;
                                        for &p in &input {
                                            sum += p;
                                        }
                                        let mut points: Vec<_> =
                                            input.into_iter().map(limbs::<G1Config>).collect();
                                        let mut c = [n];
                                        while c[0] > 1 {
                                            reduce_layer(&mut points, &[0], &mut c, &mut s);
                                            max_groups = max_groups.max(s.pairs.len() / LANES);
                                            storage(&s, max_groups);
                                        }
                                        let got = if n == 0 {
                                            G1Affine::IDENTITY
                                        } else {
                                            affine::<G1Config>(points[0])
                                        };
                                        assert_eq!(got, sum.to_affine());
                                    }
                                }
                            }
                            [
                                s.groups.capacity() * size_of::<Group<Fq>>(),
                                s.prefix_records.capacity() * size_of::<[FqBatch; 2]>(),
                                s.pairs.capacity() * size_of::<usize>(),
                                size_of::<Scratch<G1Config>>(),
                            ]
                        })
                        .unwrap(),
                );
            }
            let sizes: Vec<_> = jobs.into_iter().map(|j| j.join().unwrap()).collect();
            assert!(sizes.windows(2).all(|w| w[0] == w[1]));
            eprintln!(
                "G1_TREE std_workers={workers} stack_bytes=524288 scratch_capacity={:?}",
                sizes[0]
            );
        }
    }
}
