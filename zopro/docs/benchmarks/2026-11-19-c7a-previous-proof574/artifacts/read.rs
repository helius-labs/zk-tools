// Reads the tracepoints in a single archived perf-script recording.
use std::{collections::BTreeMap,fs::File,io::{BufRead,BufReader}};
fn ns(stamp:&str)->u64 {
    let (sec,frac)=stamp.trim_end_matches(':').split_once('.').unwrap();
    assert!(frac.len()<=9);
    sec.parse::<u64>().unwrap()*1_000_000_000+frac.parse::<u64>().unwrap()*10u64.pow(9-frac.len() as u32)
}
fn main(){
    let path=std::env::args_os().nth(1).unwrap();
    let names=["sdk_entry","witness_entry","witness_leave","interp_entry","interp_leave",
               "fft_entry","fft_leave","msms_entry","denseh_entry","denseh_leave","msms_leave","sdk_leave"];
    let mut marks=BTreeMap::<String,Vec<u64>>::new();
    let mut ticks=Vec::new();
    for line in BufReader::new(File::open(path).unwrap()).lines(){
        let l=line.unwrap();
        if l.contains("cpu-clock") {
            if l.split_ascii_whitespace().skip(4).collect::<Vec<_>>().join(" ")=="zopro_core::msm::g2_prefix::apply::<zopro_bn254::curve::G2Config, true>"{
                ticks.push(ns(l.split_ascii_whitespace().nth(1).unwrap()));
            }
            continue
        }
        let mut fields=l.split_ascii_whitespace();let _tid=fields.next();
        let Some(time)=fields.next() else {continue};let Some(tag)=fields.next() else{continue};
        let name=tag.trim_end_matches(':').rsplit(':').next().unwrap();
        if names.contains(&name){marks.entry(name.to_owned()).or_default().push(ns(time));}
    }
    assert!(ticks.windows(2).all(|v|v[0]<=v[1]));
    for name in names {assert_eq!(marks[name].len(),11,"{name}");}
    let count=|lo,hi| ticks.partition_point(|&t|t<hi)-ticks.partition_point(|&t|t<lo);
    for i in 0..11{
        let a=names.map(|name| marks[name][i]);
        assert!(a.windows(2).all(|v|v[0]<=v[1]),"invalid tracepoint order");
        let before=count(a[7],a[9]);let after=count(a[9],a[10]);assert!(before+after>0);
        let curr=a[11]-a[0];
        let gaps=[a[1]-a[0],a[2]-a[1],a[3]-a[2],a[4]-a[3],a[5]-a[4],a[6]-a[5],a[7]-a[6],a[10]-a[7],a[11]-a[10]];
        assert_eq!(gaps.iter().sum::<u64>(),curr);
        let prev=if i==0{0}else{a[0].checked_sub(marks["sdk_leave"][i-1]).unwrap()};
        print!("{i} {before} {after} {prev} {curr}");for gap in gaps{print!(" {gap}");}println!();
    }
}
