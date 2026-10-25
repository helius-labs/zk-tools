from pathlib import Path
import hashlib,json,os,struct,subprocess,time
r=Path('/tmp/zopro-g2-prefix-qualification');base=Path('/tmp/zopro-forester-format-retry');prior=Path('/tmp/zopro-forester-local')
build=json.loads((r/'build-complete.json').read_text());driver=Path(build['builds']['forester-driver']['binary'])
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert sha(driver)==build['builds']['forester-driver']['sha256']
parent=Path('/tmp/zopro-g2-prefix-sdk'); accepted=json.loads((parent/'accepted.json').read_text()); env={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|accepted['environment']|{'ZOPRO_CPU_G2_PREFIX_RECORDS':'1','ZOPRO_CPU_WORKSPACE_TRACE':'1'}
def run(label,command,cap):
 with(r/(label+'-launch.log')).open('w')as f:
  p=subprocess.run(['python3',str(r/'run-large.py'),label]+command,env=dict(env,FORESTER_MEMORY_GIB=str(cap)),stdout=f,stderr=subprocess.STDOUT)
 result=json.loads((r/'runs'/label/'result.json').read_text());assert 'G2_PREFIX_RECORDS'not in(r/'runs'/label/'stdout.log').read_text(), 'per-batch trace must remain disabled'
 events=[json.loads(l[6:])for l in(r/'runs'/label/'stdout.log').read_text().splitlines()if l.startswith('EVENT ')]
 success=p.returncode==0 and result['normal_main_exit_observed']
 print(label,'success',success,flush=True)
 return success,events,result
def prove(batch,image,cap):
 label=f'prove-{batch}-{cap}';proofs=r/('proofs-'+label)
 success,events,measured=run(label,[str(driver),'prove',str(image),str(prior/f'fixtures/{batch}'),str(proofs),'1'],cap)
 inventory=[];old=set()
 for batch_old in [250,4999]:
  for old_root in ['/tmp/zopro-canonical-grain-qualification','/tmp/zopro-small-core-entry-native-qualification']:
   folder=Path(old_root)/f'proofs-prove-{batch_old}-95'
   old.update(sha(f)for f in folder.glob('*.proof'))
 for p in sorted(proofs.glob('*.proof')):
  index=int(p.stem);assert index in [-1,0];assert sha(p)not in old
  public=p.with_suffix('.public').read_bytes();witness=(prior/f'fixtures/{batch}/{(index+1)%2}.witness').read_bytes();n=struct.unpack('>I',witness[:4])[0]
  assert public==struct.pack('>III',n,0,n)+witness[12:12+32*n]
  inventory.append(dict(path=str(p),sha256=sha(p),public_sha256=sha(p.with_suffix('.public'))))
 assert len({x['sha256']for x in inventory})==len(inventory)
 if inventory:
  with(r/(label+'-independent.log')).open('w')as f:subprocess.run([str(prior/'go-driver'),'verify',str(base/f'dump-{batch}/vk.bin'),str(proofs)],stdout=f,stderr=subprocess.STDOUT,check=True)
 if success:assert len(inventory)==2 and any(e['stage']=='complete'for e in events)
 (r/(label+'-verified.json')).write_text(json.dumps(dict(completed=time.time(),success=success,cap_GiB=cap,swap=0,proofs=inventory,events=events,measured=measured),indent=2)+'\n')
 return success