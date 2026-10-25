from pathlib import Path
import json,hashlib,subprocess,os,re,time
r=Path(__file__).resolve().parent;p=json.load(open(r/'protocol.json'));parent=Path(p['parent']);b=json.load(open(parent/'build-complete.json'))['builds'];a=json.load(open(parent/'accepted.json'));repo=Path('/home/ubuntu/src/zk-tools/zopro')
def sha(f):
 with Path(f).open('rb')as x:return hashlib.file_digest(x,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
def guard():
 assert sha(r/'memory-sdk.rs')==p['source_sha256']and sha(parent/'build-complete.json')==p['build_manifest_sha256']
 assert all(sha(parent/'workspace'/n)==h for n,h in json.load(open(parent/'source-pins.json')).items())and all(sha(repo/n)==h for n,h in a['source_files_sha256'].items())
 assert all(sha(n)==h for n,h in b['candidate']['libraries'].items())
assert(r/'started.json').exists()and(r/'build-complete.json').exists();guard();art={}
for line in(parent/'candidate-artifacts.jsonl').read_text().splitlines():
 x=json.loads(line)
 if x.get('reason')=='compiler-artifact':
  for name in x['filenames']:
   if name.endswith('.rlib'):art.setdefault(x['target']['name'],set()).add(name)
rayon=art['rayon'];assert len(rayon)==1;rayon=next(iter(rayon));cmd=b['candidate']['command'][:-3]+['-C','lto=fat','-C','codegen-units=1','--extern','rayon='+rayon,str(r/'memory-sdk.rs'),'-o',str(r/'native-sdk')]
saved=json.load(open(r/'build-complete.json'));assert saved['command']==cmd and sha(saved['binary'])==saved['binary_sha256']and all(sha(n)==h for n,h in saved['libraries'].items());print('REUSED_FROZEN_FAT_LTO_EXECUTABLE',flush=True);references={}
for f in Path('/tmp/zopro-canonical-grain-c7a/remote-results/output/latency/proofs').rglob('*.public'):references.setdefault(f.parent.name,f.read_bytes())
old=set()
for root in [parent,Path('/tmp/zopro-small-core-entry'),Path('/tmp/zopro-small-core-entry-native-qualification'),Path('/tmp/zopro-small-core-entry-qualification')]:
 for file in root.glob('*proof-inventory.json'):
  for n,h in json.load(open(file)).items():
   if n.endswith('.proof'):old.add(h)
base={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|a['environment']|{'ZOPRO_BASIS_KEY_DIR':a['basis_images'],'ZOPRO_SMALL_CORE_ENTRY_TRACE':'1'};base.pop('ZOPRO_CPU_SMALL_CORE_ENTRY',None);proofs={};rows=[]
for c in p['cases']:
 out=r/f"output/{c['shape']}-w{c['workers']}";reused=out.exists();out.mkdir(parents=True,exist_ok=True);env=base|{'RAYON_NUM_THREADS':str(c['workers'])};cmd=[str(r/'native-sdk'),c['shape'],'1','1','4',str(out/'proofs')];(out/'command.json').write_text(json.dumps(dict(command=cmd,environment={k:v for k,v in env.items()if k.startswith(('ZOPRO_','RAYON_','MALLOC_'))}),indent=2)+'\n')
 if reused:
  assert (out/'stdout.log').is_file()and(out/'stderr.log').is_file()and len(list((out/'proofs').rglob('*.proof')))==1;print('REUSED_EXISTING_PROOF',c,flush=True)
 else:
  with(out/'stdout.log').open('w')as fo,(out/'stderr.log').open('w')as fe:subprocess.run(cmd,env=env,stdout=fo,stderr=fe,check=True,timeout=180)
 traces=len(re.findall(r'^SMALL_CORE_ENTRY entered$',(out/'stderr.log').read_text(),re.M));assert traces==(4 if c['shape']=='1x2'else 0)
 if c['workers']==16:
  receipts=[json.loads(x[8:])for x in(out/'stdout.log').read_text().splitlines()if x.startswith('WORKERS ')];assert len(receipts)==2 and all(len(x['workers'])==16 for x in receipts);assert receipts[0]['workers']==receipts[1]['workers']
 with(out/'gnark.log').open('w')as f:subprocess.run(['/tmp/zopro-gnark-verify/verify','/tmp/zopro-keys',str(out/'proofs/b0-m4'),str(repo/'tools/zolana/zopro-keys.lock')],stdout=f,stderr=subprocess.STDOUT,check=True)
 assert len(re.findall(r'^verified \S+\.proof [0-9a-f]{64}$',(out/'gnark.log').read_text(),re.M))==1;files=list((out/'proofs').rglob('*.proof'));assert len(files)==1;f=files[0];assert f.with_suffix('.public').read_bytes()==references[f.parent.name];h=sha(f);assert h not in old and h not in proofs.values()
 for x in [f,f.with_suffix('.public')]:proofs[str(x.relative_to(r))]=sha(x)
 save('proof-inventory.json',proofs);rows.append(dict(case=c,passed=True,entry_hits=traces));save('progress.json',rows);print('COMPAT',c,'PASS',flush=True)
guard();save('complete.json',dict(completed=time.time(),passed=True,sdk=40,distinct_gnark=10,rows=rows,default_flag_enabled_verified=True,adopted=False));print('STANDARD_CLI_COMPATIBILITY_COMPLETE',flush=True)
