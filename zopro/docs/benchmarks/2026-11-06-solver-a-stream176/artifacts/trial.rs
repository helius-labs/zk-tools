//! Diagnostic complete solver→A caller. This is not a proving API dispatch.
use super::*;
use crate::msm_affine::borrowed_pending::early_stream::{self, Stream};
use crate::solver::early_feed::{Published, Ready};
use rayon::prelude::*;

pub struct Prepared<'key> {
    key: &'key Key,
    ready: Ready<'key>,
    forecast: usize,
}

pub struct Output {
    pub point: G1Projective,
    pub streamed: bool,
    pub grouped_fallback: bool,
    pub admission_fallback: bool,
    pub published_chunks: usize,
    pub early_chunks: usize,
    pub early_values: usize,
    pub jobs: usize,
    pub feeds_before_solver_return: usize,
    pub forecast_bytes: usize,
}

fn configuration() -> bool {
    affine_msm()
        && msm_affine::borrowed_pending::ordinary_enabled()
        && crate::simd::cpu_option("ZOPRO_CPU_TOP_TREE")
        && std::env::var("ZOPRO_CPU_DEFER_SIGN_WIDTH").as_deref() == Ok("12")
        && std::env::var("ZOPRO_CPU_DEFER_SIGN_PARTS").as_deref() == Ok("1")
}

impl<'key> Prepared<'key> {
    pub(crate) fn new(key: &'key Key) -> Option<Self> {
        if !configuration()
            || !key.canonical_wires
            || key.has_gpu()
            || key.equal_witness.is_some()
            || !(1024..65536).contains(&key.a.len())
        {
            return None;
        }
        let plan = key.solver.as_ref()?;
        let queries = key.lagrange.as_ref()?;
        let folded = queries.folded.as_ref()?;
        // Match the existing small fused-query implementation boundary.
        if !msm_affine::fused_kh::eligible(
            true,
            queries.product_scale,
            folded.private.len(),
            queries.product.len(),
        ) {
            return None;
        }
        let ready = Ready::prepare(plan)?;
        let forecast = key
            .proof_workspace?
            .checked_add(early_stream::forecast(key.a.len())?)?
            .checked_add(ready.chunk_count().checked_mul(2)?)?;
        Some(Self {
            key,
            ready,
            forecast,
        })
    }

    fn reference(&self, witness: &[Fr], admission_fallback: bool) -> Result<Output, String> {
        let plan = self.key.solver.as_ref().unwrap();
        let folded = self.key.lagrange.as_ref().unwrap().folded.as_ref().unwrap();
        let mut solution =
            plan.solve_in_with_slots(witness, self.key.domain, &folded.slots, true)?;
        let words = msm_affine::canonical(&mut solution.w);
        let scalars = plan
            .a_indices
            .par_iter()
            .map(|&i| words[i as usize])
            .collect::<Vec<_>>();
        let point = msm_affine::msm_canonical(&self.key.a, &scalars);
        std::hint::black_box((&solution.a, &solution.b, &solution.c));
        Ok(Output {
            point,
            streamed: false,
            grouped_fallback: false,
            admission_fallback,
            published_chunks: 0,
            early_chunks: 0,
            early_values: 0,
            jobs: 0,
            feeds_before_solver_return: 0,
            forecast_bytes: self.forecast,
        })
    }

    pub fn run(&self, witness_bytes: &[u8], on: bool) -> Result<Output, String> {
        if rayon::current_thread_index().is_none() {
            return rayon::join(|| self.run(witness_bytes, on), || ()).0;
        }
        let _proof_scope = alloc::ProofScope::enter(self.key.proof_workspace);
        let witness = frs(witness_bytes);
        if !on || !configuration() {
            return self.reference(&witness, false);
        }
        let Some(_reservation) = preparation::Reservation::analysis(Some(self.forecast)) else {
            return self.reference(&witness, true);
        };
        let plan = self.key.solver.as_ref().unwrap();
        let folded = self.key.lagrange.as_ref().unwrap().folded.as_ref().unwrap();
        let mut stream = Stream::new(&self.key.a);
        let mut held_solution = None;
        let outcome = rayon::scope(
            |scope| -> Result<(Option<G1Projective>, Option<G1Projective>, Published), String> {
                let result =
                    self.ready
                        .solve(&witness, self.key.domain, &folded.slots, true, |chunk| {
                            stream.publish(scope, chunk)
                        });
                stream.solver_returned();
                let (mut solution, published) = match result {
                    Ok(v) => v,
                    Err(e) => {
                        stream.cancel(scope);
                        return Err(e);
                    }
                };
                let words = msm_affine::canonical(&mut solution.w);
                let scalars = plan
                    .a_indices
                    .par_iter()
                    .map(|&i| words[i as usize])
                    .collect::<Vec<_>>();
                held_solution = Some(solution);
                let groups = crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
                    .then(|| msm_affine::scalar_groups(&scalars))
                    .flatten();
                if let Some(groups) = groups {
                    stream.cancel(scope);
                    let point = msm_affine::msm_grouped(&self.key.a, &groups);
                    return Ok((Some(point), None, published));
                }
                stream.close(scope)?;
                let top = stream.top(&self.key.a, &scalars);
                Ok((None, Some(top), published))
            },
        )?;
        std::hint::black_box(&held_solution);
        let (point, jobs, grouped) = if let Some(point) = outcome.0 {
            (point, 0, true)
        } else {
            let (point, jobs) = stream.result(outcome.1.unwrap());
            (point, jobs, false)
        };
        Ok(Output {
            point,
            streamed: true,
            grouped_fallback: grouped,
            admission_fallback: false,
            published_chunks: outcome.2.chunks,
            early_chunks: outcome.2.early_chunks,
            early_values: outcome.2.early_values,
            jobs,
            feeds_before_solver_return: stream.early_feeds(),
            forecast_bytes: self.forecast,
        })
    }
}
