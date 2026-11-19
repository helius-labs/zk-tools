"""Pack a version 2 Euler image using a B2 section verified by the experiment 505 checker."""
import argparse
from array import array
import hashlib
from pathlib import Path
import shutil
import struct
import sys


def read_exact(stream, count):
    data = stream.read(count)
    if len(data) != count:
        raise ValueError("truncated input")
    return data


def checked_copy(source, output, count):
    while count:
        data = read_exact(source, min(count, 1 << 20))
        output.write(data)
        count -= len(data)


def load_mapping(plan):
    data = plan.read_bytes()
    if len(data) < 16 or data[:8] != b"ZPGR0482" or len(data) % 4:
        raise ValueError("invalid group file header")
    words = memoryview(data)[8:].cast("I")
    n, groups = words[:2]
    if not 0 < groups < n < 1 << 31:
        raise ValueError("invalid group dimensions")
    taken = bytearray(n)
    representatives = array("I")
    tail = array("I")
    memberships = array("I")
    at = 2
    for group in range(groups):
        if at >= len(words):
            raise ValueError("missing group")
        length = words[at]
        members = words[at + 1:at + 1 + length]
        if length < 2 or len(members) != length:
            raise ValueError("invalid group length")
        for index in members:
            if index >= n or taken[index]:
                raise ValueError("overlapping or out-of-range group")
            taken[index] = 1
        representatives.append(members[0])
        tail.extend(members[1:])
        memberships.extend([group] * (length - 1))
        at += 1 + length
    if at != len(words):
        raise ValueError("trailing group file data")
    representatives.extend(i for i in range(n) if not taken[i])
    prefix = len(representatives)
    representatives.extend(tail)
    visited = bytearray(n)
    leader = 1 << 31
    for start in range(n):
        if visited[start]:
            continue
        index = start
        while not visited[index]:
            visited[index] = 1
            index = representatives[index]
        if index != start:
            raise ValueError("invalid permutation")
        if representatives[start] != start:
            representatives[start] |= leader
    trailer = array("I")
    for row, index in enumerate(representatives):
        trailer.extend((index, 0xffffffff if row < prefix else memberships[row - prefix]))
    return n, groups, prefix, trailer, taken


def encode(args):
    if sys.byteorder != "little" or array("I").itemsize != 4:
        raise ValueError("this native-layout tool requires little-endian u32 words")
    n, groups, prefix, trailer, taken = load_mapping(args.plan)
    with args.section.open("rb") as section:
        digest = hashlib.file_digest(section, "sha256").digest()
        if digest.hex() != args.section_sha256:
            raise ValueError("replacement B2 digest mismatch")
        section.seek(0)
        if args.section.stat().st_size != 128 * n:
            raise ValueError("replacement B2 length mismatch")
        with args.original.open("rb") as source:
            header = read_exact(source, 100)
            if header[:16] != b"zopro-euler-v2\0\0":
                raise ValueError("expected Euler v2 image")
            hlen, = struct.unpack_from("<Q", header, 16)
            a, b, k, z, b2 = struct.unpack_from("<5I", header, 56)
            if b != n or b2 != n:
                raise ValueError("key/plan B2 length mismatch")
            payload = (24 + hlen + 63) // 64 * 64
            source.seek(payload)
            slots, = struct.unpack("<Q", read_exact(source, 8))
            start = payload + 128 + 64 * (a + b + k + slots + z + 1)
            if start + 128 * n > args.original.stat().st_size:
                raise ValueError("truncated Euler point section")
            if args.wire:
                raw = args.wire.read_bytes()
                if len(raw) < 48 or raw[:8] != b"ZPWO0001":
                    raise ValueError("invalid public offsets header")
                width, count = struct.unpack_from("<II", raw, 40)
                if width != n or len(raw) != 48 + 36 * count:
                    raise ValueError("invalid public offsets dimensions")
                for at in range(48, len(raw), 36):
                    i, = struct.unpack_from("<I", raw, at)
                    if i >= n or taken[i]:
                        raise ValueError("wire offset must reference a singleton")
                source.seek(start)
                hasher = hashlib.sha256()
                remaining = 128 * n
                while remaining:
                    data = read_exact(source, min(remaining, 1 << 20))
                    hasher.update(data)
                    remaining -= len(data)
                if raw[8:40] != hasher.digest():
                    raise ValueError("wire file belongs to a different key")
            source.seek(16)
            with args.output.open("xb") as output:
                output.write(b"zopro-b2-grp506\0")
                checked_copy(source, output, start - 16)
                shutil.copyfileobj(section, output, 1 << 20)
                source.seek(n * 128, 1)
                shutil.copyfileobj(source, output, 1 << 20)
                output.write(struct.pack("<8sII", b"ZPB2T506", groups, prefix))
                trailer.tofile(output)
        if args.wire:
            with args.output_wire.open("xb") as out_wire:
                out_wire.write(raw[:8] + digest + raw[40:])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("original", "plan", "section", "output"):
        parser.add_argument(name, type=Path)
    parser.add_argument("section_sha256", help="hash reported by the differential checker")
    parser.add_argument("--wire", type=Path)
    parser.add_argument("--output-wire", type=Path)
    args = parser.parse_args()
    if bool(args.wire) != bool(args.output_wire):
        parser.error("the wire input and output must be supplied together")
    encode(args)
