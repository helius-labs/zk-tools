from zp_plan476 import Plan
from zp_report_common import base,cases
from collections import defaultdict,Counter,deque
def scan(sh):
 c=cases[sh];p=Plan(base/'epoch-team357/bundle'/c['image'],c['image_sha256'])
 rev=defaultdict(list);rows={};missing=set()
 for pos in p.rows():
  row=p.words[pos+1];terms=p.form(pos,1)
  missing.update(terms.keys()-p.b2)
  terms={w:a for w,a in terms.items() if w in p.b2}
  if terms:
   rows[row]=terms
   for w in terms:rev[w].append(row)
 n=len(p.b2);print(sh,'key',n,'rows',len(rows),'cols',len(rev),'nnz',sum(map(len,rev.values())),'excluded',len(missing),flush=True)
 from collections import Counter
 qs=deque(i for i,r in rows.items() if len(r)==1);removed=0
 while qs:
  i=qs.popleft()
  if len(rows[i])!=1:continue
  w=next(iter(rows[i]));removed+=1
  for j in rev.pop(w):
   del rows[j][w]
   if len(rows[j])==1:qs.append(j)
 core=[r for r in rows.values() if r];print('peeled',removed,'remaining',len(rev),'rows',len(core),'nnz',sum(map(len,core)), 'maxrow',max(map(len,core),default=0),flush=True)
 return p,core,rev,removed
if __name__=='__main__':scan('1x2')
