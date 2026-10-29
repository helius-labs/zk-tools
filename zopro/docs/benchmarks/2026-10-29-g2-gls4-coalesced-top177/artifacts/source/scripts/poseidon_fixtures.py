#!/usr/bin/env python3
"""Generate dense-row test fixtures independently with Python field arithmetic."""
from pathlib import Path
import re, struct, sys
base=Path(__file__).resolve().parents[1]
P=21888242871839275222246405745257275088548364400416034343698204186575808495617;RINV=pow(1<<256,-1,P)
source=base/'crates/zopro-core/src/solver/sparse_poseidon/constants.rs';src=source.read_text();constants={}
for t,body in re.findall(r'static T(\d+):\[Fr;\d+\]=\[(.*?)\n\];',src,re.S):
 constants[int(t)]=[sum(int(x,16)<<(64*i)for i,x in enumerate(row.split(',')))*RINV%P for row in re.findall(r'Fr::from_montgomery\(\[([^\]]+)\]\)',body)]
partial={2:56,3:57,4:56,5:60,6:60,7:63,8:64,9:63,10:60,11:66,12:60,13:65}
def add(a,b):
 c=a.copy()
 for w,v in b.items():
  z=(c.get(w,0)+v)%P
  if z:c[w]=z
  else:c.pop(w,None)
 return c
def scale(a,k):return {w:z for w,v in a.items()if(z:=v*k%P)}
def sum_forms(items):
 out={}
 for x in items:out=add(out,x)
 return out
data=bytearray(struct.pack('<I',12));counts=[]
for t in range(2,14):
 a=constants[t];p=partial[t];n=7*t+p;count=8*t+p;mds=count;pre=mds+t*t;sparse=pre+t*t
 forms=[{n+1+i:1}for i in range(t)];state=[{1+i:1}for i in range(t)]
 def dense(offset):return [sum_forms(scale(state[j],a[offset+i*t+j])for j in range(t))for i in range(t)]
 def sbox(x):
  forms.append(x);return {len(forms):1}
 state=dense(mds)
 for round in [2,3]:
  state=[sbox(add(state[i],{0:a[round*t+i]}))for i in range(t)];state=dense(pre if round==3 else mds)
 c=4*t;x=add(state[0],{0:a[c]});c+=1
 for round in range(p):
  offset=sparse+round*(2*t-1);rest={0:a[c]}if round+1<p else{}
  if round+1<p:c+=1
  for j in range(1,t):rest=add(rest,scale(state[j],a[offset+j]))
  head=sbox(x)
  for i in range(1,t):state[i]=add(state[i],scale(head,a[offset+t+i-1]))
  x=add(scale(head,a[offset]),rest)
 state[0]=x
 for round in range(4):
  state=[sbox(add(state[i],{0:a[c+i]}))for i in range(t)];c+=t;state=dense(mds)
 assert len(forms)==n and c==count
 data+=struct.pack('<II',t,n)
 for row in forms:
  data+=struct.pack('<I',len(row))
  for symbol,value in sorted(row.items()):data+=struct.pack('<I',symbol)+value.to_bytes(32,'little')
 counts.append(dict(width=t,partial=p,sboxes=n,terms=sum(map(len,forms))))

destination=Path(sys.argv[1]) if len(sys.argv)>1 else base/'crates/zopro-core/src/solver/sparse_poseidon/all-widths.bin'
destination.write_bytes(data)
print(f'Wrote {len(data)} bytes for {len(counts)} widths to {destination}')
