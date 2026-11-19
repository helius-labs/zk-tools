from zp_report_common import *
from collections import Counter
import os,random,re,time,tempfile
d=base/'product-faults609';out=d/'output';out.mkdir(parents=True,exist_ok=True)
a=docs/'2026-11-19-product-faults609/artifacts';a.mkdir(parents=True,exist_ok=True)
bundle=base/'matched-engines531/bundle';cfg=rd(bundle/'config.json')
sdk=base/'product-pool607/target/x86_64-unknown-linux-gnu/release/examples/cpu-sdk'
assert sha(sdk)==rd(base/'product-pool607/built.json')['sdk']
cases=['12x2','24x2'];modes=['main','idle','product'];jobs=[]
for k in range(3):
 cells=[(k,s,m) for s in cases for m in modes];random.Random(90609+k).shuffle(cells);jobs+=cells
def commit(msg):
 subprocess.run(['git','add','--',a.parent],cwd=repo,check=True)
 subprocess.run(['git','commit','--only','-m',msg,'--',a.parent],cwd=repo,check=True)
if not (a.parent/'protocol.md').exists():
 (a.parent/'protocol.md').write_text('''# Where warm product-pool proofs incur minor faults

Profile the same three placements on the 12×2 and 24×2 public reference requests with the exact product-pool trial executable. Across three shuffled blocks use the frozen selected widths of 24 and 28 logical workers respectively. All arms share the same main affinity within each shape. Capture one user-space call stack every 127 minor faults per thread with Linux perf. The SDK enables collection after three warmups and disables it before writing the proof. Each process times eleven complete calls and exports one proof for independent verification.

Report both the process-wide minor-fault count per proof and the leaf and caller symbols associated with sampled faults. Per-thread periods leave short-lived or low-count threads underrepresented. Profiled wall times are diagnostic. The earlier uninstrumented full-proof experiment supplies the latency comparison.
''')
 save(a/'schedule.json',jobs);commit('Fix the warm minor-fault profile across main and auxiliary workers')
while not (base/'group-cursors608/final.json').exists():
 if not any(l.endswith('python3 /tmp/zp_run608.py') for l in subprocess.check_output(['ps','-eo','args='],text=True).splitlines()):raise RuntimeError('608 stopped')
 time.sleep(3)
def family(frames):
 # The first named prover frame in the user call chain identifies the operation.
 for name in frames:
  if 'zopro_' not in name:continue
  for key,terms in [('FFT/product',['zopro_core::fft','euler_product','product_values','zopro_core::lagrange::origin']),('solver',['zopro_core::solver','solve_with_pool','solve_in_with_slots']),('scalar grouping',['scalar_groups','group_witness','group_with_gate','msm_affine::grouping']),('fixed group plans',['groups482','linear_groups','linear482']),('curve queries',['msm_affine','zopro_core::msm','cpu_msms','zopro_core::pippenger']),('row offsets',['wire_offsets','center_block','wire_offset'])]:
   if any(t in name for t in terms):return key
 return 'other/unresolved'
rows=rd(d/'rows.json') if (d/'rows.json').exists() else []
for k,s,m in jobs:
 if any((r['b'],r['sh'],r['m'])==(k,s,m) for r in rows):continue
 c=cfg['shapes'][s];q=out/f'{k}-{s}-{m}';q.mkdir(exist_ok=True)
 env=cfg['base_env']|c['env']|dict(ZOPRO_BASIS_KEY_DIR=str(bundle/'images'),ZOPRO_CPU_LAGRANGE_DIR=str(out/'cache'),ZOPRO_BENCH_WIRE_FILE=str(bundle/c['wire']),ZOPRO_GROUP_SAMPLE='1',ZOPRO_ADAPTIVE_CORRECTIONS='1')
 main=list(range(32));w=28;root=base/'large-groups497'/s;g=root/'groups.bin';key=bundle/c['image']
 if s=='12x2':
  w=24;main=list(range(16))+sorted(16+8*ccd+(k+3*i)%8 for ccd in range(2) for i in range(7))
  g=base/'nested-groups517/nested.bin';root=base/'residual-groups498';key=base/'mapped-blocks587/twelve.image';env['ZOPRO_TRIAL_CENTER_GATHER']='1'
 front=list(range(16))+sorted(16+8*ccd+(k+3*i)%8 for ccd in range(2) for i in range((w-16)//2))
 if m!='main':env['ZOPRO_BENCH_FRONT_CPUS']=','.join(map(str,front))
 if m=='product':env['ZOPRO_TRIAL_PRODUCT_POOL']='1'
 env.update(RAYON_NUM_THREADS=str(len(main)),ZOPRO_LINEAR_GROUPS=str(root),ZOPRO_TRIAL_GROUPS=str(g))
 data=q/'perf.data'
 with tempfile.TemporaryDirectory() as tmp:
  ctl=Path(tmp)/'ctl';ack=Path(tmp)/'ack';os.mkfifo(ctl);os.mkfifo(ack);env.update(ZOPRO_BENCH_PERF_CTL=str(ctl),ZOPRO_BENCH_PERF_ACK=str(ack))
  cmd=['sudo','-n','perf','record','--no-buildid-cache','-m','512','-e','minor-faults:u','-c','127','--call-graph','dwarf,4096','--delay=-1','--control',f'fifo:{ctl},{ack}','-o',data,'--','runuser','-u','ubuntu','--','env',*[f'{k}={v}' for k,v in env.items()],'taskset','-c',','.join(map(str,main)),sdk,key,bundle/c['request'],'11',q]
  with (q/'profile.log').open('w') as log:subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT,timeout=240,check=True)
 lines=(q/'profile.log').read_text().splitlines()
 def tags(t):return [l[len(t):] for l in lines if l.startswith(t)]
 su,=map(json.loads,tags('SUMMARY '));xs=list(map(json.loads,tags('SAMPLE ')))
 assert list(map(json.loads,tags('MAIN_CPUS ')))==[main] and list(map(json.loads,tags('FRONT_CPUS ')))==([front] if m!='main' else [])
 assert su['priming']==3 and su['workers']==len(main) and [r['round'] for r in xs]==list(range(11))
 raw=subprocess.check_output(['sudo','-n','perf','script','-i',data,'--demangle','-F','comm,pid,event,ip,sym,dso'],text=True)
 stacks=Counter();families=Counter();leaves=Counter()
 for record in re.split(r'\n\s*\n',raw.strip()):
  if not record.strip():continue
  frames=[]
  for line in record.splitlines():
   match=re.match(r'^\s*[0-9a-f]+\s+(.+)\s+\(([^()]*)\)\s*$',line)
   if match:frames.append(re.sub(r'\+0x[0-9a-f]+$','',match[1].strip()))
  assert frames,record[:400];stacks[tuple(frames)]+=1;families[family(frames)]+=1;leaves[frames[0]]+=1
 assert sum(families.values())>8
 proof=q/'0.proof';pub=q/'0.public';dg=sha(proof);assert sha(pub)==c['public_sha256'];msg=subprocess.check_output([bundle/'bin/verify',bundle/c['image'],c['image_sha256'],proof,pub,c['public_sha256']],text=True);assert 'VERIFIED proof_sha256='+dg in msg
 (q/'receipt').write_text(msg);pack(q/'stacks.json.gz',[dict(frames=list(f),samples=n) for f,n in stacks.items()])
 rows.append(dict(b=k,sh=s,m=m,summary=su,ms=mean(r['ms'] for r in xs),samples=xs,proof=dg,families=families,leaves=leaves,stacks=sum(families.values())));save(d/'rows.json',rows)
assert len(rows)==18
pack(a/'samples.json.gz',rows);save(a/'verification.json',extend(d,base/'group-cursors608/ledger.json',[r['proof'] for r in rows]))
table=[]
for s in cases:
 for m in modes:
  g=[r for r in rows if r['sh']==s and r['m']==m];hist=sum((Counter(r['families']) for r in g),Counter());label,n=hist.most_common(1)[0];pct=100*n/hist.total()
  table.append(f"| {s} | {m} | {median(r['summary']['minor_faults']/11 for r in g):,.0f} | {hist.total():,} | {label} ({pct:.1f}%) |")
(a.parent/'README.md').write_text('''# Sampled minor faults during complete proofs

Each cell includes three shuffled processes of eleven warm calls. The family column groups stacks by the nearest recognized prover operation. Stacks with no recognized operation are retained. The median process fault count and the number of sampled stacks have different denominators.

| Transfer | Placement | Median minor faults per proof | Sampled stacks | Most frequent family |
|---|---|---:|---:|---|
'''+ '\n'.join(table)+'\n\nAll 18 exports verified. [Protocol](protocol.md) · [Per-process leaf and family counts](artifacts/samples.json.gz) · [Uninstrumented latency and peak memory](../2026-11-19-product-pool607/README.md)\n')
shutil.copyfile(Path(__file__),a/'profile.py');commit('Locate warm page faults in grouped transfer proofs');save(d/'final.json',table)
