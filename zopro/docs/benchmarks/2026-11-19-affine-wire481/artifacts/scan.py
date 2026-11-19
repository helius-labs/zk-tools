"""Find affine scalar relations on public training requests and test their residuals."""
from collections import Counter
from hashlib import sha256
from pathlib import Path
from statistics import median

P = 21888242871839275222246405745257275088548364400416034343698204186575808495617


def inverses(values):
    prefixes = []
    product = 1
    for value in values:
        prefixes.append(product)
        product = product * value % P
    inv = pow(product, -1, P)
    for i in reversed(range(len(values))):
        prefixes[i], inv = prefixes[i] * inv % P, values[i] * inv % P
    return prefixes


def digits(value):
    n = 0
    while value:
        digit = value & 4095
        value = (value >> 12) + (digit >= 2048)
        n += digit != 0
    return n


def classify(slope, intercept):
    if slope in (1, P - 1):
        return 'signed_equal' if intercept == 0 else 'signed_shifted'
    return 'scaled' if intercept == 0 else 'scaled_shifted'


def analyze(train, test):
    size = len(train[0])
    assert all(len(s) == size for s in train + test) and size % 32 == 0
    reps = {}
    groups = set()
    kinds = {}
    witnesses = [Counter() for _ in test]
    fixed = 0
    for start in range(0, size // 32, 2048):
        entries = []
        for pos in range(start, min(start + 2048, size // 32)):
            slot = slice(pos * 32, (pos + 1) * 32)
            vals = [int.from_bytes(s[slot], 'little') for s in train]
            assert all(v < P for v in vals)
            delta = tuple((x - vals[0]) % P for x in vals[1:])
            scale = next((v for v in delta if v), 0)
            if scale:
                entries.append((pos, vals[0], scale, delta))
            else:
                fixed += 1
        invs = inverses([entry[2] for entry in entries])
        for (pos, anchor, scale, delta), inv in zip(entries, invs):
            signature = b''.join((d * inv % P).to_bytes(32, 'little') for d in delta)
            # Dictionary keys use full field encodings with exact byte equality.
            old = reps.get(signature)
            if old is None:
                reps[signature] = (pos, anchor, inv)
                continue
            rep, origin, inverse = old
            groups.add(rep)
            slope = scale * inverse % P
            offset = (anchor - slope * origin) % P
            kind = classify(slope, offset)
            totals = kinds.setdefault(kind, Counter())
            totals['members'] += 1
            valid = True
            for w, s in zip(witnesses, test):
                x = int.from_bytes(s[32*pos:32*pos+32], 'little')
                y = int.from_bytes(s[32*rep:32*rep+32], 'little')
                assert x < P and y < P
                residual = (x - (slope * y + offset)) % P
                assert (slope * y + offset + residual) % P == x
                gain = digits(x) - digits(residual)
                w['digit_change_saved'] += gain
                totals['digit_change_saved'] += gain
                w['zero_residuals'] += residual == 0
                valid &= residual == 0
            totals['matched_all_heldout'] += valid
    for w, s in zip(witnesses, test):
        w['original_digits'] = sum(digits(int.from_bytes(s[i:i+32], 'little')) for i in range(0,size,32))
    return dict(rows=size//32, heldout=len(test), training=len(train), fixed=fixed, groups=len(groups),
                classes=kinds, heldout_requests=witnesses)


def control():
    def fixture(x,y):
        vals=[x, (P-x)%P, (x+7)%P,13*x%P,(13*x+3)%P,5,y]
        return b''.join(v.to_bytes(32,'little') for v in vals)
    train=[fixture(x,y) for x,y in [(1,3),(5,2),(7,15),(24,10)]]
    r=analyze(train,[fixture(0,7),fixture(P-1,13)])
    assert r['fixed']==1 and r['groups']==1 and all(r['classes'][k]['matched_all_heldout']==1 for k in ['signed_equal','signed_shifted','scaled','scaled_shifted'])
    bad=bytearray(fixture(9,77));bad[64:96]=(128).to_bytes(32,'little')
    r=analyze(train,[bad]);assert r['classes']['signed_shifted']['matched_all_heldout']==0 and r['heldout_requests'][0]['zero_residuals']==3
