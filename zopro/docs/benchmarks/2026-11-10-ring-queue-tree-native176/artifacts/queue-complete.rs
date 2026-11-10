// Fixed queue limit derives from the original batch200 / queue199 geometry.
// This changes no Window field. Local roots outlive their borrowed descriptors.
impl<'a> Window<'a> {
    fn ring_complete(&mut self) {
        self.execute();
        let n = self.queue.len();
        if n == 0 { return; }
        assert!(n <= 199);
        let mut order = [0u32; 199];
        for (i, out) in order[..n].iter_mut().enumerate() { *out = i as u32; }
        order[..n].sort_unstable_by_key(|&i| self.queue[i as usize].0 & 0x7fff_ffff);
        let mut starts = [0usize; 200];
        let mut counts = [0usize; 199];
        let mut buckets = [0u32; 199];
        let mut groups = 0;
        for (i, &j) in order[..n].iter().enumerate() {
            let bucket = self.queue[j as usize].0 & 0x7fff_ffff;
            if i == 0 || buckets[groups - 1] != bucket {
                starts[groups] = i;
                buckets[groups] = bucket;
                groups += 1;
            }
            counts[groups - 1] += 1;
        }
        starts[groups] = n;
        // <=199 output groups fit the existing GROUP=256 reducer's single
        // chunk: no nested split while this streaming Window is suspended.
        let roots = crate::msm::group_affine_impl::<G2Config, _>(
            |i| {
                let (encoded, p) = self.queue[i as usize];
                if encoded >> 31 != 0 { -*p } else { *p }
            },
            &order[..n], &starts[..=groups], &counts[..groups],
        );
        self.queue.clear();
        let mut attachments = Vec::with_capacity(groups);
        for (&bucket, root) in buckets[..groups].iter().zip(&roots) {
            if root.is_identity() { continue; }
            let a = &mut self.affine[bucket as usize];
            if a.is_identity() { *a = *root; continue; }
            if a.x == root.x {
                if a.y == root.y { self.projective(bucket as usize, *root); }
                else { *a = G2Affine::IDENTITY; }
                continue;
            }
            attachments.push((bucket, root));
        }
        self.packed.apply_signed_borrowed_g2(&mut self.affine, &attachments);
        debug_assert!(self.batch.is_empty() && self.queue.is_empty());
        debug_assert!(self.busy.iter().all(|&word| word == 0));
    }
}
