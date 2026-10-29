#[inline(always)]
fn dif_within(a: &mut Pair, b: &mut Pair, pair: &Pair) {
    // Two apart: (a0 a1 b0 b1) with (a2 a3 b2 b3), twiddles (1, t, 1, t).
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 1, 4, 5>(a, b),
        Pair::shuffle_pair::<2, 3, 6, 7>(a, b),
    );
    let sum = lo.add(&hi);
    let difference = lo.sub(&hi).mul(pair);
    *a = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &difference);
    // One apart: (a0 a2 b0 b2) with (a1 a3 b1 b3), twiddle one.
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 2, 4, 6>(a, b),
        Pair::shuffle_pair::<1, 3, 5, 7>(a, b),
    );
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &difference);
}

