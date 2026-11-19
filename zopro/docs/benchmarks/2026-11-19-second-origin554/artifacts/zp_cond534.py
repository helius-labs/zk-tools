"""Select one zero/nonzero wire predicate using only public training fixtures."""
from collections import Counter
from zp_glv513 import P,cost

def stable(xs):
    if len(set(xs))==1:return xs[0]
    v,n=Counter(xs).most_common(1)[0]
    if not v or 4*n<3*len(xs):return 0
    return v if sum(cost(x)-cost((x-v)%P) for x in xs)>=2*len(xs) else 0

def feature_masks(tr):
    n=len(tr);full=(1<<n)-1;lookup={}
    for index,col in enumerate(zip(*tr)):
        mask=sum(1<<t for t,v in enumerate(col) if v)
        if not 2<=mask.bit_count()<=n-2:continue
        bare=min(mask,full^mask)
        item=(any(v not in (0,1) for v in col),index,mask)
        if bare not in lookup or item<lookup[bare]:lookup[bare]=item
    return list(lookup.values())

def alternatives(col,origin,mask):
    return tuple(next(iter(values)) if len(values)==1 else origin for values in
                 ({x for t,x in enumerate(col) if ((mask>>t)&1)==branch} for branch in (0,1)))

def fit(tr,stride):
    n=len(tr);ns=len(tr[0]);assert all(len(x)==ns for x in tr)
    features=feature_masks(tr)
    if not features:return None,0,0
    sampled=[(xs,stable(xs)) for i in range(0,ns,stride) if len(set(xs:=tuple(r[i] for r in tr)))>1]
    def score(f):
        mask=f[2];saved=records=0
        for col,base in sampled:
            z,nz=alternatives(col,base,mask)
            if z==nz==base:continue
            records+=int(z!=base)+int(nz!=base)
            for t,v in enumerate(col):
                pred=nz if (mask>>t)&1 else z
                if pred!=base:
                    assert v==pred
                    saved+=cost((v-base)%P)
        return (saved,-records,-f[1])
    best=max(features,key=score)
    if score(best)[0]==0:return None,len(features),0
    return best,len(features),score(best)[0]

def train_plan(tr,stride):
    chosen,nfeatures,saved=fit(tr,stride);std=[];changes=[]
    for i,col in enumerate(zip(*tr)):
        base=stable(col);std.append(base)
        if chosen and len(set(col))>1:
            z,nz=alternatives(col,base,chosen[2])
            if (z,nz)!=(base,base):changes.append((i,z,nz))
    return chosen,std,changes,nfeatures,saved

def apply(values,chosen,origins,changes):
    assert len(values)==len(origins)
    base=sum(cost((x-origin)%P) for x,origin in zip(values,origins))
    selected=bool(chosen and values[chosen[1]]);delta=uses=exact=0
    for i,z,nz in changes:
        new=nz if selected else z
        if new==origins[i]:continue
        residual=(values[i]-new)%P
        assert (new+residual)%P==values[i]
        uses+=1;exact+=residual==0
        delta+=cost(residual)-cost((values[i]-origins[i])%P)
    return dict(baseline=base,conditional=base+delta,branch=int(selected),changed=uses,zero_residuals=exact)

def control():
    z=2**180+12345;y=2**200+913
    tr=[[b, z if b==0 else y, 1] for b in [0,2,0,3,4,0]]
    c,orig,changes,_,_=train_plan(tr,1);assert c and c[1]==0 and changes
    for bit,value in [(0,z),(9,y)]:
        result=apply([bit,value,1],c,orig,changes);assert result['conditional']<result['baseline'] and result['zero_residuals']==1
    assert apply([0,79,1],c,orig,changes)['zero_residuals']==0
    assert fit([[1,2]]*5,1)[0] is None
