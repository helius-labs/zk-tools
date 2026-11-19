from collections import Counter
from zp_glv513 import cost, P
from zp_cond534 import stable

def learn(vectors):
    origins=[]
    selected=[]
    for i,column in enumerate(zip(*vectors)):
        origin=stable(column)
        origins.append(origin)
        counts=Counter(x for x in column if x!=origin)
        if counts:
            second,n=counts.most_common(1)[0]
            work=cost((second-origin)%P)
            if n>=2 and work>=3:
                selected.append((i,second,(second-origin)%P,work))
    return origins,selected

def evaluate(v,origins,selected):
    ordinary=sum(cost((x-o)%P) for x,o in zip(v,origins))
    gross=matches=0
    for i,second,d,work in selected:
        x=v[i];origin=origins[i]
        if x==second:
            assert (d+origin)%P==x and cost(d)==work
            matches+=1;gross+=work
        else:
            assert ((x-origin)%P+origin)%P==x
    return dict(ordinary_digits=ordinary,match_count=matches,gross_avoided=gross,net_avoided=gross-matches)

def control():
    tr=[[0,13,0],[0,13,0],[P//3,P//2,0],[P//3,P//2,0]]
    o,records=learn(tr)
    assert records and evaluate([P//3,P//2,0],o,records)['match_count']>0
    assert evaluate([0,1,0],o,records)['match_count']==0
