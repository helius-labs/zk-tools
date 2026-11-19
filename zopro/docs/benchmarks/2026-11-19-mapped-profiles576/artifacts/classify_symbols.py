"""Partition perf IP samples using the demangled symbol that owns each address."""
from collections import Counter
from pathlib import Path
import gzip
import json
import re
import subprocess
import sys

GROUPS = ("G1", "G2", "FFT", "origin", "Rayon iterators", "runtime", "other")


def classify(name):
    symbol = re.sub(r"\[[a-f0-9]{16}\]", "", name).lstrip("<")
    if symbol.startswith("rayon::"):
        return "Rayon iterators"
    if symbol.startswith(("rayon_core::", "crossbeam_")):
        return "runtime"
    if symbol.startswith("zopro_core::fft::"):
        return "FFT"
    if symbol.startswith("zopro_core::lagrange::origin::"):
        return "origin"
    if symbol.startswith(("zopro_core::msm", "zopro_bn254::curve::")):
        if "::G1Config" in symbol or "::stream_g1" in symbol or "::dense_h::" in symbol:
            return "G1"
        if "::G2Config" in symbol or "::g2_" in symbol:
            return "G2"
    return "other"


def main(directory):
    rows = json.loads(gzip.decompress((directory / "processes.json.gz").read_bytes()))
    names = sorted({name for row in rows for name in row["symbols"]})
    result = subprocess.run(
        ["c++filt", "-s", "rust"], input="\n".join(names) + "\n",
        text=True, capture_output=True, check=True,
    )
    names_by_raw = dict(zip(names, result.stdout.splitlines(), strict=True))
    assert all(names_by_raw[n] != n for n in names if n.startswith("_R"))
    groups = {raw: classify(name) for raw, name in names_by_raw.items()}
    summaries = []
    for request in (0, 109, 122):
        for mode in ("ordinary", "mapped"):
            cells = [r for r in rows if (r["v"], r["mode"], r["kind"]) == (request, mode, "record")]
            assert len(cells) == 3
            counts = Counter()
            for row in cells:
                counts.update(row["symbols"])
            totals = Counter()
            for raw, n in counts.items():
                totals[groups[raw]] += n
            assert totals.total() == counts.total() and totals["FFT"] > 0
            summaries.append(dict(v=request, mode=mode, categories=dict(totals),
                                  leading=[(names_by_raw[n], k) for n, k in counts.most_common(20)]))
    (directory / "symbols.json").write_text(json.dumps(summaries, indent=2) + "\n")


if __name__ == "__main__":
    main(Path(sys.argv[1]))
