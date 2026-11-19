import json
import pathlib
import random
p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
R = 1 << 260
mask = (1 << 52) - 1
rinv = pow(R, -1, p)
negpinv = (-pow(p, -1, R)) % R
def redc(x, w):
    t = x * w
    z = (t + ((t * negpinv) % R) * p) // R
    assert 0 <= z < 2 * p
    return z

def four(a, b, c, d, w):
    carry = 0
    signed = 0
    total = 0
    delta = 0
    for j in range(5):
        off = 52 * j
        u = ((a >> off) & mask) + ((b >> off) & mask)
        v = ((c >> off) & mask) + ((d >> off) & mask)
        sx = u + v + carry
        sy = u - v + (((4 * p) >> off) & mask) + signed
        total |= (sx & mask) << off
        delta |= (sy & mask) << off
        carry = sx >> 52
        signed = sy >> 52
    assert carry == signed == 0
    assert total == a+b+c+d and delta == 4*p+a+b-c-d
    if total >= 4*p: total -= 4*p
    if total >= 2*p: total -= 2*p
    return total, redc(delta, w)

edges=[0,1,p-1,p,p+1,2*p-2,2*p-1]
rng=random.Random(548)
for _ in range(2000):
    edge=lambda:rng.choice(edges) if rng.randrange(2) else rng.randrange(2*p)
    a,b,c,d,w=(edge() for _ in range(5))
    lo,hi=four(a,b,c,d,w)
    assert lo % p == (a+b+c+d) % p and hi % p == (a+b-c-d)*w*rinv%p
for k in range(64):
    n=2**(8+k%12);q=n//32;col=rng.randrange(q)
    omega=pow(5,-((p-1)//n),p)
    assert pow(omega,n,p)==1 and pow(omega,n//2,p)!=1
    def root(stage,j): return pow(omega,2**stage*(j*q+col),p)*R%p
    values=[rng.choice(edges) if k%2 else rng.randrange(2*p) for _ in range(32)]
    expected=values[:]
    for stage in range(5):
        gap=16>>stage
        for block in range(0,32,2*gap):
            for j in range(gap):
                i=block+j;a,b=expected[i],expected[i+gap]
                expected[i]=(a+b)%p
                expected[i+gap]=(a-b)*root(stage,j)*rinv%p
    for stage in [0,2]:
        gap=16>>stage;half=gap//2
        for block in range(0,32,2*gap):
            for j in range(half):
                i=block+j;a,b,c,d=(values[i+x] for x in [0,gap,half,gap+half])
                w=root(stage+1,j)
                values[i],values[i+half]=four(a,b,c,d,w)
                left=redc((a-b)%(2*p),root(stage,j));right=redc((c-d)%(2*p),root(stage,j+half))
                values[i+gap]=(left+right)%(2*p);values[i+half+gap]=redc((left-right)%(2*p),w)
    for j in range(0,32,2):
        a,b=values[j:j+2];values[j]=(a+b)%(2*p);values[j+1]=redc((a-b)%(2*p),root(4,0))
    assert [x%p for x in values]==expected
pathlib.Path(__file__).with_name('column-model.json').write_text(json.dumps(dict(independent_columns=64,column_outputs=2048,carry_stress_cases=2000,all_equal=True))+'\n')
