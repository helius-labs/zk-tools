import argparse
from array import array
from collections import Counter
import hashlib
import json
import mmap
from pathlib import Path
import struct
import sys

def sha(path):
    with path.open("rb") as f:
        return hashlib.file_digest(f, "sha256").hexdigest()

def scan(image,c):
 assert sys.byteorder=='little' and sha(image)==c['image_sha256']
 file=image.open('rb');m=mmap.mmap(file.fileno(),0,access=mmap.ACCESS_READ)
 start=m.rfind(b'ZSP1')
 while start>=8 and (struct.unpack_from('<Q',m,start-8)[0]!=len(m)-start or struct.unpack_from('<I',m,start+4)[0]!=2):
  start=m.rfind(b'ZSP1',0,start)
 assert start>=8
 head=struct.unpack_from('<13I',m,start);magic,version,nw,nin,np,nc,na,nb,ncoef,nt,ns,nslot,nops=head
 assert magic==0x3150535a and version==2
 v=memoryview(m)[start:].cast('I');aoff=13;boff=aoff+na;coff=boff+nb;toff=coff+8*ncoef;soff=toff+2*nt;pos=soff+3*ns
 bidx=set(v[boff:coff]);assert len(bidx)==nb and max(bidx)<nw
 coef=[int.from_bytes(m[start+4*coff+32*i:start+4*coff+32*(i+1)],'little') for i in range(ncoef)];prime=coef[1]+coef[3];inv=pow(coef[1],-1,prime);coef=[v*inv%prime for v in coef];assert coef[:5]==[0,1,2,prime-1,prime-2]
 slot_index=array('I',[0xffffffff])*nslot;slot_const=[None]*nslot
 bools=set();hints=[];linear=[];af=Counter();kind=Counter();rows_seen=0
 def terms_for(tag,a,b):
  if tag==0:return []
  if tag==1:return [(a,1)]
  if tag==2:return [(0,coef[a])]
  lo,hi=(a,a+1) if tag==3 else (a,b);assert tag in (3,4)
  return [(v[toff+2*j+1],coef[v[toff+2*j]]) for j in range(lo,hi)]
 def constant(tag,a,b):
  if tag==1:return 1 if a==0 else None if a<nw else slot_const[a-nw]
  if tag==0:return 0
  if tag==2:return coef[a]
  lo,hi=(a,a+1) if tag==3 else (a,b);value=0
  for j in range(lo,hi):
   coeff=v[toff+2*j]
   if v[toff+2*j+1] and coef[coeff]:return None
   value+=coef[coeff]
  return value%prime
 def expand(termvec):
  result=Counter()
  for w,coef0 in termvec:
   if w<nw:result[w]+=coef0;continue
   sj=slot_index[w-nw];assert sj!=0xffffffff
   while True:
    k=soff+3*sj;prev,ci,wire=v[k],v[k+1],v[k+2];assert wire<nw
    result[wire]+=coef0*coef[ci];prev&=0x7fffffff
    if prev==0x7fffffff:break
    if prev==0x7ffffffe:sj-=1
    else:sj=slot_index[prev-nw]
    assert 0<=sj<ns
  return {w:x%prime for w,x in result.items() if x%prime}
 for _ in range(nops):
  flag=v[pos]
  if flag:
   assert flag==1;_,hint,lo,hi,first,last=v[pos:pos+6];pos+=6
   if hint==1:
    bools.update(range(first,last));hints.append((lo,hi,first,last))
   continue
  rows_seen+=1;_,row,lo,hi,slot=v[pos:pos+5]
  assert row<nc and hi<=ns;chain=0
  for j in range(lo,hi):
   k=soff+3*j;pr,ci,w=v[k],v[k+1],v[k+2];prev=pr&0x7fffffff
   cv=coef[ci] if w==0 or coef[ci]==0 else None
   if prev==0x7fffffff:chain=cv
   else:
    prevc=chain if prev==0x7ffffffe else slot_const[prev-nw]
    chain=None if prevc is None or cv is None else (prevc+cv)%prime
   if pr>>31:
    assert slot<nslot and slot_index[slot]==0xffffffff;slot_index[slot]=j;slot_const[slot]=chain;slot+=1
  expr=[tuple(v[pos+i:pos+i+3]) for i in (5,8,11)];assigned=v[pos+14];assert assigned in (0,1)
  side,wire,ac=v[pos+15:pos+18] if assigned else (3,0,0);pos+=26 if assigned else 15
  ca=None if side==0 else constant(*expr[0]);cb=None if side==1 else constant(*expr[1])
  if ca is None and cb is None:continue
  hs=[terms_for(*e) for e in expr]
  if assigned:hs[side].append((wire,coef[ac]))
  source=hs[1] if ca is not None else hs[0];scale=ca if ca is not None else cb
  relation=expand([(w,scale*k) for w,k in source]+[(w,-k) for w,k in hs[2]])
  if relation:linear.append(relation);af['assigned' if assigned else 'checked']+=1
  else:af['tautology']+=1
 assert pos==len(v) and rows_seen==nc and 0xffffffff not in slot_index
 touched=set();queries=set();nonbool_rows=0;span_hist=Counter();bool_only=0
 for rel in linear:
  keys=set(rel)-{0};bits=keys.intersection(bools);q=keys.intersection(bidx);touched.update(q);queries.update(q-bools);nonbool_rows+=bool(q-bools);bool_only+=bool(keys) and keys==bits;span_hist[len(keys)]+=1
 # Each hint reconstructs low bits. Without an independent range proof only a
 # 254+-bit hint covers the entire canonical field element on every witness.
 hintstats=Counter()
 for lo,hi,first,last in hints:
  termlist=terms_for(4,lo,hi);terms=expand(termlist);keys=set(terms)-{0};width=last-first
  hintstats['total']+=1;hintstats['full_width']+=int(width>=prime.bit_length());hintstats['one_input_wire']+=int(len(keys)==1);hintstats['one_queried_nonboolean_input']+=int(len(keys)==1 and bool((keys&bidx)-bools));hintstats['full_width_one_queried_nonboolean_input']+=int(width>=prime.bit_length() and len(keys)==1 and bool((keys&bidx)-bools))
 res=dict(shape=c['shape'],image_sha256=c['image_sha256'],rows=nc,ops=nops,b_query=nb,boolean_b_query=len(bidx&bools),linear=len(linear),forms=dict(af),rows_touching_nonboolean_query=nonbool_rows,distinct_nonboolean_query_wires_touched=len(queries),direct_pivot_upper=min(nonbool_rows,len(queries)),distinct_query_wires_touched=len(touched),boolean_only_rows=bool_only,linear_width_histogram=dict(span_hist),bits_hints=dict(hintstats))
 del v;m.close();file.close();return res
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Count direct linear rows in a Zopro CPU key image")
    parser.add_argument("image", type=Path)
    parser.add_argument("shape")
    parser.add_argument("image_sha256")
    args = parser.parse_args()
    print(json.dumps(scan(args.image, vars(args)), indent=2))
