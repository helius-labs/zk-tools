mod qualification {
    use super::*;
    use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
    use ark_ff::{PrimeField, Zero};
    use std::sync::atomic::{AtomicUsize, Ordering};
    fn fq(x: ark_bn254::Fq) -> Fq {
        Fq::from_canonical(x.into_bigint().0).unwrap()
    }
    fn g1(p: ark_bn254::G1Affine) -> zopro_bn254::G1Affine {
        if p.is_zero() {
            zopro_bn254::G1Affine::IDENTITY
        } else {
            zopro_bn254::G1Affine::new(fq(p.x), fq(p.y))
        }
    }
    fn g2(p: ark_bn254::G2Affine) -> zopro_bn254::G2Affine {
        if p.is_zero() {
            zopro_bn254::G2Affine::IDENTITY
        } else {
            zopro_bn254::G2Affine::new(
                Fq2::new(fq(p.x.c0), fq(p.x.c1)),
                Fq2::new(fq(p.y.c0), fq(p.y.c1)),
            )
        }
    }
    fn weights() -> Vec<ark_bn254::Fr> {
        let mut seed = 0x776f726b73706c69u64;
        (0..64)
            .map(|_| {
                let mut bytes = [0u8; 32];
                for part in bytes.chunks_mut(8) {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    part.copy_from_slice(&seed.to_le_bytes());
                }
                ark_bn254::Fr::from_le_bytes_mod_order(&bytes)
            })
            .collect()
    }
    macro_rules! curve {
        ($name:ident,$config:ty,$point:ty,$ark:ty,$convert:ident) => {
            mod $name {
                use super::*;
                pub(super) struct Fixture {
                    pub points: Vec<$point>,
                    pub order: Vec<u32>,
                    pub starts: Vec<usize>,
                    pub counts: Vec<usize>,
                    pub want: Vec<$point>,
                }
                pub(super) fn fixture(counts: Vec<usize>, pattern: usize) -> Fixture {
                    let w = weights();
                    let pool: Vec<_> = <$ark>::normalize_batch(
                        &w.iter()
                            .map(|s| <$ark>::generator() * s)
                            .collect::<Vec<_>>(),
                    )
                    .into_iter()
                    .map($convert)
                    .collect();
                    let n: usize = counts.iter().sum();
                    let mut coeff = Vec::with_capacity(n);
                    let mut points = Vec::with_capacity(n);
                    for i in 0..n {
                        let j = if pattern == 1 || pattern == 2 {
                            0
                        } else {
                            (i * 17 + i / 11) % 64
                        };
                        let mut s = w[j];
                        let mut p = pool[j];
                        if pattern == 3 && i % 3 == 0 {
                            s = ark_bn254::Fr::zero();
                            p = <$point>::IDENTITY;
                        } else if pattern == 2 && i % 2 != 0 {
                            s = -s;
                            p = -p;
                        }
                        coeff.push(s);
                        points.push(p);
                    }
                    let order = (0..n as u32).rev().collect::<Vec<_>>();
                    let mut starts = vec![0];
                    let mut want = vec![];
                    for &count in &counts {
                        let lo = *starts.last().unwrap();
                        let end = lo + count;
                        let sum: ark_bn254::Fr =
                            order[lo..end].iter().map(|&i| coeff[i as usize]).sum();
                        want.push($convert((<$ark>::generator() * sum).into_affine()));
                        starts.push(end);
                    }
                    Fixture {
                        points,
                        order,
                        starts,
                        counts,
                        want,
                    }
                }
                pub(super) fn check(f: &Fixture) {
                    let saved = f.points.clone();
                    let reads = AtomicUsize::new(0);
                    let get = |i: u32| {
                        reads.fetch_add(1, Ordering::Relaxed);
                        f.points[i as usize]
                    };
                    let result = try_reduce::<$config, _>(&get, &f.order, &f.starts, &f.counts);
                    let tile = 2048 / (core::mem::size_of::<<$config as CurveConfig>::Base>() / 32);
                    if f.counts.iter().any(|&n| n > tile) {
                        assert_eq!(result.unwrap(), f.want);
                        assert_eq!(reads.load(Ordering::Relaxed), f.points.len());
                    } else {
                        assert!(result.is_none());
                        assert_eq!(reads.load(Ordering::Relaxed), 0);
                    }
                    assert_eq!(
                        group_affine(&f.points, &f.order, &f.starts, &f.counts),
                        f.want
                    );
                    let mid = f.points.len() / 2;
                    assert_eq!(
                        group_affine_prepared(
                            &f.points[..mid],
                            &f.points[mid..],
                            &f.order,
                            &f.starts,
                            &f.counts
                        ),
                        f.want
                    );
                    assert_eq!(f.points, saved);
                }
                pub(super) fn boundaries() -> usize {
                    let tile = 2048 / (core::mem::size_of::<<$config as CurveConfig>::Base>() / 32);
                    let mut cases = 0;
                    for largest in [
                        0usize,
                        1,
                        tile - 1,
                        tile,
                        tile + 1,
                        2 * tile,
                        2 * tile + 1,
                        5 * tile + 7,
                    ] {
                        for classes in [1usize, 256, 257] {
                            for pattern in 0..4 {
                                let mut counts = (0..classes).map(|i| i % 4).collect::<Vec<_>>();
                                counts[0] = largest;
                                check(&fixture(counts, pattern));
                                cases += 1;
                            }
                        }
                    }
                    check(&fixture(vec![], 0));
                    cases + 1
                }
                pub(super) fn threaded_fixture() -> Fixture {
                    let tile = 2048 / (core::mem::size_of::<<$config as CurveConfig>::Base>() / 32);
                    fixture(vec![tile + 1, 0, 1, 3, 2], 2)
                }
            }
        };
    }
    curve!(
        g1_cases,
        zopro_bn254::G1Config,
        zopro_bn254::G1Affine,
        ark_bn254::G1Projective,
        g1
    );
    curve!(
        g2_cases,
        zopro_bn254::G2Config,
        zopro_bn254::G2Affine,
        ark_bn254::G2Projective,
        g2
    );
    #[test]
    fn a_boundaries_complete_getters_and_arkworks() {
        let n = g1_cases::boundaries() + g2_cases::boundaries();
        assert_eq!(n, 194);
        println!("LARGE_CLASS_BOUNDARY_CASES={n};both_curves=true;ordinary_and_prepared=true;independent_Arkworks=true");
    }
    #[test]
    fn b_actual_workers_and_simultaneous_callers() {
        let a = g1_cases::threaded_fixture();
        let b = g2_cases::threaded_fixture();
        let calls = AtomicUsize::new(0);
        for workers in [1usize, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            let results = pool.broadcast(|ctx| {
                g1_cases::check(&a);
                g2_cases::check(&b);
                calls.fetch_add(2, Ordering::Relaxed);
                ctx.index()
            });
            assert_eq!(results, (0..workers).collect::<Vec<_>>());
        }
        assert_eq!(calls.load(Ordering::Relaxed), 106);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .stack_size(512 * 1024)
            .build()
            .unwrap();
        std::thread::scope(|scope| {
            for _ in 0..4 {
                let (pool, a, b, calls) = (&pool, &a, &b, &calls);
                scope.spawn(move || {
                    for _ in 0..7 {
                        pool.install(|| {
                            g1_cases::check(a);
                            g2_cases::check(b);
                        });
                        calls.fetch_add(2, Ordering::Relaxed);
                    }
                });
            }
        });
        assert_eq!(calls.load(Ordering::Relaxed), 162);
        println!("LARGE_CLASS_WORKERS=53;concurrent_callers=4;complete_curve_cases=162;requested_stack_bytes=524288");
    }
    #[test]
    fn c_forecast_denial_panic_and_recovery() {
        assert!(forecast::<zopro_bn254::G1Config>(usize::MAX, 2, 1, 2048).is_none());
        assert!(forecast::<zopro_bn254::G2Config>(1, usize::MAX, 32, 1024).is_none());
        for workers in [1usize, 4, 16, 32] {
            for partials in [2usize, 9, 65] {
                let expected = (257 + 1) * 8
                    + 64
                    + partials * 16
                    + 64
                    + partials * 64
                    + 64
                    + workspace_grouping_linear::<zopro_bn254::G1Config>(2048, 1).unwrap()
                        * workers.min(partials)
                    + 256;
                assert_eq!(
                    forecast::<zopro_bn254::G1Config>(257, partials, workers, 2048),
                    Some(expected)
                );
            }
        }
        struct Restore(Option<std::ffi::OsString>);
        impl Drop for Restore {
            fn drop(&mut self) {
                if let Some(v) = &self.0 {
                    std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", v)
                } else {
                    std::env::remove_var("ZOPRO_CPU_PREPARE_MAX_BYTES")
                }
            }
        }
        let restore = Restore(std::env::var_os("ZOPRO_CPU_PREPARE_MAX_BYTES"));
        std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", "0");
        let f = g1_cases::threaded_fixture();
        let reads = AtomicUsize::new(0);
        let get = |i: u32| {
            reads.fetch_add(1, Ordering::Relaxed);
            f.points[i as usize]
        };
        assert!(
            try_reduce::<zopro_bn254::G1Config, _>(&get, &f.order, &f.starts, &f.counts).is_none()
        );
        assert_eq!(reads.load(Ordering::Relaxed), 0);
        assert_eq!(
            group_affine(&f.points, &f.order, &f.starts, &f.counts),
            f.want
        );
        drop(restore);
        let caught = std::panic::catch_unwind(|| {
            let _ = try_reduce::<zopro_bn254::G1Config, _>(
                &|_: u32| -> zopro_bn254::G1Affine { panic!("intentional getter panic") },
                &f.order,
                &f.starts,
                &f.counts,
            );
        });
        assert!(caught.is_err());
        g1_cases::check(&f);
        println!(
            "LARGE_CLASS_RESOURCE_CASES=15;denial_before_getter=true;getter_panic_recovered=true"
        );
    }
}
