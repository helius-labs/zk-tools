"""Extract all B-column dependencies and write canonical inputs for the Arkworks check."""
import argparse
from collections import Counter, defaultdict, deque
from functools import lru_cache
import json
from pathlib import Path
import struct
import sys

reader = Path(__file__).resolve().parents[2] / '2026-11-19-quadratic-cancel476/artifacts'
sys.path.insert(0, str(reader))
from zp_plan476 import Plan

FQ = 21888242871839275222246405745257275088696311157297823662689037894645226208583


def relations(plan):
    rows = {}
    occurrences = defaultdict(list)
    for position in plan.rows():
        index = plan.words[position + 1]
        terms = plan.form(position, 1)
        if terms.keys() - plan.b2:
            raise ValueError('B equation contains a wire absent from the key')
        if terms:
            rows[index] = terms
            for wire in terms:
                occurrences[wire].append(index)
    if occurrences.keys() != plan.b2:
        raise ValueError('missing B wire in constraint matrix')

    singletons = deque(index for index, row in rows.items() if len(row) == 1)
    peeled = 0
    while singletons:
        row = rows[singletons.popleft()]
        if len(row) != 1:
            continue
        wire = next(iter(row))
        peeled += 1
        for index in occurrences.pop(wire):
            del rows[index][wire]
            if len(rows[index]) == 1:
                singletons.append(index)

    p = plan.prime
    inverse = lru_cache(maxsize=8192)(lambda x: pow(x, -1, p))
    pivots = {}
    for row in sorted(rows.values(), key=len):
        while row:
            wire = max(row)
            coefficient = row.pop(wire)
            if wire not in pivots:
                scale = inverse(coefficient)
                pivots[wire] = {j: x * scale % p for j, x in row.items()}
                break
            for j, x in pivots[wire].items():
                new = (row.get(j, 0) - coefficient * x) % p
                if new:
                    row[j] = new
                else:
                    row.pop(j, None)

    free = occurrences.keys() - pivots.keys()
    combinations = {j: {j: 1} for j in free}
    result = {j: {} for j in free}
    for wire, row in sorted(pivots.items()):
        combo = Counter()
        for j, coefficient in row.items():
            for k, x in combinations.get(j, {}).items():
                combo[k] -= coefficient * x
        reduced = {k: x % p for k, x in combo.items() if x % p}
        combinations[wire] = reduced
        for k, x in reduced.items():
            result[k][wire] = (-x) % p
    return peeled, result


def write_points(plan, result, path):
    a, b = plan.words[6:8]
    index = {w: j for j, w in enumerate(plan.words[13+a:13+a+b])}
    length, = struct.unpack_from('<Q', plan.image, 16)
    meta = (24 + length + 63) // 64 * 64
    ar, br, k, z, n = struct.unpack_from('<5I', plan.image, 56)
    ns, = struct.unpack_from('<Q', plan.image, meta)
    assert br == n == b and set(index) == plan.b2
    start = meta + 128 + 64 * (ar + br + k + ns + z + 1)
    rinv = pow(2**256, -1, FQ)

    def canonical(wire):
        offset = start + 128 * index[wire]
        values = [int.from_bytes(plan.image[j:j+32], 'little') for j in range(offset, offset+128, 32)]
        assert all(x < FQ for x in values)
        return b''.join((x * rinv % FQ).to_bytes(32, 'little') for x in values)

    with path.open('wb') as out:
        for wire, row in result.items():
            out.write(canonical(wire))
            out.write(struct.pack('<I', len(row)))
            for pivot, coefficient in row.items():
                out.write(coefficient.to_bytes(32, 'little'))
                out.write(canonical(pivot))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('image', type=Path)
    parser.add_argument('sha256')
    parser.add_argument('points', type=Path)
    args = parser.parse_args()
    plan = Plan(args.image, args.sha256)
    try:
        peeled, result = relations(plan)
        write_points(plan, result, args.points)
        print(json.dumps(dict(columns=len(plan.b2), rank=len(plan.b2)-len(result), peeled=peeled,
                              dependent=len(result), weights=dict(Counter(map(len, result.values()))))))
    finally:
        plan.close()
