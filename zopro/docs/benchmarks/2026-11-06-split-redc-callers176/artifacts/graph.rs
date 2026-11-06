fn vector_reg(s: &str) -> Option<usize> {
    let s = s.trim();
    let t = s
        .strip_prefix("zmm")
        .or_else(|| s.strip_prefix("ymm"))
        .or_else(|| s.strip_prefix("xmm"))?;
    let n = t.chars().take_while(char::is_ascii_digit).count();
    if n == 0 {
        return None;
    }
    let i = t[..n].parse::<usize>().ok()?;
    if i < 32 {
        Some(i)
    } else {
        None
    }
}
fn calibrated(s: &str) -> Option<(usize, Vec<usize>, u64, u64)> {
    let s = s.split('#').next().unwrap().trim();
    let (m, a) = s.split_once(char::is_whitespace)?;
    let args = a.split(',').map(str::trim).collect::<Vec<_>>();
    if args.len() != 3
        || args.iter().any(|s| s.contains(['[', '{', '}']))
        || !args[0].starts_with("zmm")
    {
        return None;
    }
    let d = vector_reg(args[0])?;
    if m == "vpmadd52luq" || m == "vpmadd52huq" {
        if !args[1].starts_with("zmm") || !args[2].starts_with("zmm") {
            return None;
        }
        return Some((d, vec![d, vector_reg(args[1])?, vector_reg(args[2])?], 4, 4));
    }
    if m == "vpaddq" || m == "vpsubq" {
        if !args[1].starts_with("zmm") || !args[2].starts_with("zmm") {
            return None;
        }
        return Some((d, vec![vector_reg(args[1])?, vector_reg(args[2])?], 2, 1));
    }
    if (m == "vpsrlq" || m == "vpsraq") && args[1].starts_with("zmm") && args[2] == "0x34" {
        return Some((d, vec![vector_reg(args[1])?], 2, 1));
    }
    None
}
fn score(block: &[(u64, String)]) -> Value {
    let mut last = [None; 32];
    let mut nodes = vec![];
    let mut scores = vec![];
    let mut predecessors = Vec::<Option<usize>>::new();
    let mut old = vec![];
    let mut breaks = BTreeMap::<String, u64>::new();
    for (ip, s) in block {
        if let Some((d, rs, w, ow)) = calibrated(s) {
            let pred = rs
                .iter()
                .filter_map(|r| last[*r])
                .max_by_key(|j: &usize| (scores[*j], old[*j]));
            let cur = pred.map(|j| scores[j]).unwrap_or(0) + w;
            let prev = pred.map(|j| old[j]).unwrap_or(0) + ow;
            last[d] = Some(nodes.len());
            nodes.push(json!({"ip_hex":format!("{ip:#x}"),"instruction":s,"calibrated_weight":w,"inherited_model_weight":ow}));
            scores.push(cur);
            old.push(prev);
            predecessors.push(pred);
        } else {
            let (m, a) = s
                .split_once(char::is_whitespace)
                .unwrap_or((s.as_str(), ""));
            if m == "vzeroall" || m == "vzeroupper" {
                last = [None; 32];
            } else if m.starts_with('v') {
                if let Some(d) = a.split(',').next().and_then(vector_reg) {
                    last[d] = None;
                    *breaks.entry(m.into()).or_default() += 1;
                }
            }
        }
    }
    let best = scores
        .iter()
        .enumerate()
        .max_by_key(|(i, v)| (**v, old[*i]))
        .map(|(i, _)| i);
    let mut path = vec![];
    let mut at = best;
    while let Some(i) = at {
        path.push(nodes[i].clone());
        at = predecessors[i];
    }
    path.reverse();
    json!({"instruction_rows":block.len(),"calibrated_register_only_rows":nodes.len(),"unknown_vector_writer_boundaries":breaks,"max_connected_calibrated_score":best.map(|i|scores[i]).unwrap_or(0),"inherited_weights_on_same_path":best.map(|i|old[i]).unwrap_or(0),"selected_chain":path})
}
fn blocks(inst: &BTreeMap<u64, String>) -> BTreeMap<u64, Value> {
    let first = *inst.first_key_value().unwrap().0;
    let mut starts = BTreeSet::from([first]);
    let rows = inst.iter().collect::<Vec<_>>();
    for (i, (_, s)) in rows.iter().enumerate() {
        let (m, a) = s
            .split_once(char::is_whitespace)
            .unwrap_or((s.as_str(), ""));
        if m.starts_with('j') || m.starts_with("call") || m.starts_with("ret") || m == "ud2" {
            if let Some((ip, _)) = rows.get(i + 1) {
                starts.insert(**ip);
            }
            if m.starts_with('j') {
                if let Some(t) = a.split_whitespace().next() {
                    if let Ok(target) = u64::from_str_radix(t, 16) {
                        if inst.contains_key(&target) {
                            starts.insert(target);
                        }
                    }
                }
            }
        }
    }
    let starts = starts.into_iter().collect::<Vec<_>>();
    let mut out = BTreeMap::new();
    for (i, start) in starts.iter().enumerate() {
        let end = starts.get(i + 1).copied().unwrap_or(u64::MAX);
        let b = inst
            .range(*start..end)
            .map(|(a, s)| (*a, s.clone()))
            .collect::<Vec<_>>();
        let mut v = score(&b);
        v["start_hex"] = json!(format!("{start:#x}"));
        v["last_hex"] = json!(format!("{:#x}", b.last().unwrap().0));
        out.insert(*start, v);
    }
    assert_eq!(
        out.values()
            .map(|v| v["instruction_rows"].as_u64().unwrap())
            .sum::<u64>(),
        inst.len() as u64
    );
    out
}
