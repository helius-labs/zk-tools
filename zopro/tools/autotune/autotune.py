#!/usr/bin/env python3
"""Tunes zopro's runtime knobs for a device, shape by shape.

Coordinate descent over the environment variables the prover reads: from
the defaults, each knob's candidates are timed with the bench, the best
kept when it beats the baseline by MARGIN and a confirming pair of runs
agrees, and the passes repeat until a pass changes nothing. A candidate is
compared with a baseline measured right before its sweep, so a laptop that
warms or a phone that throttles during the search biases neither side.

    tools/autotune/autotune.py mac    --bench target/release/zopro-bench --dumps /tmp/dumps
    tools/autotune/autotune.py android --bench bench-cut
    tools/autotune/autotune.py ios    --udid <udid>

Every run goes to <out>/runs.jsonl; the result to <out>/best.json and
<out>/report.md. Android runs through the phone's run.sh, which waits for
it to cool; iOS through tools/ios-bench/run.sh.
"""
import argparse
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

ADB = os.path.expanduser("~/Library/Android/sdk/platform-tools/adb")
HERE = Path(__file__).resolve().parent
MARGIN = 0.04  # a candidate must beat both adjacent baselines by this much, twice

# Knobs per target: the candidates tried for each ("" is the default).
KNOBS = {
    "mac": {
        "ZOPRO_G2_GPU_SHARE": ["", "0.1", "0.2", "0.3"],
        "ZOPRO_GPU_WIDTH": ["", "12", "14", "15"],
        "ZOPRO_CPU_WIDTH": ["", "13", "14", "15"],
        "ZOPRO_SOLVE_EPOCHS": ["", "8", "32", "64"],
        "ZOPRO_SORT_THREADS": ["", "1", "3", "4"],
        "ZOPRO_METAL_NTT_TILE": ["", "8", "10"],
        "ZOPRO_METAL_NTT_COLS": ["", "2", "4"],
        "RAYON_NUM_THREADS": ["", "12", "14", "16"],
    },
    "ios": {
        "ZOPRO_G2_GPU_SHARE": ["", "0.1", "0.25", "0.4"],
        "ZOPRO_GPU_WIDTH": ["", "11", "12", "14"],
        "ZOPRO_CPU_WIDTH": ["", "13", "14", "15"],
        "ZOPRO_SOLVE_EPOCHS": ["", "4", "8", "32"],
        "ZOPRO_SORT_THREADS": ["", "1", "3"],
        "ZOPRO_METAL_NTT_TILE": ["", "8", "10"],
        "ZOPRO_METAL_NTT_COLS": ["", "2", "4"],
    },
    "android": {
        "ZOPRO_G2_GPU_SHARE": ["", "0.1", "0.25", "0.4"],
        "ZOPRO_GPU_WIDTH": ["", "11", "12", "14"],
        "ZOPRO_GPU_SORT": ["", "0"],
        "ZOPRO_CPU_WIDTH": ["", "13", "14", "15"],
        "ZOPRO_SOLVE_EPOCHS": ["", "4", "8", "32"],
        "ZOPRO_SORT_THREADS": ["", "1", "3"],
        "RAYON_NUM_THREADS": ["", "6", "7"],
    },
    # The CPU path on Android: the shapes an Adreno does not take.
    "android-cpu": {
        "ZOPRO_CPU_WIDTH": ["", "13", "14", "15"],
        "ZOPRO_CPU_SPLIT": ["", "2", "3"],
        "ZOPRO_SOLVE_EPOCHS": ["", "4", "8", "32"],
        "RAYON_NUM_THREADS": ["", "6", "7"],
    },
}


def parse_median(text):
    m = re.search(r"prove_ms_median=(\d+)", text)
    return int(m.group(1)) if m else None


class Target:
    def __init__(self, args):
        self.args = args
        self.kind = args.target
        self.fixed = dict(kv.split("=", 1) for kv in args.env)

    def run(self, shape, env, rounds):
        env = {**self.fixed, **{k: v for k, v in env.items() if v != ""}}
        started = time.time()
        if self.kind == "mac":
            out = subprocess.run(
                [self.args.bench, f"{self.args.dumps}/{shape}", str(rounds)],
                env={**os.environ, **env}, capture_output=True, text=True, timeout=1800,
            )
            text = out.stdout + out.stderr
        elif self.kind in ("android", "android-cpu"):
            prefix = " ".join(f"{k}={v}" for k, v in env.items())
            cmd = f"cd /data/local/tmp/zp && {prefix} LABEL=tune sh run.sh ./{self.args.bench} dump/{shape} {rounds}"
            out = subprocess.run([ADB, "shell", cmd], capture_output=True, text=True, timeout=3600)
            text = out.stdout + out.stderr
        else:  # ios
            child = {f"DEVICETL_CHILD_{k}".replace("DEVICETL", "DEVICECTL"): v for k, v in env.items()}
            out = subprocess.run(
                [str(HERE.parent / "ios-bench" / "run.sh"), self.args.udid, f"dump/{shape}", str(rounds)],
                env={**os.environ, **child}, capture_output=True, text=True, timeout=3600,
            )
            text = out.stdout + out.stderr
        median = parse_median(text)
        failure = None if median is not None else (re.search(r"panicked.*|device lost.*|Error.*", text) or [""])[0][:200]
        return median, failure, time.time() - started


def main():
    p = argparse.ArgumentParser()
    p.add_argument("target", choices=list(KNOBS))
    p.add_argument("--bench", default="target/release/zopro-bench")
    p.add_argument("--dumps", default="/tmp/zopro-dump")
    p.add_argument("--udid")
    p.add_argument("--shapes", default="2x2,8x2,24x2,merge-54x1")
    p.add_argument("--rounds", type=int, default=5)
    p.add_argument("--env", action="append", default=[], help="fixed K=V for every run")
    p.add_argument("--out", default=None)
    p.add_argument("--passes", type=int, default=2)
    args = p.parse_args()
    out = Path(args.out or f"/tmp/autotune-{args.target}")
    out.mkdir(parents=True, exist_ok=True)
    log = open(out / "runs.jsonl", "a")
    target = Target(args)

    def measure(shape, env, why):
        rounds = args.rounds if "54" not in shape else max(3, args.rounds - 2)
        median, failure, seconds = target.run(shape, env, rounds)
        record = {"shape": shape, "env": env, "median_ms": median, "failure": failure, "why": why, "seconds": round(seconds, 1), "at": time.strftime("%H:%M:%S")}
        log.write(json.dumps(record) + "\n")
        log.flush()
        print(f"{record['at']} {shape} {why:<10} {median if median is not None else 'FAIL'} {env}", flush=True)
        return median if median is not None else float("inf")

    best = {}
    for shape in args.shapes.split(","):
        env = {}
        baseline0 = measure(shape, env, "start")
        for pass_ in range(args.passes):
            changed = False
            for knob, values in KNOBS[args.target].items():
                base = measure(shape, env, "base")
                candidates = []
                for v in values:
                    if env.get(knob, "") == v:
                        continue
                    trial = {**env, knob: v}
                    candidates.append((measure(shape, trial, f"try"), v))
                candidates.sort()
                if not candidates or candidates[0][0] >= base * (1 - MARGIN):
                    continue
                v = candidates[0][1]
                # Confirm back to back, candidate first then baseline.
                trial = {**env, knob: v}
                again = measure(shape, trial, "confirm")
                base_again = measure(shape, env, "base2")
                first = candidates[0][0]
                if first < base_again * (1 - MARGIN) and again < min(base, base_again) * (1 - MARGIN):
                    env = trial
                    changed = True
                    print(f"  -> {shape}: {knob}={v or 'default'} ({base:.0f} -> {again:.0f} ms)", flush=True)
            if not changed:
                break
        final = measure(shape, env, "final")
        best[shape] = {"env": env, "start_ms": baseline0, "final_ms": final}
        with open(out / "best.json", "w") as f:
            json.dump(best, f, indent=2)
    lines = ["| shape | start ms | tuned ms | gain | knobs |", "|---|---|---|---|---|"]
    for shape, r in best.items():
        gain = 1 - r["final_ms"] / r["start_ms"] if r["start_ms"] else 0
        knobs = " ".join(f"{k}={v}" for k, v in r["env"].items()) or "defaults"
        lines.append(f"| {shape} | {r['start_ms']:.0f} | {r['final_ms']:.0f} | {gain:+.1%} | {knobs} |")
    (out / "report.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
