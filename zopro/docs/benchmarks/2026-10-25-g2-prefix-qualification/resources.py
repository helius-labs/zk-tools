from pathlib import Path
import json,hashlib,subprocess,os,time,re,shutil
r=Path(__file__).resolve().parent;p=json.load(open(r/'resource-protocol.json'));parent=Path(p['parent']);b=json.load(open(r/'build-complete.json'))['builds'];a=json.load(open(parent/'accepted.json'));repo=Path('/home/ubuntu/src/zk-tools/zopro')
def sha(f):
 with Path(f).open('rb')as x:return hashlib.file_digest(x,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
def guard():
 assert all(sha(parent/'workspace'/n)==h for n,h in p['source_pins'].items())and all(sha(repo/n)==h for n,h in a['source_files_sha256'].items())
 assert sha(parent/'build-complete.json')==p['build_manifest_sha256']and all(sha(v['binary'])==v['sha256']for v in b.values())
 assert all(sha(n)==h for v in b.values()for n,h in v['libraries'].items())and all(sha(r/n)==h for n,h in p['consumer_sources'].items())
assert not(r/'resource-started.json').exists();save('started.json',dict(time=time.time(),pid=os.getpid()));guard();proofs={};rows=[];pressure=[];old=set();refs={}
old=set(json.load(open('/tmp/zopro-operation-model/prior-inventory.json'))['sha256s'])
for root in [Path('/tmp/zopro-operation-model'),parent]:
 for n,h in json.load(open(root/'proof-inventory.json')).items():
  if n.endswith('.proof'):old.add(h)
for n,h in json.load(open(r/'functional-proof-inventory.json')).items():
 f=r/n
 if n.endswith('.proof'):old.add(h)
 else:refs.setdefault(f.parent.name,f.read_bytes())
assert len(old)==1726
base={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|a['environment']|{'ZOPRO_BASIS_KEY_DIR':a['basis_images']}
def verify(out,count):
 files=list((out/'proofs').rglob('*.proof'));assert len(files)==count
 with(out/'gnark.log').open('w')as f:subprocess.run(['/tmp/zopro-gnark-verify/verify','/tmp/zopro-keys',str(out/'proofs'),str(repo/'tools/zolana/zopro-keys.lock')],stdout=f,stderr=subprocess.STDOUT,check=True)
 assert len(re.findall(r'^verified \S+\.proof [0-9a-f]{64}$',(out/'gnark.log').read_text(),re.M))==count
 for f in files:
  h=sha(f);assert h not in proofs.values()and h not in old and f.stat().st_size==164 and f.with_suffix('.public').read_bytes()==refs[f.parent.name]
  for x in [f,f.with_suffix('.public')]:proofs[str(x.relative_to(r))]=sha(x)
 save('proof-inventory.json',proofs)
for i,c in enumerate(p['ordinary_concurrency']):
 out=r/f"concurrent/{i:02d}-{c['label']}-w{c['workers']}-{c['shape']}-{c['on']}";out.mkdir(parents=True);binary=b[c['label']+'-concurrent']['binary'];env=base|{'RAYON_NUM_THREADS':str(c['workers']),'ZOPRO_CPU_G2_PREFIX_RECORDS':str(c['on'])};cmd=[binary,c['shape'],str(c['callers']),str(c['rounds']),str(out/'proofs')];(out/'command.json').write_text(json.dumps(dict(command=cmd,environment={k:v for k,v in env.items()if k.startswith(('ZOPRO_','RAYON_','MALLOC_'))}),indent=2)+'\n')
 with(out/'stdout.log').open('w')as fo,(out/'stderr.log').open('w')as fe:subprocess.run(cmd,env=env,stdout=fo,stderr=fe,check=True,timeout=180)
 total=c['callers']*c['rounds'];verify(out,total)
 row=dict(case=c,passed=True,sdk=total,distinct_gnark=total);rows.append(row);save('concurrency-progress.json',rows);print('CONCURRENT',out.name,'PASS',total,flush=True)
assert sum(x['sdk']for x in rows)==32;save('concurrency-complete.json',dict(completed=time.time(),passed=True,sdk=32,distinct_gnark=32,rows=rows));guard()
for i,c in enumerate(p['pressure']['cases']):
 variant,policy,n=(c[k]for k in ['variant','policy','workers']);out=r/f'pressure/{policy}-w{n}-{variant}';out.parent.mkdir(exist_ok=True);unit=f'zopro-g2-prefix-q-{i}';binary=b[('accepted'if variant=='accepted'else'candidate')+'-warm-sdk']['binary'];env=base|{'RAYON_NUM_THREADS':str(n),'ZOPRO_CPU_G2_PREFIX_RECORDS':str(int(variant=='on'))};unset=[]
 if policy=='default':
  for k in ['MALLOC_TRIM_THRESHOLD_','MALLOC_MMAP_THRESHOLD_','MALLOC_ARENA_MAX']:env.pop(k,None);unset+=['-u',k]
 cmd=['systemd-run','--user','--unit='+unit,'--wait','--pipe','--collect','-p','MemoryHigh=536870912','-p','MemoryMax=536870912','-p','MemorySwapMax=0','-p','OOMPolicy=continue','-p','RuntimeMaxSec=70','/usr/bin/env',*unset,*[k+'='+v for k,v in env.items()if k.startswith(('ZOPRO_','RAYON_','MALLOC_'))],'/usr/bin/python3',str(r/'pressure-child.py'),str(out),str(Path(a['basis_images'])/'transfer_confidential_1_2.image'),binary];(r/(out.name+'-command.json')).write_text(json.dumps(cmd,indent=2)+'\n');failure=None
 with(r/(out.name+'-unit.log')).open('w')as f:
  proc=subprocess.Popen(cmd,stdout=f,stderr=subprocess.STDOUT);start=time.monotonic()
  try:
   while not(out/'ballast-ready').exists():
    if proc.poll()is not None:raise RuntimeError('child ended before warm barrier')
    if time.monotonic()-start>30:raise TimeoutError('warm barrier timeout')
    time.sleep(.02)
   subprocess.run(['systemctl','--user','set-property','--runtime',unit+'.service','MemoryHigh=134217728','MemoryMax=201326592'],check=True,capture_output=True,timeout=15);(out/'go').write_text('go');proc.wait(timeout=60)
  except Exception as e:failure=repr(e)
  finally:
   if proc.poll()is None:subprocess.run(['systemctl','--user','stop',unit+'.service'],check=False,capture_output=True);proc.wait(timeout=10)
 receipt=json.load(open(out/'receipt.json'))if(out/'receipt.json').exists()else None;events=dict(line.split()for line in receipt['metrics']['memory.events'].splitlines())if receipt else{};passed=proc.returncode==0 and receipt is not None and receipt['returncode']==0 and not any(int(events.get(k,0))for k in ['max','oom','oom_kill'])and failure is None
 if passed:verify(out,1)
 row=dict(case=c,passed=passed,receipt=receipt,failure=failure,confirmed_prime_sdk=3 if(out/'ready').exists()else 0,sdk=5 if passed else None,distinct_gnark=1 if passed else 0);pressure.append(row);save('pressure-progress.json',pressure);print('PRESSURE',out.name,'PASS',passed,'PEAK_MIB',int(receipt['metrics']['memory.peak'])/2**20 if receipt else None,flush=True)
 subprocess.run(['systemctl','--user','stop',unit+'.service'],check=False,capture_output=True);subprocess.run(['systemctl','--user','reset-failed',unit+'.service'],check=False,capture_output=True)
 control=Path('/run/user')/str(os.getuid())/'systemd/user.control'/(unit+'.service.d')
 if control.exists():shutil.rmtree(control)
subprocess.run(['systemctl','--user','daemon-reload'],check=True,capture_output=True);guard();save('complete.json',dict(completed=time.time(),passed=all(x['passed']for x in pressure),ordinary_sdk=32,pressure_sdk=45 if all(x['passed']for x in pressure)else sum(x['sdk']or x['confirmed_prime_sdk']for x in pressure),distinct_gnark=sum(x['distinct_gnark']for x in rows+pressure),concurrency=rows,pressure=pressure,source_unchanged=True,adopted=False));print('G2_PREFIX_RESOURCE_QUALIFICATION_COMPLETE',flush=True)
