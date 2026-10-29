use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-packed-index175";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("capture.rs").exists());
    let mut c=fs::read_to_string(Path::new(P).join("capture.rs")).unwrap().replace(P,R)
        .replace("/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177",P)
        .replace("17708","17786").replace("17630","17708")
        .replace("17552","17630")
        .replace("audit[\"source_files\"], 177","audit[\"source_files\"], 176")
        .replace("audit[\"originals_exact\"], 172","audit[\"originals_exact\"], 174")
        .replace("(\"candidate\", 131)","(\"candidate\", 127)").replace("\"ON\":131","\"ON\":127")
        .replace("linked_GLS4_present_only_ON","linked_packed_index_only_ON").replace("gls4_horner_cfg","packed_index_cfg")
        .replace("ZOPRO_GLS4_HORNER_TRACE","ZOPRO_INDEX_PACK_TRACE")
        .replace("completed coalesced-top GLS4 primary pilot;not adopted","completed packed-index G2 primary pilot;not adopted")
        .replace("G2 five ordinary11bit plus cross-bucket grouped-top/fourpart GLS4;typed collective scratch/40Nscalar vector","accepted G2phi2 indexed top with globally packed1024point leaves;original field and dynamic full scratch admission");
    let a = c.find("    let required = [").unwrap();
    let b = c[a..].find("    for (label, count)").unwrap() + a;
    c.replace_range(
        a..b,
        r#"    let required = [
        "msm::packed_index::tests::geometry_covers_each_input_and_bucket_once",
        "msm::packed_index::tests::grouped_tiles_match_independent_bucket_generator_sums",
        "msm::packed_index::tests::checked_forecast_and_denied_admission_preserve_inputs",
    ];
"#,
    );
    for name in [
        "gls4.rs",
        "lattice.rs",
        "historical-gls4.rs",
        "historical-integer-oracle.json",
        "top.rs",
        "top-tests.rs",
        "coalesced-tests.rs",
    ] {
        c = c.replace(&format!("        \"{name}\",\n"), "");
    }
    c = c.replace(
        "        \"bootstrap.rs\",",
        "        \"bootstrap.rs\",\n        \"packed_index.rs\",",
    );
    let a = c.find("        Some(\"verify-traced\") => {").unwrap();
    let b = c[a..].find("        Some(\"prepare\")").unwrap() + a;
    c.replace_range(
        a..b,
        r#"        Some("verify-traced") => {
            let a:Vec<_>=std::env::args().collect();let log=fs::read_to_string(&a[2]).unwrap();
            let rows:Vec<_>=log.lines().filter(|l|l.starts_with("PACKED_INDEX entered ")).collect();
            assert_eq!(rows.len(),16,"four indexed-top calls per request, four SDK requests");
            assert!(rows.iter().all(|l|l.contains(" buckets=128 tile=1024 ")));
            assert!(Command::new(&a[3]).args(&a[4..]).status().unwrap().success());
        }
"#,
    );
    fs::write(r.join("capture.rs"), &c).unwrap();
    let a = c.find("fn prepare() {").unwrap();
    let b = c.find("\nfn push(").unwrap();
    fs::write(r.join("capture-setup.rs"), &c[a..b]).unwrap();
    let close = fs::read_to_string(Path::new(P).join("close.rs"))
        .unwrap()
        .replace(P, R)
        .replace(
            "2026-10-29-g2-gls4-coalesced-top177",
            "2026-10-29-g2-packed-index175",
        )
        .replace("17708", "17786")
        .replace(
            "assert_eq!(source.len(), 177)",
            "assert_eq!(source.len(), 176)",
        );
    fs::write(r.join("close.rs"), close).unwrap();
    let mut report = fs::read_to_string(Path::new(P).join("report.rs"))
        .unwrap()
        .replace(P, R)
        .replace(
            "2026-10-29-g2-gls4-coalesced-top177",
            "2026-10-29-g2-packed-index175",
        )
        .replace("17708", "17786")
        .replace("17,708", "17,786")
        .replace("17,630", "17,708")
        .replace(
            "# Coalesced GLS4 top reduction: primary SDK pilot",
            "# Packed indexed G2 leaves: primary SDK pilot",
        )
        .replace("new_GLS4", "new_packed_index")
        .replace("source177", "source176");
    let a = report.find("    s.push_str(\"## Change and scope").unwrap();
    let b = report[a..].find("    let code =").unwrap() + a;
    report.replace_range(a..b,r####"    s.push_str("## Change and scope\n\nThe candidate preserves accepted phi2 and changes only the native G2 indexed reducer. The original reducer makes one task per nonempty bucket tile. The candidate packs the global ordered index stream into the same 1,024-point tiles, allowing one tile to span adjacent buckets. Each leaf runs the original complete reduce_layer over all of its segments together. Final segment sums stay ordered by original bucket and use original group_affine. This shares inversion work while keeping point-sized parallel tasks. Existing adaptive prefix selection can fire differently; topology and dispatch effects are coupled.\n\nThe rejected GLS4 decomposition/maps/windows are absent. Previous ordinary-window tree, SIMD-phi and transformed-X cache experiments remain closed. There is no field formula, scalar split, width/parts, key format, pool, point lookup table or retained setup change. Worker-local limb point copies remain temporary and bounded by the original tile size.\n\nThree new tests check independent point/bucket ownership and tile boundaries; per-bucket generator sums under 1/4/32-thread pools with empty, sparse, skewed, equal/opposite/identity points; and checked cost/invalid geometry/admission denial preserving inputs. Fresh OFF/ON pass all 124/127 core tests and eight prover tests each. All 17 field files are exact, so 33 native field tests are inherited. The official generator check passes.\n\nDynamic Reservation::analysis charges the full candidate metadata and outputs, final collective grouping scratch and workers times bounded first-stage scratch/local metadata, on top of original forecasts. Segment count is at most ceil(N/tile)+bucketcount, with a tighter N bound. Metadata and partial outputs reserve fallibly before execution. Unsupported/empty/denied/overflow cases use the original reducer without changing inputs. Safe recursive Rayon joins split disjoint MaybeUninit output slices; length is published after all leaves finish. The forecast covers requested heap capacity, not minimum process RAM or worker stack bounds.\n\nSource 176 retains 174 of 175 original files exact. One cfg-only hook and module declaration in msm.rs plus one new child recover the original 175 through inverse edits. Original indexed/layer/grouping bodies, field, Window, phi, solver, FFT and key sources remain exact. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [implementation](artifacts/source/crates/zopro-core/src/msm/packed_index.rs), [source audit](artifacts/source-audit.json).\n\n## Linked route and activation\n\nNew packed-index symbols occur only in ON. Before the primary preflight Gnark check and any timing, its wrapper requires 16 packed-entry markers with 128 buckets and 1,024-point tiles: four indexed-top calls per proof times four SDK requests. Timing/profiles have no trace. Selected disassemblies establish linked structure, not dynamic operation counts or speed.\n\n| Variant | Selected packed-index functions | Selected bytes |\n|---|---:|---:|\n");
"####);
    fs::write(r.join("report.rs"), report).unwrap();
}
