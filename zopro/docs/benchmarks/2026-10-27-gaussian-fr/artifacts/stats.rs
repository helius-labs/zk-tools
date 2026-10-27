fn median(v: &[f64]) -> f64 {
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    let n = x.len();
    assert!(n > 0);
    if n % 2 == 0 {
        (x[n / 2 - 1] + x[n / 2]) / 2.
    } else {
        x[n / 2]
    }
}
fn gm(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn ci(v: &[f64]) -> Value {
    assert!(!v.is_empty());
    let n = v.len();
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    let (mut bin, mut tail, mut rank, mut cov) = (1., 0., 0, 0.);
    for k in 1..=n / 2 {
        tail += bin / 2f64.powi(n as i32);
        if 2. * tail <= 0.05 {
            rank = k;
            cov = 1. - 2. * tail;
        }
        bin *= ((n - k + 1) as f64) / (k as f64);
    }
    assert!(rank > 0);
    json!({"median":median(&x),"lower":x[rank-1],"upper":x[n-rank],"rank_one_based":rank,"coverage_under_independent_exchangeable_blocks":cov,"n":n,"all_values":v})
}
