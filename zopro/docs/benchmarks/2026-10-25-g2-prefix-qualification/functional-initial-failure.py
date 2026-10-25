from pathlib import Path
import json,hashlib,subprocess,os,re,time
r=Path(__file__).resolve().parent;p=json.loads((r/'protocol.json').read_text());parent=Path(p['parent']);a=json.loads((parent/'accepted.json').read_text());b=json.loads((r/'build-complete.json').read_text());native=json.loads((parent/'build-complete.json').read_text());repo=Path('/home/ubuntu/src/zk-tools/zopro')
def sha(path):
    with Path(path).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
assert not(r/'functional-started.json').exists();save('functional-started.json',dict(time=time.time(),pid=os.getpid()))
assert b['portable_core_tests']==89 and all(sha(parent/'workspace'/n)==h for n,h in p['source_pins'].items())
old=set(json.loads(Path('/tmp/zopro-operation-model/prior-inventory.json').read_text())['sha256s']);refs={}
for root in [Path('/tmp/zopro-operation-model'),parent]:
    for n,h in json.loads((root/'proof-inventory.json').read_text()).items():
        if n.endswith('.proof'):old.add(h)
        else:
            f=root/n if not Path(n).is_absolute()else Path(n);refs.setdefault(f.parent.name,f.read_bytes())
assert len(old)==1702
cases=[dict(backend='portable',**c,workers=4,stack=None)for c in p['portable_cases']]
cases += [dict(backend='native',shape=shape,kind='euler',on=on,workers=32,stack=stack)for shape in ['1x2','merge-54x1']for stack in [524288,1048576]for on in [0,1]]
cases += [dict(backend='native',shape=shape,kind='raw',on=on,workers=4,stack=None)for shape in ['1x2','merge-54x1']for on in [0,1]]
cases += [dict(backend='native',shape='1x2',kind=kind,on=on,workers=1,stack=None)for kind in ['euler','raw']for on in [0,1]]
assert len(cases)==24
save('functional-protocol.json',dict(declared=time.time(),cases=cases,expected_sdk=96,expected_gnark=24,build_sha256=sha(r/'build-complete.json'),parent_build_sha256=sha(parent/'build-complete.json'),source_sha256=sha(r/'functional.py'),purpose='Portable/legacy/raw API correctness, actual small worker stacks and single worker, no timing qualification.'))
base={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))and k!='RUST_MIN_STACK'}|p['native_environment'];proofs={};rows=[];inputs={}
for i,c in enumerate(cases):
    out=r/'functional'/f'{i:02d}-{c["backend"]}-{c["kind"]}-{c["shape"]}-{c["on"]}';out.mkdir(parents=True,exist_ok=False)
    directory=Path(a['basis_images'])if c['kind']=='euler'else Path('/tmp/zopro-original-constants/small/candidate/raw')
    stem=('merge_'+c['shape'][6:].replace('x','_'))if c['shape'].startswith('merge-')else'transfer_confidential_'+c['shape'].replace('x','_');image=directory/(stem+'.image');inputs[str(image)]=sha(image)
    env=base|{'RAYON_NUM_THREADS':str(c['workers']),'ZOPRO_CPU_G2_PREFIX_RECORDS':str(c['on']),'ZOPRO_BASIS_KEY_DIR':str(directory)}
    if c['kind']=='raw':env['ZOPRO_CPU_PREPARE_MAX_BYTES']='0'
    if c['stack']:env['RUST_MIN_STACK']=str(c['stack'])
    if c['backend']=='portable':env['ZOPRO_CPU_G2_PREFIX_TRACE']='1'
    binary=b['builds']['portable-sdk']['binary']if c['backend']=='portable'else native['binary'];cmd=[binary,c['shape'],'1','1','4',str(out/'proofs')]
    (out/'command.json').write_text(json.dumps(dict(command=cmd,environment={k:v for k,v in env.items()if k.startswith(('ZOPRO_','RAYON_','MALLOC_','RUST_MIN_STACK'))}),indent=2)+'\n')
    with(out/'stdout.log').open('w')as fo,(out/'stderr.log').open('w')as fe:subprocess.run(cmd,env=env,stdout=fo,stderr=fe,check=True,timeout=180)
    if c['backend']=='portable':assert'G2_PREFIX_RECORDS'not in(out/'stderr.log').read_text()
    with(out/'gnark.log').open('w')as f:subprocess.run(['/tmp/zopro-gnark-verify/verify','/tmp/zopro-keys',str(out/'proofs'),str(repo/'tools/zolana/zopro-keys.lock')],stdout=f,stderr=subprocess.STDOUT,check=True)
    assert len(re.findall(r'^verified \S+\.proof [0-9a-f]{64}$',(out/'gnark.log').read_text(),re.M))==1
    files=list((out/'proofs').rglob('*.proof'));assert len(files)==1;f=files[0];h=sha(f);assert h not in old and h not in proofs.values()and f.stat().st_size==164 and f.with_suffix('.public').read_bytes()==refs[stem]
    for f in [f,f.with_suffix('.public')]:proofs[str(f.relative_to(r))]=sha(f)
    rows.append(dict(case=c,passed=True,sdk=4,distinct_gnark=1));save('functional-progress.json',rows);save('functional-proof-inventory.json',proofs);print('FUNCTIONAL',out.name,'PASS',flush=True)
assert len(proofs)==48 and all(sha(n)==h for n,h in inputs.items())and all(sha(parent/'workspace'/n)==h for n,h in p['source_pins'].items())
save('functional-complete.json',dict(completed=time.time(),passed=True,sdk=96,distinct_gnark=24,proof_public_members=48,rows=rows,inputs=inputs,portable_field_tests=11,portable_core_tests=89,portable_raw_tests=2,portable_cubic_tests=6,adopted=False));print('G2_PREFIX_FUNCTIONAL_COMPLETE',flush=True)
