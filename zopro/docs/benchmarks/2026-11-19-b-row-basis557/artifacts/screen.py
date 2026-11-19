from collections import Counter,defaultdict
from functools import lru_cache
from zp_cond534 import stable
from zp_glv513 import P,cost
from zp_plan476 import Plan
inv=lru_cache(maxsize=8192)(lambda v:pow(v,-1,P))

def candidate_rows(plan):
    na,nb=plan.words[6:8]
    wire_ids=list(plan.words[13+na:13+na+nb])
    ix={w:i for i,w in enumerate(wire_ids)}
    hist=Counter()
    rows=[]
    for pos in plan.rows():
        form=plan.form(pos,1)
        assert not form.keys()-ix.keys()
        ids={ix[w]:c for w,c in form.items()}
        hist.update(ids.keys())
        if len(ids)>1:rows.append((plan.words[pos+1],ids))
    return [(r,form,[i for i in form if hist[i]==1]) for r,form in rows if any(hist[i]==1 for i in form)]

def lin(form,values):
    return sum(values[i]*coef for i,coef in form.items())%P

def select(rows,train):
    origins=list(map(stable,zip(*train)))
    residuals=[[(x-o)%P for x,o in zip(v,origins)] for v in train]
    selected=[]
    for r,form,pivots in rows:
        evaluated=[lin(form,v) for v in residuals]
        cost0=sum(map(cost,evaluated))
        best=None
        for i in pivots:
            scale=inv(form[i])
            original=sum(cost(v[i]) for v in residuals)
            for normalized in [False,True] if scale!=1 else [False]:
                factor=scale if normalized else 1
                transformed=sum(cost(t*factor%P) for t in evaluated) if normalized else cost0
                item=(original-transformed,-i,normalized,factor)
                if best is None or item>best:best=item
        if best[0]>=2*len(train):
            gain,j,_,factor=best;selected.append((r,form,-j,factor))
    return origins,selected

def apply(values,origins,selected,labels):
    xs=[(v-o)%P for v,o in zip(values,origins)]
    baseline=sum(map(cost,xs));delta=0;saved=0
    adjusted=labels[:];after=xs[:]
    for _,form,i,factor in selected:
        value=lin(form,xs)*factor%P;after[i]=value
        delta+=cost(xs[i])-cost(value);saved+=value==0
        # z_i = f * sum(c_j x_j). New labels satisfy the equality of dot products.
        pivot=labels[i]*inv(form[i]*factor%P)%P
        adjusted[i]=pivot
        for j,c in form.items():
            if j!=i:adjusted[j]=(adjusted[j]-c*factor*pivot)%P
    assert sum(x*p for x,p in zip(xs,labels))%P==sum(x*p for x,p in zip(after,adjusted))%P
    return dict(baseline=baseline,net_saved=delta,zero_transformed=saved)

def control():
    # Two independent pivot wires share a third base which must be corrected twice.
    selected=[(0,{0:3,2:7},0,inv(3)),(1,{1:9,2:5},1,1)]
    for labels in [[3,0,7],[1,9,13]]:
        for xs in [[0,1,2],[P-1,7,P-3]]:
            apply(xs,[0,3,2],selected,labels)
