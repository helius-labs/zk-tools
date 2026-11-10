    #[cfg(all(zopro_simd_bucket_admission,target_arch="x86_64",target_feature="avx512f",target_feature="avx512cd"))]
    #[inline]
    fn admit_packet(&mut self, points: [&'a @POINT@;8], digits: [i16;8], size: usize) -> bool {
        if self.batch.len().saturating_add(8)>size || digits.iter().any(|d|*d==0) { return false; }
        let ids=digits.map(|d|u64::from(d.unsigned_abs()-1));
        if ids.iter().any(|&i|i>=self.affine.len()as u64) {return false;}
        let words=ids.map(|i|self.busy[i as usize/64]);
        let bucket_x=ids.map(|i|self.affine[i as usize].@FIRST_X@);
        let incoming_x=points.map(|p|p.@FIRST_X@);
        if !super::packet_admission::admissible(&ids,&words,&bucket_x,&incoming_x){return false;}
        // Every old add() would take its ordinary branch. Distinct destinations
        // and room for all eight preserve the original execution boundary.
        for i in 0..8 {
            let b=ids[i]as usize;
            self.busy[b/64]|=1u64<<(b%64);
            self.batch.push((b as u32|if digits[i]<0{1<<31}else{0},points[i]));
        }
        #[cfg(test)] ADMITTED_PACKETS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
        if self.batch.len()==size {self.execute();self.take_from_queue();}
        true
    }
