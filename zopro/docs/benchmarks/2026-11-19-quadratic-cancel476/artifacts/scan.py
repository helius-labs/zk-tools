import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
from zp_plan476 import Plan, invert_mod, linear_part

def scan(image, c):
    plan = Plan(image, c['image_sha256'])
    prime = plan.prime
    inverse = invert_mod(prime)
    bidx = plan.b2
    bools = plan.bits
    nc = plan.constraints
    nb = len(bidx)
    heads = {}
    matched = tautologies = duplicates = nonlinear = max_width = 0
    stored = []
    touched = set()

    def split_form(f):
        items = sorted(((i, x) for i, x in f.items() if i != 0))
        if not items:
            return ((), 0)
        factor = items[0][1]
        if factor != 1:
            inv0 = inverse(factor)
            items = [(i, x * inv0 % prime) for i, x in items]
        return (tuple(items), factor)

    def normalized(forms):
        aa, ka = split_form(forms[0])
        bb, kb = split_form(forms[1])
        if not aa or not bb:
            return (None, 0)
        return (tuple(sorted((aa, bb))), ka * kb % prime)

    def digest(factors):
        h = hashlib.sha256()
        for terms in factors:
            h.update(len(terms).to_bytes(4, 'little'))
            for wire, c in terms:
                h.update(wire.to_bytes(4, 'little') + c.to_bytes(32, 'little'))
        return h.digest()
    tests = 0
    lhs = [{1: 1, 0: 2}, {2: 1, 0: 3}, {3: 1}]
    rhs = [{2: 2, 0: 10}, {1: 3, 0: 21}, {4: 1}]
    k0, f0 = normalized(lhs)
    k1, f1 = normalized(rhs)
    assert k0 == k1 and f1 == 6 * f0
    assert normalized([{1: 1}, {1: 1}, {}])[0] != k0
    rel = linear_part(*rhs)
    for wire, x in linear_part(*lhs).items():
        rel[wire] -= 6 * x
    assert rel == Counter({1: 12, 2: 30, 0: 174, 3: 6, 4: -1})
    for x, y in [(0, 0), (1, 7), (prime - 1, 2), (13, 91)]:
        w = [1, x, y, (x + 2) * (y + 3), 6 * (x + 7) * (y + 5)]
        assert sum((w[i] * z for i, z in rel.items())) % prime == 0
        w[4] += 1
        assert sum((w[i] * z for i, z in rel.items())) % prime == prime - 1
        tests += 2
    for at in plan.rows():
        fs = [plan.form(at, 0), plan.form(at, 1)]
        factors, scale = normalized(fs)
        if not factors:
            continue
        nonlinear += 1
        key = digest(factors)
        head = heads.setdefault(key, at)
        if head == at:
            continue
        other = [plan.form(head, i) for i in range(3)]
        f0, scale0 = normalized(other)
        assert f0 == factors
        matched += 1
        ratio = scale * inverse(scale0) % prime
        lin = linear_part(fs[0], fs[1], plan.form(at, 2))
        for w, x in linear_part(*other).items():
            lin[w] -= ratio * x
        rel = tuple(sorted(((w, x % prime) for w, x in lin.items() if x % prime)))
        if not rel:
            tautologies += 1
            continue
        stored.append(rel)
        max_width = max(max_width, len(rel))
    unique = set()
    eligible = 0
    touch_all = set()
    for rel in stored:
        inv0 = inverse(rel[0][1])
        r = tuple(((i, x * inv0 % prime) for i, x in rel))
        if r in unique:
            duplicates += 1
            continue
        unique.add(r)
        keys = {i for i, x in r if i}
        touch_all.update(keys)
        q = (keys & bidx) - bools
        if q:
            eligible += 1
            touched.update(q)
    result = dict(shape=c['shape'], image_sha256=c['image_sha256'], constraints=nc, positive_control_evaluations=tests, nonlinear_rows=nonlinear, distinct_quadratic_parts=len(heads), proportional_pairs=matched, zero_residual=tautologies, duplicate_residuals=duplicates, unique_linear_residuals=len(unique), touched_nonconstant_wires=len(touch_all), max_residual_terms=max_width, b2=nb, residuals_touching_nonbit_b2=eligible, touched_nonbit_b2=len(touched), nonbit_b2_pivot_upper=min(len(touched), eligible))
    plan.close()
    return result
if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Analyze nonlinear constraints in a Zopro CPU key image')
    parser.add_argument('image', type=Path)
    parser.add_argument('shape')
    parser.add_argument('image_sha256')
    args = parser.parse_args()
    print(json.dumps(scan(args.image, vars(args)), indent=2))
