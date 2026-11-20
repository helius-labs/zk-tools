from collections import defaultdict
from fractions import Fraction
from itertools import permutations
import json
from pathlib import Path
import random
from struct import unpack_from

# BN254 field parameters and the existing signed-endomorphism eigenvalue.
R = 21888242871839275222246405745257275088548364400416034343698204186575808495617
P = 21888242871839275222246405745257275088696311157297823662689037894645226208583
X = 4965661367192848881
LAMBDA = 4407920970296243842393367215006156084916469457145843978461
PSI = P % R
EIGENVALUES = (1, LAMBDA, PSI, LAMBDA * PSI % R)
# Short lattice documented by gnark-crypto v0.22.0, ecc/bn254/bn254.go.
BASIS = ((2*X, X, X+1, -X), (-X, X, X, 2*X+1),
         (-X-1, X, -X, -2*X), (2*X+1, X+1, -X, X))
SHIFT = 320
SOURCE = 'https://github.com/Consensys-Incorporated/gnark-crypto/blob/v0.22.0/ecc/bn254/bn254.go'

def determinant(rows):
    total = 0
    for p in permutations(range(len(rows))):
        inversions = sum(i > j for k,i in enumerate(p) for j in p[k+1:])
        product = (-1)**inversions
        for i,j in enumerate(p):
            product *= rows[i][j]
        total += product
    return total

def round_nearest(value):
    return (2*value.numerator+value.denominator)//(2*value.denominator)

D = determinant(BASIS)
assert abs(D)==R
assert all(sum(t*e for t,e in zip(row,EIGENVALUES))%R==0 for row in BASIS)
cofactors = [(-1)**i * determinant([row[1:] for j,row in enumerate(BASIS) if j!=i])
             for i in range(4)]
first_inverse = [Fraction(c,D) for c in cofactors]
assert [sum(c*row[j] for c,row in zip(first_inverse,BASIS)) for j in range(4)] == [1,0,0,0]
fixed = [round_nearest(c*(1<<SHIFT)) for c in first_inverse]
err = max(abs(Fraction(f,1<<SHIFT)-c)*(R-1) for f,c in zip(fixed,first_inverse))
bound = [sum(abs(row[j]) for row in BASIS)*(Fraction(1,2)+err) for j in range(4)]
assert all(v < 1<<64 for v in bound)
assert max(f.bit_length() for f in fixed)<=257

def four(s, approximate=True):
    if approximate:
        # Quotient rounding needs only low 128 bits in a native implementation.
        q = [(s*f+(1<<(SHIFT-1)))>>SHIFT for f in fixed]
    else:
        q = [round_nearest(s*c) for c in first_inverse]
    return tuple((s if i==0 else 0)-sum(t*row[i] for t,row in zip(q,BASIS)) for i in range(4))

def two(s):
    c1 = 7978627105577135859 + (15644699364383830999<<64) + (2<<128)
    c2 = 11953552847224317658 + (8825887400277225869<<64) + (5534624963584316111<<128) + (2<<192)
    q1 = ((s>>128)*c1 + (1<<191))>>192
    q2 = ((s>>64)*c2 + (1<<255))>>256
    a=s-q1*9931322734385697763-q2*147946756881789319010696353538189108491
    b=q1*147946756881789319000765030803803410728-q2*9931322734385697763
    assert max(abs(a),abs(b))<1<<126
    return a,b

def digits(value,bits,windows,booth):
    width=1<<bits
    v=abs(value)
    out=[]
    if booth:
        for w in range(windows):
            if w==0:
                raw=v&(width-1)
                d=raw-((raw>>(bits-1))*width)
            else:
                raw=(v>>(bits*w-1))&(2*width-1)
                d=((raw+1)>>1)-(raw>>bits)*width
            out.append(d)
    else:
        carry=0
        for w in range(windows):
            raw=((v>>(bits*w))&(width-1))+carry
            carry=(raw+width//2)>>bits
            out.append(raw-carry*width)
    assert sum(x<<(bits*i) for i,x in enumerate(out))==v
    return out if value>=0 else [-x for x in out]

def weight_tree_pairs(occupied,slots):
    # Simulate the Boolean form of weighted_sum's layered paired reduction,
    # under the same no-cancellation assumption as the bucket model.
    total=0
    for first in range(0,slots,256):
        n=min(256,slots-first)
        layers=[[i+first+1 in occupied for i in range(n)]]
        while len(layers[0])>1:
            evens=layers[0][::2]
            next_layers=[]
            for layer in layers:
                total+=sum(a and b for a,b in zip(layer[::2],layer[1::2]))
                next_layers.append([a or b for a,b in zip(layer[::2],layer[1::2])])
            next_layers.append(evens)
            layers=next_layers
    return total

def profile(values,dimensions,bits,parts,booth):
    width=127 if dimensions==2 else 65
    windows=(width+bits-1)//bits
    partition=(len(values)+parts-1)//parts
    # One absolute-magnitude bucket set for each component, window and range.
    sets=[[[set() for _ in range(dimensions)] for _ in range(parts)] for _ in range(windows)]
    arrivals=0
    leading_arrivals=0
    for i,components in enumerate(values):
        if not components:
            continue
        part=i//partition
        for j,v in enumerate(components):
            for w,d in enumerate(digits(v,bits,windows,booth)):
                if d:
                    arrivals+=1
                    leading_arrivals+=w==windows-1
                    sets[w][part][j].add(abs(d))
    phi,psi,stream,weight=0,0,0,0
    max_digit=0
    strategies={'shared': (tuple(range(dimensions)),)}
    if dimensions==4:
        strategies['paired']=((0,1),(2,3))
        strategies['separate']=((0,),(1,),(2,),(3,))
    layouts={}
    for strategy,components_by_leaf in strategies.items():
        streamed,merged,weighted=0,0,0
        for partitions in sets:
            for indices in components_by_leaf:
                part_bins=[set.union(*(part[i] for i in indices)) for part in partitions]
                global_bins=set.union(*part_bins)
                streamed+=sum(map(len,part_bins))
                merged+=len(global_bins)
                weighted+=weight_tree_pairs(global_bins,1<<(bits-1))
        layouts[strategy]=dict(stream_adds=arrivals-streamed,merge_adds=streamed-merged,
            weighted_tree_pairs=weighted,total_curve_pairs=arrivals-merged+weighted,
            logical_leaf_jobs=parts*windows*len(components_by_leaf),
            dense_bucket_slots=(1<<(bits-1))*parts*windows*len(components_by_leaf))
    if dimensions==4:
        layouts['paired']['phi_bucket_transforms'] = sum(
            len(part[1])+len(part[3]) for partitions in sets for part in partitions)
        layouts['paired']['psi_bucket_transforms'] = 0
        layouts['separate']['phi_bucket_transforms'] = 0
        layouts['separate']['psi_bucket_transforms'] = 0
    for w,partitions in enumerate(sets):
        union=set()
        for component in partitions:
            u=set.union(*component)
            union.update(u)
            stream+=len(u)
            if dimensions==2:
                phi+=len(component[1])
            else:
                high=component[2]|component[3]
                phi+=len(component[3])+len(high)+len(high|component[1])
                psi+=len(high)
        m=max(union,default=0)
        max_digit = m if w==windows-1 else max_digit
        weight+=len(union)
    layouts['shared']['phi_bucket_transforms']=phi
    layouts['shared']['psi_bucket_transforms']=psi
    return dict(dimensions=dimensions,bits=bits,parts=parts,windows=windows,range_jobs=windows*parts,
                arrivals=arrivals,leading_arrivals=leading_arrivals,leading_max_digit=max_digit,
                occupied_after_merge=weight,modeled_stream_adds=arrivals-stream,modeled_merge_adds=stream-weight,
                modeled_adds_before_weighting=arrivals-weight,
                phi_bucket_transforms=phi,psi_bucket_transforms=psi,
                dense_bucket_slots=windows*(1<<(bits-1))*parts,
                unpadded_base_scans=windows*dimensions,weighting_layouts=layouts)

def run(root):
    root=Path(root)
    build=json.loads((root/'built.json').read_text())
    booth='--cfg zopro_booth_digits' in build['environment']['RUSTFLAGS']
    preflight=json.loads((root/'preflight.json').read_text())
    rows={}
    max_bits=[0]*4
    oracle_count=0
    rng=random.Random(9404096)
    scalars = [0,1,R-1, R//2] + [2**b+v for b in range(254) for v in (-1,0,1) if 0<=2**b+v<R]
    scalars += [rng.randrange(R) for _ in range(8192)]
    for s in scalars:
        got=four(s)
        assert got==four(s,False)
        assert all(abs(v)<=m for v,m in zip(got,bound))
        assert sum(x*e for x,e in zip(got,EIGENVALUES))%R==s
    for j in preflight['proofs']:
        if j['replicate']!=0:
            continue
        raw=Path(j['capture']).read_bytes()
        n,_=unpack_from('<QQ',raw,8)
        actual2,actual4=[],[]
        for i in range(n):
            record=raw[24+33*i:24+33*(i+1)]
            identity=record[0]
            assert identity in (0,1)
            s=int.from_bytes(record[1:],'little')
            assert 0<=s<R
            a=two(s)
            b=four(s)
            assert (a[0]+a[1]*LAMBDA)%R==s
            assert sum(x*e for x,e in zip(b,EIGENVALUES))%R==s
            assert all(abs(v)<=m for v,m in zip(b,bound))
            max_bits=[max(old,abs(v).bit_length()) for old,v in zip(max_bits,b)]
            if i%127==0:
                assert four(s,False)==b
                oracle_count+=1
            actual2.append(a if not identity and s else None)
            actual4.append(b if not identity and s else None)
        configs = (('two-12x2',actual2,2,12,2),('two-11x4',actual2,2,11,4),
                   ('two-13x2',actual2,2,13,2),('two-13x4',actual2,2,13,4),
                   ('two-14x2',actual2,2,14,2),
                   ('four-11x4',actual4,4,11,4),('four-12x4',actual4,4,12,4),
                   ('four-13x2',actual4,4,13,2),('four-13x4',actual4,4,13,4),('four-13x8',actual4,4,13,8))
        details={name:profile(vals,dims,bits,parts,booth) for name,vals,dims,bits,parts in configs}
        row=dict(terms=n,active=sum(v is not None for v in actual4),shape=j['shape'],input=j['input'],
                 source_sha256=j['capture_sha256'],profiles=details)
        rows[f"{j['shape']}/{j['input']}"]=row
        print(j['shape'],j['input'],n,[(k,v['modeled_adds_before_weighting']) for k,v in details.items()],flush=True)
    result=dict(basis=BASIS,eigenvalues=EIGENVALUES,determinant=D,cofactors=cofactors,
                fixed_reciprocals=fixed,shift=SHIFT,error_numerator=err.numerator,
                error_denominator=err.denominator,integer_magnitude_bounds=[v.numerator//v.denominator for v in bound],
                max_actual_magnitude_bits=max_bits,boundary_and_random_oracles=len(scalars),
                independent_fixture_rounding_oracles=oracle_count,booth=booth,source=SOURCE,cases=rows)
    (root/'model.json').write_text(json.dumps(result,indent=2)+'\n')

if __name__=='__main__':
    import sys
    run(sys.argv[1])
