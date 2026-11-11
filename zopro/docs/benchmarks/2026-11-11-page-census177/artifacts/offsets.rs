use serde_json::{Value, json};
use std::{collections::BTreeMap, fs};
fn main() {
    let root = "/home/ubuntu/src/zopro-perf-work/page-census177";
    let old = "/home/ubuntu/src/zopro-perf-work/g1-offset-profile177";
    let raw: Value =
        serde_json::from_slice(&fs::read(format!("{root}/raw-private.json")).unwrap()).unwrap();
    let prior: Value =
        serde_json::from_slice(&fs::read(format!("{old}/result.json")).unwrap()).unwrap();
    let mut rows = Vec::new();
    let mut bins: BTreeMap<u64, [u64; 3]> = BTreeMap::new();
    for copy in 0..3 {
        let r = &raw["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["arm"] == "OFF" && r["copy"] == copy)
            .unwrap();
        let sym = r["g2_symbols"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["symbol"].as_str().unwrap().contains("g2_borrowed"))
            .unwrap();
        let symbol = sym["symbol"].as_str().unwrap();
        let start = sym["elf_vaddr"].as_u64().unwrap();
        let size = sym["bytes"].as_u64().unwrap();
        let meta =
            fs::read_to_string(format!("{old}/run/metadata-cycles-OFF-{copy}/stdout.log")).unwrap();
        let maps = meta
            .lines()
            .filter(|l| l.contains("PERF_RECORD_MMAP2") && l.ends_with(&format!("/sdk-f{copy}")))
            .collect::<Vec<_>>();
        assert_eq!(maps.len(), 1);
        let line = maps[0];
        let pid = line
            .split_whitespace()
            .next()
            .unwrap()
            .split('/')
            .next()
            .unwrap();
        let tail = line.split(": [0x").nth(1).unwrap();
        let address = u64::from_str_radix(tail.split('(').next().unwrap(), 16).unwrap();
        let offset = u64::from_str_radix(
            tail.split(" @ 0x")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap(),
            16,
        )
        .unwrap();
        let seg = r["segments"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["flags"].as_u64().unwrap() & 1 != 0)
            .unwrap();
        assert_eq!(offset, seg["mapped_file_offset"].as_u64().unwrap());
        let base = address - (seg["elf_vaddr"].as_u64().unwrap() & !4095);
        let data =
            fs::read_to_string(format!("{old}/run/decode-cycles-OFF-{copy}/stdout.log")).unwrap();
        let mut count = 0;
        let mut cpus = BTreeMap::new();
        for line in data.lines().filter(|l| l.ends_with(symbol)) {
            let p = line.split_whitespace().collect::<Vec<_>>();
            assert_eq!(p[0].split('/').next().unwrap(), pid);
            assert_eq!(p[3], "16000000");
            let ip = u64::from_str_radix(p[4], 16).unwrap();
            let off = ip.checked_sub(base + start).unwrap();
            assert!(off < size);
            bins.entry(off / 1024 * 1024).or_default()[copy] += 1;
            *cpus.entry(p[1].to_string()).or_insert(0u64) += 1;
            count += 1;
        }
        let previous = prior["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["panel"] == "cycles" && r["arm"] == "OFF" && r["block"] == copy)
            .unwrap();
        let expected = previous["diagnostic"]["symbols"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["symbol"].as_str().unwrap().contains("g2_borrowed::apply"))
            .unwrap()["samples"]
            .as_u64()
            .unwrap();
        assert_eq!(count, expected);
        rows.push(json!({"copy":copy,"pid":pid,"elf_load_bias":base,"g2_symbol_vaddr":start,"symbol_bytes":size,"samples":count,"by_cpu":cpus,"source_count_crosscheck":"pass"}));
    }
    let b=bins.iter().map(|(offset,counts)|json!({"symbol_offset":offset,"samples_by_copy":counts,"copy1_minus_mean_0_2":counts[1] as f64-(counts[0]+counts[2]) as f64/2.})).collect::<Vec<_>>();
    let result = json!({"scope":"retrospective 1KiB instruction-offset bins; sampled cycles include skid, not precise instruction latency or exclusive stalls","new_proofs":0,"rows":rows,"bins":b});
    fs::write(
        format!("{root}/offsets.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!(
        "PASS: three G2 symbol counts reproduce prior qualified profiles; {} offset bins retained",
        bins.len()
    );
}
