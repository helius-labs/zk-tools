mod kernel;
use ark_ff::{AdditiveGroup, Field as ArkField, PrimeField};
use kernel::{candidate,control,pack,unpack,Roots};
use zopro_bn254::Fr;

fn rev(j:usize)->usize{j.reverse_bits()>>(usize::BITS-4)}
fn af(x:Fr)->ark_bn254::Fr{ark_bn254::Fr::from_be_bytes_mod_order(&x.to_be_bytes())}
fn next(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
fn random(s:&mut u64)->Fr{let mut b=[0;64];for c in b.chunks_exact_mut(8){c.copy_from_slice(&next(s).to_le_bytes());}Fr::from_random_bytes(&b)}
fn expected<const FORWARD:bool>(x:&[Fr;32],w:Fr)->[ark_bn254::Fr;32]{
    let w=af(w);
    core::array::from_fn(|j|{
        let half=j/16;
        let k=if FORWARD{j%16}else{rev(j%16)};
        let step=w.pow([k as u64]);
        let mut p=ark_bn254::Fr::ONE;
        let mut out=ark_bn254::Fr::ZERO;
        for t in 0..16{let pos=if FORWARD{rev(t)}else{t};out+=af(x[16*half+pos])*p;p*=step;}
        out
    })
}
fn check<const FORWARD:bool>(x:[Fr;32],r:&Roots){
    let mut a=pack(&x);let mut b=a;
    control::<FORWARD>(&mut a,r);candidate::<FORWARD>(&mut b,r);
    let oracle=expected::<FORWARD>(&x,r.w);
    assert_eq!(unpack(&a).map(af),oracle,"original packed control");
    assert_eq!(unpack(&b).map(af),oracle,"new packed candidate");
}

#[test]
fn all_root_basis_matrices_and_both_orders(){
    for power in (1..16).step_by(2){let r=Roots::new(power);for basis in 0..32{let x:[Fr;32]=core::array::from_fn(|j|if j==basis{Fr::ONE}else{Fr::ZERO});check::<false>(x,&r);check::<true>(x,&r);}}
}

#[test]
fn boundary_fullwidth_and_changing_states(){
    let mut seed=0x4e41_5449_5645_3136;
    for power in (1..16).step_by(2){let r=Roots::new(power);for kind in 0..518{let x:[Fr;32]=core::array::from_fn(|j|match kind{0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,3=>if j%2==0{Fr::ONE}else{-Fr::ONE},4=>Fr::from(2u64).pow(&[((j%16)*16)as u64])-Fr::ONE,5=>Fr::from(2u64).pow(&[((j%16)*16)as u64])+Fr::ONE,_=>random(&mut seed)});check::<false>(x,&r);check::<true>(x,&r);}
        let mut x:[Fr;32]=core::array::from_fn(|_|random(&mut seed));
        for step in 0..129{check::<false>(x,&r);check::<true>(x,&r);let mut v=pack(&x);if step%2==0{candidate::<false>(&mut v,&r);}else{candidate::<true>(&mut v,&r);}x=unpack(&v);}
    }
}

#[test]
fn immutable_roots_shared_by_small_stack_workers(){
    for workers in [1,4,16,32]{let roots:std::sync::Arc<Vec<Roots>>=std::sync::Arc::new((1..16).step_by(2).map(Roots::new).collect());
        std::thread::scope(|scope|{let mut handles=Vec::new();for worker in 0..workers{let roots=roots.clone();handles.push(std::thread::Builder::new().stack_size(512*1024).spawn_scoped(scope,move||{let mut seed=0x5746_5441_5348_4152 ^ (workers*131+worker)as u64;for r in roots.iter(){let mut x:[Fr;32]=core::array::from_fn(|_|random(&mut seed));for _ in 0..17{check::<false>(x,r);check::<true>(x,r);let mut v=pack(&x);candidate::<false>(&mut v,r);x=unpack(&v);}}}).unwrap());}for handle in handles{handle.join().unwrap();}});
    }
}
