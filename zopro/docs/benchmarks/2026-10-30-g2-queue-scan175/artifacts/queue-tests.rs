
#[cfg(test)]
mod scan_queue_tests {
    use super::*;
    use zopro_bn254::Fr;

    #[test]
    fn blocked_tail_route_and_real_curve_outputs() {
        let g=G2Projective::generator();let p=(g*Fr::from(3u64)).to_affine();
        let mut s=Window::new(16,8,BatchAdd::default());
        s.affine[1]=(g*Fr::from(7u64)).to_affine();s.affine[2]=(g*Fr::from(11u64)).to_affine();
        s.queue=vec![(1,&p),(2,&p),(2,&p)];s.take_from_queue();
        if cfg!(zopro_g2_queue_scan){assert_eq!(s.batch.len(),2);assert_eq!(s.queue.len(),1);}else{assert_eq!(s.batch.len(),1);assert_eq!(s.queue.len(),2);}
        s.execute();s.flush_queue();let out=s.finish();
        assert_eq!(out[1],(g*Fr::from(10u64)).to_affine());assert_eq!(out[2],(g*Fr::from(17u64)).to_affine());
        assert!(out.iter().enumerate().all(|(i,p)|i==1||i==2||p.is_identity()));
    }

    #[test]
    fn actual_window_full_tails_and_exceptional_points() {
        let g=G2Projective::generator();
        for n in [0usize,1,7,8,9,199,200,201,1025,4097] {for pattern in 0..4 {
            let half=512;let size=200;let mut expected=vec![Fr::ZERO;half];let mut raw=Vec::with_capacity(n);let mut short=Vec::with_capacity(n);
            for i in 0..n {
                let b=match pattern{0=>i%half,1=>if i%5==0{3}else{i*37%half},2=>i%3,_=>i*1597%half};
                let coefficient=match pattern{2=>if i%7==0{Fr::ZERO}else{Fr::ONE},3=>if i%9==0{-Fr::ONE}else{Fr::from((i%31+1)as u64)},_=>Fr::from((i%127+1)as u64)};
                let neg=i%3==0;expected[b]+=if neg{-coefficient}else{coefficient};raw.push(g*coefficient);short.push([(b+1)as u64,if neg{1u64<<63}else{0},0,0]);
            }
            let bases=G2Projective::batch_to_affine(&raw);let before=bases.clone();let mut s=Window::new(half,size,BatchAdd::default());
            s.feed(&bases,&short,false,0,12,size);assert!(s.batch.is_empty()&&s.queue.is_empty());assert!(s.busy.iter().all(|&w|w==0));assert!(s.batch.capacity()>=size&&s.queue.capacity()>=size);
            let out=s.finish();for(b,p)in out.iter().enumerate(){assert_eq!(*p,(g*expected[b]).to_affine(),"n={n} pattern={pattern} bucket={b}");}assert_eq!(bases,before);
        }}
    }
}
