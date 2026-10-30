use serde_json::{json, Value};
const MAGIC: u32 = 0x3150535a;
const STORED: u32 = 1 << 31;
const NONE: u32 = STORED - 1;
const CHAIN: u32 = STORED - 2;
const UNWRITTEN: usize = usize::MAX;
struct Reader<'a> {
    data: &'a [u8],
    at: usize,
}
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self.at.checked_add(n).ok_or("offset overflow")?;
        let b = self.data.get(self.at..end).ok_or("truncated image")?;
        self.at = end;
        Ok(b)
    }
    fn word(&mut self) -> Result<usize, String> {
        Ok(u32::from_le_bytes(self.bytes(4)?.try_into().unwrap()) as usize)
    }
    fn words<const N: usize>(&mut self) -> Result<[usize; N], String> {
        let mut a = [0; N];
        for v in &mut a {
            *v = self.word()?;
        }
        Ok(a)
    }
}
fn need(yes: bool, msg: &str) -> Result<(), String> {
    if yes {
        Ok(())
    } else {
        Err(msg.into())
    }
}
fn range(a: usize, b: usize, n: usize) -> Result<std::ops::Range<usize>, String> {
    need(a <= b && b <= n, "invalid range")?;
    Ok(a..b)
}
struct State {
    producer: Vec<usize>,
    last_read: Vec<usize>,
    queries: [Vec<usize>; 2],
    ready: [Vec<bool>; 2],
    prefix: [usize; 2],
    snapshots: Vec<Value>,
}
impl State {
    fn read(&mut self, wire: usize, op: usize) -> Result<(), String> {
        need(wire < self.producer.len(), "wire out of range")?;
        need(self.producer[wire] != UNWRITTEN, "read before write")?;
        self.last_read[wire] = op;
        Ok(())
    }
    fn write(&mut self, wire: usize, op: usize) -> Result<(), String> {
        need(wire < self.producer.len(), "write out of range")?;
        need(self.producer[wire] == UNWRITTEN, "duplicate write")?;
        self.producer[wire] = op;
        for q in 0..2 {
            if let Ok(i) = self.queries[q].binary_search(&wire) {
                self.ready[q][i] = true;
            }
            while self.prefix[q] < self.ready[q].len() && self.ready[q][self.prefix[q]] {
                self.prefix[q] += 1;
            }
        }
        Ok(())
    }
    fn snapshot(&mut self, cut: usize) {
        self.snapshots.push(json!({"op_cut":cut,"A_ready":self.ready[0].iter().filter(|&&v|v).count(),"B_ready":self.ready[1].iter().filter(|&&v|v).count(),"A_prefix":self.prefix[0],"B_prefix":self.prefix[1]}));
    }
}
pub fn census(data: &[u8]) -> Result<Value, String> {
    let mut r = Reader { data, at: 0 };
    need(r.word()? == MAGIC as usize, "bad magic")?;
    need(r.word()? == 2, "unsupported version")?;
    let [nw, ni, np, nc, na, nb, ncoef, nt, ns, nslots, nops] = r.words::<11>()?;
    need(
        np > 0 && np <= ni && ni <= nw && nslots <= ns,
        "invalid dimensions",
    )?;
    let need_bytes = 4u64 * (na as u64 + nb as u64)
        + 32 * ncoef as u64
        + 8 * nt as u64
        + 12 * ns as u64
        + 16 * nops as u64;
    need(
        need_bytes <= (data.len() - r.at) as u64,
        "lists exceed file",
    )?;
    let mut queries = [Vec::with_capacity(na), Vec::with_capacity(nb)];
    for (q, n) in queries.iter_mut().zip([na, nb]) {
        for _ in 0..n {
            let x = r.word()?;
            need(
                x < nw && q.last().is_none_or(|&last| last < x),
                "unsorted query",
            )?;
            q.push(x);
        }
    }
    r.bytes(ncoef.checked_mul(32).ok_or("coefficient overflow")?)?;
    let mut terms = Vec::with_capacity(nt);
    for _ in 0..nt {
        let [c, w] = r.words::<2>()?;
        need(c < ncoef && w < nw, "invalid term")?;
        terms.push(w);
    }
    let mut sums = Vec::with_capacity(ns);
    for _ in 0..ns {
        let [p, c, w] = r.words::<3>()?;
        need(c < ncoef && w < nw, "invalid sum term")?;
        sums.push((p as u32, w));
    }
    let nv = nw.checked_add(nslots).ok_or("value overflow")?;
    let mut state = State {
        producer: vec![UNWRITTEN; nv],
        last_read: vec![0; nv],
        ready: [vec![false; na], vec![false; nb]],
        queries,
        prefix: [0; 2],
        snapshots: vec![],
    };
    for w in 0..ni {
        state.write(w, 0)?;
    }
    let cuts: Vec<_> = (0..=10).map(|q| nops * q / 10).collect();
    let mut next_cut = 0;
    while next_cut < cuts.len() && cuts[next_cut] == 0 {
        state.snapshot(0);
        next_cut += 1;
    }
    let mut rows = vec![false; nc];
    let (mut hints, mut constraints) = (0, 0);
    for i in 1..=nops {
        match r.word()? {
            1 => {
                let [k, a, b, c, d] = r.words::<5>()?;
                need(k <= 1 && c < d && d - c <= 256 && d <= nw, "invalid hint")?;
                need(k != 0 || d - c == 1, "invalid inverse hint")?;
                for j in range(a, b, nt)? {
                    state.read(terms[j], i)?;
                }
                for w in c..d {
                    state.write(w, i)?;
                }
                hints += 1;
            }
            0 => {
                let [row, a, b, first_slot] = r.words::<4>()?;
                need(row < nc && !rows[row], "invalid constraint row")?;
                rows[row] = true;
                let mut slot = nw + first_slot;
                for j in range(a, b, ns)? {
                    let (p, w) = sums[j];
                    state.read(w, i)?;
                    let prev = p & !STORED;
                    if prev != NONE && prev != CHAIN {
                        state.read(prev as usize, i)?;
                    }
                    if p & STORED != 0 {
                        state.write(slot, i)?;
                        slot += 1;
                    }
                }
                for _ in 0..3 {
                    let [tag, a, b] = r.words::<3>()?;
                    match tag {
                        0 => {}
                        1 => state.read(a, i)?,
                        2 => need(a < ncoef, "invalid constant")?,
                        3 => {
                            need(a < nt, "invalid term index")?;
                            state.read(terms[a], i)?;
                        }
                        4 => {
                            for j in range(a, b, nt)? {
                                state.read(terms[j], i)?;
                            }
                        }
                        _ => return Err("invalid expression".into()),
                    }
                }
                match r.word()? {
                    0 => {}
                    1 => {
                        let [side, w, c] = r.words::<3>()?;
                        need(side <= 2 && w < nw && c < ncoef, "invalid assignment")?;
                        r.bytes(32)?;
                        state.write(w, i)?;
                    }
                    _ => return Err("invalid assignment tag".into()),
                }
                constraints += 1;
            }
            _ => return Err("invalid op tag".into()),
        }
        while next_cut < cuts.len() && cuts[next_cut] == i {
            state.snapshot(i);
            next_cut += 1;
        }
    }
    need(r.at == data.len(), "trailing bytes")?;
    need(rows.iter().all(|&v| v), "missing constraint")?;
    need(
        state.producer.iter().all(|&p| p != UNWRITTEN),
        "unwritten value",
    )?;
    need(next_cut == cuts.len(), "missing cuts")?;
    let mut result = serde_json::Map::new();
    for (q, name) in ["A", "B"].into_iter().enumerate() {
        let query = &state.queries[q];
        let mut records = vec![];
        for (cut_index, &cut) in cuts.iter().enumerate() {
            let ready = query.iter().filter(|&&w| state.producer[w] <= cut).count();
            let prefix = query
                .iter()
                .take_while(|&&w| state.producer[w] <= cut)
                .count();
            let last = query
                .iter()
                .filter(|&&w| state.producer[w] <= cut && state.last_read[w] <= cut)
                .count();
            need(
                state.snapshots[cut_index][format!("{name}_ready")] == ready,
                "independent ready counts disagree",
            )?;
            need(
                state.snapshots[cut_index][format!("{name}_prefix")] == prefix,
                "independent prefix counts disagree",
            )?;
            records.push(json!({"program_percent":cut_index*10,"ops_completed":cut,"arbitrary_order_ready_points":ready,"key_order_prefix_points":prefix,"past_final_solver_read_points":last}));
        }
        result.insert(name.into(),json!({"points":query.len(),"sorted_unique_indices":true,"input_ready_points":query.iter().filter(|&&w|w<ni).count(),"cuts":records}));
    }
    Ok(
        json!({"complete":true,"solver_image_bytes":data.len(),"wires":nw,"inputs":ni,"public":np,"constraints":nc,"ops":nops,"hint_ops":hints,"constraint_ops":constraints,"terms":nt,"sums":ns,"stored_slots":nslots,"queries":result,"all_values_defined_once":true,"all_solver_reads_defined":true,"online_and_producer_index_counts_agree":true,"model":"serialized program order; op counts are not time weights or parallel-epoch readiness","new_proofs":0,"new_timing_samples":0}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Vec<u8> {
        let mut words = vec![MAGIC, 2, 5, 2, 1, 2, 3, 2, 5, 2, 1, 1, 3, 0, 3, 4, 1, 2];
        words.extend([0; 40]);
        words.extend([1, 1, 1, 2]);
        words.extend([STORED | NONE, 1, 3]);
        words.extend([1, 1, 0, 1, 2, 4]);
        words.extend([0, 0, 0, 1, 0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 1, 2, 4, 1]);
        words.extend([0; 8]);
        words.extend([0, 1, 0, 0, 1, 1, 2, 0, 1, 4, 0, 2, 1, 0, 0]);
        words.into_iter().flat_map(u32::to_le_bytes).collect()
    }
    #[test]
    fn stored_slot_and_hint_readiness() {
        let v = census(&fixture()).unwrap();
        assert_eq!(v["queries"]["A"]["points"], 3);
        assert_eq!(
            v["queries"]["A"]["cuts"][0]["arbitrary_order_ready_points"],
            1
        );
        assert_eq!(
            v["queries"]["A"]["cuts"][5]["arbitrary_order_ready_points"],
            2
        );
        assert_eq!(v["queries"]["B"]["cuts"][5]["key_order_prefix_points"], 2);
        assert_eq!(
            v["queries"]["A"]["cuts"][10]["past_final_solver_read_points"],
            3
        );
        assert_eq!(v["hint_ops"], 1);
    }
    #[test]
    fn reject_undefined_read_and_duplicate_write() {
        let mut s = State {
            producer: vec![0, UNWRITTEN],
            last_read: vec![0; 2],
            queries: [vec![0, 1], vec![]],
            ready: [vec![true, false], vec![]],
            prefix: [1, 0],
            snapshots: vec![],
        };
        assert!(s.read(1, 1).is_err());
        assert!(s.write(0, 1).is_err());
        s.write(1, 1).unwrap();
        s.read(1, 2).unwrap();
        assert_eq!(s.last_read[1], 2);
        assert_eq!(s.prefix[0], 2);
    }
    #[test]
    fn reject_truncated_unsorted_and_trailing() {
        let b = fixture();
        assert!(census(&b[..b.len() - 1]).is_err());
        let mut x = b.clone();
        x.extend([0; 4]);
        assert!(census(&x).is_err());
        let mut x = b.clone();
        x[14 * 4..15 * 4].copy_from_slice(&0u32.to_le_bytes());
        assert!(census(&x).is_err());
    }
}
