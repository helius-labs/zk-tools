from pathlib import Path
import json,hashlib,os,subprocess,time
r=Path(__file__).resolve().parent;p=json.loads((r/'protocol.json').read_text());parent=Path(p['parent']);a=json.loads((parent/'accepted.json').read_text());native=json.loads((parent/'build-complete.json').read_text())
def sha(path):
    with Path(path).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
def guard():
    assert sha(parent/'build-complete.json')==p['parent_build_sha256']
    assert all(sha(parent/'workspace'/n)==h for n,h in p['source_pins'].items())
    assert all(sha(Path('/home/ubuntu/src/zk-tools/zopro')/n)==h for n,h in a['source_files_sha256'].items())
    assert all(sha(r/n)==h for n,h in p['consumer_sources'].items())
assert not(r/'build-started.json').exists();save('build-started.json',dict(time=time.time(),pid=os.getpid()));guard();builds={}
accepted=json.loads(Path(a['native_cli_build']).read_text())
specs={'accepted':dict(command=accepted['command'],libraries=accepted['libraries']),'candidate':dict(command=native['command'],libraries={v['path']:v['sha256']for v in native['libraries'].values()})}
for label in ['accepted','candidate']:
    spec=specs[label];assert all(sha(n)==h for n,h in spec['libraries'].items())
    for source in ['warm-sdk.rs']+(['concurrent.rs']if label=='candidate'else[]):
        name=label+'-'+source.removesuffix('.rs');cmd=spec['command'][:-3]+[str(r/source),'-o',str(r/name)]
        with(r/(name+'-link.log')).open('w')as f:subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,check=True)
        builds[name]=dict(binary=str(r/name),sha256=sha(r/name),command=cmd,libraries=spec['libraries']);print('LINKED',name,flush=True)
art={}
for line in(parent/'artifacts.jsonl').read_text().splitlines():
    v=json.loads(line)
    if v.get('reason')=='compiler-artifact':
        for n in v['filenames']:
            if n.endswith('.rlib'):art.setdefault(v['target']['name'],set()).add(n)
assert len(art['log'])==1;log=next(iter(art['log']));cmd=native['command'][:-3]+['--extern','log='+log,str(r/'forester.rs'),'-o',str(r/'forester-driver')]
with(r/'forester-link.log').open('w')as f:subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,check=True)
builds['forester-driver']=dict(binary=str(r/'forester-driver'),sha256=sha(r/'forester-driver'),command=cmd,libraries=specs['candidate']['libraries']|{log:sha(log)});print('LINKED forester-driver',flush=True)
env={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|p['native_environment']|{'RUSTFLAGS':p['portable_flags'],'CARGO_TARGET_DIR':str(r/'target-portable'),'CARGO_BUILD_JOBS':'16','RAYON_NUM_THREADS':'4'}
for package,count in [('zopro-bn254',11),('zopro-core',85)]:
    with(r/(package+'-portable-tests.log')).open('w')as f:subprocess.run(['cargo','test','--offline','--locked','--release','-p',package,'--lib','--','--test-threads=1'],cwd=parent/'workspace',env=env,stdout=f,stderr=subprocess.STDOUT,check=True)
    assert f'test result: ok. {count} passed; 0 failed;'in(r/(package+'-portable-tests.log')).read_text();print('PORTABLE_TESTS',package,count,flush=True)
with(r/'portable-prover-tests.log').open('w')as f:subprocess.run(['cargo','test','--offline','--locked','--release','-p','zopro-prover','--lib','--test','cubic','--','--test-threads=1'],cwd=parent/'workspace',env=env,stdout=f,stderr=subprocess.STDOUT,check=True)
s=(r/'portable-prover-tests.log').read_text();assert'test result: ok. 2 passed; 0 failed;'in s and'test result: ok. 6 passed; 0 failed;'in s
with(r/'portable-artifacts.jsonl').open('w')as fo,(r/'portable-cargo.log').open('w')as fe:subprocess.run(['cargo','build','--offline','--locked','--release','-p','zopro-bench','--lib','--message-format=json'],cwd=parent/'workspace',env=env,stdout=fo,stderr=fe,check=True)
portable={}
for line in(r/'portable-artifacts.jsonl').read_text().splitlines():
    v=json.loads(line)
    if v.get('reason')=='compiler-artifact':
        for n in v['filenames']:
            if n.endswith('.rlib'):portable.setdefault(v['target']['name'],set()).add(n)
cmd=['rustc','--edition=2021','-O','-C','target-cpu=x86-64','-C','lto=fat','-C','codegen-units=1','-L','dependency='+str(r/'target-portable/release/deps')];libs={}
for name in ['zopro_core','zopro_bn254','zopro_prover','zopro_zolana','memmap2','serde_json','libc','rayon']:
    assert len(portable[name])==1;path=next(iter(portable[name]));cmd+=['--extern',name+'='+path];libs[path]=sha(path)
cmd+=[str(parent/'sdk.rs'),'-o',str(r/'portable-sdk')]
with(r/'portable-link.log').open('w')as f:subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,check=True)
builds['portable-sdk']=dict(binary=str(r/'portable-sdk'),sha256=sha(r/'portable-sdk'),command=cmd,libraries=libs,flags=p['portable_flags'])
guard();save('build-complete.json',dict(completed=time.time(),builds=builds,portable_field_tests=11,portable_core_tests=85,portable_raw_tests=2,portable_cubic_tests=6,native_libraries_rebuilt=False,source_unchanged=True));print('G2_PREFIX_QUALIFICATION_BUILD_COMPLETE',flush=True)
