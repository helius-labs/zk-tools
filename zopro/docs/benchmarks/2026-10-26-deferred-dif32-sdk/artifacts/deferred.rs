//! Bounded DIF32 integration. Ordinary roots originate in canonical Quad::new.
use super::*;

#[inline(always)]
pub(super) fn apply<const CACHED: bool>(
    values: &mut [Pair; 32],
    roots: &Twiddles,
    level: usize,
    q: usize,
    column: usize,
) {
    Pair::dif32_deferred(values, |stage, j| {
        let gap = 16 >> stage;
        if CACHED {
            broadcast(&roots.columns[level][31 * column + 32 - 2 * gap + j])
        } else {
            broadcast(&roots.layers[level + stage][j * q + column])
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    fn plan(n: usize) -> Plan {
        let generator = Fr::root_of_unity(n).unwrap().pow(&[3]);
        let coset = Fr::from(7u64);
        Plan::new(n, generator, coset, denominator(n, coset).unwrap())
    }
    fn pair_input(row: usize) -> Pair {
        Pair::new(&core::array::from_fn(|lane| {
            Fr::from((row * 1597 + lane * 173 + 101) as u64).pow(&[65537])
        }))
    }
    fn assert_canonical_root(root: &Quad) {
        let value = broadcast(root);
        assert_eq!(core::mem::size_of::<Pair>(), 320);
        // Read the actual normalized R260 root bits; get() would canonicalize
        // and could conceal a violation of the stronger belowp precondition.
        let raw: [[u64; 8]; 5] = unsafe { core::mem::transmute_copy(&value) };
        let p = Fr::MODULUS;
        let mask = (1u64 << 52) - 1;
        let digits: [u64; 5] = core::array::from_fn(|j| {
            let shift = j * 52;
            let word = shift / 64;
            let low = p[word] >> (shift % 64);
            let high = if shift % 64 != 0 && word + 1 < 4 {
                p[word + 1] << (64 - shift % 64)
            } else {
                0
            };
            (low | high) & mask
        });
        for lane in 0..8 {
            assert!((0..5).all(|j| raw[j][lane] <= mask));
            let ordering = (0..5)
                .rev()
                .find_map(|j| (raw[j][lane] != digits[j]).then(|| raw[j][lane] < digits[j]));
            assert_eq!(ordering, Some(true));
        }
    }
    fn quad_column(
        local: &mut [Pair; 32],
        roots: &Twiddles,
        level: usize,
        q: usize,
        column: usize,
    ) {
        let mut lower = local.map(|v| Quad::unpair(&v).0);
        let mut upper = local.map(|v| Quad::unpair(&v).1);
        for stage in 0..5 {
            let gap = 16 >> stage;
            for block in (0..32).step_by(2 * gap) {
                for j in 0..gap {
                    let low = block + j;
                    let high = low + gap;
                    let root = &roots.layers[level + stage][j * q + column];
                    for values in [&mut lower, &mut upper] {
                        let (a, b) = (values[low], values[high]);
                        values[low] = a.add(&b);
                        values[high] = a.sub(&b).mul(root);
                    }
                }
            }
        }
        for i in 0..32 {
            local[i] = Quad::pair(&lower[i], &upper[i]);
        }
    }
    #[test]
    fn plan_samples_explicit_mode_and_twisted_roots_keep_fallback() {
        let plan = plan(8192);
        let expected = std::env::var("ZOPRO_CPU_DEFERRED_DIF32").as_deref() == Ok("1");
        assert_eq!(plan.roots.deferred_dif32, expected);
        assert_eq!(plan.inverse_roots.deferred_dif32, expected);
        assert!(!Twiddles::empty().deferred_dif32);
        assert!(!plan.roots.twisted(Fr::from(7u64)).deferred_dif32);
        println!(
            "DEFERRED_NATIVE_LAYOUT Plan={} Twiddles={} Pair={}",
            core::mem::size_of::<Plan>(),
            core::mem::size_of::<Twiddles>(),
            core::mem::size_of::<Pair>()
        );
    }
    #[test]
    fn actual_packed_columns_match_quad_reference_with_canonical_roots() {
        let mut outputs = 0;
        let mut cached_columns = 0;
        let mut canonical_roots = 0;
        for n in [8192usize, 32768, 262144] {
            let mut plan = plan(n);
            plan.inverse_roots.prepare_columns();
            let roots = &mut plan.inverse_roots;
            for root in roots
                .layers
                .iter()
                .flatten()
                .chain(roots.columns.iter().flatten())
            {
                assert_canonical_root(root);
                canonical_roots += 8;
            }
            for level in (0..roots.layers.len()).step_by(5) {
                if level + 4 >= roots.layers.len() {
                    continue;
                }
                let q = roots.layers[level].len() / 16;
                if q == 0 {
                    continue;
                }
                let mut columns = vec![0, q / 2, q - 1];
                columns.sort();
                columns.dedup();
                for column in columns {
                    let input = core::array::from_fn(pair_input);
                    let mut reference = input;
                    quad_column(&mut reference, roots, level, q, column);
                    roots.deferred_dif32 = true;
                    let mut uncached = input;
                    column_kernel::<false, false>(&mut uncached, roots, level, q, column);
                    for i in 0..32 {
                        assert_eq!(uncached[i].get(), reference[i].get());
                        outputs += 8;
                    }
                    if roots.columns.get(level).is_some_and(|v| !v.is_empty()) {
                        cached_columns += 1;
                        let mut cached = input;
                        column_kernel::<true, false>(&mut cached, roots, level, q, column);
                        for i in 0..32 {
                            assert_eq!(cached[i].get(), reference[i].get());
                            outputs += 8;
                        }
                    }
                    // The disabled dispatcher must still use the original body.
                    roots.deferred_dif32 = false;
                    let mut off = input;
                    column_kernel::<false, false>(&mut off, roots, level, q, column);
                    for i in 0..32 {
                        assert_eq!(off[i].get(), reference[i].get());
                        outputs += 8;
                    }
                }
            }
        }
        assert!(cached_columns > 0, "cached dispatcher must be exercised");
        println!("DEFERRED_ACTUAL_COLUMN_OUTPUTS={outputs} CANONICAL_ROOT_LANES={canonical_roots} CACHED_COLUMNS={cached_columns}");
    }
    fn full_transform(n: usize, roots: &Twiddles, tag: usize) {
        let mut a = vec![Fr::ZERO; n];
        let mut b = vec![Fr::ZERO; n];
        for i in 0..n - 3 {
            a[i] = Fr::from((i * 131 + tag * 17 + 1) as u64).square();
            if i < n / 2 + 1 {
                b[i] = Fr::from((i * 257 + tag * 31 + 7) as u64).square();
            }
        }
        let mut aq = super::super::super::quads(&a);
        let mut bq = super::super::super::quads(&b);
        super::super::super::dif(&mut aq, roots, 0);
        super::super::super::dif(&mut bq, roots, 0);
        let mut packed = (0..n / 4)
            .map(|i| {
                PackedPair::store(Pair::new(&core::array::from_fn(|lane| {
                    if lane < 4 {
                        a[4 * i + lane]
                    } else {
                        b[4 * i + lane - 4]
                    }
                })))
            })
            .collect::<Vec<_>>();
        stored_transform::<PackedPair, false>(&mut packed, roots, 0);
        for (i, value) in packed.iter().enumerate() {
            let (lo, hi) = Quad::unpair(&value.load());
            assert_eq!(lo.get(), aq[i].get(), "A fieldquad{i}");
            assert_eq!(hi.get(), bq[i].get(), "B fieldquad{i}");
        }
    }
    #[test]
    fn packed_full_transforms_match_legacy_quads_with_cached_and_uncached_roots() {
        for n in [4096usize, 8192, 32768, 262144] {
            let mut plan = plan(n);
            plan.inverse_roots.deferred_dif32 = true;
            plan.inverse_roots.columns.clear();
            full_transform(n, &plan.inverse_roots, 3);
            plan.inverse_roots.prepare_columns();
            full_transform(n, &plan.inverse_roots, 5);
        }
    }
    #[test]
    fn concurrent_shared_root_transforms_use_small_worker_stacks() {
        let mut plan = plan(8192);
        plan.inverse_roots.deferred_dif32 = true;
        plan.inverse_roots.prepare_columns();
        let plan = std::sync::Arc::new(plan);
        let pool = std::sync::Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .stack_size(512 * 1024)
                .build()
                .unwrap(),
        );
        let mut handles = Vec::new();
        for tag in 0..4 {
            let plan = plan.clone();
            let pool = pool.clone();
            handles.push(
                std::thread::Builder::new()
                    .stack_size(512 * 1024)
                    .spawn(move || pool.install(|| full_transform(8192, &plan.inverse_roots, tag)))
                    .unwrap(),
            );
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}
