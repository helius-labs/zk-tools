"""Train bounded, fixed signed pairs on one panel and score on another."""
from collections import defaultdict
from pathlib import Path
import json,sys
from array import array
from zp_glv513 import P,split,signed_digits
def panel(paths):
    blobs=[p.read_bytes() for p in paths];assert len({len(b) for b in blobs})==1 and len(blobs[0])%32==0
    vals=[[int.from_bytes(b[i:i+32],'little') for i in range(0,len(b),32)] for b in blobs]
    assert all(max(x)<P for x in vals)
    return vals
def digits(x):
    parts=split(x)
    ds=[list(signed_digits(q)) for q in parts]
    assert all(sum(v<<(12*i) for i,v in enumerate(row))==part for row,part in zip(ds,parts))
    return ds[0]+ds[1]
def cube(values):
    from functools import lru_cache
    cached=lru_cache(32768)(digits)
    return [array('h',(d for x in row for d in cached(x))) for row in values]
def pairs(matrix, eligible, width=22):
    edges=set()
    for j in range(width):
        pools=defaultdict(list)
        for i in eligible:
            offset=i*width+j
            col=tuple(row[offset] for row in matrix)
            if sum(bool(v) for v in col)<=len(matrix)//2:continue
            sign=1 if next(v for v in col if v)>0 else -1
            key=col if sign==1 else tuple(-v for v in col)
            group=pools[key]
            for partner,other in group:edges.add((partner,i,sign*other))
            group.append((i,sign))
            if len(group)>4:group.pop(0)
    proposals=[]
    for x,y,s in sorted(edges):
        saved=sum(1 for row in matrix for j in range(width)
                  if row[width*x+j] and row[width*x+j]==s*row[width*y+j])
        if saved:proposals.append((-saved,x,y,s))
    used=set();chosen=[]
    for _,x,y,s in sorted(proposals):
        if x in used or y in used:continue
        used.update([x,y]);chosen.append((x,y,s))
    return chosen,len(edges)
def score(matrix,values,chosen,stable):
    result=[]
    for fields,rows in zip(matrix,values):
        total=sum(bool(x) for x in fields)
        stats=[0,0,0];partial=0
        for a,b,s in chosen:
            n=sum(1 for j in range(22) if fields[a*22+j] and fields[a*22+j]==s*fields[b*22+j])
            if (rows[a]-s*rows[b])%P==0:
                stats[2]+=n;continue
            stats[0]+=n;stats[1]+=bool(n);partial+=n>1
        fixed=sum(sum(bool(v) for v in fields[i*22:i*22+22]) for i in stable if rows[i]==stable[i])
        result.append(dict(ordinary=total,saved=stats[0],matching_pairs=stats[1],multi_window_pairs=partial,already_whole=stats[2],after_fixed=total-fixed))
    return result
def main(train,held):
    v=panel(train);seen=set();eligible=[];stable={}
    for i,key in enumerate(zip(*v)):
        if len(set(key))==1 and key[0]:stable[i]=key[0]
        if len(set(key))<3:continue
        neg=tuple((-x)%P for x in key);canonical=min(key,neg)
        if canonical in seen:continue
        seen.add(canonical);eligible.append(i)
    t=cube(v);chosen,edges=pairs(t,eligible);w=panel(held)
    return dict(counts=dict(eligible=len(eligible),candidate_edges=edges,selected=len(chosen),retained_affine_bytes=128*len(chosen)),held=score(cube(w),w,chosen,stable))
if __name__=='__main__':
    fixture=[[2,1,3,2,4,5,-3,6,9],[3,5,7,3,9,7,-4,7,8],[6,2,1,6,3,3,-5,3,8]]
    assert pairs(fixture,[0,1,2],3)[0]==[(0,1,1)]
    for row in fixture:row[3:6]=[-x for x in row[3:6]]
    assert pairs(fixture,[0,1,2],3)[0]==[(0,1,-1)]
    n=int(sys.argv[1]);paths=list(map(Path,sys.argv[2:]));print(json.dumps(main(paths[:n],paths[n:])))
