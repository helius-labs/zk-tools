from pathlib import Path
import json,hashlib,subprocess,os,re,time
r=Path(__file__).resolve().parent;p=json.load(open(r/'protocol.json'));compat=Path('/tmp/zopro-small-core-entry-native-compat');c=json.load(open(compat/'build-complete.json'));a=json.load(open('/tmp/zopro-canonical-grain-adoption/production-source-manifest.json'));prod=json.load(open('/tmp/zopro-small-core-entry-native-production/results.json'));assert prod['passed'];assert not(r/'started.json').exists();(r/'started.json').write_text(json.dumps(dict(time=time.time(),pid=os.getpid()))+'\n');rows=[];inventory={}
def sha(f):
 with Path(f).open('rb')as x:return hashlib.file_digest(x,'sha256').hexdigest()
old=set()
for root in [compat,Path('/tmp/zopro-small-core-entry-native-production')]:
 for n,h in json.load(open(root/'proof-inventory.json')).items():
  if n.endswith('.proof'):old.add(h)
public=next((compat/'output/1x2-w32/proofs').rglob('*.public')).read_bytes();base={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|a['environment']|{'ZOPRO_SMALL_CORE_ENTRY_TRACE':'1','RAYON_NUM_THREADS':'32'}
for variant in p['variants']:
 binary=a['native_binary']if variant=='accepted'else c['binary'];assert sha(binary)==(a['native_binary_sha256']if variant=='accepted'else c['binary_sha256']);out=r/'output'/variant;out.parent.mkdir(exist_ok=True);label=variant;cmd=['/usr/bin/python3',str(r/'child.py'),str(out),str(Path(a['basis_images'])/'transfer_confidential_1_2.image'),binary];env=base|{'ZOPRO_CPU_SMALL_CORE_ENTRY':str(int(variant=='on'))}
 with(r/(variant+'-launch.log')).open('w')as f:proc=subprocess.run(['/usr/bin/python3',str(r/'run-unit.py'),label]+cmd,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=140)
 measured=json.load(open(r/'runs'/label/'result.json'));receipt=json.load(open(out/'receipt.json'))if(out/'receipt.json').exists()else None;success=proc.returncode==0 and measured['normal_main_exit_observed']and receipt is not None and receipt['returncode']==0
 if receipt:
  events=dict(line.split()for line in receipt['metrics']['memory.events'].splitlines());success=success and events['oom']=='0'and events['oom_kill']=='0'and receipt['metrics']['memory.high'].strip()=='max'and receipt['metrics']['memory.swap.max'].strip()=='0'
 if success:
  lines=(out/'stdout.log').read_text().splitlines();samples=[json.loads(x[7:])for x in lines if x.startswith('SAMPLE ')];assert len(samples)==51;trace=len(re.findall(r'^SMALL_CORE_ENTRY entered$',(out/'stderr.log').read_text(),re.M));assert trace==(54 if variant=='on'else 0)
  with(out/'gnark.log').open('w')as f:subprocess.run(['/tmp/zopro-gnark-verify/verify','/tmp/zopro-keys',str(out/'proofs/b0-m4'),'/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock'],stdout=f,stderr=subprocess.STDOUT,check=True)
  files=list((out/'proofs').rglob('*.proof'));assert len(files)==1 and len(re.findall(r'^verified \S+\.proof [0-9a-f]{64}$',(out/'gnark.log').read_text(),re.M))==1;proof=files[0];h=sha(proof);assert h not in old and h not in inventory.values()and proof.with_suffix('.public').read_bytes()==public
  for path in [proof,proof.with_suffix('.public')]:inventory[str(path.relative_to(r))]=sha(path)
 row=dict(variant=variant,passed=success,measured=measured,receipt=receipt,sdk=54 if success else None);rows.append(row);(r/'progress.json').write_text(json.dumps(rows,indent=2)+'\n');(r/'proof-inventory.json').write_text(json.dumps(inventory,indent=2)+'\n');print('SUSTAINED',variant,'PASS',success,'PEAK_MIB',int(receipt['metrics']['memory.peak'])/2**20 if receipt else None,flush=True)
(r/'complete.json').write_text(json.dumps(dict(completed=time.time(),passed=all(x['passed']for x in rows),sdk=162 if all(x['passed']for x in rows)else None,distinct_gnark=len(inventory)//2,rows=rows,adopted=False),indent=2)+'\n');print('SUSTAINED_CHECK_COMPLETE',flush=True)
