"""Recompute the published fault classification from the saved user-space stacks."""
from collections import Counter
import gzip
import json
from pathlib import Path
import re
import subprocess

directory = Path(__file__).resolve().parent
def read(name):
    with gzip.open(directory / name, "rt") as stream:
        return json.load(stream)

rows = read("samples.json.gz")
captures = read("stacks.json.gz")
assert [(r["b"], r["sh"], r["m"]) for r in rows] == [
    (c["block"], c["shape"], c["mode"]) for c in captures
]
stacksets = [capture["stacks"] for capture in captures]
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
 assert fs == r["families"] and ls == r["leaves"] and packed == r["pack_leaf"]
print(f"Verified {len(rows)} captures and {sum(r['stacks'] for r in rows):,} sampled stacks.")
