
pub(crate) struct Prepared {
    pub(crate) positions:Vec<u32>,
    pub(crate) workspace:usize,
    _owner:crate::preparation::Reservation,
}
static POSITIONS:AtomicUsize=AtomicUsize::new(0);
static WORKSPACE:AtomicUsize=AtomicUsize::new(0);
pub(crate)fn snapshot()->([usize;4],usize,usize){(std::array::from_fn(|i|ROUTES[i].load(Ordering::Relaxed)),POSITIONS.load(Ordering::Relaxed),WORKSPACE.load(Ordering::Relaxed))}
pub(crate)fn prepare(plan:&crate::solver::Plan,n:usize,normal:usize)->Option<Prepared>{
    if std::env::var("ZOPRO_CPU_BOOLEAN_QUERY").as_deref()!=Ok("1") || !(1024..65536).contains(&n) || n!=plan.b_indices.len(){return None;}
    let out=build(plan,normal)?;
    POSITIONS.fetch_max(out.positions.len(),Ordering::Relaxed);
    WORKSPACE.fetch_max(out.workspace,Ordering::Relaxed);
    Some(out)
}
fn merge_ranges(ranges:&mut Vec<core::ops::Range<usize>>){
    ranges.sort_unstable_by_key(|r|(r.start,r.end));
    let mut used=0;
    for i in 0..ranges.len(){let r=ranges[i].clone();if used!=0&&r.start<=ranges[used-1].end{ranges[used-1].end=ranges[used-1].end.max(r.end);}else{ranges[used]=r;used+=1;}}
    ranges.truncate(used);
}
fn each_position(indices:&[u32],ranges:&[core::ops::Range<usize>],mut f:impl FnMut(u32))->Option<usize>{
    if indices.windows(2).any(|p|p[0]>=p[1]) || ranges.iter().any(|r|r.start>=r.end) || ranges.windows(2).any(|r|r[0].end>=r[1].start){return None;}
    let mut r=0;let mut count=0;
    for(i,&wire)in indices.iter().enumerate(){while r<ranges.len()&&ranges[r].end<=wire as usize{r+=1;}if r==ranges.len(){break;}if ranges[r].start<=wire as usize{f(u32::try_from(i).ok()?);count+=1;}}
    Some(count)
}
fn build(plan:&crate::solver::Plan,normal:usize)->Option<Prepared>{
    use crate::preparation::{Reservation,allocation_bytes};
    let mut count=0usize;plan.for_each_boolean_range(|_|count+=1);
    if count==0{return None;}
    let range_peak=workspace::vector::<core::ops::Range<usize>>(count,false)?;
    let _scratch_owner=Reservation::analysis(Some(range_peak))?;
    let mut ranges=Vec::new();ranges.try_reserve_exact(count).ok()?;
    if allocation_bytes(&ranges)?>range_peak{return None;}
    plan.for_each_boolean_range(|r|ranges.push(r));merge_ranges(&mut ranges);
    let n=each_position(&plan.b_indices,&ranges,|_|{})?;
    if n==0{return None;}
    let extra=crate::msm::workspace_boolean_indexed(n)?;
    let meta=workspace::vector::<u32>(n,false)?;
    // Independent owner guarantees baseline+overlap remain forecast even when
    // the ordinary solver schedule chooses its allocation-free fallback.
    let owner=Reservation::schedule(0,Some(meta),Some(normal.checked_add(extra)?))?;
    let mut positions=Vec::new();positions.try_reserve_exact(n).ok()?;
    if allocation_bytes(&positions)?>meta{return None;}
    assert_eq!(each_position(&plan.b_indices,&ranges,|i|positions.push(i))?,n);
    let owner=owner.retain(allocation_bytes(&positions));
    Some(Prepared{positions,workspace:extra,_owner:owner})
}
