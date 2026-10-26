use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};
fn counts(path: &Path) -> BTreeMap<String, u64> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            let f = l.split_whitespace().collect::<Vec<_>>();
            assert_eq!(f.len(), 2);
            (f[0].to_owned(), f[1].parse().unwrap())
        })
        .collect()
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 2);
    let root = Path::new(&args[0]);
    let h = root.join("output/hardware");
    let container: Value =
        serde_json::from_slice(&fs::read(root.join("container-resources.json")).unwrap()).unwrap();
    assert_eq!(container["memory"], 30u64 << 30);
    assert_eq!(container["memory_swap"], 30u64 << 30);
    assert_eq!(container["cpu_quota"], 0);
    assert_eq!(container["nano_cpus"], 0);
    assert_eq!(container["cpuset_cpus"], "0-95");
    assert_eq!(container["pids_limit"], 1024);
    let limits = container["ulimits"].as_array().unwrap();
    assert!(
        limits
            .iter()
            .any(|l| l["Name"] == "memlock" && l["Soft"] == 268435456 && l["Hard"] == 268435456)
    );
    for suffix in ["before", "after"] {
        let max = fs::read_to_string(h.join(format!("{suffix}-cpu.max"))).unwrap();
        assert_eq!(max.split_whitespace().next(), Some("max"));
        assert_eq!(
            fs::read_to_string(h.join(format!("{suffix}-memory.swap.current")))
                .unwrap()
                .trim(),
            "0"
        );
        assert_eq!(
            fs::read_to_string(h.join(format!("{suffix}-memory.max")))
                .unwrap()
                .trim(),
            (30u64 << 30).to_string()
        );
    }
    let mut deltas = BTreeMap::new();
    for file in ["cpu.stat", "memory.events"] {
        let before = counts(&h.join(format!("before-{file}")));
        let after = counts(&h.join(format!("after-{file}")));
        let mut delta = BTreeMap::new();
        for (k, old) in before {
            let now = after[&k];
            assert!(now >= old);
            let d = now - old;
            if file == "memory.events" || k == "nr_throttled" || k == "throttled_usec" {
                assert_eq!(d, 0, "resource event {file}/{k}");
            }
            delta.insert(k, d);
        }
        deltas.insert(file, delta);
    }
    let result = json!({"status":"resource checks passed","deltas":deltas,"container":container,"scope":"Observed no quota, throttle/OOM/memory event deltas or swap under this30GiB configuration. This is not a pressure test or minimumRAM determination."});
    fs::write(
        &args[1],
        format!("{}\n", serde_json::to_string_pretty(&result).unwrap()),
    )
    .unwrap();
    println!("RESOURCE_CHECKS_PASSED");
}
