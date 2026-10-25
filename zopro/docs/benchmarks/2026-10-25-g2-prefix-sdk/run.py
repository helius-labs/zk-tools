from pathlib import Path
import json,hashlib,subprocess,os,re,time,statistics,math
r=Path(__file__).resolve().parent;p=json.load(open(r/'protocol.json'));a=json.load(open(r/'accepted.json'));c=json.load(open(r/'build-complete.json'));repo=Path('/home/ubuntu/src/zk-tools/zopro');pins=json.load(open(r/'source-pins.json'));proofs={};rows=[];pre=[];old=json.load(open('/tmp/zopro-small-core-entry-adoption/proof-inventory.json'));old_hashes={h for n,h in old.items()if n.endswith('.proof')};references={}
for inv in ['/tmp/zopro-smaller-groups/proof-inventory.json','/tmp/zopro-smaller-groups/memory/proof-inventory.json','/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-25-sparse-grouped-g2/proof-inventory.json','/tmp/zopro-sparse-grouped-g2-remaining/proof-inventory.json','/tmp/zopro-borrowed-grouped-g2/proof-inventory.json','/tmp/zopro-borrowed-grouped-g2/portable/proof-inventory.json','/tmp/zopro-borrowed-profile/proof-inventory.json','/tmp/zopro-batched-steal/proof-inventory.json','/tmp/zopro-operation-model/proof-inventory.json']:
 old_hashes.update(h for n,h in json.load(open(inv)).items()if n.endswith('.proof'))
assert len(old_hashes)==1473
def sha(f):
 with Path(f).open('rb')as x:return hashlib.file_digest(x,'sha256').hexdigest()
def save(n,x):(r/n).write_text(json.dumps(x,indent=2)+'\n')
for n,h in old.items():
 if n.endswith('.public'):
  f=Path(n);references.setdefault(f.parent.name,f.read_bytes())
def guard():
 assert sha(r/'run.py')==p['controller_sha256'];assert sha(p['accepted_binary'])==p['accepted_binary_sha256']and sha(c['binary'])==c['binary_sha256'];assert all(sha(v['path'])==v['sha256']for v in c['libraries'].values());assert all(sha(r/'workspace'/n)==h for n,h in pins.items());assert all(sha(repo/n)==h for n,h in a['source_files_sha256'].items());assert sha(r/'sdk.rs')==p['driver_sha256'];assert'lto=fat'in c['command']and'codegen-units=1'in c['command']
assert not Path('/proc',str(json.load(open(r/'build-started.json'))['pid'])).exists();assert not(r/'run-started.json').exists();save('run-started.json',dict(time=time.time(),pid=os.getpid()));guard();base={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|a['environment']|{'ZOPRO_BASIS_KEY_DIR':a['basis_images']};shapes=['1x2','2x2','4x2','8x2','12x2','16x2','24x2','merge-24x1','merge-54x1'];input_pins={}
for shape in shapes:
 stem=('merge_'+shape[6:].replace('x','_'))if shape.startswith('merge-')else'transfer_confidential_'+shape.replace('x','_');image=Path(a['basis_images'])/(stem+'.image');assert sha(image)in{x['sha256']for x in a['basis_image_hashes']};fixture=repo/'tools/zolana/fixtures'/((shape if shape.startswith('merge-')else'transfer-'+shape)+'.json');input_pins[str(image)]=sha(image);input_pins[str(fixture)]=sha(fixture)
save('input-pins.json',input_pins)
def run(name,variant,case,trace=False):
 out=r/name;out.mkdir(parents=True,exist_ok=False);binary=p['accepted_binary']if variant=='accepted'else c['binary'];env=base|{'RAYON_NUM_THREADS':str(case['workers'])}
 if variant!='default':env[p['runtime_flag']]=str(int(variant=='on'))
 if trace:env[p['trace_flag']]='1'
 cmd=[binary,case['shape'],str(case['rounds']),'1','4',str(out/'proofs')];(out/'command.json').write_text(json.dumps(dict(command=cmd,environment={k:v for k,v in env.items()if k.startswith(('ZOPRO_','RAYON_','MALLOC_'))}),indent=2)+'\n')
 with(out/'stdout.log').open('w')as fo,(out/'stderr.log').open('w')as fe:subprocess.run(cmd,env=env,stdout=fo,stderr=fe,check=True,timeout=180)
 lines=(out/'stdout.log').read_text().splitlines();samples=[json.loads(x[7:])for x in lines if x.startswith('SAMPLE ')];summary=[json.loads(x[8:])for x in lines if x.startswith('SUMMARY ')];assert len(summary)==1 and len(samples)==case['rounds']and summary[0]['prime_per_mode_block']==3 and summary[0]['blocks']==1 and summary[0]['modes']==[4];assert all(x['mode']==4 and x['block']==0 for x in samples)
 receipts=[json.loads(x[8:])for x in lines if x.startswith('WORKERS ')]
 if case['workers']==16:assert len(receipts)==2 and receipts[0]['workers']==receipts[1]['workers']and all(len(x['workers'])==16 and x['pin']for x in receipts)
 else:assert not receipts
 if trace:
  text=(out/'stderr.log').read_text();hits=re.findall(r'G2_PREFIX_RECORDS enabled=(true|false) batch=(\d+)',text)
  assert hits and all(enabled==('true'if variant=='on'else'false')for enabled,n in hits)and any(int(n)>=8 for enabled,n in hits),(name,text[-3000:])
 with(out/'gnark.log').open('w')as f:subprocess.run(['/tmp/zopro-gnark-verify/verify','/tmp/zopro-keys',str(out/'proofs/b0-m4'),str(repo/'tools/zolana/zopro-keys.lock')],stdout=f,stderr=subprocess.STDOUT,check=True)
 assert len(re.findall(r'^verified \S+\.proof [0-9a-f]{64}$',(out/'gnark.log').read_text(),re.M))==1;files=list((out/'proofs').rglob('*.proof'));assert len(files)==1;f=files[0];assert f.stat().st_size==164 and f.with_suffix('.public').read_bytes()==references[f.parent.name];h=sha(f);assert h not in old_hashes and h not in proofs.values()
 for q in [f,f.with_suffix('.public')]:proofs[str(q.relative_to(r))]=sha(q)
 save('proof-inventory.json',proofs);return dict(variant=variant,case=case,median_ms=statistics.median(x['ms']for x in samples),samples=samples,summary=summary[0],receipts=receipts)
for shape in shapes:
 for variant in ['off','on']:
  name=f'preflight/{shape}-{variant}';case=dict(shape=shape,workers=4,rounds=1);pre.append(run(name,variant,case,True));print('PREFLIGHT',name,flush=True)
pre.append(run('preflight/default-off-primary32','default',dict(shape='1x2',workers=32,rounds=1),True));save('preflight-complete.json',dict(completed=time.time(),rows=pre,sdk=76,distinct_gnark=19));guard()
cases=p['pilot']['cases'];variants=p['pilot']['variants'];blocks=p['pilot']['blocks']
for block in range(blocks):
 shift=block%len(cases);order=cases[shift:]+cases[:shift]
 if block%2:order=order[::-1]
 for ci,case in enumerate(order):
  shift=(ci+block)%len(variants);vo=variants[shift:]+variants[:shift]
  if block%2:vo=vo[::-1]
  for variant in vo:
   name=f"timing/b{block}-{case['shape']}-w{case['workers']}-{variant}";save('progress.json',dict(time=time.time(),name=name,completed=len(rows),expected=210));row=run(name,variant,case);row['block']=block;rows.append(row);save('rows.json',rows);print('TIMED',name,row['median_ms'],flush=True)
assert len(rows)==210 and len(proofs)==458;guard();assert all(sha(n)==h for n,h in input_pins.items());result={}
for case in cases:
 key=f"{case['shape']}-w{case['workers']}";result[key]={}
 for variant in variants:
  vs=[x for x in rows if x['variant']==variant and x['case']==case];cell=dict(median_ms=statistics.median(x['median_ms']for x in vs),peak_rss_mib=statistics.median(x['summary']['process_peak_rss_kib']/1024 for x in vs),comparisons={})
  for control in ['accepted','off']:
   ratios=[x['median_ms']/next(y['median_ms']for y in rows if y['case']==case and y['block']==x['block']and y['variant']==control)for x in vs];cell['comparisons'][control]=dict(ratios=ratios,median_ratio=statistics.median(ratios),wins=sum(x<1 for x in ratios),max_ratio=max(ratios))
  result[key][variant]=cell
primary=result['1x2-w32']['on'];primary_ok=all(primary['comparisons'][v]['median_ratio']<1 and primary['comparisons'][v]['wins']>=4 for v in ['accepted','off']);nonreg=all(cell['on']['comparisons'][v]['median_ratio']<=1.02 and cell['on']['comparisons'][v]['max_ratio']<=1.05 for cell in result.values()for v in ['accepted','off']);calibration=all(cell['off']['comparisons']['accepted']['median_ratio']<=1.03 for cell in result.values());ms=primary['median_ms'];geometric={v:math.exp(statistics.mean(math.log(cell['on']['comparisons'][v]['median_ratio'])for k,cell in result.items()if k.endswith('-w32')))for v in ['accepted','off']};save('results.json',dict(geometric_ratios=geometric,completed=time.time(),summary=result,passed=primary_ok and nonreg and calibration and all(v<1 for v in geometric.values()),primary_passed=primary_ok,nonregression_passed=nonreg,calibration_passed=calibration,primary_m5=dict(cpu_reference_ms=37,metal_reference_ms=24,candidate_ms=ms,cpu_speedup=37/ms,metal_speedup=24/ms,goal10x=37/ms>=10),sdk=7216,distinct_gnark=229,proof_public_members=458,source_unchanged=True,adopted=False,goal_achieved=False));print('G2_PREFIX_SDK_ALL_NINE_COMPLETE',json.dumps(result),flush=True)

cpu=[37,59,109,189,284,335,528,507];metal=[24,30,48,74,101,124,176,175];shapes=['1x2','2x2','4x2','8x2','12x2','16x2','24x2','merge-24x1'];data=json.load(open(r/'results.json'));data['m5']=[dict(shape=s,candidate_ms=result[s+'-w32']['on']['median_ms'],m5_cpu_ms=c,m5_metal_ms=m,cpu_speedup=c/result[s+'-w32']['on']['median_ms'],metal_speedup=m/result[s+'-w32']['on']['median_ms'],goal10x=c/result[s+'-w32']['on']['median_ms']>=10)for s,c,m in zip(shapes,cpu,metal)];data['goal_all8_10x']=all(v['goal10x']for v in data['m5']);save('results.json',data)
