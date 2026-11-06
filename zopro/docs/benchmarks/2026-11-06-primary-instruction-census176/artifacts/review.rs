use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/primary-instruction-census176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn hx(s: &str) -> u64 {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}
fn kind(m: &str, op: &str) -> String {
    let family = if m.starts_with("vpmadd52") {
        "IFMA"
    } else if m.starts_with("vmov") {
        "vector_move"
    } else if m.starts_with('v')
        && (m.contains("perm") || m.contains("shuf") || m.contains("unpck") || m.contains("align"))
    {
        "vector_shuffle"
    } else if m.starts_with('v') {
        "vector_other"
    } else if m.starts_with('j') || m.starts_with("call") || m.starts_with("ret") {
        return "control".into();
    } else if m.starts_with("nop") {
        return "nop".into();
    } else if m.starts_with("lea") {
        return "scalar_address".into();
    } else if m.starts_with("mov") {
        "scalar_move"
    } else {
        "scalar_other"
    };
    let suffix = if op.contains('(') {
        if op.contains("%rsp") {
            "RSP_memory"
        } else {
            "other_memory"
        }
    } else {
        "register_or_immediate"
    };
    format!("{family}_{suffix}")
}
fn main() {
    assert!(!r("review.json").exists());
    let a = read(r("analysis.json"));
    let pins = read(r("pins.json"));
    for (p, sha) in pins.as_object().unwrap() {
        assert_eq!(hash(p), sha.as_str().unwrap());
    }
    assert_eq!(pins.as_object().unwrap().len(), 218);
    let step = r("workflow/accepted-instruction-census");
    let exit = read(step.join("exit.json"));
    assert_eq!(exit["success"], true);
    assert_eq!(exit["returncode"], 0);
    for s in ["stdout", "stderr"] {
        assert_eq!(
            hash(step.join(format!("{s}.log"))),
            exit[format!("{s}_sha256")].as_str().unwrap()
        );
    }
    assert_eq!(read(r("workflow/result.json"))["status"], "complete");
    let plan = read(r("plan.json"));
    assert_eq!(
        &read(step.join("command.json"))["argv"].as_array().unwrap()[4..],
        plan["steps"][0]["argv"].as_array().unwrap()
    );
    let nm = fs::read_to_string(r("nm.stdout")).unwrap();
    let mut syms = BTreeMap::new();
    for l in nm.lines() {
        let f = l.split_whitespace().collect::<Vec<_>>();
        if f.len() == 4 && ["t", "T"].contains(&f[2]) {
            let size = hx(f[1]);
            if size > 0 {
                assert!(syms.insert(f[3].to_string(), (hx(f[0]), size)).is_none());
            }
        }
    }
    let mut command_count = 0;
    for stem in std::iter::once("nm".to_string()).chain((0..12).map(|i| format!("symbol-{i:02}"))) {
        let cmd = read(r(&format!("{stem}.command.json")));
        assert_eq!(cmd["success"], true);
        assert_eq!(cmd["returncode"], 0);
        assert_eq!(hash(r(&format!("{stem}.stdout"))), cmd["stdout_sha256"]);
        assert_eq!(hash(r(&format!("{stem}.stderr"))), cmd["stderr_sha256"]);
        assert_eq!(
            cmd["argv"].as_array().unwrap().last().unwrap(),
            &a["binary"]
        );
        command_count += 1;
    }
    assert_eq!(command_count, 13);
    let mut all = vec![];
    let mut inference = vec![];
    let mut counts = BTreeMap::<String, usize>::new();
    let producer = read(format!("{Q}/analysis.json"));
    for (w, label) in [(16, "w16"), (32, "w32")] {
        let name = format!("cycles-1x2-w{w}-accepted");
        let mut rows = vec![];
        let mut lo = 0u64;
        let mut hi = u64::MAX;
        let mut constraints = 0;
        let p = &producer["profiles"][&name];
        let pid = p["actual_pid"].as_u64().unwrap();
        let tids = p["tids"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect::<BTreeSet<_>>();
        for l in fs::read_to_string(format!("{Q}/runs/decode-{name}/stdout.log"))
            .unwrap()
            .lines()
        {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            let f = l.split_whitespace().collect::<Vec<_>>();
            assert!(f.len() >= 6);
            let (pp, tt) = f[0].split_once('/').unwrap();
            assert_eq!(pp.parse::<u64>().unwrap(), pid);
            assert!(tids.contains(&tt.parse::<u64>().unwrap()));
            assert!(f[1].trim_matches(['[', ']']).parse::<usize>().unwrap() < w);
            assert_eq!(f[3].parse::<u64>().unwrap(), 16000000);
            let pc = hx(f[4]);
            let raw = f[5..].join(" ");
            if raw.starts_with("_R") {
                if let Some(&(first, size)) = syms.get(&raw) {
                    lo = lo.max(pc - (first + size - 1));
                    hi = hi.min(pc - first);
                    constraints += 1;
                }
            }
            rows.push((pc, raw));
        }
        let bias = lo.div_ceil(4096) * 4096;
        assert!(lo <= bias && bias <= hi && bias + 4096 > hi);
        assert!(constraints > 1000);
        let coverage = &a["profile_coverage"][label];
        assert_eq!(coverage["mapping"]["ELF_load_bias"], bias);
        assert_eq!(coverage["total_samples"], rows.len());
        assert_eq!(p["samples"], rows.len());
        let start = coverage["mapping"]["runtime_start"].as_u64().unwrap();
        let end = start + coverage["mapping"]["runtime_bytes"].as_u64().unwrap();
        let mut own = 0;
        for (pc, raw) in &rows {
            if *pc >= start && *pc < end {
                own += 1;
                let (first, size) = syms.get(raw).unwrap();
                let virt = pc - bias;
                assert!(*first <= virt && virt < first + size);
                *counts.entry(raw.clone()).or_default() += 1;
            }
        }
        assert_eq!(coverage["own_executable_samples"], own);
        assert_eq!(coverage["in_executable_no_exact_defined_symbol"], 0);
        assert_eq!(coverage["outside_executable_samples"], rows.len() - own);
        inference.push(json!({"profile":label,"symbol_interval_constraints":constraints,"lower_bound":lo,"upper_bound":hi,"unique_page_aligned_bias":bias,"matches_ELF_MMAP_derivation":true}));
        all.push((label.to_string(), rows, bias, start, end));
    }
    let mut ranked = counts.into_iter().collect::<Vec<_>>();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let mut total = 0;
    let mut sites = 0;
    let mut summary = vec![];
    for (i, f) in a["selected_functions"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let raw = f["raw_symbol"].as_str().unwrap();
        assert_eq!(raw, ranked[i].0);
        assert_eq!(f["combined_samples"], ranked[i].1);
        let (first, size) = syms[raw];
        assert_eq!(f["ELF_start"], first);
        assert_eq!(f["bytes"], size);
        let stem = f["objdump_stem"].as_str().unwrap();
        let cmd = read(r(&format!("{stem}.command.json")));
        assert_eq!(cmd["argv"][3], format!("--disassemble={raw}"));
        let mut ins = BTreeMap::new();
        let mut static_bins = BTreeMap::<String, usize>::new();
        for l in fs::read_to_string(r(&format!("{stem}.stdout")))
            .unwrap()
            .lines()
        {
            let t = l.trim().split_whitespace().collect::<Vec<_>>();
            if t.len() < 2 || !t[0].ends_with(':') {
                continue;
            }
            let Ok(pc) = u64::from_str_radix(t[0].trim_end_matches(':'), 16) else {
                continue;
            };
            assert!(first <= pc && pc < first + size);
            let text = l.trim().split_once(':').unwrap().1.trim();
            let (m, op) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
            let op = op.split('#').next().unwrap().trim();
            let k = kind(m, op);
            *static_bins.entry(k.clone()).or_default() += 1;
            assert!(ins.insert(pc, (m.to_string(), op.to_string(), k)).is_none());
        }
        assert_eq!(f["instruction_sites"], ins.len());
        assert_eq!(f["static_classes"], json!(static_bins));
        sites += ins.len();
        let mut pc_counts = BTreeMap::<u64, BTreeMap<String, usize>>::new();
        let mut selected_profile_total = 0;
        let mut row =
            json!({"rank":i+1,"symbol":f["symbol"],"bytes":size,"instructions":ins.len()});
        for (label, samples, bias, start, end) in &all {
            let mut bins = BTreeMap::<String, usize>::new();
            let mut n = 0;
            let mut rsp = 0;
            let mut ifma = 0;
            for (pc, s) in samples
                .iter()
                .filter(|(pc, s)| s == raw && *pc >= *start && *pc < *end)
            {
                let _ = s;
                let virt = pc - bias;
                let (_, _, k) = ins.get(&virt).unwrap();
                *bins.entry(k.clone()).or_default() += 1;
                *pc_counts
                    .entry(virt)
                    .or_default()
                    .entry(label.clone())
                    .or_default() += 1;
                n += 1;
                if k.ends_with("RSP_memory") {
                    rsp += 1;
                }
                if k.starts_with("IFMA_") {
                    ifma += 1;
                }
            }
            let observed = &f["profile_counts"][label];
            assert_eq!(observed["selected_symbol_samples"], n);
            assert_eq!(observed["exact_instruction_starts"], n);
            assert!(observed["unresolved_or_interior_IPs"]
                .as_array()
                .unwrap()
                .is_empty());
            assert_eq!(observed["classes"], json!(bins));
            row[label] = json!({"samples":n,"IFMA_instruction_samples":ifma,"explicit_RSP_operand_samples":rsp});
            selected_profile_total += n;
            total += n;
        }
        assert_eq!(selected_profile_total, ranked[i].1);
        let rows = f["all_sampled_instructions"].as_array().unwrap();
        assert_eq!(rows.len(), pc_counts.len());
        for p in rows {
            let pc = p["ELF_pc"].as_u64().unwrap();
            let (m, op, k) = &ins[&pc];
            assert_eq!(p["instruction"]["offset"], pc - first);
            assert_eq!(p["instruction"]["mnemonic"], m.as_str());
            assert_eq!(p["instruction"]["operands"], op.as_str());
            assert_eq!(p["instruction"]["class"], k.as_str());
            assert_eq!(p["samples"], json!(pc_counts[&pc]));
        }
        summary.push(row);
    }
    assert_eq!(a["exact_selected_instruction_samples"], total);
    assert_eq!(a["unresolved_selected_IPs"], 0);
    let ledger = read(
        "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176/proof-sha256-after.json",
    );
    assert_eq!(ledger.as_array().unwrap().len(), 30846);
    let out = json!({"review_passed":true,"pins_verified":218,"workflow_commands":1,"read_only_subcommands":13,"mapping_inference_independent_of_ELF_parser":inference,"selected_functions":12,"instruction_sites":sites,"exact_selected_samples":total,"all_selected_IPs_exact_instruction_starts":true,"source_and_sample_conservation_passed":true,"summary":summary,"sampled_instruction_class_is_not_causal_cost":true,"producer_combined_gate_remains_failed":true,"new_performance_recordings":0,"new_proofs":0,"ledger":30846,"goal_status":"active/unmet0/8"});
    fs::write(r("review.json"), serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("Review PASS218pins/13subcommands/12functions/{sites}instruction sites/{total}samples/independent page-bias inference; no new performance");
}
