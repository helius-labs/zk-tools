from pathlib import Path
from collections import defaultdict, Counter
from statistics import mean
from hashlib import sha256
import json, re
base=Path('../general-width279'); root=base/'remote/output';rs=json.loads((base/'host-summary.json').read_bytes())['rows']
def verified(folder):
 data=(folder/'stdout.log').read_bytes();v=json.loads((folder/'exit.json').read_bytes());assert v['success'] and sha256(data).hexdigest()==v['stdout_sha256'];return data.decode()
profiles=[]
for r in rs:
 j=r['job']
 if j['mode']!='trace' or j['pre']:continue
 name=j['name'];b=j['block'];s=verified(root/f'b{b}-review/decode-{name}');assert 'zero-lost' in verified(root/f'b{b}-review/loss-{name}')
 calls=[];current=defaultdict(list);req=[];sdk=None;slide=None
 for line in s.splitlines():
  f=line.split();tag,action=f[4].split(':')[1].rsplit('_',1);thread=f[0];t=int(f[2][:-1].replace('.',''));addr=int(f[5].strip('()'),16)
  if tag=='sdk':
   if action=='enter':assert sdk is None and not calls;sdk=t
   else:
    assert not any(current.values()) and len(calls)==4;calls.sort(key=lambda v:v[2]);assert Counter(x[0] for x in calls)=={'H':1,'grouped G1':2,'G2':1}
    end=calls[-1][2];assert sdk<=min(x[1] for x in calls) and t>=end
    rows={k:sorted([v for v in calls if v[0]==k],key=lambda v:v[2])[-1] for k in ['H','grouped G1','G2']}
    req.append(dict(last=calls[-1][0],lead_ms=(end-calls[-2][2])/1e6,h_duration_ms=(rows['H'][2]-rows['H'][1])/1e6,h_after_other_ms=(rows['H'][2]-max(rows[k][2] for k in ['grouped G1','G2']))/1e6,raw_end_to_sdk_ms=(t-end)/1e6,intervals=[[v[0],(v[1]-sdk)/1e6,(v[2]-sdk)/1e6] for v in calls]));sdk=None;calls=[]
  elif tag in ('g1raw','g2raw'):
   if action=='enter':
    if tag=='g1raw':
     load=addr-0x1d33d0
     if slide is None:slide=load
     assert slide==load
    current[thread].append((tag,addr,t))
   else:
    ot,ip,start=current[thread].pop();assert ot==tag and ip==addr;assert slide is not None
    ret=int(f[7].strip('()'),16)-slide
    if tag=='g1raw':k={0x4ae364:'H',0x1d3315:'grouped G1'}[ret]
    else:k='G2'
    calls.append((k,start,t))
 assert len(req)==31
 profiles.append(dict(shape=j['shape'],workers=j['w'],block=b,samples=req))
for sh in ['merge-24x1','16x2']:
 for w in [48,64]:
  vs=[v for r in profiles if (r['shape'],r['workers'])==(sh,w) for v in r['samples']];assert len(vs)==93
  print(sh,w,Counter(v['last'] for v in vs),round(mean(max(v['h_after_other_ms'],0) for v in vs),3),round(mean(v['h_duration_ms'] for v in vs),3))
Path('profiles.json').write_text(json.dumps(profiles))
