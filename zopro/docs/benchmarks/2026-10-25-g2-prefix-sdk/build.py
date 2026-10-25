from pathlib import Path
import json,hashlib,os,subprocess,time
r=Path(__file__).resolve().parent;w=r/'workspace';p=json.loads((r/'protocol.json').read_text());a=json.loads((r/'accepted.json').read_text());pins=json.loads((r/'source-pins.json').read_text())
def sha(path):
    with Path(path).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
assert not(r/'build-started.json').exists();(r/'build-started.json').write_text(json.dumps(dict(time=time.time(),pid=os.getpid()))+'\n')
assert all(sha(w/n)==h for n,h in pins.items())and all(sha(Path('/home/ubuntu/src/zk-tools/zopro')/n)==h for n,h in a['source_files_sha256'].items())
e={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|a['environment']|{'RUSTFLAGS':p['rustflags'],'CARGO_TARGET_DIR':str(r/'target-native'),'CARGO_BUILD_JOBS':'16','RAYON_NUM_THREADS':'4',p['runtime_flag']:'1'}
with(r/'core-tests.log').open('w')as f:subprocess.run(['cargo','test','--offline','--locked','--release','-p','zopro-core','--lib','--','--test-threads=1'],cwd=w,env=e,stdout=f,stderr=subprocess.STDOUT,check=True)
assert'test result: ok. 106 passed; 0 failed;'in(r/'core-tests.log').read_text();print('NATIVE_CORE106_PASS',flush=True)
with(r/'prover-tests.log').open('w')as f:subprocess.run(['cargo','test','--offline','--locked','--release','-p','zopro-prover','--lib','--test','cubic','--','--test-threads=1'],cwd=w,env=e,stdout=f,stderr=subprocess.STDOUT,check=True)
s=(r/'prover-tests.log').read_text();assert'test result: ok. 2 passed; 0 failed;'in s and'test result: ok. 6 passed; 0 failed;'in s;print('RAW2_CUBIC6_PASS',flush=True)
with(r/'artifacts.jsonl').open('w')as fo,(r/'cargo-build.log').open('w')as fe:subprocess.run(['cargo','build','--offline','--locked','--release','-p','zopro-bench','--lib','--message-format=json'],cwd=w,env=e,stdout=fo,stderr=fe,check=True)
art={}
for line in(r/'artifacts.jsonl').read_text().splitlines():
    x=json.loads(line)
    if x.get('reason')=='compiler-artifact':
        for n in x['filenames']:
            if n.endswith('.rlib'):art.setdefault(x['target']['name'],set()).add(n)
libs={};cmd=['rustc','--edition=2021','-O','-C','target-cpu=native','-C','lto=fat','-C','codegen-units=1','-L','dependency='+str(r/'target-native/release/deps')]
for name in ['zopro_core','zopro_bn254','zopro_prover','zopro_zolana','memmap2','serde_json','libc','rayon']:
    assert len(art[name])==1;f=next(iter(art[name]));libs[name]=dict(path=f,sha256=sha(f));cmd+=['--extern',name+'='+f]
cmd += [str(r/'sdk.rs'),'-o',str(r/'native-sdk')]
with(r/'link.log').open('w')as f:subprocess.run(cmd,env=e,stdout=f,stderr=subprocess.STDOUT,check=True)
assert all(sha(w/n)==h for n,h in pins.items())
(r/'build-complete.json').write_text(json.dumps(dict(completed=time.time(),binary=str(r/'native-sdk'),binary_sha256=sha(r/'native-sdk'),command=cmd,libraries=libs,rustflags=p['rustflags'],source_pins_sha256=sha(r/'source-pins.json'),driver_sha256=sha(r/'sdk.rs'),native_core_tests=106,raw_tests=2,cubic_tests=6,field_source_unchanged=True,production_changed=False),indent=2)+'\n')
print('G2_PREFIX_SDK_BUILD_COMPLETE',flush=True)
