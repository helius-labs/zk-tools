from pathlib import Path
import json,hashlib,subprocess,os,re,time,statistics,math
r=Path(__file__).resolve().parent;p=json.load(open(r/'protocol.json'));native=Path(p['native_parent']);compat=Path(p['compat_parent']);a=json.load(open(native/'accepted.json'));c=json.load(open(compat/'build-complete.json'));repo=Path('/home/ubuntu/src/zk-tools/zopro');pins=json.load(open(native/'source-pins.json'));proofs={};rows=[];old=set();references={}
def sha(f):
 with Path(f).open('rb')as x:return hashlib.file_digest(x,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
def guard():
 assert json.load(open(compat/'complete.json'))['passed'];assert sha(p['accepted_binary'])==p['accepted_binary_sha256']and sha(c['binary'])==c['binary_sha256']
 assert all(sha(n)==h for n,h in c['libraries'].items())and all(sha(native/'workspace'/n)==h for n,h in pins.items())and all(sha(repo/n)==h for n,h in a['source_files_sha256'].items())
 assert sha(compat/'memory-sdk.rs')==json.load(open(compat/'protocol.json'))['source_sha256']
 assert 'lto=fat'in c['command']and 'codegen-units=1'in c['command']
assert not(r/'started.json').exists();save('started.json',dict(time=time.time(),pid=os.getpid()));guard();save('input-pins.json',dict(native_build_sha256=sha(native/'build-complete.json'),compat_build_sha256=sha(compat/'build-complete.json'),binaries={p['accepted_binary']:p['accepted_binary_sha256'],c['binary']:c['binary_sha256']}))
for folder in [native,compat,Path('/tmp/zopro-small-core-entry-native-qualification'),Path('/tmp/zopro-small-core-entry-qualification')]:
 for f in folder.glob('*proof-inventory.json'):
  for n,h in json.load(open(f)).items():
   if n.endswith('.proof')and isinstance(h,str):old.add(h)
for f in Path('/tmp/zopro-canonical-grain-c7a/remote-results/output/latency/proofs').rglob('*.public'):references.setdefault(f.parent.name,f.read_bytes())
base={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|a['environment']|{'ZOPRO_BASIS_KEY_DIR':a['basis_images']}
def run(name,variant,case):
 out=r/name;out.mkdir(parents=True,exist_ok=False);binary=p['accepted_binary']if variant=='accepted'else c['binary'];env=base|{'RAYON_NUM_THREADS':str(case['workers']),'ZOPRO_CPU_SMALL_CORE_ENTRY':str(int(variant=='on'))};cmd=[binary,case['shape'],str(case['rounds']),'1','4',str(out/'proofs')];(out/'command.json').write_text(json.dumps(dict(command=cmd,environment={k:v for k,v in env.items()if k.startswith(('ZOPRO_','RAYON_','MALLOC_'))}),indent=2)+'\n')
 with(out/'stdout.log').open('w')as fo,(out/'stderr.log').open('w')as fe:subprocess.run(cmd,env=env,stdout=fo,stderr=fe,check=True,timeout=180)
 lines=(out/'stdout.log').read_text().splitlines();samples=[json.loads(x[7:])for x in lines if x.startswith('SAMPLE ')];summary=[json.loads(x[8:])for x in lines if x.startswith('SUMMARY ')];assert len(summary)==1 and len(samples)==case['rounds']and summary[0]['prime_per_mode_block']==3 and summary[0]['blocks']==1 and summary[0]['modes']==[4]
 assert all(x['mode']==4 and x['block']==0 for x in samples);receipts=[json.loads(x[8:])for x in lines if x.startswith('WORKERS ')]
 if case['workers']==16:
  assert len(receipts)==2 and receipts[0]['workers']==receipts[1]['workers']and all(len(x['workers'])==16 and x['pin']for x in receipts)
 else:assert not receipts
 with(out/'gnark.log').open('w')as f:subprocess.run(['/tmp/zopro-gnark-verify/verify','/tmp/zopro-keys',str(out/'proofs/b0-m4'),str(repo/'tools/zolana/zopro-keys.lock')],stdout=f,stderr=subprocess.STDOUT,check=True)
 assert len(re.findall(r'^verified \S+\.proof [0-9a-f]{64}$',(out/'gnark.log').read_text(),re.M))==1;files=list((out/'proofs').rglob('*.proof'));assert len(files)==1;f=files[0];assert f.stat().st_size==164 and f.with_suffix('.public').read_bytes()==references[f.parent.name];h=sha(f);assert h not in old and h not in proofs.values()
 for f in [f,f.with_suffix('.public')]:proofs[str(f.relative_to(r))]=sha(f)
 save('proof-inventory.json',proofs);return dict(variant=variant,case=case,median_ms=statistics.median(x['ms']for x in samples),samples=samples,summary=summary[0],receipts=receipts)
cases=p['cases'];variants=p['variants']
for block in range(p['blocks']):
 shift=block%len(cases);order=cases[shift:]+cases[:shift]
 if block%2:order=order[::-1]
 for ci,case in enumerate(order):
  shift=(ci+block)%len(variants);vo=variants[shift:]+variants[:shift]
  if block%2:vo=vo[::-1]
  for variant in vo:
   name=f"timing/b{block}-{case['shape']}-w{case['workers']}-{variant}";save('progress.json',dict(time=time.time(),name=name,completed=len(rows),expected=210));row=run(name,variant,case);row['block']=block;rows.append(row);save('rows.json',rows);print('TIMED',name,row['median_ms'],flush=True)
assert len(rows)==210 and len(proofs)==420;guard();result={};geometric={}
for case in cases:
 key=f"{case['shape']}-w{case['workers']}";result[key]={}
 for variant in variants:
  vs=[x for x in rows if x['variant']==variant and x['case']==case];cell=dict(median_ms=statistics.median(x['median_ms']for x in vs),maxrss_mib=max(x['summary']['process_peak_rss_kib']for x in vs)/1024,comparisons={})
  for control in ['accepted','off']:
   ratios=[x['median_ms']/next(y['median_ms']for y in rows if y['case']==case and y['block']==x['block']and y['variant']==control)for x in vs];cell['comparisons'][control]=dict(ratios=ratios,median_ratio=statistics.median(ratios),wins=sum(x<1 for x in ratios))
  result[key][variant]=cell
for control in ['accepted','off']:
 ratios=[math.exp(statistics.mean(math.log(result[f"{case['shape']}-w32"]['on']['comparisons'][control]['ratios'][block])for case in cases if case['workers']==32))for block in range(p['blocks'])];geometric[control]=dict(block_ratios=ratios,pooled_ratio=math.exp(statistics.mean(map(math.log,ratios))),wins=sum(x<1 for x in ratios))
primary=result['1x2-w32']['on'];primary_ok=all(primary['comparisons'][v]['median_ratio']<1 and primary['comparisons'][v]['wins']>=4 for v in ['accepted','off']);aggregate_ok=all(x['pooled_ratio']<=1 and x['wins']>=4 for x in geometric.values());nonregression=all(c['on']['comparisons'][v]['median_ratio']<=1.02 for c in result.values()for v in ['accepted','off']);refs={'1x2':(37,24),'2x2':(59,30),'4x2':(109,48),'8x2':(189,74),'12x2':(284,101),'16x2':(335,124),'24x2':(528,176),'merge-24x1':(507,175)};m5={s:dict(cpu_reference_ms=cpu,metal_reference_ms=metal,candidate_ms=result[s+'-w32']['on']['median_ms'],cpu_speedup=cpu/result[s+'-w32']['on']['median_ms'],metal_speedup=metal/result[s+'-w32']['on']['median_ms'],goal10x=cpu/result[s+'-w32']['on']['median_ms']>=10)for s,(cpu,metal)in refs.items()};save('results.json',dict(completed=time.time(),summary=result,geometric=geometric,m5=m5,primary_passed=primary_ok,aggregate_passed=aggregate_ok,nonregression_passed=nonregression,passed=primary_ok and aggregate_ok and nonregression,sdk=4914,distinct_gnark=210,source_unchanged=True,adopted=False,goal_achieved=all(x['goal10x']for x in m5.values())));print('PRODUCTION_NATIVE_CLI_COMPARISON_COMPLETE',flush=True)
