#[test]
fn prefix_cutoff_matches_independent_ordered_map() {
    use std::collections::BTreeMap;
    let mut admitted_rejections = 0;
    let mut accepted = 0;
    for n in [256usize, 2049, 8193, 32768, 65536] {
        for kind in 0..10 {
            let boundary = n * if (32768..65536).contains(&n)
                && crate::simd::cpu_option("ZOPRO_CPU_SMALL_GROUPS")
            {
                95
            } else {
                80
            } / 100;
            let unique = match kind {
                0 => 1,
                1 => 17,
                2 | 5 => boundary - 1,
                3 | 6 => boundary,
                4 | 7 => boundary + 1,
                _ => n - n / 12,
            };
            let mut words: Vec<[u64; 4]> = (0..n)
                .map(|i| {
                    let index = if kind >= 5 { n - 1 - i } else { i };
                    let v = if index < unique { index + 1 } else { 1 } as u64;
                    [v, v.wrapping_mul(0x9e3779b97f4a7c15), !v, v.rotate_left(27)]
                })
                .collect();
            if kind == 8 {
                // Force duplicate samples without claiming representative sampling.
                let stride = n.div_ceil(n.min(2048));
                for i in (0..n).step_by(stride) {
                    words[i] = [1, 2, 3, 4];
                }
            }
            if kind == 9 {
                for i in (0..n).step_by(5) {
                    words[i] = [0; 4];
                }
            }
            let mut oracle = BTreeMap::<[u64; 4], Vec<u32>>::new();
            let mut first = Vec::new();
            for (i, &value) in words.iter().enumerate() {
                if value == [0; 4] {
                    continue;
                }
                if !oracle.contains_key(&value) {
                    first.push(value);
                }
                oracle.entry(value).or_default().push(i as u32);
            }
            for policy in [false, true] {
                let sample = grouping_sample(&words, policy);
                let want = sample.as_ref().is_some_and(|(_, percent)| {
                    oracle.len() as u128 * 100 <= n as u128 * *percent as u128
                });
                let actual = scalar_groups_policy(&words, policy);
                assert_eq!(actual.is_some(), want, "n={n} kind={kind} policy={policy}");
                if !want {
                    admitted_rejections += usize::from(sample.is_some());
                    continue;
                }
                accepted += 1;
                let group = actual.unwrap();
                assert_eq!(group.len, n);
                assert_eq!(group.values, first);
                assert_eq!(group.starts.len(), first.len() + 1);
                assert_eq!(group.starts[0], 0);
                for (class, value) in first.iter().enumerate() {
                    assert_eq!(group.counts[class], oracle[value].len());
                    assert_eq!(
                        &group.order[group.starts[class]..group.starts[class + 1]],
                        &oracle[value]
                    );
                }
                assert_eq!(
                    group.order.len(),
                    oracle.values().map(Vec::len).sum::<usize>()
                );
            }
        }
    }
    assert!(accepted > 0 && admitted_rejections > 0);
    println!("PREFIX_CUTOFF_ORACLE_CASES=100 accepted={accepted} admitted_rejections={admitted_rejections}");
}
