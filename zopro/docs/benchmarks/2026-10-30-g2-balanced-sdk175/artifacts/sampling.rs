fn category(s:&str)->&'static str{
    if s=="[unknown]"||s=="[unknown] (deleted)" {"unknown"}
    else if s.contains("::grouping::")||s.contains("scalar_groups")||s.contains("group_witness"){"grouping"}
    else if s.contains("::fft::"){"FFT"}
    else if s.contains("::inverse::"){"inverse"}
    else if s.contains("::msm::")||s.contains("::msm_affine::")||s.contains("::curve::"){"MSM_curve"}
    else if s.contains("::solver::")||s.contains("sparse_poseidon"){"solver"}
    else if s.contains("::verify::")||s.contains("::pairing::")||s.contains("::fq12::")||s.contains("::fq6::"){"verification_field_extensions"}
    else if s.contains("::alloc::linux::")||s.contains("__rust_alloc")||s.contains("__rust_dealloc")||s.contains("malloc")||s.starts_with("free")||s.starts_with("realloc"){"allocator"}
    else if s.contains("memcpy")||s.contains("memset")||s.contains("memmove"){"memory_primitives"}
    else if s.contains("crossbeam_epoch"){"crossbeam_epoch"}
    else if s.contains("rayon_core")||s.contains("std::sync")||s.contains("futex"){"runtime_or_inlined_payload"}
    else{"other"}
}
fn sample(line:&str)->Option<(u32,u32,u32,u64,String)>{let l=line.trim();if l.is_empty()||l.starts_with('#'){return None;}let f=l.split_whitespace().collect::<Vec<_>>();assert!(f.len()>=6,"bad sample {line}");let(pid,tid)=f[0].split_once('/').unwrap();let cpu=f[1].trim_matches(['[',']']).parse().unwrap();let time=f[2].trim_end_matches(':').parse::<f64>().unwrap();assert!(time.is_finite()&&time>0.);let period=f[3].parse::<u64>().unwrap();assert!(period>0);let _ip=u64::from_str_radix(f[4].trim_start_matches("0x"),16).unwrap();let raw=f[5..].join(" ");let symbol=rustc_demangle::try_demangle(&raw).map(|s|format!("{s:#}")).unwrap_or(raw);Some((pid.parse().unwrap(),tid.parse().unwrap(),cpu,period,symbol))}
fn median(a:&[f64])->f64{assert!(!a.is_empty());let mut s=a.to_vec();s.sort_by(f64::total_cmp);s[s.len()/2]}
