fn profile_owners(meta:&str,binary:&str)->(u32,BTreeSet<u32>){
 let owners=meta.lines().filter(|l|l.contains("PERF_RECORD_MMAP")&&l.contains(binary)).map(|l|l.split_whitespace().next().unwrap().split_once('/').unwrap().0.parse::<u32>().unwrap()).collect::<BTreeSet<_>>();assert_eq!(owners.len(),1);let pid=*owners.first().unwrap();let mut tids=BTreeSet::from([pid]);
 for line in meta.lines(){if let Some((_,tail))=line.split_once("PERF_RECORD_FORK("){let(child,_)=tail.split_once(')').unwrap();let(p,t)=child.split_once(':').unwrap();let(p,t)=(p.parse::<u32>().unwrap(),t.parse::<u32>().unwrap());if p==pid{tids.insert(t);}}}
 assert!(tids.len()>1);(pid,tids)
}
#[cfg(test)]mod metadata_tests{
 use super::*;
 #[test]fn owner_binary_and_fork_thread_membership(){let meta="123/123 [000] 10.1: PERF_RECORD_MMAP2 123/123: [0x1000] r-xp /x/candidate-sdk\n123/123 [001] 10.2: PERF_RECORD_FORK(123:124):(123:123)\n123/123 [001] 10.3: PERF_RECORD_FORK(123:125):(123:123)\n900/900 [001] 10.4: PERF_RECORD_FORK(900:901):(900:900)";assert_eq!(profile_owners(meta,"/x/candidate-sdk"),(123,BTreeSet::from([123,124,125])));}
 #[test]#[should_panic]fn reject_multiple_binary_owners(){profile_owners("1/1 [000] 1.: PERF_RECORD_MMAP2 /x/sdk\n2/2 [000] 1.: PERF_RECORD_MMAP2 /x/sdk","/x/sdk");}
 #[test]#[should_panic]fn reject_absent_binary_owner(){profile_owners("1/1 [000] 1.: PERF_RECORD_MMAP2 /x/other","/x/sdk");}
}

