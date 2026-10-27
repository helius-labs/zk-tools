// Only called when no full SIMD group exists and each active tail has its own lane.
fn invert_short_lanes(c: [Fq; LANES], used: usize) -> [Fq; LANES] {
    debug_assert!(used > 0 && used < LANES);
    let mut prefixes = [Fq::ONE; LANES];
    prefixes[0] = c[0];
    for i in 1..used {
        prefixes[i] = prefixes[i - 1] * c[i];
    }
    let mut inverse = prefixes[used - 1].inverse().unwrap();
    let mut result = [Fq::ONE; LANES];
    for i in (1..used).rev() {
        result[i] = inverse * prefixes[i - 1];
        inverse *= c[i];
    }
    result[0] = inverse;
    result
}
