import argparse
from array import array
from collections import Counter
import hashlib
import json
import mmap
from pathlib import Path
import struct
import sys

def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def scan(image, case):
    """Count certified linear identities without loading a witness. The image must match its expected SHA-256 digest."""
    assert sys.byteorder == 'little' and sha(image) == case['image_sha256']
    file = image.open('rb')
    image_data = mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ)
    start = image_data.rfind(b'ZSP1')
    while start >= 8 and (struct.unpack_from('<Q', image_data, start - 8)[0] != len(image_data) - start or struct.unpack_from('<I', image_data, start + 4)[0] != 2):
        start = image_data.rfind(b'ZSP1', 0, start)
    assert start >= 8
    head = struct.unpack_from('<13I', image_data, start)
    magic, version, wire_count, input_count, public_count, constraint_count, a_count, b_count, coefficient_count, term_count, sum_count, slot_count, operation_count = head
    assert magic == 827347802 and version == 2
    words = memoryview(image_data)[start:].cast('I')
    a_offset = 13
    b_offset = a_offset + a_count
    coefficient_offset = b_offset + b_count
    term_offset = coefficient_offset + 8 * coefficient_count
    sum_offset = term_offset + 2 * term_count
    cursor = sum_offset + 3 * sum_count
    b_query_wires = set(words[b_offset:coefficient_offset])
    assert len(b_query_wires) == b_count and max(b_query_wires) < wire_count
    coefficients = [int.from_bytes(image_data[start + 4 * coefficient_offset + 32 * i:start + 4 * coefficient_offset + 32 * (i + 1)], 'little') for i in range(coefficient_count)]
    prime = coefficients[1] + coefficients[3]
    inv = pow(coefficients[1], -1, prime)
    coefficients = [words * inv % prime for words in coefficients]
    assert coefficients[:5] == [0, 1, 2, prime - 1, prime - 2]
    slot_writers = array('I', [4294967295]) * slot_count
    slot_constants = [None] * slot_count
    hint_bits = set()
    hints = []
    linear = []
    row_types = Counter()
    kind = Counter()
    rows_seen = 0

    def terms_for(tag, a, b):
        """Decode one expression as terms of the form (wire, coefficient). Wire zero holds the constant one."""
        if tag == 0:
            return []
        if tag == 1:
            return [(a, 1)]
        if tag == 2:
            return [(0, coefficients[a])]
        lo, hi = (a, a + 1) if tag == 3 else (a, b)
        assert tag in (3, 4)
        return [(words[term_offset + 2 * j + 1], coefficients[words[term_offset + 2 * j]]) for j in range(lo, hi)]

    def expression_constant(tag, a, b):
        """Return a certified constant, or None when an input value is required."""
        if tag == 1:
            return 1 if a == 0 else None if a < wire_count else slot_constants[a - wire_count]
        if tag == 0:
            return 0
        if tag == 2:
            return coefficients[a]
        lo, hi = (a, a + 1) if tag == 3 else (a, b)
        value = 0
        for j in range(lo, hi):
            coeff = words[term_offset + 2 * j]
            if words[term_offset + 2 * j + 1] and coefficients[coeff]:
                return None
            value += coefficients[coeff]
        return value % prime

    def expand(terms_with_slots):
        """Expand stored sum prefixes into original scalar wires and combine repeated coefficients modulo the field."""
        result = Counter()
        for w, coef0 in terms_with_slots:
            if w < wire_count:
                result[w] += coef0
                continue
            sum_index = slot_writers[w - wire_count]
            assert sum_index != 4294967295
            while True:
                k = sum_offset + 3 * sum_index
                prev, ci, wire = (words[k], words[k + 1], words[k + 2])
                assert wire < wire_count
                result[wire] += coef0 * coefficients[ci]
                prev &= 2147483647
                if prev == 2147483647:
                    break
                if prev == 2147483646:
                    sum_index -= 1
                else:
                    sum_index = slot_writers[prev - wire_count]
                assert 0 <= sum_index < sum_count
        return {w: x % prime for w, x in result.items() if x % prime}
    first_product = {}
    product_counts = Counter()
    derived = {}
    boolean_equations = set()

    def affine_factor(expressions, assigned_wire, assigned_coeff):
        """Recognize a single formal wire with a nonzero multiplier and an additive constant."""
        tag, a, b = expressions
        if tag == 4 and b - a > 4:
            return None
        terms = terms_for(tag, a, b)
        if assigned_wire is not None:
            terms.append((assigned_wire, assigned_coeff))
        coeffs = Counter()
        for w, k in terms:
            coeffs[w] += k
        offset = coeffs.pop(0, 0) % prime
        wires = [(w, k % prime) for w, k in coeffs.items() if k % prime]
        if len(wires) != 1:
            return None
        w, k = wires[0]
        return (w, k, offset)

    def read_product_row(at):
        """Decode the two candidate factors including the assignment contribution on its constraint side."""
        ex = [tuple(words[at + i:at + i + 3]) for i in (5, 8, 11)]
        side, wire, assignment_coefficient = words[at + 15:at + 18] if words[at + 14] else (3, 0, 0)
        factors = [affine_factor(ex[z], wire if side == z else None, coefficients[assignment_coefficient]) for z in (0, 1)]
        return (ex, side, wire, assignment_coefficient, factors)

    def linear_side_of_product(ex, side, wire, assignment_coefficient, pairs):
        """Return the linear expression and scale equated to the row’s single quadratic monomial."""
        (wire_a, factor_a, offset_a), (wire_b, factor_b, offset_b) = pairs
        cs = terms_for(*ex[2])
        if side == 2:
            cs.append((wire, coefficients[assignment_coefficient]))
        cs.extend([(0, -offset_a * offset_b), (wire_a, -factor_a * offset_b), (wire_b, -factor_b * offset_a)])
        return (expand(cs), factor_a * factor_b % prime)

    def record_terms(eq, kind):
        """Remove zero coefficients and normalize a nonzero relation before counting distinct equations."""
        eq = {w: k % prime for w, k in eq.items() if k % prime}
        if not eq:
            product_counts[kind + '_identities'] += 1
            return
        pivot = min(eq)
        scale = pow(eq[pivot], -1, prime)
        norm = tuple(((w, k * scale % prime) for w, k in sorted(eq.items())))
        derived[norm] = derived.get(norm, 0) + 1
        product_counts[kind + '_nontrivial'] += 1
        if kind == 'boolean':
            boolean_equations.add(norm)
    for _ in range(operation_count):
        row_pos = cursor
        flag = words[cursor]
        if flag:
            assert flag == 1
            _, hint, lo, hi, first, last = words[cursor:cursor + 6]
            cursor += 6
            if hint == 1:
                hint_bits.update(range(first, last))
                hints.append((lo, hi, first, last))
            continue
        rows_seen += 1
        _, row, lo, hi, slot = words[cursor:cursor + 5]
        assert row < constraint_count and hi <= sum_count
        chain = 0
        for j in range(lo, hi):
            k = sum_offset + 3 * j
            pr, ci, w = (words[k], words[k + 1], words[k + 2])
            prev = pr & 2147483647
            term_constant = coefficients[ci] if w == 0 or coefficients[ci] == 0 else None
            if prev == 2147483647:
                chain = term_constant
            else:
                previous_constant = chain if prev == 2147483646 else slot_constants[prev - wire_count]
                chain = None if previous_constant is None or term_constant is None else (previous_constant + term_constant) % prime
            if pr >> 31:
                assert slot < slot_count and slot_writers[slot] == 4294967295
                slot_writers[slot] = j
                slot_constants[slot] = chain
                slot += 1
        expressions = [tuple(words[cursor + i:cursor + i + 3]) for i in (5, 8, 11)]
        assigned = words[cursor + 14]
        assert assigned in (0, 1)
        side, wire, assignment_coefficient = words[cursor + 15:cursor + 18] if assigned else (3, 0, 0)
        cursor += 26 if assigned else 15
        constant_a = None if side == 0 else expression_constant(*expressions[0])
        constant_b = None if side == 1 else expression_constant(*expressions[1])
        if constant_a is None and constant_b is None:
            factors = [affine_factor(expressions[z], wire if side == z else None, coefficients[assignment_coefficient]) for z in (0, 1)]
            if all(factors):
                wire_a, wire_b = (factors[0][0], factors[1][0])
                key = min(wire_a, wire_b) << 32 | max(wire_a, wire_b)
                product_counts['rows_with_simple_factors'] += 1
                old = first_product.get(key)
                if old is not None or (wire_a == wire_b and wire_a in hint_bits):
                    rhs, scale = linear_side_of_product(expressions, side, wire, assignment_coefficient, factors)
                    if old is not None:
                        prior, mult = linear_side_of_product(*read_product_row(old))
                        eq = Counter({w: t * mult for w, t in rhs.items()})
                        for w, t in prior.items():
                            eq[w] -= t * scale
                        record_terms(eq, 'repeated')
                    if wire_a == wire_b and wire_a in hint_bits:
                        eq = Counter(rhs)
                        eq[wire_a] -= scale
                        record_terms(eq, 'boolean')
                if old is None:
                    first_product[key] = row_pos
            continue
        hs = [terms_for(*e) for e in expressions]
        if assigned:
            hs[side].append((wire, coefficients[assignment_coefficient]))
        source = hs[1] if constant_a is not None else hs[0]
        scale = constant_a if constant_a is not None else constant_b
        relation = expand([(w, scale * k) for w, k in source] + [(w, -k) for w, k in hs[2]])
        if relation:
            linear.append(relation)
            row_types['assigned' if assigned else 'checked'] += 1
        else:
            row_types['tautology'] += 1
    assert cursor == len(words) and rows_seen == constraint_count and (4294967295 not in slot_writers)
    touched = set()
    queries = set()
    nonbool_rows = 0
    span_hist = Counter()
    bool_only = 0
    for rel in linear:
        keys = set(rel) - {0}
        bits = keys.intersection(hint_bits)
        q = keys.intersection(b_query_wires)
        touched.update(q)
        queries.update(q - hint_bits)
        nonbool_rows += bool(q - hint_bits)
        bool_only += bool(keys) and keys == bits
        span_hist[len(keys)] += 1
    hintstats = Counter()
    for lo, hi, first, last in hints:
        termlist = terms_for(4, lo, hi)
        terms = expand(termlist)
        keys = set(terms) - {0}
        width = last - first
        hintstats['total'] += 1
        hintstats['full_width'] += int(width >= prime.bit_length())
        hintstats['one_input_wire'] += int(len(keys) == 1)
        hintstats['one_queried_nonboolean_input'] += int(len(keys) == 1 and bool((keys & b_query_wires) - hint_bits))
        hintstats['full_width_one_queried_nonboolean_input'] += int(width >= prime.bit_length() and len(keys) == 1 and bool((keys & b_query_wires) - hint_bits))
    res = dict(shape=case['shape'], image_sha256=case['image_sha256'], rows=constraint_count, ops=operation_count, b_query=b_count, boolean_b_query=len(b_query_wires & hint_bits), linear=len(linear), forms=dict(row_types), rows_touching_nonboolean_query=nonbool_rows, distinct_nonboolean_query_wires_touched=len(queries), direct_pivot_upper=min(nonbool_rows, len(queries)), distinct_query_wires_touched=len(touched), boolean_only_rows=bool_only, linear_width_histogram=dict(span_hist), bits_hints=dict(hintstats))
    touching = [{w for w, k in eq if w and w in b_query_wires and (w not in hint_bits)} for eq in derived]
    direct_normalized = set()
    for eq in linear:
        inv = pow(eq[min(eq)], -1, prime)
        direct_normalized.add(tuple(((w, k * inv % prime) for w, k in sorted(eq.items()))))
    distinct = len(set().union(*touching)) if touching else 0
    res['products'] = dict(counts=dict(product_counts), distinct_pairs=len(first_product), distinct_relations=len(derived), boolean_distinct=len(boolean_equations), already_in_direct_scan=len(set(derived) & direct_normalized), eqs_touching=sum((bool(q) for q in touching)), wires_touched=distinct, direct_pivot_upper=min(sum((bool(q) for q in touching)), distinct), width_histogram=dict(Counter((sum((w != 0 for w, k in eq)) for eq in derived))))
    del words
    image_data.close()
    file.close()
    return res
if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Count direct linear rows in a Zopro CPU key image')
    parser.add_argument('image', type=Path)
    parser.add_argument('shape')
    parser.add_argument('image_sha256')
    args = parser.parse_args()
    print(json.dumps(scan(args.image, vars(args)), indent=2))
