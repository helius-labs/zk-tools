"""Score public offsets in the two signed BN254 G2 scalar components."""
from collections import Counter
from functools import lru_cache
import json
from pathlib import Path
import sys
P = 21888242871839275222246405745257275088548364400416034343698204186575808495617
LAMBDA = 4407920970296243842393367215006156084916469457145843978461
C1 = 7978627105577135859 + (15644699364383830999 << 64) + (2 << 128)
C2 = 14437538753401545464386762078465048853644945885975034865370
@lru_cache(65536)
def split(x):
    q1 = (((x >> 128) * C1) + (1 << 191)) >> 192
    q2 = (((x >> 64) * C2) + (1 << 255)) >> 256
    lo = x - q1 * 9931322734385697763 - q2 * 147946756881789319010696353538189108491
    hi = q1 * 147946756881789319000765030803803410728 - q2 * 9931322734385697763
    assert max(abs(lo), abs(hi)) < 1 << 126 and (lo + LAMBDA * hi) % P == x
    return lo, hi

def signed_digits(x):
    mag = abs(x)
    sign = -1 if x < 0 else 1
    for offset in range(0, 126, 12):
        bits = ((mag << 1) >> offset) & 8191
        yield sign * (((bits + 1) >> 1) - ((bits >> 12) << 12))

@lru_cache(65536)
def cost(x):
    return sum(bool(d) for part in split(x) for d in signed_digits(part))

def components(values):
    # Complete repeated halves and individual repeated signed radial digits
    # can each form a public scalar via the endomorphism eigenvalue.
    counts = Counter(values)
    modal, freq = counts.most_common(1)[0]
    complete = modal if freq * 4 >= 3 * len(values) else 0
    columns = zip(*(signed_digits(v) for v in values))
    stable = 0
    for index, column in enumerate(columns):
        if column[0] and all(v == column[0] for v in column[1:]):
            stable += column[0] << (index * 12)
    return complete, stable

def pick(values):
    first = values[0]
    if all(x == first for x in values):
        return first, first, 'fixed'
    lo, hi = zip(*(split(v) for v in values))
    l1, l2 = components(lo)
    h1, h2 = components(hi)
    modal, freq = Counter(values).most_common(1)[0]
    full = modal if freq * 4 >= len(values) * 3 else 0
    orig = sum(map(cost, values))
    fullcost = sum(cost((x-full) % P) for x in values)
    if orig-fullcost < 2*len(values):
        full, fullcost = 0, orig
    best, bestcost, method = full, fullcost, 'modal' if full else 'none'
    seen = {0, full}
    for l in (0, l1, l2):
        for h in (0, h1, h2):
            trial = (l + LAMBDA*h) % P
            if trial in seen:
                continue
            seen.add(trial)
            work = sum(cost((x-trial) % P) for x in values)
            if work < bestcost and orig-work >= 2*len(values):
                best, bestcost, method = trial, work, 'components'
    return full, best, method

def run(train, hold):
    data = [p.read_bytes() for p in train]
    assert len({len(x) for x in data}) == 1 and len(data[0]) % 32 == 0
    selected = []
    counts = Counter()
    for i in range(0, len(data[0]), 32):
        training = [int.from_bytes(blob[i:i+32], 'little') for blob in data]
        assert max(training) < P
        stable = training[0] if all(v==training[0] for v in training) else 0
        whole, combined, _ = pick(training)
        selected.append((stable, whole, combined))
        counts['stable_rows'] += stable != 0
        counts['whole_rows'] += whole != 0
        counts['combined_rows'] += combined != 0
        counts['partial_rows'] += combined not in (0, whole)
    stats = []
    for path in hold:
        blob = path.read_bytes()
        assert len(blob) == 32*len(selected)
        scores = [0]*4
        for row,(stable, whole, combined) in enumerate(selected):
            x = int.from_bytes(blob[32*row:32*(row+1)], 'little')
            assert x < P
            for j, offset in enumerate([0, stable, whole, combined]):
                residual = (x - offset) % P
                assert (offset + residual) % P == x
                scores[j] += cost(residual)
        stats.append(scores)
    return dict(counts=counts, held=stats)

if __name__ == '__main__':
    # Oracle checks use integer recomposition, including negative Booth digits.
    for x in [0,1,P-1,2**127,2**250,P//2,123456789]:
        for half in split(x):
            assert sum(d << (12*j) for j,d in enumerate(signed_digits(half))) == half
    hi = 0x123456789abcdeffedcba98765
    synthetic = [(LAMBDA*hi + i*127) % P for i in range(1,9)]
    _, score, _ = pick(synthetic)
    assert score and all(cost((x-score)%P) < cost(x) for x in synthetic)
    count = int(sys.argv[1]);paths=list(map(Path,sys.argv[2:]))
    print(json.dumps(run(paths[:count],paths[count:])))
