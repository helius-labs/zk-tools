from zp_column_probe552 import scan
from zp_report_common import base,save
from collections import Counter
from functools import lru_cache
import sys
def rank(sh):
 p,core,rev,removed=scan(sh);prime=p.prime
 inv=lru_cache(maxsize=8192)(lambda x:pow(x,-1,prime))
 piv={}
 for row in sorted(core,key=len):
  while row:
   c=max(row)
   scale=row[c]
   if c not in piv:
    q=inv(scale);piv[c]={w:x*q%prime for w,x in row.items() if w!=c};break
   del row[c]
   for w,x in piv[c].items():
    a=(row.get(w,0)-x*scale)%prime
    if a:row[w]=a
    else:row.pop(w,None)
 free=rev.keys()-piv.keys();print('rank',removed+len(piv),'deficiency',len(free),'echelon nnz',sum(map(len,piv.values())),flush=True)
 # The free basis vectors propagate into pivot columns in ascending wire order.
 expr={w:{w:1} for w in free};relations={w:{} for w in free}
 for c,r in sorted(piv.items()):
  acc=Counter()
  for w,x in r.items():
   for j,a in expr.get(w,{}).items():acc[j]-=a*x
  acc={j:a%prime for j,a in acc.items() if a%prime};expr[c]=acc
  for j,a in acc.items():relations[j][c]=(-a)%prime
 result=dict(sh=sh,columns=len(p.b2),peeled=removed,rank=removed+len(piv),free=len(free),weights=dict(sorted(Counter(len(r) for r in relations.values()).items())))
 d=base/'b-column-rank552';d.mkdir(exist_ok=True);save(d/(sh+'-relations.json'),relations);save(d/(sh+'.json'),result)
 print(result,flush=True);p.close()
if __name__=='__main__':rank(sys.argv[1])
