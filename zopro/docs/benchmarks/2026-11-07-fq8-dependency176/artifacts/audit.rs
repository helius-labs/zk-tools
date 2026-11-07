use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fq8-dependency176";
const P: &str = "/home/ubuntu/src/zopro-perf-work/cios-outer-loop176";
const C: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-instruction-calibration176/artifacts/analysis.json";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
#[derive(Clone)]
struct Ins {
    addr: u64,
    op: String,
    args: Vec<String>,
}
#[derive(Clone)]
struct Edge {
    from: usize,
    weight: u64,
    role: String,
}
#[derive(Clone)]
struct Node {
    ins: Ins,
    edges: Vec<Edge>,
    ready: u64,
    parent: Option<usize>,
    lag: u64,
    known: bool,
    zero: bool,
}
fn reg(s: &str) -> Option<u8> {
    for prefix in ["xmm", "ymm", "zmm"] {
        if let Some(n) = s.trim().strip_prefix(prefix) {
            if !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) {
                let n = n.parse::<u8>().unwrap();
                assert!(n < 32);
                return Some(n);
            }
        }
    }
    None
}
fn decode(s: &str) -> Vec<Ins> {
    let mut ins = Vec::new();
    for line in s.lines() {
        let Some((a, b)) = line.trim().split_once(':') else {
            continue;
        };
        let Ok(addr) = u64::from_str_radix(a, 16) else {
            continue;
        };
        let b = b.split('#').next().unwrap().trim();
        let mut words = b.splitn(2, char::is_whitespace);
        let op = words.next().unwrap().to_string();
        let args = words
            .next()
            .unwrap_or("")
            .split(',')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        ins.push(Ins { addr, op, args });
    }
    ins
}
fn graph(ins: &[Ins], source_latency: u64) -> Vec<Node> {
    let mut values: [Option<usize>; 32] = [None; 32];
    let mut nodes: Vec<Node> = Vec::new();
    for i in ins {
        let dst = i.args.first().and_then(|a| reg(a));
        let zero = i.op.starts_with("vpxor")
            && i.args.len() == 3
            && i.args[1] == i.args[2]
            && reg(&i.args[1]).is_some();
        let ifma = i.op.starts_with("vpmadd52");
        let arithmetic = matches!(i.op.as_str(), "vpaddq" | "vpsubq")
            && i.args.first().is_some_and(|a| a.starts_with("zmm"));
        let shift = matches!(i.op.as_str(), "vpsrlq" | "vpsraq")
            && i.args.last().is_some_and(|a| a == "0x34")
            && i.args.first().is_some_and(|a| a.starts_with("zmm"));
        let known = zero || ifma || arithmetic || shift;
        let mut reads = Vec::new();
        if !zero {
            if ifma {
                assert_eq!(i.args.len(), 3);
                reads.push((dst.unwrap(), 4, "accumulator"));
            }
            for (j, a) in i.args.iter().enumerate() {
                if j == 0 && dst.is_some() {
                    continue;
                }
                if let Some(r) = reg(a) {
                    let (weight, role) = if ifma {
                        (source_latency, "multiply_source_unknown")
                    } else if arithmetic {
                        (2, "add_sub")
                    } else if shift {
                        (2, "right_shift52")
                    } else {
                        (0, "uncalibrated_or_copy")
                    };
                    reads.push((r, weight, role));
                }
            }
        }
        let mut edges = Vec::new();
        let mut ready = 0;
        let mut parent = None;
        let mut lag = 0;
        for (r, w, role) in reads {
            if let Some(from) = values[r as usize] {
                let t = nodes[from].ready + w;
                if t > ready || (t == ready && w > lag) {
                    ready = t;
                    parent = Some(from);
                    lag = w;
                }
                edges.push(Edge {
                    from,
                    weight: w,
                    role: role.to_string(),
                });
            } else if w > ready {
                ready = w;
                parent = None;
                lag = w;
            }
        }
        let index = nodes.len();
        nodes.push(Node {
            ins: i.clone(),
            edges,
            ready,
            parent,
            lag,
            known,
            zero,
        });
        if let Some(r) = dst {
            values[r as usize] = Some(index);
        }
    }
    nodes
}
fn qualify() {
    let a=decode("10: vpxor xmm0,xmm0,xmm0\n11: vpmadd52luq zmm0,zmm1,zmm2\n12: vpaddq zmm3,zmm0,zmm4\n13: vpsrlq zmm3,zmm3,0x34\n14: vpmadd52luq zmm5,zmm3,zmm2\n");
    let low = graph(&a, 0);
    let high = graph(&a, 4);
    assert_eq!(
        low.iter().map(|n| n.ready).collect::<Vec<_>>(),
        [0, 4, 6, 8, 8]
    );
    assert_eq!(
        high.iter().map(|n| n.ready).collect::<Vec<_>>(),
        [0, 4, 6, 8, 12]
    );
    let mut b = a.clone();
    b.extend(decode("15: vpxord xmm3,xmm3,xmm3\n16: vmovdqa64 zmm7,zmm3\n17: vpsrlq zmm7,zmm7,0x30\n18: vpmadd52huq zmm7,zmm1,zmm2\n"));
    let n = graph(&b, 0);
    assert_eq!(n[5].ready, 0);
    assert_eq!(n[6].ready, 0);
    assert_eq!(n[7].ready, 0);
    assert!(!n[7].known);
    assert_eq!(n[8].ready, 4);
    assert_eq!(n[8].edges[0].role, "accumulator");
}
fn check_relaxation(nodes: &[Node]) {
    let mut distance = vec![0; nodes.len()];
    // Initialize source-less incoming edges with their separately retained lag.
    for (i, n) in nodes.iter().enumerate() {
        if n.parent.is_none() {
            distance[i] = n.lag;
        }
    }
    // Independent all-edges relaxation, without using the one-pass readiness recurrence.
    for _ in 0..=nodes.len() {
        let mut changed = false;
        for (i, n) in nodes.iter().enumerate() {
            for e in &n.edges {
                assert!(e.from < i);
                let candidate = distance[e.from] + e.weight;
                if candidate > distance[i] {
                    distance[i] = candidate;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    for (i, n) in nodes.iter().enumerate() {
        assert_eq!(distance[i], n.ready, "node {i}");
    }
}
fn main() {
    assert!(!Path::new(&format!("{R}/analysis.json")).exists());
    for (p, h) in load(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    qualify();
    let calibration = load(C);
    assert_eq!(calibration["status"], "pass");
    let mut measured = BTreeMap::new();
    for c in calibration["cases"].as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let median = c["cycles_per_target_instruction"]["median"]
            .as_f64()
            .unwrap();
        measured.insert(id.to_string(), median);
    }
    for id in ["ifma-lo-acc-lat", "ifma-hi-acc-lat"] {
        assert!((measured[id] - 4.0).abs() < 0.01);
    }
    for id in ["add-lat", "sub-lat", "shr52-lat", "sar52-lat"] {
        assert!((measured[id] - 2.0).abs() < 0.01);
    }
    for id in ["ifma-lo-reg-tp", "ifma-hi-reg-tp"] {
        assert!((measured[id] - 0.5).abs() < 0.01);
    }
    let parent = load(format!("{P}/analysis.json"));
    let mut cases = BTreeMap::new();
    for (name, sites) in [("probe_fq8_mul", 100usize), ("probe_fq8_sum2", 150)] {
        let instructions = decode(
            &fs::read_to_string(format!(
                "{P}/continuation/decode-accepted-{name}/stdout.log"
            ))
            .unwrap(),
        );
        let prefix_len = instructions
            .iter()
            .position(|i| i.op.starts_with('j'))
            .unwrap_or(instructions.len());
        let prefix = &instructions[..prefix_len];
        assert_eq!(
            prefix
                .iter()
                .filter(|i| i.op.starts_with("vpmadd52"))
                .count(),
            sites
        );
        assert_eq!(
            parent["panels"][format!("accepted/{name}")]["IFMA_sites"],
            sites
        );
        let mut rows = Vec::new();
        let mut optimistic_graph = Vec::new();
        for source_latency in 0..=4 {
            let nodes = graph(prefix, source_latency);
            check_relaxation(&nodes);
            let (last, peak) = nodes
                .iter()
                .enumerate()
                .max_by_key(|(_, n)| n.ready)
                .unwrap();
            let mut path = Vec::new();
            let mut cursor = Some(last);
            let mut path_weight = 0;
            while let Some(i) = cursor {
                let n = &nodes[i];
                path_weight += n.lag;
                path.push(json!({"index":i,"offset":n.ins.addr-instructions[0].addr,"op":n.ins.op,"operands":n.ins.args,"ready":n.ready,"incoming_lag":n.lag}));
                cursor = n.parent;
            }
            path.reverse();
            assert_eq!(path_weight, peak.ready);
            rows.push(json!({"multiply_source_weight":source_latency,"is_measured_source_latency":false,"completion_cycles_in_model":peak.ready,"critical_chain":path}));
            if source_latency == 0 {
                optimistic_graph=nodes.iter().enumerate().map(|(i,n)|json!({"index":i,"offset":n.ins.addr-instructions[0].addr,"op":n.ins.op,"operands":n.ins.args,"ready":n.ready,"known_form":n.known,"zero_idiom":n.zero,"edges":n.edges.iter().map(|e|json!({"from":e.from,"weight":e.weight,"role":e.role})).collect::<Vec<_>>()})).collect();
            }
        }
        let unknown: BTreeMap<_, _> = prefix
            .iter()
            .filter(|i| {
                !(i.op.starts_with("vpxor")
                    || i.op.starts_with("vpmadd52")
                    || matches!(i.op.as_str(), "vpaddq" | "vpsubq")
                    || matches!(i.op.as_str(), "vpsrlq" | "vpsraq")
                        && i.args.last().is_some_and(|a| a == "0x34"))
            })
            .fold(BTreeMap::<String, usize>::new(), |mut a, i| {
                *a.entry(i.op.clone()).or_default() += 1;
                a
            });
        cases.insert(name,json!({"prefix_instructions":prefix_len,"IFMA_sites":sites,"nominal_IFMA_resource_floor_cycles":sites as f64*0.5,"branch_boundary_offset":instructions.get(prefix_len).map(|i|i.addr-instructions[0].addr),"hypothetical_source_weight_rows":rows,"optimistic_graph":optimistic_graph,"unmeasured_or_zero_cost_forms":unknown}));
    }
    let out = json!({"cases":cases,"calibration_medians":measured,"toy_chains_passed":true,"all_edges_relaxation_passed":true,"model_only":true,"unknown_multiply_source_latencies_not_measured":true,"memory_dependencies_ignored":true,"new_native_calls":0,"new_timings":0,"new_proofs":0,"adopted":false});
    fs::write(
        format!("{R}/analysis.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    for (name, c) in out["cases"].as_object().unwrap() {
        println!(
            "{name}: resource floor {}; completion at hypothetical source0..4 = {:?}",
            c["nominal_IFMA_resource_floor_cycles"],
            c["hypothetical_source_weight_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| r["completion_cycles_in_model"].as_u64().unwrap())
                .collect::<Vec<_>>()
        );
    }
}
