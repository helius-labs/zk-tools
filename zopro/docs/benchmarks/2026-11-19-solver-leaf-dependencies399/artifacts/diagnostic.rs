// Key-load diagnostic. Emit task lengths and dependency IDs only.
fn trace_leaf_dependencies(
    plan: &Plan, epochs: &[Vec<Vec<u32>>], refined: &FxMap<usize, Vec<Vec<Vec<u32>>>>,
    scratch: &mut Scratch,
) {
    let mut leaves = Vec::new();
    for (e, tasks) in epochs.iter().enumerate() {
        for (p, task) in tasks.iter().enumerate() {
            if let Some(waves) = refined.get(&(task[0] as usize)) {
                for (w, parts) in waves.iter().enumerate() {
                    for part in parts { leaves.push((e, p, w, part.as_slice())); }
                }
            } else { leaves.push((e, p, 0, task.as_slice())); }
        }
    }
    scratch.producer.fill(u32::MAX);
    scratch.parent.fill(u32::MAX);
    let mut duplicate = false;
    for (id, (_, _, _, ops)) in leaves.iter().enumerate() {
        for &op in *ops {
            assert_eq!(scratch.parent[op as usize], u32::MAX);
            scratch.parent[op as usize] = id as u32;
            each_write(plan, &plan.ops[op as usize], |wire| {
                duplicate |= scratch.producer[wire] != u32::MAX;
                scratch.producer[wire] = op;
            });
        }
    }
    assert!(scratch.parent.iter().all(|&id| id != u32::MAX));
    if duplicate { println!("LEAF_DUPLICATE"); return; }
    let mut edges = std::collections::BTreeSet::new();
    for (id, &(e,p,w,ops)) in leaves.iter().enumerate() {
        edges.clear();
        for &op in ops {
            plan.reads(&plan.ops[op as usize], &mut scratch.reads);
            for &wire in &scratch.reads {
                let writer = scratch.producer[wire];
                if writer == u32::MAX || writer == op { continue; }
                assert!(writer < op);
                let pred = scratch.parent[writer as usize] as usize;
                if pred != id {
                    let (pe, pp, pw, _) = leaves[pred];
                    assert!(pe < e || (pe == e && pp == p && pw < w));
                    edges.insert(pred);
                }
            }
        }
        println!("LEAF {id} {e} {p} {w} {} {:?}",ops.len(),edges);
    }
}
