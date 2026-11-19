"""Integer model for the correction factor computed from four raw top limbs."""
import itertools,json,random,pathlib
P=21888242871839275222246405745257275088548364400416034343698204186575808495617
B=1<<52;T=B**4;u=P//T;k=B//(u+1)
# Bound S/p - t*k/B by (t+4)/u - t/(u+1) + t/B, for t<8*(u+1).
# Check this strict upper bound is less than one using exact integers.
tmax=8*(u+1)
assert 4*(u+1)*B+tmax*B+tmax*u*(u+1) < u*(u+1)*B
def check(xs):
    t=sum(x//T for x in xs)
    q=t*k//B
    assert 0<=q<=7 and t<B and 0<=sum(xs)-q*P<2*P
    carry=0;out=0
    for j in range(5):
        sh=j*52;digit=carry+sum((x>>sh)&(B-1) for x in xs)-q*((P>>sh)&(B-1))
        assert -(1<<63)<=digit<1<<63
        out|=(digit&(B-1))<<sh;carry=digit>>52
    assert carry==0 and out==sum(xs)-q*P
    return q
def run():
    seen=set();count=0;edges=[0,1,T-1,T,P-1,P,P+1,2*P-2,2*P-1]
    for xs in itertools.product(edges,repeat=4):
        seen.add(check(xs));count+=1
    # Cross every attainable estimate transition with both zero and maximal low limbs.
    for q in range(1,8):
        threshold=(q*B+k-1)//k
        for t in range(threshold-3,threshold+4):
            highs=[t//4+(j<t%4) for j in range(4)]
            for tails in itertools.product([0,T-1],repeat=4):
                xs=[h*T+tail for h,tail in zip(highs,tails)]
                if max(xs)<2*P:seen.add(check(xs));count+=1
    rng=random.Random(555)
    for _ in range(12000):
        seen.add(check([rng.randrange(2*P) for _ in range(4)]));count+=1
    assert seen==set(range(8))
    return dict(cases=count,quotients=sorted(seen),reciprocal=k,top_prime_limb=u)
if __name__=='__main__':print(json.dumps(run()))
