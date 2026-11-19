"""Read the linear forms in a ZSP1 version 2 solver image."""
import hashlib
import mmap
import struct
import sys
from array import array
from collections import Counter
from functools import lru_cache


class Plan:
    def __init__(self, path, digest):
        with path.open("rb") as file:
            if hashlib.file_digest(file, "sha256").hexdigest() != digest:
                raise ValueError("unexpected key image")
            self.image = mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ)
        offset = self.image.rfind(b"ZSP1")
        while offset >= 8:
            length, = struct.unpack_from("<Q", self.image, offset - 8)
            version, = struct.unpack_from("<I", self.image, offset + 4)
            if length == len(self.image) - offset and version == 2:
                break
            offset = self.image.rfind(b"ZSP1", 0, offset)
        if offset < 8 or sys.byteorder != "little":
            raise ValueError("expected a little-endian v2 solver image")
        self.words = memoryview(self.image)[offset:].cast("I")
        (_, _, self.wires, _, _, self.constraints, a, b, coefficients,
         terms, sums, slots, self.ops) = self.words[:13]
        first = 13 + a
        self.b2 = set(self.words[first:first+b])
        assert len(self.b2) == b
        first += b
        raw = [int.from_bytes(self.image[offset+4*first+32*i:offset+4*first+32*(i+1)], "little")
               for i in range(coefficients)]
        self.prime = raw[1] + raw[3]
        scale = pow(raw[1], -1, self.prime)
        self.coefficients = [x * scale % self.prime for x in raw]
        assert self.coefficients[:5] == [0, 1, 2, self.prime-1, self.prime-2]
        self.terms = first + 8 * coefficients
        self.sums = self.terms + 2 * terms
        self.first_op = self.sums + 3 * sums
        self.slot_index = array("I", [0xffffffff]) * slots
        self.bits = set()

    def rows(self):
        pos = self.first_op
        count = 0
        for _ in range(self.ops):
            if self.words[pos]:
                op, hint, _, _, first, last = self.words[pos:pos+6]
                assert op == 1
                if hint == 1:
                    self.bits.update(range(first, last))
                pos += 6
                continue
            _, row, first, last, slot = self.words[pos:pos+5]
            assert row < self.constraints
            for j in range(first, last):
                if self.words[self.sums+3*j] >> 31:
                    assert self.slot_index[slot] == 0xffffffff
                    self.slot_index[slot] = j
                    slot += 1
            assigned = self.words[pos+14]
            assert assigned in (0, 1)
            yield pos
            count += 1
            pos += 26 if assigned else 15
        assert count == self.constraints and pos == len(self.words)
        assert 0xffffffff not in self.slot_index

    def form(self, row, side):
        """Expand stored slots and include the assigned wire on its encoded side."""
        start = row + 5 + 3 * side
        tag, a, b = self.words[start:start+3]
        terms = []
        if tag == 1:
            terms.append((a, 1))
        elif tag == 2:
            terms.append((0, self.coefficients[a]))
        elif tag in (3, 4):
            for j in range(a, a+1 if tag == 3 else b):
                ci, wire = self.words[self.terms+2*j:self.terms+2*j+2]
                terms.append((wire, self.coefficients[ci]))
        else:
            assert tag == 0
        if self.words[row+14] and self.words[row+15] == side:
            wire, ci = self.words[row+16:row+18]
            terms.append((wire, self.coefficients[ci]))
        values = Counter()
        for wire, scalar in terms:
            if wire < self.wires:
                values[wire] += scalar
                continue
            j = self.slot_index[wire-self.wires]
            while True:
                off = self.sums + 3*j
                prev, ci, source = self.words[off:off+3]
                assert source < self.wires
                values[source] += scalar * self.coefficients[ci]
                prev &= 0x7fffffff
                if prev == 0x7fffffff:
                    break
                j = j-1 if prev == 0x7ffffffe else self.slot_index[prev-self.wires]
        return {w: x % self.prime for w, x in values.items() if x % self.prime}

    def close(self):
        self.words.release()
        self.image.close()


def linear_part(a, b, c):
    ca, cb = a.get(0, 0), b.get(0, 0)
    result = Counter({i: -x for i, x in c.items()})
    result[0] += ca * cb
    for i, x in a.items():
        if i:
            result[i] += cb * x
    for i, x in b.items():
        if i:
            result[i] += ca * x
    return result


def invert_mod(prime):
    @lru_cache(maxsize=4096)
    def inverse(x):
        return pow(x, -1, prime)
    return inverse
