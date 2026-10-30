//! Share the existing two narrow Phi2 top digits through compact joint sums.
use super::*;
use zopro_bn254::Fq2;
use crate::msm_affine::grouping_cost::{add, mul, vector};
const HALF:usize=64;
const SIDE:usize=2*HALF+1;
const CELLS:usize=SIDE*SIDE;
// Same point limit as the original indexed G2 leaf. A fixed component
// digit leaves only SIDE possible values of the other joint digit.
const TILE:usize=2048/(core::mem::size_of::<Fq2>()/32);
fn marginal_jobs(u:usize)->usize{u.div_ceil(TILE-(SIDE-1)).max(1)}
fn bounded_ranges(counts:&[usize])->Vec<(usize,usize)>{
    assert_eq!(counts.len(),2*HALF);
    assert!(counts.iter().all(|&n|n<=SIDE));
    let jobs=marginal_jobs(counts.iter().sum());
    let mut ranges=Vec::with_capacity(jobs);
    let(mut first,mut used)=(0,0);
    for(i,&n)in counts.iter().enumerate(){
        if used+n>TILE{ranges.push((first,i));first=i;used=0;}
        used+=n;
    }
    ranges.push((first,counts.len()));assert!(ranges.len()<=jobs);ranges
}

/// Full optional charge, including existing typed local-tree bounds. This
/// deliberately charges the original-size outputs as well as the shared sums.
fn peak(n:usize)->Option<usize>{
    if n>=u32::MAX as usize{return None;}
    let u=n.min(CELLS-1);let jobs=u.div_ceil(256).max(1);
    let labels=vector::<u16>(u,false)?;
    let compact=add(add(vector::<usize>(u,false)?,vector::<usize>(add(u,1)?,false)?)?,labels)?;
    let input=vector::<u32>(n,false)?;
    let routing=add(add(vector::<usize>(CELLS,false)?,compact)?,input)?;
    let first=add(add(add(compact,input)?,vector::<G2Affine>(u,false)?)?,crate::msm::workspace_grouping_linear::<G2Config>(n,jobs)?)?;
    // Joint sums/labels, signed output + its negative split, prior high
    // output, bounded output blocks and their exact-sized concatenation.
    let marginal=add(add(add(labels,vector::<G2Affine>(u,false)?)?,vector::<u32>(u,false)?)?,
        add(add(mul(vector::<usize>(2*HALF,false)?,2)?,vector::<usize>(2*HALF+1,false)?)?,
            add(mul(vector::<G2Affine>(2*HALF,false)?,2)?,vector::<G2Affine>(HALF,false)?)?)?)?;
    let jobs=marginal_jobs(u);
    let blocks=add(add(vector::<(usize,usize)>(jobs,false)?,vector::<Vec<G2Affine>>(jobs,true)?)?,
        add(vector::<G2Affine>(2*HALF,false)?,mul(jobs,64)?)?)?;
    let second=add(add(marginal,blocks)?,crate::msm::workspace_grouping_linear::<G2Config>(u,jobs)?)?;
    Some(routing.max(first).max(second))
}
fn key(s:&[u64;4])->usize{
    let a=hb_digit(s,false,10,12);let b=hb_digit(s,true,10,12);
    assert!(a.unsigned_abs()<=HALF as u16&&b.unsigned_abs()<=HALF as u16);
    (a as isize+HALF as isize)as usize*SIDE+(b as isize+HALF as isize)as usize
}
fn digits(label:u16)->(i16,i16){let x=label as usize;((x/SIDE)as i16-HALF as i16,(x%SIDE)as i16-HALF as i16)}
fn slot(d:i16)->Option<usize>{if d==0{None}else{Some(d.unsigned_abs()as usize-1+if d<0{HALF}else{0})}}
fn marginal(points:&[G2Affine],labels:&[u16],high:bool)->Vec<G2Affine>{
    let read=|label|{let(a,b)=digits(label);if high{b}else{a}};
    let mut counts=vec![0usize;2*HALF];
    for(p,&label)in points.iter().zip(labels){if !p.is_identity(){if let Some(b)=slot(read(label)){counts[b]+=1;}}}
    let mut starts=Vec::with_capacity(2*HALF+1);starts.push(0);
    for &n in &counts{starts.push(starts.last().unwrap()+n);}
    let mut positions=starts[..2*HALF].to_vec();let mut order=vec![0u32;*starts.last().unwrap()];
    for(i,(p,&label))in points.iter().zip(labels).enumerate(){if !p.is_identity(){if let Some(b)=slot(read(label)){order[positions[b]]=i as u32;positions[b]+=1;}}}
    let ranges=bounded_ranges(&counts);
    let blocks:Vec<_>=ranges.par_iter().map(|&(first,end)|{
        // Each range has <=128 classes: the existing 256-class reducer
        // runs one nonforking leaf over these disjoint point references.
        crate::msm::group_affine(points,&order,&starts[first..=end],&counts[first..end])
    }).collect();
    let mut sums=Vec::with_capacity(2*HALF);
    for block in blocks{sums.extend(block);}
    assert_eq!(sums.len(),2*HALF);
    let mut negative=sums.split_off(HALF);for p in &mut negative{p.y=-p.y;}
    merge_buckets(&mut sums,&negative);sums
}
fn build(bases:&[G2Affine],short:&[[u64;4]])->Vec<G2Affine>{
    let zero=HALF*SIDE+HALF;let mut histogram=vec![0usize;CELLS];
    for(p,s)in bases.iter().zip(short){if !p.is_identity(){let k=key(s);if k!=zero{histogram[k]+=1;}}}
    let u=histogram.iter().filter(|&&n|n!=0).count();
    let mut labels=Vec::with_capacity(u);let mut counts=Vec::with_capacity(u);
    let mut starts=Vec::with_capacity(u+1);starts.push(0);
    for(k,cursor)in histogram.iter_mut().enumerate(){let n=*cursor;if n!=0{labels.push(k as u16);counts.push(n);*cursor=*starts.last().unwrap();starts.push(starts.last().unwrap()+n);}}
    let mut order=vec![0u32;*starts.last().unwrap()];
    for(i,(p,s))in bases.iter().zip(short).enumerate(){if !p.is_identity(){let k=key(s);if k!=zero{order[histogram[k]]=i as u32;histogram[k]+=1;}}}
    drop(histogram);
    let sums=crate::msm::group_affine(bases,&order,&starts,&counts);
    drop(order);drop(starts);drop(counts);
    let mut high=marginal(&sums,&labels,true);
    for p in &mut high{if !p.is_identity(){p.x=p.x.mul_by_fp(&BETA);}}
    let low=marginal(&sums,&labels,false);merge_buckets(&mut high,&low);high
}
fn admitted<F>(bases:&[G2Affine],short:&[[u64;4]],reserve:F)->Option<Vec<G2Affine>>
where F:FnOnce(Option<usize>)->Option<crate::preparation::Reservation>{
    assert_eq!(bases.len(),short.len());
    let _reservation=reserve(peak(bases.len()))?;
    Some(build(bases,short))
}
pub(super) fn try_window(bases:&[G2Affine],short:&[[u64;4]])->Option<Vec<G2Affine>>{
    admitted(bases,short,crate::preparation::Reservation::analysis)
}

#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::Fr;
    fn signed(x:i16)->Fr{let v=Fr::from(x.unsigned_abs()as u64);if x<0{-v}else{v}}
    fn lambda()->Fr{Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461")}
    fn short(a:i16,b:i16)->[u64;4]{
        let part=|x:i16|{let m=if x.unsigned_abs()==64{(1u128<<126)-1}else{(x.unsigned_abs()as u128)<<120};m|if x<0{1u128<<127}else{0}};
        let(a,b)=(part(a),part(b));[a as u64,(a>>64)as u64,b as u64,(b>>64)as u64]
    }
    fn check(points:&[G2Affine],weights:&[Fr],words:&[[u64;4]]){
        let before_points=points.to_vec();let before_words=words.to_vec();
        let out=try_window(points,words).expect("test headroom admits optional top");
        let want=words.iter().zip(weights).map(|(s,&p)|{let a=hb_digit(s,false,10,12);let b=hb_digit(s,true,10,12);(signed(a)+lambda()*signed(b))*p}).fold(Fr::ZERO,|a,b|a+b);
        assert_eq!(crate::msm::weighted_affine(&out).to_affine(),(G2Projective::generator()*want).to_affine());
        assert_eq!(points,before_points);assert_eq!(words,before_words);assert_eq!(out.len(),64);
    }
    #[test]fn joint_top_complete_generator_sums_and_custom_pools(){
        let n=1025;let g=G2Projective::generator();let weights=(0..n).map(|i|if i%17==0{Fr::ZERO}else if i%3==0{-Fr::from((i%31+1)as u64)}else{Fr::from((i%31+1)as u64)}).collect::<Vec<_>>();
        let points=G2Projective::batch_to_affine(&weights.iter().map(|&w|g*w).collect::<Vec<_>>());
        let words=(0..n).map(|i|phi_split(Fr::from((i+3)as u64).pow(&[127]).to_canonical())).collect::<Vec<_>>();
        for threads in[1,4,16,32]{rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(||{for n in[0,1,2,3,7,8,9,255,256,257,511,1025]{check(&points[..n],&weights[..n],&words[..n]);}});}
    }
    #[test]fn joint_top_signed_extrema_sparse_dense_and_complete_exceptions(){
        let n=2051;let g=G2Projective::generator();let mut words=(0..n).map(|i|match i%5{0=>short(64,-64),1=>short(-64,64),2=>short(0,9),3=>short(-7,0),_=>short(0,0)}).collect::<Vec<_>>();
        for kind in 0..4{let weights=(0..n).map(|i|match kind{0=>Fr::ONE,1=>if i%2==0{Fr::ONE}else{-Fr::ONE},2=>Fr::ZERO,_=>Fr::from((i%19)as u64)}).collect::<Vec<_>>();let points=G2Projective::batch_to_affine(&weights.iter().map(|&w|g*w).collect::<Vec<_>>());check(&points,&weights,&words);for(i,s)in words.iter_mut().enumerate(){*s=short((i/129%129)as i16-64,(i%129)as i16-64);}}
    }
    #[test]fn joint_top_denial_and_checked_capacity_cost(){
        let points=[G2Affine::GENERATOR];let words=[short(64,-64)];let mut called=false;
        assert!(admitted(&points,&words,|p|{called=true;assert!(p.unwrap()>0);None}).is_none());assert!(called);
        assert_eq!(points,[G2Affine::GENERATOR]);assert_eq!(words,[short(64,-64)]);assert_eq!(peak(usize::MAX),None);
        for n in[1,7,8,255,256,257,1024,32768,65535]{assert!(peak(n).unwrap()>=n*core::mem::size_of::<u32>());}
    }
    #[test]fn bounded_marginal_partition_and_complete_bin_sums(){
        let mut seed=1597u64;
        for kind in 0..132 {
            let counts:Vec<_>=(0..2*HALF).map(|i|match kind{
                0=>0,1=>SIDE,2=>usize::from(i==127)*SIDE,
                3=>if i<7{129}else if i==7{121}else{usize::from(i==8)},
                _=>{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;(seed as usize)%(SIDE+1)}
            }).collect();
            let ranges=bounded_ranges(&counts);let mut last=0;
            for(i,&(first,end))in ranges.iter().enumerate(){
                assert_eq!(first,last);assert!(first<end);let n=counts[first..end].iter().sum::<usize>();
                assert!(n<=TILE);if i+1<ranges.len(){assert!(n>=TILE-(SIDE-1));}last=end;
            }
            assert_eq!(last,counts.len());assert!(ranges.len()<=marginal_jobs(counts.iter().sum()));
        }
        let g=G2Projective::generator();let coefficients=(0..19).map(|i|Fr::from(i as u64)-Fr::from(9u64)).collect::<Vec<_>>();
        let bases=G2Projective::batch_to_affine(&coefficients.iter().map(|&v|g*v).collect::<Vec<_>>());
        let labels=(0..CELLS).filter(|&k|k!=HALF*SIDE+HALF).map(|k|k as u16).collect::<Vec<_>>();
        for n in[0,1,1023,1024,1025,CELLS-1]{
            let points=(0..n).map(|i|bases[i%19]).collect::<Vec<_>>();
            for high in[false,true]{let mut want=vec![Fr::ZERO;HALF];
                for(i,&label)in labels[..n].iter().enumerate(){let(a,b)=digits(label);let d=if high{b}else{a};if d!=0{want[d.unsigned_abs()as usize-1]+=if d<0{-coefficients[i%19]}else{coefficients[i%19]};}}
                let got=marginal(&points,&labels[..n],high);for(i,p)in got.iter().enumerate(){assert_eq!(*p,(g*want[i]).to_affine());}
            }
        }
    }
}
