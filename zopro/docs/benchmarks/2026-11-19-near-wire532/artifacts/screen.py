"""Search bounded pairs in public training vectors and score exact residuals."""
from collections import Counter
from pathlib import Path
from zp_glv513 import P,LAMBDA,cost
import json,sys

def signed(x):
    x=x%P
    return x if 2*x<=P else x-P

def select(cols, bits=(48,96,128), cap=6):
    eligible=[i for i,col in enumerate(cols) if len(set(col))>=3]
    slopes=(1,-1,LAMBDA,-LAMBDA)
    deltas={i:tuple((v-cols[i][0])%P for v in cols[i][1:]) for i in eligible}
    costs={i:sum(map(cost,cols[i])) for i in eligible}
    edges={};tested=set()
    for nbits in bits:
        for shift in (0,1<<(nbits-1)):
            def key(values):
                return tuple(((v+shift)%P)>>nbits for v in values)
            targets={}
            for j in eligible:
                bucket=targets.setdefault(key(deltas[j]),[])
                if len(bucket)<cap:bucket.append(j)
            for i in eligible:
                for slope in slopes:
                    sig=deltas[i] if slope==1 else tuple((slope*v)%P for v in deltas[i])
                    for j in targets.get(key(sig),()):
                        if j==i or (i,j,slope) in tested:continue
                        tested.add((i,j,slope))
                        origin=(cols[j][0]-slope*cols[i][0])%P
                        ds=[signed(y-slope*x-origin) for x,y in zip(cols[i],cols[j])]
                        if not any(ds) or max(map(abs,ds)) >= 1<<nbits:continue
                        for offset in dict.fromkeys([0,origin]):
                            errors=[(y-slope*x-offset)%P for x,y in zip(cols[i],cols[j])]
                            if not all(v==0 or abs(signed(v)).bit_length()<=nbits for v in errors):continue
                            gain=costs[j]-sum(map(cost,errors))
                            if gain<2*len(errors):continue
                            edge=(gain,i,j,slope,offset)
                            k=tuple(sorted((i,j)))
                            if k not in edges or edges[k][0]<gain:edges[k]=edge
    used=set();plan=[]
    for _,i,j,slope,offset in sorted(edges.values(),key=lambda e:(-e[0],e[1],e[2],e[3])):
        if i in used or j in used:continue
        used.update((i,j));plan.append((i,j,slope,offset))
    return plan,dict(eligible=len(eligible),distinct_candidates=len(tested),profitable_edges=len(edges),disjoint=len(plan))

def score(values,plan):
    ordinary=sum(map(cost,values));delta=0;small=0;exact=0
    for i,j,slope,offset in plan:
        residual=(values[j]-slope*values[i]-offset)%P
        assert (slope*values[i]+offset+residual)%P==values[j]
        delta+=cost(residual)-cost(values[j]);small+=abs(signed(residual)).bit_length()<=128;exact+=residual==0
    return dict(ordinary=ordinary,paired=ordinary+delta,narrow=small,exact=exact)

def read(path):
    data=path.read_bytes();assert len(data)%32==0
    result=[int.from_bytes(data[i:i+32],'little') for i in range(0,len(data),32)]
    assert all(x<P for x in result)
    return result

def control():
    src=[(i**10*564731996614218372893+t**9*7037289217)%P for i,t in zip(range(7109,7115),[5010,6109,33999,78781,73020,199103])]
    a=[(LAMBDA*x+245679123456789123456789+(j+1)**4)%P for j,x in enumerate(src)]
    b=[(2*x+3)%P for x in src]
    plan,_=select([src,a,b]);assert plan, 'failed synthetic near-endomorphism control'
    for h in range(len(src)):
        z=score([src[h],a[h],b[h]],plan);assert z['paired']<z['ordinary']
    exact,_=select([src,[(LAMBDA*x+5)%P for x in src]])
    assert not exact, 'exact affine family was already studied'
if __name__=='__main__':
    control();t=int(sys.argv[1]);paths=list(map(Path,sys.argv[2:]));train=list(map(read,paths[:t]))
    # Subtract whole-row offsets fixed on training requests only.
    offsets=[c[0] if len(set(c))==1 else 0 for c in zip(*train)]
    cols=[tuple((v-off)%P for v in c) for off,c in zip(offsets,zip(*train))]
    plan,info=select(cols);stats=[]
    for path in paths[t:]:
        row=read(path);assert len(row)==len(offsets)
        stats.append(score([(v-off)%P for v,off in zip(row,offsets)],plan))
    print(json.dumps(dict(counts=info,slopes=Counter('signed' if z[2] in (1,-1) else 'endomorphism' for z in plan),held=stats)))
