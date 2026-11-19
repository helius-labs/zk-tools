"""Screen equivalent BN254 GLV splits with eleven signed radix-4096 windows."""
from collections import Counter
from functools import lru_cache
from math import ceil
import random
import sys
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parent))
from zp_glv513 import P,LAMBDA,split
A=9931322734385697763
B=147946756881789319000765030803803410728
C=147946756881789319010696353538189108491
assert A*A+B*C==P and (A-LAMBDA*B)%P==0 and (C+LAMBDA*A)%P==0
LIMITS=(126,127,128,130)

def digits(x):
    mag=abs(x);sgn=1 if x>=0 else -1
    out=[]
    for k in range(11):
        t=((mag<<1)>>(12*k))&8191
        out.append(sgn*(((t+1)>>1)-((t>>12)<<12)))
    return out

def count(x):
    mag=abs(x)<<1
    return sum(((mag>>i)&8191) not in (0,8191) for i in range(0,132,12))

def head(x):
    m=abs(x)<<1
    t=m>>120&8191
    return abs(((t+1)>>1)-((t>>12)<<12))

@lru_cache(maxsize=8192)
def best(x):
    lo,hi=split(x)
    ordinary=count(lo)+count(hi)
    start=(ordinary,max(head(lo),head(hi)),0,0)
    opts=[start]*len(LIMITS)
    accepted=[0]*len(LIMITS)
    cap=(1<<LIMITS[-1])+(1<<126)
    # Invert [[A,C],[-B,A]]. Both coordinate changes have magnitude < cap.
    ni=cap*(A+C)//P
    nj=cap*(A+B)//P
    for i in range(-ni,ni+1):
        for j in range(-nj,nj+1):
            u=lo+i*A+j*C;v=hi-i*B+j*A
            bits=max(abs(u),abs(v)).bit_length()
            if bits>LIMITS[-1]:continue
            item=(count(u)+count(v),max(head(u),head(v)),i,j)
            for k,b in enumerate(LIMITS):
                if bits<=b:
                    accepted[k]+=1
                    if item[:2]<opts[k][:2]:opts[k]=item
    for k,(op,top,i,j) in enumerate(opts):
        u=lo+i*A+j*C;v=hi-i*B+j*A
        assert (u+LAMBDA*v)%P==x and max(abs(u),abs(v))<1<<LIMITS[k]
        du,dv=digits(u),digits(v)
        assert sum(d<<(12*j) for j,d in enumerate(du))==u and sum(d<<(12*j) for j,d in enumerate(dv))==v
        assert sum(z!=0 for z in du+dv)==op and max(abs(du[-1]),abs(dv[-1]))==top
    return ordinary,start[1],opts,accepted

def measure(values,seed,n=256):
    idx=random.Random(seed).sample(range(len(values)),min(len(values),n))
    totals=[[0]*5 for _ in LIMITS];baseline=0;max_top=0
    for i in idx:
        old,top,opts,accepted=best(values[i])
        baseline+=old;max_top=max(max_top,top)
        for row,(changes,peak,_,_),qty in zip(totals,opts,accepted):
            row[0]+=changes;row[1]=max(row[1],peak);row[2]+=old!=changes
            row[3]+=qty;row[4]=max(row[4],qty)
    return dict(sample=len(idx),baseline=baseline,base_top=max_top,
        limits={str(b):dict(digits=t[0],max_top=t[1],improved_rows=t[2],representatives=t[3],max_representatives=t[4]) for b,t in zip(LIMITS,totals)})

def control():
    for x in [0,1,P-1,LAMBDA,P//2,(1<<247)+35]:
        best(x)
    for x in [0,-2048,2048,4095,4096,(1<<126)-1,(1<<130)-1,1-(1<<130)]:
        ds=digits(x)
        assert count(x)==sum(z!=0 for z in ds) and sum(d<<(12*k) for k,d in enumerate(ds))==x
