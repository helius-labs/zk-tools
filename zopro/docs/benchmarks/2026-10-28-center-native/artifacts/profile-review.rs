use serde_json::{json,Value};use std::{collections::BTreeMap,fmt::Write,fs};
const R:&str="/home/ubuntu/src/zopro-perf-work/center-native";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-center-native";
fn main(){let mut a:Value=serde_json::from_slice(&fs::read(format!("{R}/analysis.json")).unwrap()).unwrap();let mut result=BTreeMap::new();let mut table=String::from("| Workers | Variant | Warm cycles samples | Lost | Centered namespace share | Centered borrowed-loop share |\n|---|---|---:|---:|---:|---:|\n");
    for w in[32,16]{for label in["control","off","on"]{let name=format!("profile-w{w}-{label}");let profile=&mut a["profiles"][&name];let period=profile["period"].as_u64().unwrap();let mut symbols=BTreeMap::<String,u64>::new();
        for(s,n)in profile["symbols"].as_object().unwrap(){let demangled=format!("{:#}",rustc_demangle::demangle(s));*symbols.entry(demangled).or_default()+=n.as_u64().unwrap();}
        assert_eq!(symbols.values().sum::<u64>(),period);
        let centered:u64=symbols.iter().filter(|(s,_)|s.contains("::centered::")).map(|(_,n)|*n).sum();
        let center_loop:u64=symbols.iter().filter(|(s,_)|s.contains("::borrowed_pending::msm_impl")&&s.contains("true")).map(|(_,n)|*n).sum();
        let mut categories=BTreeMap::new();for(key,needle)in[("g1_stream","::msm::stream_g1_impl"),("g2_prefix","::msm::g2_prefix::apply"),("epoch_pin","crossbeam_epoch::default::with_handle"),("scalar_inverse","::inverse::inverse"),("g1_g2_affine_reduce","::msm::reduce_layer")]{let value:u64=symbols.iter().filter(|(s,_)|s.contains(needle)).map(|(_,n)|*n).sum();categories.insert(key,value);}
        profile["demangled_centered_namespace_period"]=json!(centered);profile["demangled_centered_borrowed_loop_period"]=json!(center_loop);
        writeln!(table,"| {w} | {label} | {} | 0 | {:.3}% | {:.3}% |",profile["samples"],100.*centered as f64/period as f64,100.*center_loop as f64/period as f64).unwrap();
        result.insert(name,json!({"period":period,"samples":profile["samples"],"demangled_symbols":symbols,"centered_namespace_period":centered,"centered_borrowed_loop_period":center_loop,"selected_categories":categories}));
    }}
    a["profile_symbol_matcher_limitation"]=json!("Original literal centered:: match was applied to mangled symbols and returnedzero; demangled fields andprofile-review.json correct the labeling. No PMU data or capture was changed.");
    fs::write(format!("{R}/analysis.json"),serde_json::to_vec_pretty(&a).unwrap()).unwrap();fs::write(format!("{D}/artifacts/analysis.json"),serde_json::to_vec_pretty(&a).unwrap()).unwrap();
    fs::write(format!("{R}/profile-review.json"),serde_json::to_vec_pretty(&json!(result)).unwrap()).unwrap();fs::copy(format!("{R}/profile-review.json"),format!("{D}/artifacts/profile-review.json")).unwrap();
    let report_path=format!("{D}/README.md");let mut report=fs::read_to_string(&report_path).unwrap();
    report=report.replacen("The declared primary SDK screen **SDK screen rejected**.","The native candidate did not establish a full-proof gain and was not adopted. At32workers, accepted/candidateOFF/candidateON block-median times are8.498849/8.545417/8.565815ms. The paired ON/accepted ratio is1.008541670, or0.854%slower, with interval[1.002988799,1.015216649] and2/15wins.",1);
    let first=report.find("| Workers | Variant | Warm cycles samples |").unwrap();let last=report[first..].find("Each profile contains one SDK PID.").unwrap()+first;report.replace_range(first..last,&format!("{table}\n"));
    let note="The initial literal namespace matcher saw mangled Rust names and returnedzero. Offline demangling corrects that labeling in the table and analysis; no sample or capture changed. The centered namespace includes the encoder, while the specialized centered borrowed-loop symbol includes existing A-query queue, arithmetic and final-window work as well as new digit handling. Their periods must not be interpreted as isolated overhead. Primary ON period shares remain dominated by G1 stream arithmetic(~19.04%), G2 prefix arithmetic(~14.67%), epoch pinning(~8.55%), and scalar inversion(~7.83%); category shares alone do not establish the cause of the latency difference.\n\n";
    let pos=report.find("Each profile contains one SDK PID.").unwrap();report.insert_str(pos,note);
    report=report.replace("The following accepted local timings", "The secondary16-worker paired medians are positive(ON/accepted0.987497, ON/OFF0.993021), but OFF/control calibration fails and both upper nonregression bounds exceed1.02. These estimates are retained without selecting them as an established win.\n\nThe following accepted local timings");
    fs::write(report_path,report).unwrap();println!("Demangled six storedprofiles; corrected namespace matcher; no capture repeated");
}
