
#[cfg(test)]
mod fallback_stack_test {
    use super::*;
    #[test]
    fn owned_borrowed_full_tails_and_inverse_modes_on_small_stack() {
        std::thread::Builder::new().stack_size(512 * 1024).spawn(|| {
            let generator = Projective::<G2Config>::generator();
            for n in [0usize, 1, 7, 8, 9, 199, 200, 201, 511, 512] {
                let initial = (0..n).map(|i| (generator * Fr::from((10000+i) as u64)).to_affine()).collect::<Vec<_>>();
                let incoming = (0..n).map(|i| (generator * Fr::from((200000+i) as u64)).to_affine()).collect::<Vec<_>>();
                let expected = (0..n).map(|i| {
                    let a=Fr::from((10000+i) as u64);let b=Fr::from((200000+i) as u64);
                    (generator * if i%3==0 {a-b} else {a+b}).to_affine()
                }).collect::<Vec<_>>();
                let owned=incoming.iter().enumerate().map(|(i,p)|(i as u32,if i%3==0 {-*p} else {*p})).collect::<Vec<_>>();
                let borrowed=incoming.iter().enumerate().map(|(i,p)|((i as u32)|if i%3==0 {1<<31} else {0},p)).collect::<Vec<_>>();
                for canonical in [false,true] {for vector_inverse in [false,true] {
                    for signed_borrowed in [false,true] {
                        let mut buckets=initial.clone();let mut scratch=Fq2AddScratch {canonical,vector_inverse,..Default::default()};
                        if signed_borrowed {
                            if canonical {super::super::g2_borrowed::apply::<G2Config,true>(&mut buckets,&borrowed,&mut scratch)}
                            else {super::super::g2_borrowed::apply::<G2Config,false>(&mut buckets,&borrowed,&mut scratch)}
                        } else if canonical {super::super::g2_prefix::apply::<G2Config,true>(&mut buckets,&owned,&mut scratch)}
                        else {super::super::g2_prefix::apply::<G2Config,false>(&mut buckets,&owned,&mut scratch)}
                        assert_eq!(buckets,expected,"n={n} canonical={canonical} vector={vector_inverse} borrowed={signed_borrowed}");
                    }
                }}
            }
        }).unwrap().join().unwrap();
    }
}
