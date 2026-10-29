
#[cfg(zopro_queue_drain)]
impl<'a> Window<'a>{
    fn drain_queued_batches(&mut self){
        if !super::queue_drain::eligible(&self.queue,self.batch.len()){return;}
        self.execute();
        while super::queue_drain::eligible(&self.queue,0){
            self.take_from_queue();
            self.execute();
        }
    }
}
#[cfg(test)]mod queue_drain_tests{
    use super::*;
    use zopro_bn254::Fr;
    #[test]fn drain_and_fallback_match_independent_buckets(){
        let g=G2Projective::generator();let buckets=64;let lanes=crate::simd::BATCH_LANES;
        let coefficients=(0..200).map(|i|Fr::from(i as u64+100001)).collect::<Vec<_>>();let incoming=G2Projective::batch_to_affine(&coefficients.iter().map(|c|g**c).collect::<Vec<_>>());
        let initial=(0..buckets).map(|i|Fr::from(i as u64+71)).collect::<Vec<_>>();let bases=G2Projective::batch_to_affine(&initial.iter().map(|c|g**c).collect::<Vec<_>>());
        for pending in[0,lanes-1,lanes,32]{for n in[0,1,lanes-1,lanes,lanes+1,31,127,199]{for shape in 0..4{
            let mut state=Window::new(buckets,200,BatchAdd::default());state.affine.clone_from(&bases);let mut expected=initial.clone();
            for i in 0..pending{state.add(i as u32,&incoming[i]);expected[i]+=coefficients[i];}
            for i in 0..n{let b=match shape{0=>i%buckets,1=>0,2=>(i/2)%buckets,_=>(i*13)%buckets};let negative=(shape==2&&i%2!=0)||(shape==3&&i%3==0);let k=if shape==2{i/2}else{i};let encoded=b as u32|if negative{1<<31}else{0};state.queue.push((encoded,&incoming[k]));expected[b]+=if negative{-coefficients[k]}else{coefficients[k]};}
            state.flush_queue();assert!(state.queue.is_empty());state.execute();let actual=state.finish();for i in 0..buckets{assert_eq!(actual[i],(g*expected[i]).to_affine(),"pending{pending} n{n} shape{shape} b{i}");}
        }}}
        let mut state=Window::new(buckets,200,BatchAdd::default());state.affine.clone_from(&bases);for i in 0..lanes{state.add(i as u32,&incoming[i]);state.queue.push((i as u32,&incoming[i+lanes]));}
        state.flush_queue();assert_eq!(state.batch.is_empty(),cfg!(zopro_queue_drain));assert_eq!(state.projective.is_empty(),cfg!(zopro_queue_drain));
        let mut state=Window::new(buckets,200,BatchAdd::default());state.affine.clone_from(&bases);let mut expected=initial.clone();state.affine[0]=incoming[0];expected[0]=coefficients[0];state.affine[1]=-incoming[1];expected[1]=-coefficients[1];state.affine[2]=G2Affine::IDENTITY;expected[2]=Fr::ZERO;
        for i in 0..lanes{state.queue.push((i as u32,&incoming[i]));expected[i]+=coefficients[i];}state.flush_queue();state.execute();let actual=state.finish();for i in 0..buckets{assert_eq!(actual[i],(g*expected[i]).to_affine(),"exception bucket{i}");}
    }
}
