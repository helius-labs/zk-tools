from zp_report_common import *
from collections import Counter
import re
d=base/'product-faults609';a=docs/'2026-11-19-product-faults609/artifacts'
rows=rd(d/'rows.json')
stacksets=[]
for r in rows:
 path=d/'output'/f"{r['b']}-{r['sh']}-{r['m']}"/'stacks.json.gz'
 stacksets.append(json.loads(gzip.decompress(path.read_bytes())))
names=sorted({f for rs in stacksets for stack in rs for f in stack['frames']})
resolved=subprocess.check_output(['c++filt','-s','rust'],input='\n'.join(names)+'\n',text=True).splitlines()
assert len(resolved)==len(names)
lookup={f:re.sub(r'\[[0-9a-f]{16}\]','',v) for f,v in zip(names,resolved)}
def family(frames):
 for name in frames:
  if 'zopro_' not in name:continue
  for key,terms in [('FFT/product',['zopro_core::fft','euler_product','product_values','zopro_core::lagrange::origin']),('solver',['zopro_core::solver','solve_with_pool','solve_in_with_slots']),('scalar grouping',['scalar_groups','group_witness','group_with_gate','msm_affine::grouping']),('fixed group plans',['groups482','linear_groups','linear482']),('curve queries',['msm_affine','zopro_core::msm','cpu_msms','zopro_core::pippenger']),('row offsets',['wire_offsets','center_block','wire_offset'])]:
   if any(t in name for t in terms):return key
 return 'other/unresolved'
for r,stacks in zip(rows,stacksets):
 fs=Counter();ls=Counter();packed=0
 for st in stacks:
  frames=[lookup[f] for f in st['frames']];n=st['samples']
  fs[family(frames)]+=n;ls[frames[0]]+=n
  if 'input_columns::pack_inverse_columns' in frames[0]:packed+=n
 assert fs.total()==r['stacks']
 r.update(families=fs,leaves=ls,pack_leaf=packed)
pack(a/'samples.json.gz',rows)
table=[]
for sh in ['12x2','24x2']:
 for m in ['main','idle','product']:
  g=[r for r in rows if r['sh']==sh and r['m']==m];fs=sum((Counter(r['families']) for r in g),Counter());label,n=fs.most_common(1)[0];p=sum(r['pack_leaf'] for r in g);t=fs.total()
  table.append(f"| {sh} | {m} | {median(r['summary']['minor_faults']/11 for r in g):,.0f} | {t:,} | {label} ({100*n/t:.1f}%) | {p:,} ({100*p/t:.1f}%) |")
s='''# Sampled minor faults during complete proofs

Moving 24×2 product evaluation to 28 workers concentrated warm page faults in the first inverse FFT layer. The last column counts samples whose faulting instruction was inside the input-packing function itself. It writes the first transformed columns into a newly allocated output vector. The samples identify the instruction that touched the page. They do not establish why the allocator supplied that page.

Each cell includes three shuffled processes of eleven warm calls. The family column groups stacks by the nearest recognized prover operation after Rust symbol demangling. The median process fault count and the number of sampled stacks have different denominators.

| Transfer | Placement | Median minor faults per proof | Sampled stacks | Most frequent family | Direct pack_inverse_columns leaf |
|---|---|---:|---:|---|---:|
'''+ '\n'.join(table)+'\n\nAll 18 exports verified. [Protocol](protocol.md) · [Per-process leaf and family counts](artifacts/samples.json.gz) · [Uninstrumented latency and peak memory](../2026-11-19-product-pool607/README.md)\n'
(a.parent/'README.md').write_text(s)
print(s)
