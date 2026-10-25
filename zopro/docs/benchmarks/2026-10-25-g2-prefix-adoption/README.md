# Native G2 prefix-record adoption

The qualified native performance configuration now uses prefix/norm records for G2 streaming additions. Reverse evaluation rereads original nonaliasing coordinates and shares prefix/inverse scaling across both complex components. It removes one elementary Fq product per full point and reduces streaming record payload from nine vectors to two. G1 retains its previous algorithm. No proving-key expansion, image-format change or retained per-key lookup table is introduced.

The accepted seven-block SDK comparison measures primary8.606433ms versus8.691594ms accepted and8.729365ms identical OFF. Paired improvements are1.24% and1.41%, with5/7 and6/7 winning blocks. All-nine geometric improvements are0.74% and0.49%. Merge54 is1.76% slower than accepted and primary16 is0.54% slower by paired median; both remain within the declared2% nonregression tolerance. All observations are retained in the full comparison.

The native configuration requires --cfg zopro_g2_prefix_records and ZOPRO_CPU_G2_PREFIX_RECORDS=1; the runtime omission remains OFF for this opt-in path. Its value is fixed at scratch construction. The exact flags, environment, executable and168 source pins are recorded in the manifest. The promoted Rust files match the qualified source byte for byte.

Native106 core, two raw-format and six cubic tests pass. Baseline portable11 field,89 core, two raw and six cubic tests pass. Compatibility covers Euler/raw/preparation-denied inputs, single workers,512KiB/1MiB stacks and concurrent callers. Across performance and qualification:7,413 SDK calls plus four large proofs,304 distinct independently verified exports,608 proof/public files rehashed before promotion.

Cold startup passes at192MiB with swap0. Warm controls pass at128MiB soft/192MiB hard after explicitly touching32MiB of live ballast, under both allocator policies and retained96-worker controls. Actual250 and4999 each produce two independently verified proofs under95GiB with zero swap/OOM. Batch4999 takes160.111/154.724s and peaks at94.693420GiB process RSS; hard-limit reclaim events are reported. These capacities are tested feasibility, not minimumRAM.

The G2 component alone improves6.1% with record capacity72,000 to16,000 bytes. Ordinary primary process RSS in the full campaign is111.25MiB versus108.25MiB accepted, so the scratch reduction is not claimed as a whole-process RSS saving. Source-level memory forecasts remain conservative, and actual small-stack proofs pass. The failed combined G1/G2 component remains rejected.

No new AWS, Aeglos or physical-phone measurement is claimed. The full original all-eight10x M5 CPU objective remains unmet. The supplied-reference CPU/Metal table is in the full performance report.

[Full performance results](../2026-10-25-g2-prefix-sdk/README.md), [compatibility and memory qualification](../2026-10-25-g2-prefix-qualification/README.md), [failed combined component](../2026-10-25-prefix-records-component/README.md), [source change](source.patch), [exact accepted configuration](production-source-manifest.json).
