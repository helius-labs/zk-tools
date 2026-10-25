from pathlib import Path
import json,hashlib,os,subprocess,re,time
r=Path(__file__).resolve().parent;p=json.loads((r/'protocol.json').read_text());parent=Path(p['parent']);a=json.loads((parent/'accepted.json').read_text());native=json.loads((parent/'build-complete.json').read_text());repo=Path('/home/ubuntu/src/zk-tools/zopro')
def sha(path):
    with Path(path).open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
assert json.loads((r/'complete.json').read_text())['passed']and not(r/'cold-started.json').exists();save('cold-started.json',dict(time=time.time(),pid=os.getpid()))
cases=[dict(variant=v,policy=policy)for policy in ['default','retained']for v in ['accepted','off','on']]
save('cold-protocol.json',dict(declared=time.time(),cases=cases,hard_mib=192,soft_mib=192,swap=0,source_sha256=sha(r/'cold.py'),child_sha256=sha(r/'cold-child.py'),scope='Cold launch entirely inside192MiB group, with an owned copied key image charged to the group.3primes+1SDKproof, no earlier512MiB warm stage. Strict no max/oom/oom_kill or timeout.'))
base={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|a['environment'];refs={};old=set(json.loads(Path('/tmp/zopro-operation-model/prior-inventory.json').read_text())['sha256s'])
for root,name in [(Path('/tmp/zopro-operation-model'),'proof-inventory.json'),(parent,'proof-inventory.json'),(r,'functional-proof-inventory.json'),(r,'proof-inventory.json')]:
    for n,h in json.loads((root/name).read_text()).items():
        if n.endswith('.proof'):old.add(h)
        else:
            f=Path(n)if Path(n).is_absolute()else root/n;refs.setdefault(f.parent.name,f.read_bytes())
assert len(old)==1767
proofs={};rows=[]
for i,c in enumerate(cases):
    out=r/'cold'/f'{i:02d}-{c["policy"]}-{c["variant"]}';out.mkdir(parents=True,exist_ok=False);unit=f'zopro-g2-prefix-cold-{i}'
    env=base|{'RAYON_NUM_THREADS':'32','ZOPRO_CPU_G2_PREFIX_RECORDS':str(int(c['variant']=='on'))};unset=[]
    if c['policy']=='default':
        for key in ['MALLOC_TRIM_THRESHOLD_','MALLOC_MMAP_THRESHOLD_','MALLOC_ARENA_MAX']:env.pop(key,None);unset+=['-u',key]
    binary=a['native_binary']if c['variant']=='accepted'else native['binary'];cmd=['systemd-run','--user','--unit='+unit,'--wait','--pipe','--collect','-p','MemoryHigh=201326592','-p','MemoryMax=201326592','-p','MemorySwapMax=0','-p','RuntimeMaxSec=75','/usr/bin/env',*unset,*[k+'='+v for k,v in env.items()if k.startswith(('ZOPRO_','RAYON_','MALLOC_'))],'/usr/bin/python3',str(r/'cold-child.py'),str(out),str(Path(a['basis_images'])/'transfer_confidential_1_2.image'),binary]
    (out/'command.json').write_text(json.dumps(cmd,indent=2)+'\n')
    with(out/'unit.log').open('w')as f:result=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,timeout=90)
    receipt=json.loads((out/'receipt.json').read_text())if(out/'receipt.json').exists()else None;events=dict(line.split()for line in receipt['metrics']['memory.events'].splitlines())if receipt else{}
    passed=result.returncode==0 and receipt is not None and receipt['returncode']==0 and not any(int(events.get(k,0))for k in ['max','oom','oom_kill'])
    if passed:
        with(out/'gnark.log').open('w')as f:subprocess.run(['/tmp/zopro-gnark-verify/verify','/tmp/zopro-keys',str(out/'proofs/b0-m4'),str(repo/'tools/zolana/zopro-keys.lock')],stdout=f,stderr=subprocess.STDOUT,check=True)
        assert len(re.findall(r'^verified \S+\.proof [0-9a-f]{64}$',(out/'gnark.log').read_text(),re.M))==1
        files=list((out/'proofs').rglob('*.proof'));assert len(files)==1;f=files[0];h=sha(f);assert h not in old and h not in proofs.values()and f.with_suffix('.public').read_bytes()==refs[f.parent.name]
        for f in [f,f.with_suffix('.public')]:proofs[str(f.relative_to(r))]=sha(f)
    rows.append(dict(case=c,passed=passed,receipt=receipt,sdk=4 if passed else None,distinct_gnark=int(passed)));save('cold-progress.json',rows);save('cold-proof-inventory.json',proofs);print('COLD',out.name,passed,flush=True)
    subprocess.run(['systemctl','--user','stop',unit+'.service'],capture_output=True,check=False);subprocess.run(['systemctl','--user','reset-failed',unit+'.service'],capture_output=True,check=False)
assert all(sha(parent/'workspace'/n)==h for n,h in p['source_pins'].items())and all(sha(repo/n)==h for n,h in a['source_files_sha256'].items())
save('cold-complete.json',dict(completed=time.time(),passed=all(x['passed']for x in rows),sdk=sum(x['sdk']or 0 for x in rows),distinct_gnark=len(proofs)//2,rows=rows,adopted=False));print('COLD_QUALIFICATION_COMPLETE',flush=True)
