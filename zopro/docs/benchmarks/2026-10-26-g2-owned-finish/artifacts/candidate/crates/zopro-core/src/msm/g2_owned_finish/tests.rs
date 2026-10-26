use super::*;
use zopro_bn254::{G2Config,G2Affine,G2Projective};
fn points(n:usize,start:u64)->(Vec<G2Affine>,Vec<Fr>){
    let g=G2Affine::GENERATOR;let mut p=g*Fr::from(start);let mut out=Vec::new();let mut weights=Vec::new();
    for i in 0..n{let weight=Fr::from(start+i as u64);let negative=i%3==0;out.push(if negative{-p}else{p});weights.push(if negative{-weight}else{weight});p+=g;}
    (G2Projective::batch_to_affine(&out),weights)
}
#[test]
fn full_batches_match_generator_oracle_and_consume_only_x(){
    for n in [8usize,16,80,200,352,512] {
        let size=(n+1).next_power_of_two();let (base,bw)=points(size,7);let(incoming,iw)=points(n,1_000_003);
        let indices=(0..n).map(|i|i*37%size).collect::<Vec<_>>();
        let original=incoming.iter().enumerate().map(|(i,&p)|(indices[i]as u32,p)).collect::<Vec<_>>();
        let mut weights=bw;for i in 0..n{weights[indices[i]]+=iw[i];}
        let expected=G2Projective::batch_to_affine(&weights.iter().map(|&w|G2Affine::GENERATOR*w).collect::<Vec<_>>());
        for canonical in [false,true]{for vector_inverse in[false,true]{
            let mut buckets=base.clone();let mut batch=original.clone();let mut scratch=BatchAdd::<G2Config>::default();scratch.direct=true;scratch.fq2.prefix_enabled=true;scratch.fq2.canonical=canonical;scratch.fq2.vector_inverse=vector_inverse;
            let mut reference=base.clone();let mut ref_work=Fq2AddScratch{canonical,vector_inverse,..Default::default()};
            if canonical{super::super::g2_prefix::apply::<G2Config,true>(&mut reference,&original,&mut ref_work);}else{super::super::g2_prefix::apply::<G2Config,false>(&mut reference,&original,&mut ref_work);}
            scratch.apply_owned(&mut buckets,&mut batch);assert_eq!(buckets,expected);assert_eq!(buckets,reference);
            for i in 0..n{assert_eq!(batch[i].0,original[i].0);assert_eq!(batch[i].1.x,base[indices[i]].x);assert_eq!(batch[i].1.y,original[i].1.y);}
            assert_eq!(scratch.fq2.groups.capacity(),0);assert_eq!(scratch.fq2.prefix_records.len(),n/LANES);assert_eq!(scratch.fq2.prefix_records.capacity(),ref_work.prefix_records.capacity());
        }}
    }
}
#[test]
fn fallbacks_preserve_incoming_points_and_match_complete_addition(){
    for n in[0usize,1,7,8,9,15,16,17,80]{let size=(n+1).next_power_of_two();let(base,_)=points(size,13);let(incoming,_)=points(n,1_000_033);let original=incoming.iter().enumerate().map(|(i,&p)|(i as u32,p)).collect::<Vec<_>>();let mut expected=base.clone();for &(i,p)in &original{expected[i as usize]=(base[i as usize]+p).to_affine();}
        for (direct,prefix)in[(false,false),(false,true),(true,false),(true,true)]{if direct&&prefix&&n>0&&n%LANES==0{continue;}let mut work=BatchAdd::<G2Config>::default();work.direct=direct;work.fq2.prefix_enabled=prefix;for canonical in[false,true]{for vector_inverse in[false,true]{work.fq2.canonical=canonical;work.fq2.vector_inverse=vector_inverse;let mut buckets=base.clone();let mut batch=original.clone();work.apply_owned(&mut buckets,&mut batch);assert_eq!(buckets,expected);assert_eq!(batch,original);}}}
    }
}
#[test]
fn reused_owned_scratch_preserves_layer_and_bucket_contract(){
    let(mut actual,mut weights)=points(1024,97);let mut work=BatchAdd::<G2Config>::default();work.direct=true;work.fq2.prefix_enabled=true;
    for(round,n)in[512usize,8,352,16,200,8].into_iter().enumerate(){let(input,iw)=points(n,10_000_019+round as u64*1_000_003);let mut batch=input.iter().enumerate().map(|(i,&p)|(((i*73+round*127)%1024)as u32,p)).collect::<Vec<_>>();let before=actual.clone();let indices=batch.iter().map(|x|x.0).collect::<Vec<_>>();for i in 0..n{let k=indices[i]as usize;assert_ne!(actual[k].x,input[i].x);weights[k]+=iw[i];}work.fq2.canonical=round%2==0;work.fq2.vector_inverse=round%3!=0;work.apply_owned(&mut actual,&mut batch);
        for i in 0..n{let k=indices[i]as usize;assert_eq!(batch[i].0,indices[i]);assert_eq!(batch[i].1.x,before[k].x);}
        let expected=G2Projective::batch_to_affine(&weights.iter().map(|&w|G2Affine::GENERATOR*w).collect::<Vec<_>>());assert_eq!(actual,expected);assert_eq!(work.fq2.groups.capacity(),0);
    }
}
