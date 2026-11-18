use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone)]
struct Event { tag:String, tid:u64, cpu:u64, start:i64, end:i64 }
fn ns(s:&str)->i64 {
    let (sec,frac)=s.trim_end_matches(':').split_once('.').unwrap();
    sec.parse::<i64>().unwrap()*1_000_000_000+frac.parse::<i64>().unwrap()*10_i64.pow(9-frac.len() as u32)
}
pub fn analyze(trace:&str,prefix:&str,n:usize,workers:&BTreeMap<u64,u64>)->BTreeMap<String,Vec<f64>>{
    let mut calls=Vec::new();
    let mut stacks:BTreeMap<u64,Vec<Event>>=BTreeMap::new();
    let mut process=BTreeSet::new();
    for line in trace.lines().map(str::trim).filter(|l|!l.is_empty()&&!l.starts_with('#')){
        let f:Vec<_>=line.split_whitespace().collect();
        let(p,t)=f[0].split_once('/').unwrap();
        let(pid,tid)=(p.parse::<u64>().unwrap(),t.parse::<u64>().unwrap());
        process.insert(pid);
        let cpu=f[1].trim_matches(['[',']']).parse::<u64>().unwrap();
        assert!(cpu<workers.len() as u64);
        if tid!=pid{assert_eq!(workers.get(&tid),Some(&cpu));}
        let time=ns(f[2]);assert_eq!(f[3],"1");
        let(tag,action)=f[4].trim_end_matches(':').strip_prefix(prefix).unwrap().rsplit_once('_').unwrap();
        let event=Event{tag:tag.into(),tid,cpu,start:time,end:time};
        let stack=stacks.entry(tid).or_default();
        match action{
            "mark"=>calls.push(event),
            "enter"=>stack.push(event),
            "exit"=>{let mut e=stack.pop().unwrap();assert_eq!(e.tag,tag);assert!(e.start<=time);e.end=time;calls.push(e)}
            _=>panic!("unknown probe {tag} {action}")
        }
    }
    assert_eq!(process.len(),1);assert!(stacks.values().all(Vec::is_empty));
    // Seven unique calls and 22 partition calls.
    assert_eq!(calls.len(),29*n,"unexpected number of resolved calls or markers");
    let mut result=BTreeMap::<String,Vec<f64>>::new();
    let mut last=0;
    for sdk in calls.iter().filter(|c|c.tag=="sdk"){
        assert!(last<=sdk.start);last=sdk.end;
        let request:Vec<_>=calls.iter().filter(|c|c.start>=sdk.start&&c.end<=sdk.end).collect();
        assert_eq!(request.len(),29);
        let one=|tag:&str|{let mut it=request.iter().copied().filter(|c|c.tag==tag);
            let e=it.next().unwrap_or_else(||panic!("missing {tag}"));assert!(it.next().is_none());e};
        let sol=one("solver");let fft=one("fft");let a=one("a");let raw=one("raw");let canonical=one("canonical");
        let check=one("cert");let success=check;
        assert_eq!(check.tid,success.tid);assert!(check.start<=success.end);
        let parts:Vec<_>=request.iter().copied().filter(|c|c.tag=="part").collect();assert_eq!(parts.len(),22);
        let first=parts.iter().min_by_key(|c|c.start).unwrap();
        let tail=parts.iter().max_by_key(|c|c.end).unwrap();
        let later=fft.end.max(success.end);
        let marks=[sdk.start,sol.start,sol.end,fft.start.min(check.start),later,raw.start,first.start,tail.end,raw.end,canonical.end,sdk.end];
        let keys=["to_solver","solve","prepare","fft_check","to_g2","to_fill","fill_span","fold_tail","after_g2","sdk_tail"];
        assert!(marks.windows(2).all(|v|v[0]<=v[1]),"bad interval ordering {marks:?}");
        for (k,v) in keys.iter().zip(marks.windows(2)){result.entry(k.to_string()).or_default().push((v[1]-v[0])as f64/1e6);}
        for(k,v)in [("sdk",sdk.end-sdk.start),("fft",fft.end-fft.start),("check",success.end-check.start),("check_after_fft_start",check.start-fft.start),("check_end_minus_fft",success.end-fft.end),("a",a.end-a.start),("raw",raw.end-raw.start),("a_exit_minus_raw",a.end-raw.end),("last_fill",tail.end-tail.start)]{result.entry(k.into()).or_default().push(v as f64/1e6);}
        for(k,v)in [("fft_cpu",fft.cpu),("check_cpu",check.cpu),("raw_cpu",raw.cpu),("a_cpu",a.cpu),("last_fill_cpu",tail.cpu),("solver_cpu",sol.cpu),("sdk_cpu",sdk.cpu)]{result.entry(k.into()).or_default().push(v as f64);}
    }
    assert!(result.values().all(|v|v.len()==n));
    result
}
