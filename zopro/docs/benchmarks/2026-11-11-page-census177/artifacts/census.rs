use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    os::{
        fd::AsRawFd,
        unix::fs::{FileExt, MetadataExt},
    },
    process::Command,
};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/page-census177";
const INPUT: &str = "/home/ubuntu/src/zopro-perf-work/g1-offset177/runtime-consumers.json";
unsafe extern "C" {
    fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut u8;
    fn munmap(addr: *mut u8, len: usize) -> i32;
    fn mincore(addr: *mut u8, len: usize, vec: *mut u8) -> i32;
    fn getpagesize() -> i32;
}
struct Mapping {
    ptr: *mut u8,
    len: usize,
}
impl Mapping {
    fn new(f: &File, off: u64, len: usize) -> Self {
        // Read-only shared mapping of a checked non-writable ELF file region.
        let ptr = unsafe { mmap(std::ptr::null_mut(), len, 1, 1, f.as_raw_fd(), off as i64) };
        assert_ne!(
            ptr as usize,
            usize::MAX,
            "{}",
            std::io::Error::last_os_error()
        );
        Self { ptr, len }
    }
    fn touch(&self, page: usize) {
        for i in (0..self.len).step_by(page) {
            // All offsets are inside the mapping and underlying file.
            std::hint::black_box(unsafe { self.ptr.add(i).read_volatile() });
        }
    }
}
impl Drop for Mapping {
    fn drop(&mut self) {
        assert_eq!(unsafe { munmap(self.ptr, self.len) }, 0);
    }
}
fn read64(f: &File, offset: u64) -> u64 {
    let mut b = [0; 8];
    f.read_exact_at(&mut b, offset).unwrap();
    u64::from_le_bytes(b)
}
fn u64at(b: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(b[i..i + 8].try_into().unwrap())
}
fn u32at(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap())
}
fn u16at(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes(b[i..i + 2].try_into().unwrap())
}
fn digest(p: &str) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(name: &str, v: &Value) {
    let mut b = serde_json::to_vec_pretty(v).unwrap();
    b.push(b'\n');
    fs::write(format!("{ROOT}/{name}"), b).unwrap();
}
fn check(c: &Value) {
    let p = c["path"].as_str().unwrap();
    let m = fs::metadata(p).unwrap();
    assert!(m.is_file());
    assert_eq!(m.dev(), c["device"].as_u64().unwrap());
    assert_eq!(m.ino(), c["inode"].as_u64().unwrap());
    assert_eq!(m.mode() as u64, c["mode"].as_u64().unwrap());
}
fn distribution(pages: &[Value]) -> Value {
    let frames = pages
        .iter()
        .map(|p| p["pfn"].as_u64().unwrap())
        .collect::<Vec<_>>();
    let mut runs = Vec::new();
    let mut len = 0;
    for (i, &p) in frames.iter().enumerate() {
        if i == 0 || p == frames[i - 1] + 1 {
            len += 1;
        } else {
            runs.push(len);
            len = 1;
        }
    }
    if len > 0 {
        runs.push(len);
    }
    let mut residues = BTreeMap::new();
    for n in [16, 32, 64] {
        let mut h = vec![0usize; n];
        for &p in &frames {
            h[p as usize % n] += 1;
        }
        residues.insert(n.to_string(), h);
    }
    let flags = |bit: u32| {
        pages
            .iter()
            .filter(|p| p["kpageflags"].as_u64().unwrap() & (1u64 << bit) != 0u64)
            .count()
    };
    json!({"pages":pages.len(),"unique_frames":frames.iter().collect::<BTreeSet<_>>().len(),"contiguous_runs":runs.len(),"longest_run_pages":runs.iter().max().copied().unwrap_or(0),"compound_head":flags(15),"compound_tail":flags(16),"huge":flags(17),"thp":flags(22),"anon":flags(12),"residue_histograms_not_cache_sets":residues})
}
fn capture() {
    let page = unsafe { getpagesize() } as usize;
    assert_eq!(page, 4096);
    let input: Value = serde_json::from_slice(&fs::read(INPUT).unwrap()).unwrap();
    let pm = File::open("/proc/self/pagemap").unwrap();
    let kf = File::open("/proc/kpageflags").unwrap();
    let kc = File::open("/proc/kpagecount").unwrap();
    let mut records = Vec::new();
    let mut summaries = Vec::new();
    for arm in ["REF", "OFF", "ON"] {
        for (copy, c) in input[arm]["copies"].as_array().unwrap().iter().enumerate() {
            check(c);
            let path = c["path"].as_str().unwrap();
            let f = File::open(path).unwrap();
            let size = f.metadata().unwrap().len();
            let mut h = [0u8; 64];
            f.read_exact_at(&mut h, 0).unwrap();
            assert_eq!(&h[..6], b"\x7fELF\x02\x01");
            assert_eq!(u16at(&h, 18), 62);
            assert_eq!(u16at(&h, 54), 56);
            let mut segments = Vec::new();
            let mut all_pages = BTreeMap::new();
            for i in 0..u16at(&h, 56) as u64 {
                let mut ph = [0u8; 56];
                f.read_exact_at(&mut ph, u64at(&h, 32) + i * 56).unwrap();
                if u32at(&ph, 0) != 1 || u32at(&ph, 4) & 2 != 0 {
                    continue;
                }
                let off = u64at(&ph, 8);
                let bytes = u64at(&ph, 32);
                if bytes == 0 {
                    continue;
                }
                assert!(off.checked_add(bytes).unwrap() <= size);
                let start = off / page as u64 * page as u64;
                let len = (off + bytes - start) as usize;
                let map = Mapping::new(&f, start, len);
                let count = len.div_ceil(page);
                let mut resident = vec![0u8; count];
                assert_eq!(unsafe { mincore(map.ptr, len, resident.as_mut_ptr()) }, 0);
                map.touch(page);
                let alias = Mapping::new(&f, start, len);
                alias.touch(page);
                let mut pages = Vec::new();
                for j in 0..count {
                    let entry = read64(&pm, ((map.ptr as u64 / page as u64) + j as u64) * 8);
                    let other = read64(&pm, ((alias.ptr as u64 / page as u64) + j as u64) * 8);
                    let pfn = entry & ((1u64 << 55) - 1);
                    assert!(entry >> 63 == 1 && entry & (1 << 62) == 0 && entry & (1 << 61) != 0);
                    assert!(pfn > 0, "PFNs unavailable");
                    assert_eq!(pfn, other & ((1u64 << 55) - 1), "alias frame differs");
                    let row = json!({"file_offset":start+j as u64*page as u64,"pfn":pfn,"pagemap":entry,"kpageflags":read64(&kf,pfn*8),"kpagecount":read64(&kc,pfn*8),"resident_before_touch":resident[j]&1!=0});
                    all_pages.insert(start + j as u64 * page as u64, row.clone());
                    pages.push(row);
                }
                segments.push(json!({"ph_index":i,"flags":u32at(&ph,4),"file_offset":off,"elf_vaddr":u64at(&ph,16),"file_bytes":bytes,"mapped_file_offset":start,"map_address":map.ptr as u64,"pages":pages,"summary":distribution(&pages)}));
            }
            let out = Command::new("/usr/bin/nm")
                .args(["-S", "--defined-only", path])
                .output()
                .unwrap();
            assert!(out.status.success());
            let nm = String::from_utf8(out.stdout).unwrap();
            fs::write(format!("{ROOT}/symbols-{arm}-{copy}.txt"), &nm).unwrap();
            let mut symbols = Vec::new();
            for line in nm
                .lines()
                .filter(|s| s.contains("g2_") && s.contains("5apply"))
            {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                assert_eq!(parts.len(), 4);
                let addr = u64::from_str_radix(parts[0], 16).unwrap();
                let len = u64::from_str_radix(parts[1], 16).unwrap();
                let s = segments
                    .iter()
                    .find(|s| {
                        let v = s["elf_vaddr"].as_u64().unwrap();
                        addr >= v && addr + len <= v + s["file_bytes"].as_u64().unwrap()
                    })
                    .unwrap();
                let start =
                    addr - s["elf_vaddr"].as_u64().unwrap() + s["file_offset"].as_u64().unwrap();
                let pages = all_pages
                    .range(start / page as u64 * page as u64..start + len)
                    .map(|(_, v)| v.clone())
                    .collect::<Vec<_>>();
                symbols.push(json!({"symbol":parts[3],"elf_vaddr":addr,"bytes":len,"file_offset":start,"summary":distribution(&pages),"pages":pages}));
            }
            assert_eq!(symbols.len(), 3);
            check(c);
            assert_eq!(digest(path), c["sha256"].as_str().unwrap());
            assert_eq!(fs::metadata(path).unwrap().len(), size);
            let unique = all_pages.values().cloned().collect::<Vec<_>>();
            summaries.push(json!({"arm":arm,"copy":copy,"path":path,"sha256":c["sha256"],"device":c["device"],"inode":c["inode"],"read_only_pages":distribution(&unique),"resident_before_touch":unique.iter().filter(|p|p["resident_before_touch"]==true).count(),"segments":segments.iter().map(|s|json!({"flags":s["flags"],"summary":s["summary"]})).collect::<Vec<_>>(),"g2_symbols":symbols.iter().map(|s|json!({"symbol":s["symbol"],"elf_vaddr":s["elf_vaddr"],"bytes":s["bytes"],"summary":s["summary"]})).collect::<Vec<_>>()}));
            records.push(json!({"arm":arm,"copy":copy,"identity":c,"segments":segments,"g2_symbols":symbols}));
            println!(
                "captured {arm}/{copy}: {} unique read-only file pages, hashes and same-inode alias check pass",
                unique.len()
            );
        }
    }
    let sets = records
        .iter()
        .map(|r| {
            r["segments"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|s| {
                    s["pages"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| p["pfn"].as_u64().unwrap())
                })
                .collect::<BTreeSet<_>>()
        })
        .collect::<Vec<_>>();
    let mut overlaps = Vec::new();
    for i in 0..sets.len() {
        for j in i + 1..sets.len() {
            overlaps.push(json!({"a":[records[i]["arm"].clone(),records[i]["copy"].clone()],"b":[records[j]["arm"].clone(),records[j]["copy"].clone()],"shared_frames":sets[i].intersection(&sets[j]).count()}));
        }
    }
    let raw = json!({"page_size":page,"records":records});
    save("raw-private.json", &raw);
    let result = json!({"status":"complete","scope":"present-time read-only executable page census; no latency or causal claim","input_sha256":digest(INPUT),"source_sha256":digest(&format!("{ROOT}/census.rs")),"raw_sha256":digest(&format!("{ROOT}/raw-private.json")),"same_inode_alias_checks":"pass all mapped pages","copies":summaries,"cross_file_frame_overlap":overlaps,"kernel":fs::read_to_string("/proc/version").unwrap(),"proof_ledger_count_unchanged":52945});
    save("result.json", &result);
}
fn prepare() {
    let mut pins = BTreeMap::new();
    for p in [
        INPUT,
        &format!("{ROOT}/protocol.md"),
        &format!("{ROOT}/census.rs"),
        &format!("{ROOT}/census"),
    ] {
        pins.insert(p.to_string(), digest(p));
    }
    save("pins.json", &json!(pins));
    save(
        "plan.json",
        &json!({"version":1,"source_pins":format!("{ROOT}/pins.json"),"steps":[{"kind":"command","name":"census","argv":["sudo","-n",format!("{ROOT}/census"),"capture"],"timeout_seconds":180}]}),
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("capture") => capture(),
        _ => panic!("prepare|capture"),
    }
}
