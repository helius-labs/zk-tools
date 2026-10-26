//! Native CPU requested-capacity planning from stable parents and leaf scratch.
//! Leaf kernels here do not fork while their scratch is live. Suspended window
//! parents retain buckets/indices, not one full streaming Window per task.
use super::*;
use core::mem::size_of;
use zopro_bn254::{
    ifma8::{Ifma8, PackedIfma8},
    FrConfig, G1Config, G2Config,
};
use super::grouping_cost::{add, mul, hash_table, limits as group_limits};
pub(crate) use super::grouping_cost::vector;
fn explicit(name: &str) -> Option<Option<usize>> {
    match std::env::var(name) {
        Ok(s) => Some(Some(s.parse().ok()?)),
        Err(std::env::VarError::NotPresent) => Some(None),
        Err(_) => None,
    }
}
fn unsigned_bits<C: CurveConfig>(n: usize) -> usize
where
    C::Base: BatchInverse,
{
    let delta = std::env::var("ZOPRO_CPU_UNSIGNED_DELTA")
        .ok()
        .and_then(|v| v.parse::<isize>().ok())
        .unwrap_or(0)
        .clamp(-4, 4);
    let explicit = std::env::var("ZOPRO_CPU_UNSIGNED_WIDTH")
        .ok()
        .and_then(|v| v.parse::<usize>().ok());
    let mut bits = explicit
        .unwrap_or_else(|| width(n).saturating_add_signed(delta))
        .clamp(3, 15);
    if explicit.is_none() && crate::simd::cpu_option("ZOPRO_CPU_UNSIGNED_CACHE") {
        while bits > 3 {
            let point = size_of::<Affine<C>>();
            if (1usize << bits) * (point + 4) + batch_size(bits + 1) * (point * 7 / 2 + 48)
                <= 400 * 1024
            {
                break;
            }
            bits -= 1;
        }
    }
    bits
}
fn parts(n: usize) -> Option<usize> {
    let default = if (1024..(1 << 18)).contains(&n)
        && rayon::current_num_threads() >= 16
        && crate::simd::cpu_option("ZOPRO_CPU_MERGE_SPLIT")
    {
        2
    } else {
        1
    };
    Some(
        explicit("ZOPRO_CPU_UNSIGNED_PARTS")?
            .unwrap_or(default)
            .clamp(1, 32),
    )
}
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Profile {
    pub held: usize,
    pub leaf: usize,
}
impl Profile {
    fn maximum(self, other: Self) -> Self {
        Self {
            held: self.held.max(other.held),
            leaf: self.leaf.max(other.leaf),
        }
    }
    fn retain(self, extra: usize) -> Option<Self> {
        Some(Self {
            held: add(self.held, extra)?,
            leaf: self.leaf,
        })
    }
}
fn window<C: CurveConfig>(half: usize, batch: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    let affine = vector::<Affine<C>>(half, false)?;
    let base = add(
        add(
            add(affine, vector::<Projective<C>>(half, true)?)?,
            vector::<u32>(half, false)?,
        )?,
        vector::<usize>(half, true)?,
    )?;
    let base = add(base, vector::<u64>(half.div_ceil(64), false)?)?;
    let base = add(base, mul(vector::<(u32, Affine<C>)>(batch, false)?, 2)?)?;
    let base = add(base, mul(vector::<C::Base>(batch, false)?, 2)?)?;
    let fill = add(base, crate::msm::workspace_batch::<C>(batch)?)?;
    let normalize = add(
        add(
            vector::<Projective<C>>(half, false)?,
            vector::<C::Base>(half, false)?,
        )?,
        affine,
    )?;
    add(fill, normalize)
}
fn shape<C: CurveConfig>(half: usize, windows: usize, parts: usize, batch: usize) -> Option<Profile>
where
    C::Base: BatchInverse,
{
    let affine = vector::<Affine<C>>(half, false)?;
    let partials = vector::<(Projective<C>, Affine<C>)>(half.div_ceil(256), false)?;
    let per_parent = add(
        add(mul(affine, parts)?, partials)?,
        vector::<Vec<Affine<C>>>(parts, false)?,
    )?;
    let held = add(
        mul(per_parent, windows)?,
        vector::<Projective<C>>(windows, false)?,
    )?;
    // Streaming fill + exceptional conversion have no Rayon calls. Their
    // affine output is already charged in the suspended-parent bucket sets.
    let fill = window::<C>(half, batch)?.checked_sub(affine)?;
    let merge = if parts > 1 {
        let n = half.min(512);
        add(
            vector::<(u32, Affine<C>)>(n, false)?,
            crate::msm::workspace_batch::<C>(n)?,
        )?
    } else {
        0
    };
    let weighted = crate::msm::workspace_weighted::<C>(half.min(256))?;
    Some(Profile {
        held,
        leaf: fill.max(merge).max(weighted),
    })
}
fn top<C: CurveConfig>(n: usize, bits: usize) -> Option<Profile>
where
    C::Base: BatchInverse,
{
    let buckets = 1usize.checked_shl(bits as u32)?;
    let tile = 2048 / (size_of::<C::Base>() / 32);
    let tiles = add(n.div_ceil(tile), buckets)?;
    // Parent arrays stay live across tile and final-group parallel passes.
    let held = add(
        add(
            mul(vector::<usize>(add(buckets, 1)?, false)?, 6)?,
            vector::<u32>(n, false)?,
        )?,
        vector::<(usize, usize)>(tiles, true)?,
    )?;
    let held = add(
        add(held, vector::<Affine<C>>(tiles, false)?)?,
        vector::<u32>(tiles, false)?,
    )?;
    let held = add(
        add(held, vector::<Affine<C>>(buckets, false)?)?,
        vector::<(Projective<C>, Affine<C>)>(buckets.div_ceil(256), false)?,
    )?;
    // Each tile's local reduction is nonforking; do not charge n point copies
    // simultaneously. The final group can contain every tile of one bucket.
    let tile_leaf = crate::msm::workspace_grouping::<C>(n.min(tile), 1)?;
    let final_leaf = crate::msm::workspace_grouping::<C>(tiles, 1)?;
    Some(Profile {
        held,
        leaf: tile_leaf
            .max(final_leaf)
            .max(crate::msm::workspace_weighted::<C>(buckets.min(256))?),
    })
}
fn one<C: CurveConfig>(n: usize, bits: usize, parts: usize, signed: bool) -> Option<Profile>
where
    C::Base: BatchInverse,
{
    let windows = (Fr::MODULUS_BITS as usize + usize::from(signed)).div_ceil(bits);
    let half = 1usize.checked_shl((bits - usize::from(signed)) as u32)?;
    let mut p = shape::<C>(
        half,
        windows,
        parts,
        batch_size(bits + usize::from(!signed)).max(32),
    )?;
    let top_bits = Fr::MODULUS_BITS as usize - (windows - 1) * bits;
    if n >= 1024 && top_bits <= 8 {
        let t = top::<C>(n, top_bits)?;
        p.held = add(p.held, t.held)?;
        p.leaf = p.leaf.max(t.leaf);
    }
    Some(p)
}
fn hb<C: CurveConfig>(n: usize) -> Option<Profile>
where
    C::Base: BatchInverse,
{
    let mut p = shape::<C>(1 << 10, 128usize.div_ceil(11), 2, batch_size(11).max(32))?;
    let t = top::<C>(n.div_ceil(2), 6)?;
    let component_buckets = mul(vector::<Affine<C>>(64, false)?, 2)?;
    p.held = add(
        p.held,
        add(
            mul(add(t.held, component_buckets)?, 2)?,
            vector::<[u64; 4]>(n, false)?,
        )?,
    )?;
    p.leaf = p.leaf.max(t.leaf);
    for bits in [12usize] {
        let windows=127usize.div_ceil(bits);
        let top_bits=126-(windows-1)*bits;
        let mut q=shape::<C>(1<<(bits-1),windows,2,batch_size(bits).max(32))?;
        let top=top::<C>(n.div_ceil(2),top_bits+1)?;
        // Two sign classes and split-off negative sums. High component remains
        // live while low is reduced; retain a conservative original scalar copy.
        let extra=mul(vector::<Affine<C>>(1<<top_bits,false)?,3)?;
        q.held=add(q.held,add(mul(add(top.held,extra)?,2)?,vector::<[u64;4]>(n,false)?)?)?;
        q.leaf=q.leaf.max(top.leaf);p=p.maximum(q);
    }
    Some(p)
}
fn raw<C: CurveConfig>(n: usize, g2_query: bool) -> Option<Profile>
where
    C::Base: BatchInverse,
{
    if n == 0 {
        return Some(Profile::default());
    }
    if g2_query && (1024..65536).contains(&n) {
        return hb::<C>(n);
    }
    if C::Base::deferred_sign() && (1024..65536).contains(&n) {
        return one::<C>(
            n,
            explicit("ZOPRO_CPU_DEFER_SIGN_WIDTH")?
                .unwrap_or(13)
                .clamp(10, 14),
            explicit("ZOPRO_CPU_DEFER_SIGN_PARTS")?
                .unwrap_or(2)
                .clamp(1, 2),
            true,
        );
    }
    if n >= 256 {
        return one::<C>(n, unsigned_bits::<C>(n), parts(n)?, false);
    }
    let bits = width(n);
    if !(3..=15).contains(&bits) {
        return None;
    }
    let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(bits);
    let half = 1usize << (bits - 1);
    let batch = batch_size(bits);
    // The small signed path retains its parallel pieces while collecting
    // window sums. Both projective output arrays coexist until reduction.
    let results = vector::<Projective<C>>(windows, false)?;
    if batch == 0 {
        return Some(Profile {
            held: mul(results, 2)?,
            leaf: vector::<Projective<C>>(half, false)?,
        });
    }
    shape::<C>(half, windows, 1, batch)?.retain(results)
}
fn raw_envelope<C: CurveConfig>(n: usize, hb_allowed: bool) -> Option<Profile>
where
    C::Base: BatchInverse,
{
    let mut p = Profile::default();
    for m in 1..=n.min(255) {
        p = p.maximum(raw::<C>(m, false)?);
    }
    if n >= 256 {
        let mut maximum = explicit("ZOPRO_CPU_UNSIGNED_WIDTH")?
            .map_or(if size_of::<C::Base>() == 32 { 12 } else { 11 }, |n| {
                n.clamp(3, 15)
            });
        if n < 65536 && (C::Base::deferred_sign() || hb_allowed) {
            maximum = (256..=n.min(1023))
                .map(unsigned_bits::<C>)
                .max()
                .unwrap_or(3);
        }
        let part = explicit("ZOPRO_CPU_UNSIGNED_PARTS")?
            .unwrap_or_else(|| {
                if rayon::current_num_threads() >= 16 {
                    2
                } else {
                    1
                }
            })
            .clamp(1, 32);
        for bits in 3..=maximum {
            p = p.maximum(one::<C>(n, bits, part, false)?);
        }
        if C::Base::deferred_sign() && n >= 1024 {
            p = p.maximum(one::<C>(
                n.min(65535),
                explicit("ZOPRO_CPU_DEFER_SIGN_WIDTH")?
                    .unwrap_or(13)
                    .clamp(10, 14),
                explicit("ZOPRO_CPU_DEFER_SIGN_PARTS")?
                    .unwrap_or(2)
                    .clamp(1, 2),
                true,
            )?);
        }
        if hb_allowed && n >= 1024 {
            p = p.maximum(hb::<C>(n.min(65535))?);
        }
    }
    Some(p)
}
fn envelope<C: CurveConfig>(n: usize, grouping: bool, g2_query: bool) -> Option<Profile>
where
    C::Base: BatchInverse,
{
    let original = raw::<C>(n, g2_query)?;
    if n < 256 || n >= u32::MAX as usize || !grouping || !crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
    {
        return Some(original);
    }
    let percent =
        if (32768..65536).contains(&n) && crate::simd::cpu_option("ZOPRO_CPU_SMALL_GROUPS") {
            95
        } else {
            80
        };
    let u = mul(n, percent)? / 100;
    let (build, retained) = group_limits::<C>(n, u)?;
    let grouped = raw_envelope::<C>(u, g2_query && n < 65536)?.retain(retained)?;
    Some(Profile {
        held: original.held.max(build).max(grouped.held),
        leaf: original.leaf.max(grouped.leaf),
    })
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Geometry {
    pub wires: usize,
    pub slots: usize,
    pub inputs: usize,
    pub public: usize,
    pub domain: usize,
    pub a: usize,
    pub b: usize,
    pub k: usize,
    pub h: usize,
    pub correction: usize,
}

/// Component-only envelope. Sum all suspended parent jobs conservatively;
/// neither the existing raw path nor the proof scheduler selects this probe.
#[cfg(zopro_g2_level_tree)]
pub(super) fn level_tree_peak(n: usize) -> Option<(usize,usize)> {
    if n >= 65536 { return None; }
    let workers=rayon::current_num_threads().max(1);
    let old=raw::<G2Config>(n,true)?;
    let ordinary=add(old.held,mul(old.leaf,workers)?)?;
    let half=2048usize;
    let part=n.div_ceil(2);
    let routing=add(vector::<u32>(part,false)?,mul(vector::<usize>(half+1,false)?,3)?)?;
    // One high result survives low construction; include both outputs and
    // all group buffers, including map_init states retained until return.
    let held=mul(vector::<Affine<G2Config>>(half,false)?,2)?;
    let grouped=crate::msm::workspace_grouping_linear::<G2Config>(part,8)?;
    let per=add(add(routing,held)?,grouped)?;
    let tree=add(ordinary,mul(per,22)?)?;
    Some((ordinary,tree))
}

#[cfg(zopro_g2_prefix_tree)]
pub(super) fn prefix_tree_peak(n:usize)->Option<usize>{
    let (ordinary,_)=level_tree_peak(n)?;
    let part=n.div_ceil(2);let half=2048usize;
    let routing=add(vector::<u32>(part,false)?,mul(vector::<usize>(half+1,false)?,3)?)?;
    let held=mul(vector::<Affine<G2Config>>(half,false)?,2)?;
    let grouped=crate::msm::workspace_g2_prefix_groups(part,8)?;
    add(ordinary,mul(add(add(routing,held)?,grouped)?,22)?)
}
#[cfg(zopro_g2_index_gather)]
pub(super) fn index_gather_peak(n:usize)->Option<usize>{
    let (ordinary,_)=level_tree_peak(n)?;
    let part=n.div_ceil(2);let half=2048usize;
    let routing=add(vector::<u32>(part,false)?,mul(vector::<usize>(half+1,false)?,3)?)?;
    let held=mul(vector::<Affine<G2Config>>(half,false)?,2)?;
    let grouped=crate::msm::workspace_g2_gather(part,8)?;
    add(ordinary,mul(add(add(routing,held)?,grouped)?,22)?)
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Forecast {
    pub fields: usize,
    pub msms: usize,
    pub total: usize,
}
pub(crate) fn forecast(g: Geometry) -> Option<Forecast> {
    if !cfg!(zopro_g2_horner_buckets) {
        return None;
    }
    if !crate::affine_msm()
        || !crate::simd::cpu_option("ZOPRO_CPU_PACKED")
        || !crate::simd::cpu_option("ZOPRO_CPU_DIRECT")
        || !crate::simd::cpu_option("ZOPRO_CPU_UNSIGNED")
        || !crate::simd::cpu_option("ZOPRO_CPU_UNSIGNED_CACHE")
        || !crate::simd::cpu_option("ZOPRO_CPU_QUOTIENT8")
    {
        return None;
    }
    if !g.domain.is_power_of_two() || g.domain < 8 || g.public > g.inputs || g.inputs > g.wires {
        return None;
    }
    if explicit("ZOPRO_CPU_SPLIT")?.is_some_and(|v| v != 1) {
        return None;
    }
    let wires = vector::<Fr>(add(g.wires, g.slots)?, false)?;
    let witness = mul(vector::<Fr>(g.inputs, false)?, 3)?;
    let extracted = add(vector::<Fr>(g.a, false)?, vector::<Fr>(g.b, false)?)?;
    let scalar = vector::<Fr>(g.domain, false)?;
    let expanded = vector::<Ifma8<FrConfig>>(g.domain / 4, false)?;
    let compact = vector::<PackedIfma8<FrConfig>>(g.domain / 4, false)?;
    let packing = add(mul(scalar, 3)?, expanded)?;
    let leaves = add(
        add(mul(scalar, 1 + usize::from(g.correction > 0))?, compact)?,
        add(expanded, mul(g.domain.div_ceil(128), 64)?)?,
    )?;
    let fft = packing.max(leaves);
    let fields = add(
        add(add(wires, witness)?, extracted)?,
        add(fft, vector::<Fr>(g.slots, false)?)?,
    )?;
    let mont_h = g.h >= (1 << 18) && crate::simd::cpu_option("ZOPRO_CPU_MONTGOMERY_H");
    let p = [
        envelope::<G1Config>(g.a, true, false)?,
        envelope::<G1Config>(g.b, true, false)?,
        envelope::<G1Config>(g.k, true, false)?,
        envelope::<G2Config>(g.b, true, true)?,
        envelope::<G1Config>(g.h, !mont_h, false)?,
        envelope::<G1Config>(g.correction, false, false)?,
    ];
    let held = p.iter().try_fold(0usize, |a, p| add(a, p.held))?;
    let leaf = p.iter().map(|p| p.leaf).max().unwrap_or(0);
    #[cfg(zopro_real_kh)]
    let (held, leaf) = if super::fused_kh::eligible(true, Fr::ONE, g.k, g.h) {
        // Internal randomized proving may replace K/H with one width-13
        // stream. Keep the maximum of separate and shared alternatives;
        // public parts and mismatched scaling still use separate queries.
        // All twenty parents are charged, including the indexed top window.
        let mut shared = shape::<G1Config>(1 << 12, 255usize.div_ceil(13), 1,
            batch_size(13).max(32))?;
        // The highest 7-bit window performs two original indexed reductions,
        // potentially concurrently; retain both parent arrays and max leaf.
        for n in [g.k, g.h] {
            let t = top::<G1Config>(n, 7)?;
            shared.held = add(shared.held, t.held)?;
            shared.leaf = shared.leaf.max(t.leaf);
        }
        let others = held.checked_sub(p[2].held)?.checked_sub(p[4].held)?;
        (held.max(add(others, shared.held)?), leaf.max(shared.leaf))
    } else { (held, leaf) };
    let msms = add(held, mul(leaf, rayon::current_num_threads().max(1))?)?;
    Some(Forecast {
        fields,
        msms,
        total: add(add(fields, msms)?, 64 * 1024)?,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_phase_forecast_overflow_and_cache_envelopes() {
        assert!(vector::<Fr>(usize::MAX, true).is_none());
        assert!(hash_table::<([u64; 4], u32)>(usize::MAX).is_none());
        for n in [256, 1024, 65535, 65536, 262143, 262144, 1048576] {
            if explicit("ZOPRO_CPU_UNSIGNED_WIDTH").unwrap().is_none() {
                assert!(unsigned_bits::<G1Config>(n) <= 12 && unsigned_bits::<G2Config>(n) <= 11);
            }
        }
    }
    #[test]
    fn grouped_metadata_endpoints_cover_every_cardinality() {
        // Exhaust the cardinality range with the same linear typed upper bounds;
        // do not derive the rule from any proving fixture's observed allocation.
        let workers = rayon::current_num_threads().max(1);
        let classes = workers * 256;
        for n in [256usize, 1025, 8193, 32769] {
            for curve in [1, 2] {
                let (point, limb, group) = if curve == 1 {
                    (size_of::<zopro_bn254::G1Affine>(), 64, 100)
                } else {
                    (size_of::<zopro_bn254::G2Affine>(), 128, 180)
                };
                let bound = |u: usize| {
                    (88 + point) * u + 4 * n + (limb + 8 + group) * (n - u + u.min(classes))
                };
                let u = n * 80 / 100;
                let max = [1, u, u.min(classes).max(1)]
                    .into_iter()
                    .map(bound)
                    .max()
                    .unwrap();
                for v in 1..=u {
                    assert!(bound(v) <= max);
                }
            }
        }
    }
}
