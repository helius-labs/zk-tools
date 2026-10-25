from pathlib import Path
import json,hashlib,os,subprocess,time,statistics,math
r=Path(__file__).resolve().parent;w=r/'workspace';p=json.loads((r/'protocol.json').read_text());a=json.loads((r/'accepted.json').read_text());pins=json.loads((r/'source-pins.json').read_text())
def sha(path):
    with Path(path).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
assert not(r/'started.json').exists();save('started.json',dict(time=time.time(),pid=os.getpid()))
assert all(sha(w/n)==h for n,h in pins.items())and all(sha(Path('/home/ubuntu/src/zk-tools/zopro')/n)==h for n,h in a['source_files_sha256'].items())
e={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|a['environment']|{'RUSTFLAGS':p['rustflags'],'CARGO_TARGET_DIR':str(r/'target-native'),'CARGO_BUILD_JOBS':'16','RAYON_NUM_THREADS':'4'}
with(r/'core-tests.log').open('w')as f:subprocess.run(['cargo','test','--offline','--locked','--release','-p','zopro-core','--lib','--','--test-threads=1'],cwd=w,env=e,stdout=f,stderr=subprocess.STDOUT,check=True)
assert'test result: ok. 106 passed; 0 failed;'in(r/'core-tests.log').read_text();print('CORE106_PASS',flush=True)
with(r/'artifacts.jsonl').open('w')as fo,(r/'cargo-build.log').open('w')as fe:subprocess.run(['cargo','build','--offline','--locked','--release','-p','zopro-bench','--lib','--message-format=json'],cwd=w,env=e,stdout=fo,stderr=fe,check=True)
art={}
for line in(r/'artifacts.jsonl').read_text().splitlines():
    x=json.loads(line)
    if x.get('reason')=='compiler-artifact':
        for n in x['filenames']:
            if n.endswith('.rlib'):art.setdefault(x['target']['name'],set()).add(n)
libs={};cmd=['rustc','--edition=2021','-O','-C','target-cpu=native','-C','lto=fat','-C','codegen-units=1','-L','dependency='+str(r/'target-native/release/deps')]
for name in ['zopro_core','serde_json','libc']:
    assert len(art[name])==1;f=next(iter(art[name]));libs[name]=dict(path=f,sha256=sha(f));cmd+=['--extern',name+'='+f]
cmd += [str(r/'component.rs'),'-o',str(r/'component')]
with(r/'link.log').open('w')as f:subprocess.run(cmd,env=e,stdout=f,stderr=subprocess.STDOUT,check=True)
save('build-complete.json',dict(completed=time.time(),command=cmd,libraries=libs,binary_sha256=sha(r/'component'),source_pins_sha256=sha(r/'source-pins.json'),core_tests=106))
assert all(sha(w/n)==h for n,h in pins.items());print('BUILD_COMPLETE',flush=True)
# No build or second benchmark continues after this boundary.
save('run-started.json',dict(time=time.time(),pid=os.getpid(),binary_sha256=sha(r/'component'),driver_sha256=sha(r/'component.rs')))
with(r/'run.log').open('w')as f:subprocess.run([str(r/'component'),str(r/'measurements')],env=e,stdout=f,stderr=subprocess.STDOUT,check=True,timeout=180)
rows=json.loads((r/'measurements/rows.json').read_text());assert len(rows)==186
cases={}
for case in p['cases']:
    own=[x for x in rows if x['curve']==case['curve']and x['batch']==case['batch']];assert len(own)==62
    old=sorted((x for x in own if x['mode']==0),key=lambda x:x['block']);new=sorted((x for x in own if x['mode']==1),key=lambda x:x['block']);assert[x['block']for x in old]==list(range(31))==[x['block']for x in new]
    ratios=[y['ns']/x['ns']for x,y in zip(old,new)];ratio=statistics.median(ratios);wins=sum(x<1 for x in ratios)
    cases[f"g{case['curve']}-b{case['batch']}"]=dict(paired_median_ratio=ratio,wins=wins,block_ratios=ratios,old_ns_per_batch=statistics.median(x['ns']/256 for x in old),new_ns_per_batch=statistics.median(x['ns']/256 for x in new),old_scratch_capacity_bytes=sorted({x['scratch_capacity_bytes']for x in old}),new_scratch_capacity_bytes=sorted({x['scratch_capacity_bytes']for x in new}),passed=ratio<1 and wins>=21)
geo=math.exp(statistics.mean(math.log(x['paired_median_ratio'])for x in cases.values()))
assert all(sha(w/n)==h for n,h in pins.items())and all(sha(Path('/home/ubuntu/src/zk-tools/zopro')/n)==h for n,h in a['source_files_sha256'].items())
save('results.json',dict(completed=time.time(),cases=cases,geometric_ratio=geo,passed=geo<1 and all(x['passed']for x in cases.values()),core_tests=106,checksum_comparisons=93,actual_sdk_proofs=0,distinct_gnark=0,adopted=False,goal_achieved=False,normal_prover_dispatch_unchanged=True,source_unchanged=True))
print('PREFIX_RECORDS_COMPONENT_COMPLETE',json.dumps(cases),flush=True)
