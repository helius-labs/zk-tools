import argparse
from collections import Counter, defaultdict, deque
import json
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "2026-11-19-quadratic-cancel476" / "artifacts"))
from zp_plan476 import Plan, invert_mod, linear_part

def scan(image, c):
    plan = Plan(image, c['image_sha256'])
    prime = plan.prime
    inverse = invert_mod(prime)
    bidx = plan.b2
    bools = plan.bits
    nc = plan.constraints
    nb = len(bidx)

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
    positions = []
    incidence = defaultdict(set)
    wire_supports = []
    for at in plan.rows():
        left = set(plan.form(at, 0))
        right = set(plan.form(at, 1))
        left.discard(0)
        right.discard(0)
        if not left or not right:
            continue
        idx = len(positions)
        positions.append(at)
        support = left | right
        wire_supports.append(support)
        for wire in support:
            incidence[wire].add(idx)

    def peel(incidence, supports, live):
        fringe = deque((k for k, s in incidence.items() if len(s) == 1))
        while fringe:
            k = fringe.popleft()
            if len(incidence[k]) != 1:
                continue
            row = next(iter(incidence[k]))
            assert row in live
            live.remove(row)
            for t in supports[row]:
                incidence[t].remove(row)
                if len(incidence[t]) == 1:
                    fringe.append(t)
    live = set(range(len(positions)))
    peel(incidence, wire_supports, live)
    wire_live = len(live)
    del incidence, wire_supports

    def quadratic(fs):
        q = Counter()
        for i, x in fs[0].items():
            if not i:
                continue
            for j, y in fs[1].items():
                if j:
                    q[min(i, j) << 32 | max(i, j)] += x * y
        return {i: z % prime for i, z in q.items() if z % prime}
    monomials = defaultdict(set)
    supports = {}
    total = 0
    for idx in live:
        qs = quadratic([plan.form(positions[idx], j) for j in range(3)])
        assert qs
        supports[idx] = set(qs)
        total += len(qs)
        for monomial in qs:
            monomials[monomial].add(idx)
    distinct = len(monomials)
    peel(monomials, supports, live)
    del monomials, supports

    def add_scaled(dst, src, k):
        for i, x in src.items():
            t = (dst.get(i, 0) + k * x) % prime
            if t:
                dst[i] = t
            else:
                dst.pop(i, None)

    def normalized_at(poly, inv):
        return {k: x * inv % prime for k, x in poly.items()}

    def reduce_polys(constraint_forms):
        pivots = {}
        linear_pivots = {}
        dependent = 0
        tautologies = 0
        maxfill = 0
        for fs in constraint_forms:
            q = quadratic(fs)
            l = {k: x % prime for k, x in linear_part(*fs).items() if x % prime}
            while q:
                maxfill = max(maxfill, len(q))
                lead = max(q)
                a = q[lead]
                if lead not in pivots:
                    iv = inverse(a)
                    pivots[lead] = (normalized_at(q, iv), normalized_at(l, iv))
                    break
                pq, pl = pivots[lead]
                add_scaled(q, pq, -a)
                add_scaled(l, pl, -a)
            if q:
                continue
            dependent += 1
            if not l:
                tautologies += 1
            while l:
                lead = max(l)
                a = l[lead]
                if lead not in linear_pivots:
                    linear_pivots[lead] = normalized_at(l, inverse(a))
                    break
                add_scaled(l, linear_pivots[lead], -a)
        return (pivots, linear_pivots, dependent, tautologies, maxfill)
    demo = [[{1: 1}, {3: 1}, {4: 1}], [{2: 1}, {3: 1}, {5: 1}], [{1: 1, 2: 1}, {3: 1}, {6: 1}]]
    pp, ll, dd, _, _ = reduce_polys(demo)
    assert len(pp) == 2 and dd == 1 and (ll == {6: {4: prime - 1, 5: prime - 1, 6: 1}})
    for support_list, expected in [([{1, 3}, {2, 3}, {1, 2, 3}, {7}], {0, 1, 2}), ([{1}, {1, 2}, {2, 3}, {3, 4}], set())]:
        incidence = defaultdict(set)
        for r, terms in enumerate(support_list):
            for term in terms:
                incidence[term].add(r)
        test_live = set(range(len(support_list)))
        peel(incidence, support_list, test_live)
        assert test_live == expected
    pivots, linear_pivots, dependent, tautologies, maxfill = reduce_polys(([plan.form(positions[idx], j) for j in range(3)] for idx in sorted(live)))
    qtouch = set()
    candidates = 0
    for l in linear_pivots.values():
        touched = (set(l) & bidx) - bools - {0}
        if touched:
            candidates += 1
            qtouch.update(touched)
    assert len(pivots) + dependent == len(live)
    result = dict(shape=c['shape'], image_sha256=c['image_sha256'], positive_control_evaluations=tests, peeling_controls=2, three_row_rank_controls=1, nonlinear_rows=len(positions), after_wire_peeling=wire_live, monomial_entries=total, distinct_monomials=distinct, after_monomial_peeling=len(live), survivor_quadratic_rank=len(pivots), quadratic_nullity=dependent, trivial_zero_residuals_before_linear_reduction=tautologies, linear_consequence_rank=len(linear_pivots), dense_b2_pivot_upper=min(candidates, len(qtouch)), max_elimination_row=maxfill, b2=nb)
    plan.close()
    return result
if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Analyze nonlinear constraints in a Zopro CPU key image')
    parser.add_argument('image', type=Path)
    parser.add_argument('shape')
    parser.add_argument('image_sha256')
    args = parser.parse_args()
    print(json.dumps(scan(args.image, vars(args)), indent=2))
