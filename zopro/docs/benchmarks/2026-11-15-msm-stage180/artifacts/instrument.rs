use std::{fs,collections::BTreeMap};
use serde_json::json;
use sha2::{Digest,Sha256};
fn main(){
 let root="/home/ubuntu/src/zopro-perf-work/msm-stage180";
 let p=format!("{root}/aeglos/cpu/msm_impl.hpp");let original=fs::read_to_string(&p).unwrap();
 assert_eq!(format!("{:x}",Sha256::digest(original.as_bytes())),"6a57cf5dd2312a1bcbadefa6b409c600dc33dd38cc138d11e3849b23d99aac85");
 let mut s=original.clone();let mut edits=vec![];
 let changes=[
 ("#include \"projective.hpp\"", "#include \"projective.hpp\"\n#include \"bench_msm_phases.hpp\""),
 ("    std::vector<Job *> jobs(given, given + count);", "    bench::PhaseTrace<> trace(bench::phases_enabled(), pool.size(), count);\n    trace.start(\"begin\");\n    std::vector<Job *> jobs(given, given + count);"),
 ("    std::vector<size_t> offsets(count + 1);\n    auto pass = [&](Phase phase) {", "    std::vector<size_t> offsets(count + 1);\n    trace.stop();\n    auto pass = [&](Phase phase) {\n      const char *names[] = {\"recode\",\"count\",\"scatter\",\"accumulate\",\"merge\",\"sums\"};\n      trace.start(names[size_t(phase)]);"),
 ("        jobs[j]->step(phase, i - offsets[j]);\n      });", "        jobs[j]->step(phase, i - offsets[j]);\n      });\n      trace.stop(offsets[count], offsets.data(), count);"),
 ("    for (Job *job : jobs)\n      job->place();", "    trace.start(\"place\");\n    for (Job *job : jobs)\n      job->place();\n    trace.stop();"),
 ("    for (size_t round = 0;; ++round) {\n      size_t pairs = 0;", "    for (size_t round = 0;; ++round) {\n      trace.start(\"merge_plan\");\n      size_t pairs = 0;"),
 ("        pairs += job->merge_round(round);\n      if (pairs == 0)", "        pairs += job->merge_round(round);\n      trace.stop(pairs);\n      if (pairs == 0)"),
 ("    pool.run(count, [&](size_t j) { jobs[j]->finish(); });\n    return AEGLOS_OK;", "    trace.start(\"finish\");\n    pool.run(count, [&](size_t j) { jobs[j]->finish(); });\n    trace.stop(count);\n    trace.finish();\n    return AEGLOS_OK;")];
 for (old,new) in changes{assert_eq!(s.matches(old).count(),1,"anchor {old}");s=s.replacen(old,new,1);edits.push(json!({"old":old,"new":new}));}
 let mut restored=s.clone();for v in edits.iter().rev(){let new=v["new"].as_str().unwrap();assert_eq!(restored.matches(new).count(),1);restored=restored.replacen(new,v["old"].as_str().unwrap(),1);}assert_eq!(restored,original);
 fs::write(&p,s).unwrap();fs::write(format!("{root}/source-edits.json"),serde_json::to_vec_pretty(&json!({"edits":edits,"original_sha256":"6a57cf5dd2312a1bcbadefa6b409c600dc33dd38cc138d11e3849b23d99aac85","recovery_exact":true})).unwrap()).unwrap();
 let pinned:serde_json::Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-pmu-c7a-aeglos/artifacts/aeglos-source-identity.json").unwrap()).unwrap();
 let mut pins=BTreeMap::new();for (name,want) in pinned["build"]["aeglos_candidate_sources"].as_object().unwrap(){let path=format!("{root}/aeglos/{name}");if name=="cpu/msm_impl.hpp"{continue;}let bytes=fs::read(&path).unwrap();let got=format!("{:x}",Sha256::digest(bytes));assert_eq!(want.as_str().unwrap(),got,"source mismatch {name}");pins.insert(path,got);}
 for name in ["cpu/msm_impl.hpp","cpu/bench_msm_phases.hpp"]{let path=format!("{root}/aeglos/{name}");pins.insert(path.clone(),format!("{:x}",Sha256::digest(fs::read(path).unwrap())));}
 fs::write(format!("{root}/native-source-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();println!("Exact source recovery; {} source pins",pins.len());
}
