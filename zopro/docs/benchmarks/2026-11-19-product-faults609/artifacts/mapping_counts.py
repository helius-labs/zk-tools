"""Join first-layer page-fault samples to preceding perf MMAP2 records."""
import argparse
from collections import Counter, defaultdict
import json
from pathlib import Path
import re
import subprocess

MAPPING = re.compile(r'PERF_RECORD_MMAP2 (\d+)/\d+: \[0x([a-f0-9]+)\(0x([a-f0-9]+)\).*?\]: (\S+) (.*)')
FAULT = re.compile(r'^\s*(\d+)/\d+\s+([\d.]+): minor-faults:u:\s+([a-f0-9]+)\s')
PACK = 'pack_inverse_columns'
SIZE = (64 << 20) + 4096

def inspect(data):
    lines = subprocess.check_output([
        'sudo', '-n', 'perf', 'script', '--ns', '--show-mmap-events', '-i', data,
        '-F', 'pid,tid,time,event,addr,sym,ip',
    ], text=True).splitlines()
    mappings = defaultdict(list)
    hit_records = set()
    hit_addresses = set()
    pack = matched = 0
    for index, line in enumerate(lines):
        mapping = MAPPING.search(line)
        if mapping:
            pid, start, size, perm, name = mapping.groups()
            mappings[pid].append((int(start, 16), int(size, 16), perm, name, index))
            continue
        fault = FAULT.match(line)
        if fault is None or PACK not in lines[index + 1]:
            continue
        pack += 1
        pid, _, address = fault.groups()
        addr = int(address, 16)
        for start, size, perm, name, record in reversed(mappings[pid]):
            if start <= addr < start + size:
                if size == SIZE and perm == 'rw-p' and name == '//anon':
                    matched += 1
                    hit_records.add(record)
                    hit_addresses.add(start)
                break
    return dict(pack_samples=pack, matching_samples=matched,
                matching_records=len(hit_records), distinct_ranges=len(hit_addresses))

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path, help='capture directory containing rows.json and output/')
    args = parser.parse_args()
    rows = json.loads((args.output / 'rows.json').read_text())
    result = []
    for row in rows:
        if row['sh'] == '24x2':
            block, mode = row['b'], row['m']
            data = args.output / 'output' / f'{block}-24x2-{mode}' / 'perf.data'
            result.append(dict(block=block, mode=mode, **inspect(data)))
    print(json.dumps(result, indent=2))
