#[allow(dead_code, unused_imports, unused_mut)]
mod captured_parser {
    include!("run.rs");
    pub fn remainders(name: &str) -> Value {
        let j = read(r("jobs.json"))
            .as_array()
            .unwrap()
            .iter()
            .find(|j| j["name"] == name)
            .unwrap()
            .clone();
        let (spans, _, _) =
            parse(&fs::read_to_string(logs(&j).join(format!("decode-{name}/stdout.log"))).unwrap());
        let mut rows = vec![];
        for sdk in spans.iter().filter(|s| s.kind == "sdk") {
            let join = spans
                .iter()
                .find(|s| s.kind == "join" && within(s, sdk))
                .unwrap();
            let g2 = spans
                .iter()
                .find(|s| s.kind == "g2" && within(s, sdk))
                .unwrap();
            assert!(within(g2, join));
            rows.push(json!({"SDK_minus_join_ms":((sdk.end-sdk.start)-(join.end-join.start))as f64/1e6,"G2_enter_after_join_ms":(g2.start-join.start)as f64/1e6,"join_after_G2_ms":(join.end-g2.end)as f64/1e6}));
        }
        let mut medians = BTreeMap::new();
        for key in [
            "SDK_minus_join_ms",
            "G2_enter_after_join_ms",
            "join_after_G2_ms",
        ] {
            medians.insert(
                key,
                median(
                    &rows
                        .iter()
                        .map(|v| v[key].as_f64().unwrap())
                        .collect::<Vec<_>>(),
                ),
            );
        }
        json!({"rows":rows,"medians":medians})
    }
}
fn main() {
    let root = std::path::Path::new("/home/ubuntu/src/zopro-perf-work/current174-stages");
    let mut out = serde_json::Map::new();
    for case in ["1x2-w32", "1x2-w16"] {
        for b in 0..3 {
            let name = format!("b{b}-{case}-on");
            let a = captured_parser::remainders(&name);
            println!("{name} {}", a["medians"]);
            out.insert(name, a);
        }
    }
    std::fs::write(
        root.join("remainders.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
}
