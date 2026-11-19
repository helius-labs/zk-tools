"""Measure signed-window insertions and suffix sums for public BN254 coefficients."""
from pathlib import Path
import json
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "2026-11-19-trained-digits541/artifacts"))
from functools import lru_cache
from zp_glv513 import P, LAMBDA, split, signed_digits

@lru_cache(maxsize=131072)
def digits(x):
    result = tuple(signed_digits(x))
    assert sum(d << (12*j) for j,d in enumerate(result)) == x
    return result

def signed_min(x):
    return x if 2*x < P else x-P

def suffix_terms(terms, block):
    """Each term is (nonnegative scalar, model point), sorted by scalar."""
    work = 0
    output = []
    for i in range(0, len(terms), block):
        group = terms[i:i+block]
        suffix = 0
        for j in range(len(group)-1, -1, -1):
            scalar, point = group[j]
            suffix = (suffix + point) % P
            work += j != len(group)-1
            gap = scalar - (group[j-1][0] if j else 0)
            if gap:
                output.append((gap, suffix))
    return output, work

def cost(components, suffix_adds):
    occupied = [set() for _ in range(11)]
    arrivals = 0
    for scalar in components:
        for w,d in enumerate(digits(scalar)):
            if d:
                occupied[w].add(abs(d))
                arrivals += 1
    seeds = sum(map(len, occupied))
    return dict(arrivals=arrivals,seeds=seeds,suffix_adds=suffix_adds,
                adds=arrivals-seeds+suffix_adds)

def run(blob):
    assert len(blob)%32 == 0
    xs=[int.from_bytes(blob[i:i+32],'little') for i in range(0,len(blob),32)]
    assert xs and all(x<P for x in xs)
    scalars=sorted({abs(signed_min(x)) for x in xs if x})
    # Once exact and opposite scalars have been grouped, their aggregate points
    # are arbitrary. Assign each a deterministic point in the scalar model.
    terms=[(x, pow(17+i,17,P)) for i,x in enumerate(scalars)]
    expected=sum(x*w for x,w in terms)%P
    components=[[],[]]
    for x,w in terms:
        for j,k in enumerate(split(x)):
            if k:components[j].append((abs(k),w if k>0 else -w))
    for vec in components:vec.sort()
    standard=cost([k for vec in components for k,_ in vec],0)
    data=dict(baseline=standard)
    def verify(terms):return sum(k*w for k,w in terms)%P
    assert (verify(components[0])+LAMBDA*verify(components[1]))%P==expected
    for block in [2,8,32,len(terms)+1]:
        label='full' if block>len(terms) else str(block)
        single,work=suffix_terms(terms,block)
        assert verify(single)==expected
        data['scalar-'+label]=cost([k for x,_ in single for k in split(x)],work)
        low,lw=suffix_terms(components[0],block); high,hw=suffix_terms(components[1],block)
        assert (verify(low)+LAMBDA*verify(high))%P==expected
        data['component-'+label]=cost([k for k,_ in low+high],lw+hw)
    tiny={str(bits):sum((b-a).bit_length()<=bits for a,b in zip(scalars,scalars[1:])) for bits in [1,16,64,126,224]}
    return dict(original_rows=len(xs),distinct_signed=len(scalars),adjacent_short=tiny,schemes=data)

if __name__ == "__main__":
    print(json.dumps(run(Path(sys.argv[1]).read_bytes()), indent=2))
