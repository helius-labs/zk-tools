//! Coordinate boundary for one native eight-point borrowed G2 group.
use super::*;

#[inline(never)]
pub(super) fn finish(
    ax: &[FqBatch; 2],
    ay: &[FqBatch; 2],
    bx: &[FqBatch; 2],
    slope: &[FqBatch; 2],
) -> ([FqBatch; 2], [FqBatch; 2]) {
    let x = Fq2::lanes_sub(&Fq2::lanes_square(slope), &Fq2::lanes_add(ax, bx));
    let y = Fq2::lanes_sub(&Fq2::lanes_mul(slope, &Fq2::lanes_sub(ax, &x)), ay);
    (x, y)
}
