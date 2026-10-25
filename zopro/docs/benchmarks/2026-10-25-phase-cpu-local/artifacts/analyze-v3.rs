//! Read-only phase accounting for the fixed diagnostic; run through xtask.
use serde_json::{Value, json};
use std::{collections::{BTreeMap, BTreeSet}, fs, path::{Path, PathBuf}};
const PHASES: [&str; 16] = ["sdk_input", "encode", "core_entry", "solver",
    "canonical_and_correction", "gather", "fft_product", "queries", "parts_output",
    "core_cleanup", "assembly", "prover_return", "verify", "sdk_return", "sdk_output", "done"];
const COARSE: [&str; 9] = ["sdk_input", "encode", "core_entry", "assembly",
    "prover_return", "verify", "sdk_return", "sdk_output", "done"];
fn sequence(case: &str, stamps: &[Value]) -> &'static [&'static str] {
    let actual=stamps.iter().map(|s|string(s,"phase")).collect::<Vec<_>>();
    if actual==PHASES { return &PHASES; }
    assert!(case.starts_with("merge-54x1-") && actual==COARSE,
        "unsupported phase sequence for {case}: {actual:?}");
    &COARSE
}
fn unsigned(v: &Value, key: &str) -> u64 { v[key].as_u64().unwrap() }
fn signed(v: &Value, key: &str) -> i64 { v[key].as_i64().unwrap() }
fn string<'a>(v: &'a Value, key: &str) -> &'a str { v[key].as_str().unwrap() }
fn read(path: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(path).unwrap()).unwrap() }
fn write(path: impl AsRef<Path>, value: &Value) {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn median(mut values: Vec<f64>) -> f64 {
    assert!(!values.is_empty()); values.sort_by(f64::total_cmp);
    let mid = values.len()/2;
    if values.len().is_multiple_of(2) { (values[mid-1]+values[mid])/2. } else { values[mid] }
}
fn aligned(assigned: u64, total: u64) -> bool {
    total != 0 && assigned <= total && assigned as f64 / total as f64 >= 0.90
}
fn time_ns(text: &str) -> u64 {
    let (seconds, fraction) = text.strip_suffix(':').unwrap().split_once('.').unwrap();
    assert!(fraction.len() == 9 && fraction.bytes().all(|b|b.is_ascii_digit()));
    seconds.parse::<u64>().unwrap().checked_mul(1_000_000_000).unwrap()
        .checked_add(fraction.parse::<u64>().unwrap()).unwrap()
}
#[derive(Debug)]
struct Interval { start: u64, end: u64, phase: String }
fn sample_bucket<'a>(time: u64, intervals: &'a [Interval], windows: &[(u64,u64)]) -> &'a str {
    let index = intervals.partition_point(|i|i.start <= time);
    if index > 0 && time < intervals[index-1].end { return &intervals[index-1].phase; }
    if windows.iter().any(|&(start,end)|start <= time && time <= end) { "boundary_ambiguous" }
    else { "outside_requests" }
}
#[derive(Default)]
struct Cost { wall_ms: f64, user_ms: f64, system_ms: f64, switches: i64,
    involuntary: i64, minor: i64, major: i64 }
impl Cost {
    fn add(&mut self, a: &Value, b: &Value) {
        let middle = |s: &Value|unsigned(s,"clock_before_ns")+
            (unsigned(s,"clock_after_ns")-unsigned(s,"clock_before_ns"))/2;
        self.wall_ms += (middle(b)-middle(a)) as f64/1e6;
        for key in ["user_us","system_us","voluntary_switches","involuntary_switches","minor_faults","major_faults"] {
            assert!(signed(b,key) >= signed(a,key), "negative {key} delta");
        }
        self.user_ms += (signed(b,"user_us")-signed(a,"user_us")) as f64/1000.;
        self.system_ms += (signed(b,"system_us")-signed(a,"system_us")) as f64/1000.;
        self.switches += signed(b,"voluntary_switches")-signed(a,"voluntary_switches");
        self.involuntary += signed(b,"involuntary_switches")-signed(a,"involuntary_switches");
        self.minor += signed(b,"minor_faults")-signed(a,"minor_faults");
        self.major += signed(b,"major_faults")-signed(a,"major_faults");
    }
    fn value(&self, rounds: f64) -> Value {
        json!({"wall_ms":self.wall_ms/rounds,"user_cpu_ms":self.user_ms/rounds,
            "system_cpu_ms":self.system_ms/rounds,"cpu_ms":(self.user_ms+self.system_ms)/rounds,
            "occupied_logical_cpus":(self.user_ms+self.system_ms)/self.wall_ms,
            "voluntary_switches":self.switches as f64/rounds,
            "involuntary_switches":self.involuntary as f64/rounds,
            "minor_faults":self.minor as f64/rounds,"major_faults":self.major as f64/rounds})
    }
}
fn main() {
    let args = std::env::args_os().skip(1).map(PathBuf::from).collect::<Vec<_>>();
    assert_eq!(args.len(), 3, "usage: analyze PERF_ROOT DECODE_ROOT OUTPUT");
    let (root, decode, output) = (&args[0], &args[1], &args[2]);
    fs::create_dir(output).unwrap();
    let protocol = read(root.join("protocol.json"));
    let result = read(root.join("result.json"));
    let rows = read(root.join("rows.json"));
    let mut observations = Vec::new();
    let mut counts = BTreeMap::<String,usize>::new();
    let mut total_samples = 0u64;
    let mut total_sdk = 0u64;
    for row in rows.as_array().unwrap() {
        let phase = string(row,"phase"); let case = string(row,"case");
        let variant = string(row,"variant"); let block = unsigned(row,"block");
        let rounds = row["samples_ms"].as_array().unwrap().len();
        total_sdk += rounds as u64+unsigned(&row["summary"],"priming");
        let directory = root.join(format!("{phase}/b{block}/{case}/{variant}"));
        let stdout = fs::read_to_string(directory.join("run/stdout.log")).unwrap();
        let events = stdout.lines().filter_map(|l|l.strip_prefix("PHASE_CPU "))
            .map(|l|serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();
        if variant == "accepted" { assert!(events.is_empty()); continue; }
        assert_eq!(events.len(), rounds);
        for (index, event) in events.iter().enumerate() {
            assert_eq!(unsigned(event,"round"),index as u64);
            assert_eq!(event["enabled"].as_bool().unwrap(), variant=="diagnostic-on");
        }
        if variant == "diagnostic-off" {
            assert!(events.iter().all(|e|e["stamps"].as_array().unwrap().is_empty())); continue;
        }
        assert_eq!(variant,"diagnostic-on");
        let mut costs = BTreeMap::<String,Cost>::new();
        let mut intervals = Vec::new(); let mut windows = Vec::new();
        let mut caller = None; let mut observer_ns = 0u64;
        let mut request_total = Cost::default();
        for event in &events {
            let stamps = event["stamps"].as_array().unwrap();
            let expected=sequence(case,stamps);
            let pid = signed(&stamps[0],"tid");
            assert_eq!(*caller.get_or_insert(pid),pid);
            for (stamp, expected) in stamps.iter().zip(expected) {
                assert_eq!(string(stamp,"phase"),*expected);
                let start=unsigned(stamp,"clock_before_ns"); let end=unsigned(stamp,"clock_after_ns");
                assert!(start<=end); windows.push((start,end)); observer_ns += end-start;
            }
            for pair in stamps.windows(2) {
                assert!(unsigned(&pair[0],"clock_after_ns")<=unsigned(&pair[1],"clock_before_ns"));
                let label=string(&pair[0],"phase");
                costs.entry(label.into()).or_default().add(&pair[0],&pair[1]);
                intervals.push(Interval {start:unsigned(&pair[0],"clock_after_ns"),
                    end:unsigned(&pair[1],"clock_before_ns"),phase:label.into()});
            }
            request_total.add(&stamps[0], stamps.last().unwrap());
        }
        for pair in intervals.windows(2) { assert!(pair[0].end<=pair[1].start); }
        let sum_cpu=costs.values().map(|c|c.user_ms+c.system_ms).sum::<f64>();
        assert!((sum_cpu-request_total.user_ms-request_total.system_ms).abs()<1e-6);
        let loop_cpu=row["summary"]["user_cpu_ms"].as_f64().unwrap()+row["summary"]["system_cpu_ms"].as_f64().unwrap();
        assert!(sum_cpu<=loop_cpu+0.001);
        let mut sample_buckets = BTreeMap::<String,BTreeMap<String,u64>>::new();
        let mut sample_counts = BTreeMap::<String,u64>::new();
        let mut samples = 0u64; let mut period = 0u64;
        if phase=="profile" {
            assert_eq!(unsigned(&row["profile"],"lost_records"),0);
            let text=fs::read_to_string(decode.join(format!("decode-b{block}-{case}/stdout.log"))).unwrap();
            let mut pids=BTreeSet::new();
            for line in text.lines().filter(|l|!l.trim().is_empty()&&!l.starts_with('#')) {
                let mut fields=line.split_whitespace();
                let pid_tid=fields.next().unwrap();
                let (pid,tid)=pid_tid.split_once('/').unwrap();
                let pid=pid.parse::<i64>().unwrap(); let _tid=tid.parse::<i64>().unwrap();
                pids.insert(pid); assert_eq!(Some(pid),caller);
                let time=time_ns(fields.next().unwrap());
                let weight=fields.next().unwrap().parse::<u64>().unwrap();
                let address=fields.next().unwrap(); assert!(u64::from_str_radix(address,16).is_ok());
                let raw=fields.collect::<Vec<_>>().join(" "); assert!(!raw.is_empty());
                let symbol=rustc_demangle::try_demangle(&raw).map(|d|format!("{d:#}")).unwrap_or(raw);
                let bucket=sample_bucket(time,&intervals,&windows).to_owned();
                let category=protocol["symbol_categories"].as_array().unwrap().iter().find(|category|
                    category["contains"].as_array().unwrap().iter().any(|v|symbol.contains(v.as_str().unwrap())))
                    .map_or("other",|v|string(v,"name"));
                *sample_buckets.entry(bucket.clone()).or_default().entry(category.into()).or_default()+=weight;
                *sample_counts.entry(bucket).or_default()+=1;
                samples+=1; period+=weight;
            }
            assert_eq!(pids.len(),1); assert_eq!(samples,unsigned(&row["profile"],"samples"));
            assert_eq!(period,unsigned(&row["profile"],"period"));
            assert_eq!(sample_counts.values().sum::<u64>(),samples);
            assert!(samples>1000); total_samples+=samples;
            let assigned=sample_buckets.iter().filter(|(bucket,_)|
                bucket.as_str()!="boundary_ambiguous"&&bucket.as_str()!="outside_requests")
                .flat_map(|(_,categories)|categories.values()).sum::<u64>();
            assert!(aligned(assigned,period),
                "sample/request clock alignment failed for {case} b{block}: {assigned}/{period}");
        }
        *counts.entry(format!("{phase}/{case}")).or_default()+=1;
        let values=costs.iter().map(|(name,cost)|(name.clone(),cost.value(rounds as f64))).collect::<BTreeMap<_,_>>();
        observations.push(json!({"case":case,"block":block,"capture":phase,"rounds":rounds,
            "fine_phase_complete":costs.len()==PHASES.len()-1,
            "phases":values,"request":request_total.value(rounds as f64),
            "observer_bracket_us_per_request":observer_ns as f64/1000./rounds as f64,
            "unattributed_loop_cpu_ms_per_request":(loop_cpu-sum_cpu)/rounds as f64,
            "sample_counts":sample_counts,"category_periods":sample_buckets,"samples":samples,"period":period}));
    }
    let variants=protocol["variants"].as_array().unwrap().len() as u64;
    let blocks=unsigned(&protocol,"blocks");
    let profile_blocks=protocol["profile_blocks"].as_u64().unwrap_or(blocks);
    let cases=protocol["cases"].as_array().unwrap();
    let expected_sdk=variants*(blocks*cases.iter().map(|c|unsigned(c,"rounds")+3).sum::<u64>()+
        profile_blocks*cases.iter().map(|c|unsigned(c,"profile_rounds")+3).sum::<u64>());
    assert_eq!(total_sdk,expected_sdk);
    assert_eq!(rows.as_array().unwrap().len() as u64,variants*(blocks+profile_blocks)*cases.len() as u64);
    for (key,count) in &counts { assert_eq!(*count as u64,if key.starts_with("profile/"){profile_blocks}else{blocks}); }
    assert_eq!(counts.len(),cases.len()*2);
    let mut aggregates=BTreeMap::<String,Value>::new();
    for case in protocol["cases"].as_array().unwrap() {
        let name=string(case,"name");
        let selected=observations.iter().filter(|r|string(r,"case")==name&&string(r,"capture")=="profile").collect::<Vec<_>>();
        assert_eq!(selected.len() as u64,profile_blocks);
        let mut phases=BTreeMap::<String,Value>::new();
        let labels=selected[0]["phases"].as_object().unwrap().keys().map(String::as_str).collect::<Vec<_>>();
        for label in labels {
            let mut data=BTreeMap::<String,Value>::new();
            for key in ["wall_ms","cpu_ms","user_cpu_ms","system_cpu_ms","occupied_logical_cpus","voluntary_switches","minor_faults","major_faults"] {
                data.insert(key.into(),json!(median(selected.iter().map(|r|r["phases"][label][key].as_f64().unwrap()).collect())));
            }
            let mut categories=BTreeMap::<String,u64>::new();
            let mut samples=0u64;
            for row in &selected {
                samples+=row["sample_counts"][label].as_u64().unwrap_or(0);
                if let Some(periods)=row["category_periods"][label].as_object() {
                    for (category,period) in periods { *categories.entry(category.clone()).or_default()+=period.as_u64().unwrap(); }
                }
            }
            let period=categories.values().sum::<u64>();
            data.insert("samples".into(),json!(samples));
            data.insert("category_periods".into(),json!(categories));
            data.insert("category_percent".into(),json!(categories.iter().map(|(k,v)|
                (k.clone(),if period==0 {0.} else {*v as f64*100./period as f64})).collect::<BTreeMap<_,_>>()));
            phases.insert(label.into(),json!(data));
        }
        aggregates.insert(name.into(),json!({"phases":phases,
            "fine_phase_complete":selected.iter().all(|r|r["fine_phase_complete"]==true)}));
    }
    write(output.join("observations.json"),&json!(observations));
    write(output.join("result.json"),&json!({"status":"primary fine diagnostic complete; merge coarse only","adopted":false,
        "all_case_fine_phase_coverage_passed":observations.iter().all(|row|row["fine_phase_complete"]==true),
        "sample_alignment_passed":true,
        "coverage_limit":"Merge54, when present, uses a different inner solving path and has9coarse stamps; its solver/FFT/MSM split is unavailable. The original local all-case fine-coverage failure and unaligned default-clock samples remain retained.",
        "goal_achieved":false,"sdk_requests":total_sdk,"new_independent_exports":unsigned(&result,"distinct_verified_exports"),
        "on_profile_samples":total_samples,"aggregates":aggregates,"latency_calibration":result["latency"],
        "limits":"Process-wide accounting includes spinning; local16/32 does not establish C7a96 behavior. Bracketed boundary and outside-request samples retained separately. Phase medians need not sum."}));
    println!("PHASE_ANALYSIS_OK SDK={total_sdk} ON_PROFILE_SAMPLES={total_samples}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_clock_coverage_cannot_pass_on_sample_count_alone() {
        assert!(!aligned(0,0));
        assert!(!aligned(0,100_000));
        assert!(!aligned(89,100));
        assert!(aligned(90,100));
        assert!(!aligned(101,100));
    }
    #[test]
    fn merge_coarse_path_is_explicit_and_does_not_validate_primary() {
        let coarse=COARSE.iter().map(|s|json!({"phase":s})).collect::<Vec<_>>();
        assert_eq!(sequence("merge-54x1-w32",&coarse),COARSE);
        assert!(std::panic::catch_unwind(||sequence("1x2-w32",&coarse)).is_err());
        let fine=PHASES.iter().map(|s|json!({"phase":s})).collect::<Vec<_>>();
        assert_eq!(sequence("1x2-w32",&fine),PHASES);
    }
    #[test]
    fn nanosecond_times_are_exact_and_malformed_values_fail() {
        assert_eq!(time_ns("4338683.557385345:"),4_338_683_557_385_345);
        for value in ["1.2:","1.000000000","1.x00000000:","-1.000000000:"] {
            assert!(std::panic::catch_unwind(||time_ns(value)).is_err());
        }
    }
    #[test]
    fn boundary_windows_and_request_gaps_are_not_forced_into_phases() {
        let intervals=[Interval{start:12,end:19,phase:"solver".into()},Interval{start:22,end:29,phase:"queries".into()}];
        let windows=[(10,12),(19,22),(29,31)];
        assert_eq!(sample_bucket(15,&intervals,&windows),"solver");
        assert_eq!(sample_bucket(20,&intervals,&windows),"boundary_ambiguous");
        assert_eq!(sample_bucket(22,&intervals,&windows),"queries");
        assert_eq!(sample_bucket(29,&intervals,&windows),"boundary_ambiguous");
        assert_eq!(sample_bucket(32,&intervals,&windows),"outside_requests");
    }
    #[test]
    fn phase_cpu_deltas_conserve_process_work_and_reject_negative_counters() {
        let stamp=|time,user,system,switches|json!({"clock_before_ns":time,"clock_after_ns":time,
            "user_us":user,"system_us":system,"voluntary_switches":switches,
            "involuntary_switches":0,"minor_faults":0,"major_faults":0});
        let (a,b,c)=(stamp(0,0,0,0),stamp(1_000_000,250,50,2),stamp(3_000_000,400,100,4));
        let mut sum=Cost::default();sum.add(&a,&b);sum.add(&b,&c);
        assert_eq!(sum.value(1.)["cpu_ms"],json!(0.5));
        assert_eq!(sum.value(1.)["occupied_logical_cpus"],json!(0.5/3.));
        assert!(std::panic::catch_unwind(||Cost::default().add(&b,&a)).is_err());
    }
}
