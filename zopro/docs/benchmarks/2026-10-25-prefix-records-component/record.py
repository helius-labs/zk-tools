from pathlib import Path
import json,hashlib,shutil,time,subprocess
r=Path(__file__).resolve().parent;repo=Path('/home/ubuntu/src/zk-tools/zopro');out=repo/'docs/benchmarks/2026-10-25-prefix-records-component'
def sha(p):
    with Path(p).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def read(n):return json.loads((r/n).read_text())
x=read('results.json');a=read('accepted.json');b=read('build-complete.json')
assert not x['passed']and x['core_tests']==106 and x['checksum_comparisons']==93
assert all(sha(repo/n)==h for n,h in a['source_files_sha256'].items())
assert all(sha(r/'workspace'/n)==h for n,h in read('source-pins.json').items())
assert sha(r/'component')==b['binary_sha256']and all(sha(v['path'])==v['sha256']for v in b['libraries'].values())
assert not Path('/proc',str(read('started.json')['pid'])).exists()
assert not out.exists();out.mkdir(parents=True)
for f in sorted(r.rglob('*')):
    if not f.is_file():continue
    n=f.relative_to(r)
    if n.parts[0]in ['target-native','workspace','__pycache__']or f.name in ['component','record-complete.json']:continue
    dest=out/n;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(f,dest)
for n,h in read('source-pins.json').items():
    if a['source_files_sha256'].get(n)==h:continue
    dest=out/'source'/n;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(r/'workspace'/n,dest)
lines=['# Prefix-only streaming records: mixed component result','','The combined G1/G2 design fails its declared component gate and is not integrated into the prover. G1 slows by3.2–3.6%, while G2 improves by6.1% in every paired block. All negative results remain part of the decision; a possible G2-specific follow-up requires a separate prospective SDK experiment.','','| Kernel/batch | Old ns/batch | New ns/batch | Paired new/old | Winning blocks | Old record capacity B | New record capacity B | Pass |','| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |']
for name,c in x['cases'].items():lines.append(f"| {name} | {c['old_ns_per_batch']:.2f} | {c['new_ns_per_batch']:.2f} | {c['paired_median_ratio']:.6f} | {c['wins']}/31 | {c['old_scratch_capacity_bytes'][0]:,} | {c['new_scratch_capacity_bytes'][0]:,} | {c['passed']} |")
lines += ['',
    f"The geometric ratio across the three cases is{x['geometric_ratio']:.6f}. The original gate requires each case below1 with at least21/31 wins and an aggregate below1. Both G1 cases have zero wins. That combined gate remains failed; the G2 result does not waive it.", '',
    'The accepted streaming kernel stores coordinates and scaled numerators for reverse evaluation. This private design instead stores one vector prefix for G1 and a prefix/norm pair for G2. It rereads original bucket and input coordinates on reverse. A batch contains distinct bucket indices, so a reverse write cannot alter another group\'s inputs. Scalar-tail blocks are reused verbatim and retain their original inversion handling. G1 uses the same number of products with additional coordinate packing; G2 combines prefix and inverse once before scaling both numerator components, removing one elementary Fq product per full point. This changes arithmetic order and memory access together, so the timings do not isolate either effect.', '',
    'G1 record payload falls from five vectors to one (80%); G2 falls from nine to two (77.8%). Reported capacities cover only group-record vectors after priming. They exclude bucket arrays, scalar tails, allocator overhead, library state and all other memory. There is one additional empty vector header in each private scratch type, and old group vectors remain unallocated on the candidate path. This is not peak process RSS, minimum RAM or full-prover memory qualification.', '',
    'Native validation passes all106 core tests, including two new tests against complete scalar curve formulas. New cases cover signed/unsigned G1, signed-point G2, zero length, short vectors, tails, several batch sizes, both canonical and inversion settings, and three scratch reuses. They perform396 full-buffer comparisons. Candidate checks assert that the original group vector stays unallocated. Existing field primitives are unchanged.', '',
    'Thirty-one fixed rotated/reversed blocks compare direct calls to old and new kernels in one native fat-LTO executable pinned to CPU0. Representative batches are200 and352 for G1 and200 for G2, selected before compilation using the [operation census](../2026-10-25-operation-model/README.md). Every sample updates the same valid bucket geometry256 times after three prime calls; point generation, allocation setup and full output checksums are outside timing. All93 paired full-bucket checksums agree. The original G1 control uses the accepted reserve/write storage mode. No build or other owned CPU benchmark overlaps measurement.', '',
    'No normal SDK dispatch was changed. This component generates zero SDK or Gnark proofs and establishes no full-request latency improvement, portable behavior, memory-pressure feasibility, concurrency, large-circuit result, AWS or phone performance. Accepted production remains the same167 files and historical8.689205ms primary. The all-eight10x M5 CPU objective remains unmet. No source is adopted or pushed.', '',
    'The G1 result shows that reducing source record bytes alone does not guarantee faster execution. The G2 result motivates a distinct question: whether a G2-only path preserves a benefit in complete fresh proofs while G1 stays on the accepted implementation. That investigation would retain this failed combined result, declare fresh controls and guards before SDK measurement, and require memory and compatibility qualification before adoption.', '',
    '[Protocol](protocol.json), [results](results.json), [raw observations](measurements/rows.json), [scalar-tail audit](tail-reuse-audit.json), [source](source/crates/zopro-core/src/msm/prefix_records.rs), [driver](component.rs), [build](build-complete.json).'
]
(out/'README.md').write_text('\n'.join(lines)+'\n')
receipt=dict(completed=time.time(),report=str(out/'README.md'),passed=False,geometric_ratio=x['geometric_ratio'],g2_paired_ratio=x['cases']['g2-b200']['paired_median_ratio'],g2_wins=31,g1_paired_ratios={k:v['paired_median_ratio']for k,v in x['cases'].items()if k.startswith('g1')},native_core_tests=106,full_buffer_oracle_checks=396,paired_component_checksums=93,actual_sdk_proofs=0,distinct_gnark=0,production_pins_verified=167,private_pins_verified=168,normal_sdk_dispatch_unchanged=True,adopted=False,goal_achieved=False,owned_live_jobs=[])
(out/'validation.json').write_text(json.dumps(receipt,indent=2)+'\n');hashes={str(f.relative_to(out)):sha(f)for f in sorted(out.rglob('*'))if f.is_file()};(out/'SHA256SUMS.json').write_text(json.dumps(hashes,indent=2)+'\n');assert all(sha(out/n)==h for n,h in hashes.items());(r/'record-complete.json').write_text(json.dumps(receipt|dict(archive_files=len(hashes)),indent=2)+'\n');print(json.dumps(receipt,indent=2))
