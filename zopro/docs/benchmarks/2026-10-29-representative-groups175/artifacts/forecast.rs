use core::mem::size_of;
pub(super) fn add(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}
pub(super) fn mul(a: usize, b: usize) -> Option<usize> {
    a.checked_mul(b)
}
pub(crate) fn vector<T>(n: usize, growing: bool) -> Option<usize> {
    if n == 0 {
        return Some(0);
    }
    let capacity = if growing {
        n.max(4).checked_next_power_of_two()?
    } else {
        n
    };
    add(mul(capacity, size_of::<T>())?, 64)
}
pub(super) fn hash_table<T>(n: usize) -> Option<usize> {
    let buckets = add(mul(n, 8)?, 6)?
        .checked_div(7)?
        .checked_next_power_of_two()?
        .max(4);
    add(
        add(mul(buckets, add(size_of::<T>(), 1)?)?, 80)?,
        add(mul(buckets / 2, add(size_of::<T>(), 1)?)?, 80)?,
    )
}
