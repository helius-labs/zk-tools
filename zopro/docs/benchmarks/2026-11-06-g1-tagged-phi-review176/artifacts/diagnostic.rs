/// Read-only inputs for a private work model; never called by proving.
#[doc(hidden)]
pub struct TaggedPhiInputs {
    pub k: Vec<[u64; 4]>,
    pub h: Vec<[u64; 4]>,
    pub k_identity: Vec<bool>,
    pub h_identity: Vec<bool>,
    pub geometry: [usize; 5],
}
impl Key {
    #[doc(hidden)]
    pub fn tagged_phi_inputs(&self, witness: &[u8]) -> Result<TaggedPhiInputs, String> {
        let plan = self.solver.as_ref().ok_or("missing solver")?;
        let q = self.lagrange.as_ref().ok_or("missing product query")?;
        let folded = q.folded.as_ref().ok_or("missing folded private query")?;
        assert!(q.euler && q.product_scale == Fr::ONE);
        let solver::Solution { w, a, b, c } =
            plan.solve_in_with_slots(&frs(witness), self.domain, &folded.slots, false)?;
        drop(c);
        let product = q.product_values(&self.plan, a, b);
        let k = w[plan.n_public..]
            .iter()
            .map(Fr::to_canonical)
            .collect::<Vec<_>>();
        let h = product.iter().map(Fr::to_canonical).collect::<Vec<_>>();
        assert_eq!(k.len(), folded.private.len());
        assert_eq!(h.len(), q.product.len());
        assert!(msm_affine::fused_kh::eligible(
            true,
            q.product_scale,
            k.len(),
            h.len()
        ));
        Ok(TaggedPhiInputs {
            k,
            h,
            k_identity: folded.private.iter().map(|p| p.is_identity()).collect(),
            h_identity: q.product.iter().map(|p| p.is_identity()).collect(),
            geometry: [
                self.domain,
                plan.n_public,
                folded.slots.len(),
                folded.private.len(),
                q.product.len(),
            ],
        })
    }
}
#[doc(hidden)]
pub fn tagged_phi_split(s: [u64; 4]) -> [u64; 4] {
    msm_affine::tagged_phi_split_diagnostic(s)
}
