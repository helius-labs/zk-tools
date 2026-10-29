#[derive(Debug,PartialEq,Eq)]
pub struct Groups { pub len:usize,pub values:Vec<[u64;4]>,pub order:Vec<u32>,pub starts:Vec<usize>,pub counts:Vec<usize> }

pub fn original(scalars:&[[u64;4]],capacity:usize,percent:usize,state:ahash::RandomState)->Option<Groups>{
    let mut groups =
        std::collections::HashMap::with_capacity_and_hasher(capacity, state);
    let mut values = Vec::new();
    let mut classes = vec![u32::MAX; scalars.len()];
    let mut counts: Vec<usize> = Vec::new();
    for (scalar, class) in scalars.iter().zip(&mut classes) {
        if *scalar == [0; 4] {
            continue;
        }
        let index = *groups.entry(*scalar).or_insert_with(|| {
            values.push(*scalar);
            counts.push(0);
            (values.len() - 1) as u32
        });
        *class = index;
        counts[index as usize] += 1;
    }
    drop(groups);
    if values.len() * 100 > scalars.len() * percent {
        return None;
    }
    if values.is_empty() {
        return Some(Groups {
            len: scalars.len(),
            values,
            order: Vec::new(),
            starts: vec![0],
            counts,
        });
    }
    let mut starts = Vec::with_capacity(counts.len() + 1);
    starts.push(0);
    for &count in &counts {
        starts.push(starts.last().unwrap() + count);
    }
    let mut positions = starts[..counts.len()].to_vec();
    let mut order = vec![0u32; *starts.last().unwrap()];
    for (i, &class) in classes.iter().enumerate() {
        if class != u32::MAX {
            order[positions[class as usize]] = i as u32;
            positions[class as usize] += 1;
        }
    }
    drop(classes);
    drop(positions);
    Some(Groups {
        len: scalars.len(),
        values,
        order,
        starts,
        counts,
    })
}

pub fn candidate(scalars:&[[u64;4]],capacity:usize,percent:usize,state:ahash::RandomState)->Option<Groups>{
    let mut groups = super::index_map::IndexMap::new(capacity, state);
    let mut values = Vec::new();
    let mut classes = vec![u32::MAX; scalars.len()];
    let mut counts: Vec<usize> = Vec::new();
    for (scalar, class) in scalars.iter().zip(&mut classes) {
        if *scalar == [0; 4] {
            continue;
        }
        let (index, fresh) = groups.insert(scalar, &mut values);
        if fresh { counts.push(0); }
        *class = index;
        counts[index as usize] += 1;
    }
    drop(groups);
    if values.len() * 100 > scalars.len() * percent {
        return None;
    }
    if values.is_empty() {
        return Some(Groups {
            len: scalars.len(),
            values,
            order: Vec::new(),
            starts: vec![0],
            counts,
        });
    }
    let mut starts = Vec::with_capacity(counts.len() + 1);
    starts.push(0);
    for &count in &counts {
        starts.push(starts.last().unwrap() + count);
    }
    let mut positions = starts[..counts.len()].to_vec();
    let mut order = vec![0u32; *starts.last().unwrap()];
    for (i, &class) in classes.iter().enumerate() {
        if class != u32::MAX {
            order[positions[class as usize]] = i as u32;
            positions[class as usize] += 1;
        }
    }
    drop(classes);
    drop(positions);
    Some(Groups {
        len: scalars.len(),
        values,
        order,
        starts,
        counts,
    })
}
