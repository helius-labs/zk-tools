use super::*;
use ark_ec::{CurveGroup,PrimeGroup};
use ark_ff::PrimeField;
#[test]
fn packet_feed_matches_independent_curve_buckets_threads_exceptions_and_tails(){
    use zopro_bn254::Fr;
    let generator=@PROJECTIVE@::generator();
    let before=ADMITTED_PACKETS.load(std::sync::atomic::Ordering::Relaxed);
    let mut cases=0;
    for workers in [1usize,4,16,32]{
        let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
        pool.install(||{
            for size in [8usize,200]{for n in [0usize,7,8,9,15,32,200,201,352,353,1025]{for mode in 0..4usize{
                let half=64usize;
                let initial=(0..half).map(|i|if mode==1&&i%13==0{Fr::ZERO}else{Fr::from(i as u64+17)}).collect::<Vec<_>>();
                let mut expected=initial.iter().map(|f|ark_bn254::Fr::from_bigint(ark_ff::BigInt(f.to_canonical())).unwrap()).collect::<Vec<_>>();
                let mut points=Vec::new();let mut words=Vec::new();
                for i in 0..n{
                    let id=if i<8{i}else{(i*37+11)%half};
                    let incoming=match mode{1 if i%5==0=>Fr::ZERO,2=>initial[id],3=>-initial[id],_=>Fr::from(i as u64+900001)};
                    let negative=i%3==0;
                    let magnitude=(id+1)as u64;
                    let raw=if negative{4096-magnitude}else{magnitude};
                    points.push(generator*incoming);words.push([raw,0,raw,0]);
                    let scalar=ark_bn254::Fr::from_bigint(ark_ff::BigInt(incoming.to_canonical())).unwrap();
                    expected[id]+=if negative{-scalar}else{scalar};
                }
                let points=@PROJECTIVE@::batch_to_affine(&points);let unchanged=points.clone();
                let mut state=@NEW_WINDOW@;
                state.affine=@PROJECTIVE@::batch_to_affine(&initial.iter().map(|s|generator**s).collect::<Vec<_>>());
                let batch_ptr=state.batch.as_ptr();let batch_cap=state.batch.capacity();let queue_ptr=state.queue.as_ptr();let queue_cap=state.queue.capacity();
                @FEED@
                assert_eq!(state.batch.as_ptr(),batch_ptr);assert_eq!(state.batch.capacity(),batch_cap);assert_eq!(state.queue.as_ptr(),queue_ptr);assert_eq!(state.queue.capacity(),queue_cap);
                let got=state.finish();assert_eq!(points,unchanged);
                let reference=@ARK_PROJECTIVE@::normalize_batch(&expected.iter().map(|s|@ARK_PROJECTIVE@::generator()**s).collect::<Vec<_>>());
                for (i,(actual,expected))in got.iter().zip(reference).enumerate(){
                    let native=@CONVERT@;
                    assert_eq!(*actual,native,"workers{workers}/n{n}/size{size}/mode{mode}/bucket{i}");
                }
                cases+=1;
            }}}
        });
    }
    let admitted=ADMITTED_PACKETS.load(std::sync::atomic::Ordering::Relaxed)-before;
    assert_eq!(cases,352);assert!(admitted>0);
    println!("PACKET_NATIVE_CURVE=@LABEL@ CASES={cases} BUCKET_COMPARISONS={} ADMITTED_PACKETS={admitted} INPUTS_UNCHANGED=true BATCH_QUEUE_POINTER_CAPACITY_STABLE=true",cases*64);
}
