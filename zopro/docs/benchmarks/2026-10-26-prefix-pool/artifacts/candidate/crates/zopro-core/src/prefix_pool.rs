//! Small-proof preparation before entering the global arithmetic pool.
//! Experimental, explicit opt-in; no key tables or copied wire buffers.
use super::*;
use rayon::prelude::*;
use std::sync::OnceLock;

const WORKERS: usize = 8;
const STACK_BYTES: usize = 2 << 20;
// A forecast, not a measured upper bound: explicit stacks plus 128KiB per
// worker for guard pages/registry/allocator bookkeeping. Retained for pool life.
const FORECAST_BYTES: usize = WORKERS * (STACK_BYTES + (128 << 10));

pub(super) struct Pool {
    workers: rayon::ThreadPool,
    _charge: preparation::Reservation,
}
static POOL: OnceLock<Option<Pool>> = OnceLock::new();

fn route(enabled: bool, workers: usize, in_pool: bool, concurrency: Option<usize>) -> bool {
    enabled && workers > 16 && !in_pool && concurrency == Some(1)
}

pub(super) fn select() -> Option<&'static Pool> {
    // Avoid even initializing Rayon or querying headroom when disabled.
    if std::env::var("ZOPRO_CPU_PREFIX_POOL").ok().as_deref() != Some("1") {
        return None;
    }
    let concurrency = match std::env::var("ZOPRO_CPU_PROOF_CONCURRENCY") {
        Ok(v) => v.parse().ok(),
        Err(std::env::VarError::NotPresent) => Some(1),
        Err(_) => None,
    };
    if !route(true, rayon::current_num_threads(), rayon::current_thread_index().is_some(), concurrency) {
        return None;
    }
    POOL.get_or_init(|| {
        let charge = preparation::Reservation::analysis(Some(FORECAST_BYTES))?;
        let workers = rayon::ThreadPoolBuilder::new().num_threads(WORKERS)
            .stack_size(STACK_BYTES).thread_name(|i|format!("zopro-prefix-{i}"))
            .build().ok()?;
        Some(Pool { workers, _charge: charge })
    }).as_ref()
}

// w is exclusively owned, converted exactly once and never used as field
// elements afterward. Moving this object between pools moves Vec descriptors.
struct Prepared {
    w: Vec<Fr>,
    a: Vec<[u64;4]>,
    b: Vec<[u64;4]>,
    product: Vec<Fr>,
    correction: G1Projective,
}

fn canonical_gathers(mut w: Vec<Fr>, plan: &solver::Plan) -> (Vec<Fr>, Vec<[u64;4]>, Vec<[u64;4]>) {
    let words = msm_affine::canonical(&mut w);
    let (a,b) = rayon::join(
        || plan.a_indices.par_iter().map(|&i|words[i as usize]).collect(),
        || plan.b_indices.par_iter().map(|&i|words[i as usize]).collect());
    (w,a,b)
}

pub(super) fn solve(pool: &Pool, key: &Key, plan: &solver::Plan, witness: &[Fr],
    materialize_c: bool, queries: &lagrange::Queries, folded: &lagrange::Folded,
    reclaim: Option<&alloc::ProofScope>) -> Result<Vec<u8>,String>
{
    if std::env::var_os("ZOPRO_PREFIX_POOL_TRACE").is_some() {
        eprintln!("PREFIX_POOL preparation={} arithmetic={}",WORKERS,rayon::current_num_threads());
    }
    let prepared = pool.workers.install(|| {
        let solver::Solution {mut w,a,b,c} = plan.solve_in_with_slots(witness,key.domain,&folded.slots,materialize_c)?;
        if let Some(scope) = reclaim { scope.release_unused(&mut w); }
        drop(c);
        let correction = folded.public_term(&w[..plan.n_public]);
        let (w,sa,sb) = canonical_gathers(w,plan);
        let product = queries.product_values(&key.plan,a,b);
        Ok::<_,String>(Prepared {w,a:sa,b:sb,product,correction})
    })?;
    // install has returned: no preparation jobs can still use these buffers.
    // Only now enter the global pool. The caller owns the single ProofScope.
    rayon::join(|| {
        let Prepared {w,a,mut b,mut product,correction} = prepared;
        let sk = &msm_affine::storage_words(&w)[plan.n_public..];
        let z = &queries.product[..product.len()];
        let ((ar,kh),bs2) = rayon::join(
            || rayon::join(||msm_affine::msm_canonical(&key.a,&a),
                ||msm_affine::fused_kh::fused(&folded.private,sk,z,msm_affine::canonical(&mut product),13)),
            || {
                if (1024..65536).contains(&key.b2.len()) {
                    msm_affine::g2_query_exclusive(&key.b2,&mut b)
                } else {
                    let groups = crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
                        .then(||msm_affine::scalar_groups(&b)).flatten();
                    groups.as_ref().map_or_else(||msm_raw_b2(&key.b2,&b),
                        |g|msm_grouped_b2(&key.b2,g))
                }
            });
        let mut out = Vec::with_capacity(384);
        for p in [ar,G1Projective::IDENTITY,kh+correction,G1Projective::IDENTITY] { output_g1(&mut out,p); }
        output_g2(&mut out,bs2);
        Ok(out)
    },||()).0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routing_preserves_low_core_custom_pool_and_concurrency_fallbacks() {
        assert!(route(true,32,false,Some(1)));
        for n in 0..=16 { assert!(!route(true,n,false,Some(1))); }
        assert!(!route(false,96,false,Some(1)));
        assert!(!route(true,96,true,Some(1)));
        for c in [None,Some(0),Some(2),Some(usize::MAX)] { assert!(!route(true,96,false,c)); }
        let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();
        assert!(pool.install(||rayon::current_thread_index().is_some()));
    }
    #[test]
    fn canonical_storage_moves_without_conversion_or_buffer_copy() {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(WORKERS).build().unwrap();
        let values:Vec<_> = (0..1024).map(|i|Fr::from(i as u64)).collect();
        let expected:Vec<_> = values.iter().map(Fr::to_canonical).collect();
        let ptr = values.as_ptr() as usize;
        let moved = pool.install(move || {
            let mut values=values;
            assert_eq!(rayon::current_num_threads(),WORKERS);
            assert_eq!(msm_affine::canonical(&mut values),expected);
            values
        });
        assert_eq!(moved.as_ptr() as usize,ptr);
        for (i,words) in msm_affine::storage_words(&moved).iter().enumerate() {
            assert_eq!(*words,Fr::from(i as u64).to_canonical());
        }
    }
    #[test]
    fn proof_scope_can_cover_both_pools() {
        fn send_sync<T:Send+Sync>() {}
        send_sync::<alloc::ProofScope>();
        send_sync::<Prepared>();
        send_sync::<Pool>();
        assert_eq!(FORECAST_BYTES,17 << 20);
    }
}
