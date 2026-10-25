from pathlib import Path
import json,hashlib,shutil,time
r=Path(__file__).resolve().parent;repo=Path('/home/ubuntu/src/zk-tools/zopro');out=repo/'docs/benchmarks/2026-10-25-g2-prefix-sdk'
def sha(p):
    with Path(p).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def read(n):return json.loads((r/n).read_text())
x=read('results.json');a=read('accepted.json');b=read('build-complete.json');pins=read('source-pins.json')
assert x['sdk']==7216 and x['distinct_gnark']==229 and not x['adopted']
assert not Path('/proc',str(read('run-started.json')['pid'])).exists()
assert all(sha(repo/n)==h for n,h in a['source_files_sha256'].items())and all(sha(r/'workspace'/n)==h for n,h in pins.items())
assert sha(b['binary'])==b['binary_sha256']and all(sha(v['path'])==v['sha256']for v in b['libraries'].values())
assert len(read('proof-inventory.json'))==458 and all(sha(r/n)==h for n,h in read('proof-inventory.json').items())
assert all(sha(n)==h for n,h in read('input-pins.json').items())
assert not out.exists();out.mkdir(parents=True)
for f in sorted(r.rglob('*')):
    if not f.is_file():continue
    n=f.relative_to(r)
    if n.parts[0]in ['target-native','workspace','__pycache__']or f.name in ['native-sdk','record-complete.json']:continue
    dest=out/n;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(f,dest)
for n,h in pins.items():
    if a['source_files_sha256'].get(n)==h:continue
    dest=out/'source'/n;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(r/'workspace'/n,dest)
primary=x['summary']['1x2-w32']['on']
lines=['# G2-only inversion-prefix records: full SDK comparison','',
    f"The G2-only candidate {'passes'if x['passed']else'fails'} its declared SDK performance gate. Primary ON latency is{primary['median_ms']:.6f}ms; the all-nine geometric paired ratios are{x['geometric_ratios']['accepted']:.6f} against accepted and{x['geometric_ratios']['off']:.6f} against the identical OFF binary. No source is adopted by this performance campaign.",'',
    'This is a new G2-only hypothesis after the [combined G1/G2 component](../2026-10-25-prefix-records-component/README.md) failed. That original failure remains unchanged. The G1 candidate is absent. Native G2 streaming stores an inversion prefix and norm, then rereads original coordinates on reverse. Combining prefix and inverse once before scaling both complex numerator components saves one elementary Fq product per full point. Scalar tails use the original code verbatim. A batch\'s distinct bucket indices preserve original inputs until their unique reverse writes.','',
    '| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF | ON peak RSS MiB |','| --- | ---: | ---: | ---: | ---: | ---: | ---: |']
for name,c in x['summary'].items():
    on=c['on'];lines.append(f"| {name} | {c['accepted']['median_ms']:.6f} | {c['off']['median_ms']:.6f} | {on['median_ms']:.6f} | {on['comparisons']['accepted']['median_ratio']:.6f} | {on['comparisons']['off']['median_ratio']:.6f} | {on['peak_rss_mib']:.2f} |")
lines += ['',
    f"Primary gate: {x['primary_passed']}; per-case nonregression: {x['nonregression_passed']}; OFF calibration: {x['calibration_passed']}. All original observations and failed conditions are retained. The gate requires a positive primary paired median with at least4/7 wins against both controls, all-nine geometric improvement against both, each of ten cases within2% paired median and5% in every block, and OFF within3% of accepted. There is no minimum gain floor or unchanged reroll.",'',
    'Seven fixed rotated/reversed blocks compare accepted, candidate OFF and ON over all nine32-worker cases plus a16-worker physical-core-pinned primary control. Each process excludes key loading and three prime SDK calls, then measures31 complete fresh requests. Requests include witness generation, proof randomness, proving, internal verification and serialization. The standard six-argument driver is unchanged. Native CPU, fat LTO and one codegen unit match accepted. No owned build, profile or second benchmark overlaps timing. Whole-process RSS includes loading and priming and is not minimum required RAM.','',
    'All nine shapes have OFF/ON four-worker functional preflights; a missing-option primary32 case confirms the experimental default is OFF. First-use traces confirm actual G2 stream selection on every shape, including full vector batches. Tracing is absent from timing. Native validation passes106 core, two raw-format and six cubic tests with the runtime option enabled. The two added tests check complete-curve formulas, short/zero lengths, tails, signs, canonical and inverse modes, repeated scratch use and actual dispatch/control behavior. Field source is unchanged.','',
    'The campaign completes7,216 SDK calls and229 distinct independently Gnark-verified final exports. All458 proof/public files match hashes and public fixtures; proof hashes are fresh against1,473 previous exports. Production167 and private168 source pins, inputs, libraries and binaries are checked before and after. G1 algorithm bodies are restored to accepted; additional private scratch fields and code layout can affect generated code, so the fresh OFF control remains required.','',
    'The new path reduces only G2 streaming group-record payload from nine vectors to two. It adds one vector header and three bool fields to native private scratch; the unused old group vector stays unallocated on the new path. The component\'s72KB-to16KB capacity result is not a full-prover memory bound. Resource-forecast and generated-stack review, portable behavior, caller concurrency, cold/warm pressure transitions, and actual250/4999 qualification remain outstanding before any promotion. The runtime option is read at scratch construction; immutable key data, solver image format and proving-key representation are unchanged.','',
    '| Shape | Candidate local ms | Supplied M5 CPU ms | CPU speedup | Supplied Metal ms | Metal speedup |','| --- | ---: | ---: | ---: | ---: | ---: |']
for m in x['m5']:lines.append(f"| {m['shape']} | {m['candidate_ms']:.6f} | {m['m5_cpu_ms']} | {m['cpu_speedup']:.3f}x | {m['m5_metal_ms']} | {m['metal_speedup']:.3f}x |")
lines += ['',
    f"All-eight10x CPU target achieved: {x['goal_all8_10x']}. Supplied M5 numbers are historical references, not fresh Apple measurements. No M5 reference was supplied for merge54. No AWS, Aeglos or physical-phone comparison, source adoption, commit or push is part of this campaign. Accepted production remains the [small-core entry baseline](../2026-10-25-small-core-entry-adoption/README.md).",'',
    '[Protocol](protocol.json), [results](results.json), [raw observations](rows.json), [scope audit](scope-audit.json), [proof hashes](proof-inventory.json), [build](build-complete.json), [source](source/crates/zopro-core/src/msm/g2_prefix.rs).']
(out/'README.md').write_text('\n'.join(lines)+'\n')
receipt=dict(completed=time.time(),report=str(out/'README.md'),passed=x['passed'],primary_passed=x['primary_passed'],nonregression_passed=x['nonregression_passed'],calibration_passed=x['calibration_passed'],geometric_ratios=x['geometric_ratios'],primary_ms=primary['median_ms'],primary_rss_mib=primary['peak_rss_mib'],sdk=7216,distinct_gnark=229,proof_public_members=458,native_core_tests=106,raw_tests=2,cubic_tests=6,production_pins_verified=167,private_pins_verified=168,adopted=False,goal_achieved=False,owned_live_jobs=[])
(out/'validation.json').write_text(json.dumps(receipt,indent=2)+'\n');hashes={str(f.relative_to(out)):sha(f)for f in sorted(out.rglob('*'))if f.is_file()};(out/'SHA256SUMS.json').write_text(json.dumps(hashes,indent=2)+'\n');assert all(sha(out/n)==h for n,h in hashes.items());(r/'record-complete.json').write_text(json.dumps(receipt|dict(archive_files=len(hashes)),indent=2)+'\n');print(json.dumps(receipt,indent=2))
