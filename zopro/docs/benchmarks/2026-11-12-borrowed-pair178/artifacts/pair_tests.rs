#[test]
fn both_signs_padding_tails_and_poisoned_destinations() {
    let (input,weights)=points(70);let original=input.clone();
    let indices=(0..input.len()).filter(|&i|!input[i].is_identity()).take(36).collect::<Vec<_>>();
    let g=G2Projective::generator();
    let expected=G2Projective::batch_to_affine(&(0..18).flat_map(|i| {
        let a=weights[indices[2*i]];let b=weights[indices[2*i+1]];
        [g*(a+b),g*(a-b),g*(-a+b),g*(-a-b)]
    }).collect::<Vec<_>>());
    let mut adder=BatchAdd::<G2Config>::default();assert!(adder.borrowed_g2_enabled());
    let mut storage=PairStorage::default();let mut output=vec![input[indices[35]];32];
    for mask in 0u8..=255 { for n in 0..=18 {
        let mut batch=storage.take();batch.reserve_exact(32);output.fill(input[indices[35]]);
        for i in 0..n {
            let left=Leaf {point:&input[indices[2*i]],negative:mask>>(i%8)&1!=0};
            let right=Leaf {point:&input[indices[2*i+1]],negative:(mask.rotate_left(3)^0x5a)>>(i%8)&1!=0};
            assert!(!append(&mut output,&mut batch,i,left,right));
        }
        // Exercise scalar tails without padding, as well as the tree's padded path.
        for padded in [false,true] {
            output.fill(input[indices[35]]);
            if padded {let(real,dummy)=execute(&mut output,&mut batch,&mut adder,n);assert_eq!((real,dummy),(n,(8-n%8)%8));}
            else {adder.apply_signed_pair_g2(&mut output,&batch);}
            for i in 0..n {let l=(mask>>(i%8)&1)as usize;let r=((mask.rotate_left(3)^0x5a)>>(i%8)&1)as usize;assert_eq!(output[i],expected[4*i+2*l+r]);}
        }
        let address=batch.as_ptr();let capacity=batch.capacity();storage.restore(batch);
        assert!(storage.0.is_empty());assert_eq!(storage.0.as_ptr(),address);assert_eq!(storage.0.capacity(),capacity);
    }}
    assert_eq!(input,original);
}
