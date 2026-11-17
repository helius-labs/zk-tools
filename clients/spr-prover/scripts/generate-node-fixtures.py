#!/usr/bin/env python3
"""Export deterministic, constraint-checked requests using the pinned protocol helpers.

Needs Go 1.27 and local Git checkouts at the commits in key-catalog-source.json
and runtime.lock.json. Only a temporary, exclusively created Go test is added
to the protocol checkout; it is removed even when generation fails.
"""
import argparse
import gzip
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def git(source, *args):
    return subprocess.check_output(["git", "-C", str(source), *args])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--protocol-source", type=Path, required=True)
    parser.add_argument("--runtime-source", type=Path, required=True)
    args = parser.parse_args()
    protocol = args.protocol_source.resolve()
    runtime = args.runtime_source.resolve()
    protocol_revision = json.loads((ROOT / "key-catalog-source.json").read_text())["revision"]
    runtime_pin = json.loads((ROOT / "runtime.lock.json").read_text())["zopro"]
    assert git(protocol, "rev-parse", "HEAD").decode().strip() == protocol_revision
    subprocess.run(["git", "-C", str(protocol), "diff", "--exit-code", "HEAD", "--", "prover/server"], check=True)
    keys = json.loads((ROOT / "zopro-keys.lock").read_text())["keys"]
    transfers = sorted(tuple(map(int, match.groups())) for key in keys
                       if (match := re.fullmatch(r"transfer_confidential_(\d+)_(\d+)\.key", key)))
    package = protocol / "prover/server/circuits/spp_transaction/shared"
    original = (package / "request_fixture_test.go").read_text()
    body = original.split("func TestRequestFixture(t *testing.T) {", 1)[1].split(
        '\tif os.Getenv("UPDATE_REQUEST_FIXTURE")', 1)[0]
    body = body.replace("protocol.Shape{NInputs: 2, NOutputs: 2}", "shape")
    shapes = ",\n".join(f"{{NInputs: {i}, NOutputs: {o}}}" for i, o in transfers)
    source = '''package shared_test
import (
 "encoding/json"
 "fmt"
 "math/big"
 "os"
 "path/filepath"
 "testing"
 "testing/cryptotest"
 "github.com/consensys/gnark-crypto/ecc"
 "github.com/consensys/gnark/test"
 defaultring "github.com/helius-labs/zolana/prover/server/circuits/spp_transaction/default"
 "github.com/helius-labs/zolana/prover/server/prover-test/spp/protocol"
 transfer "github.com/helius-labs/zolana/prover/server/prover/transfer_eddsa"
)
func TestNodeBenchmarkFixtures(t *testing.T) {
 for _, shape := range []protocol.Shape{
''' + shapes + ''',
 } {
  t.Run(fmt.Sprintf("%dx%d", shape.NInputs, shape.NOutputs), func(t *testing.T) {
''' + body + '''
   path := filepath.Join(os.Getenv("SPR_FIXTURE_OUTPUT"), fmt.Sprintf("transfer-%dx%d.json", shape.NInputs, shape.NOutputs))
   if err := os.WriteFile(path, data, 0600); err != nil { t.Fatal(err) }
  })
 }
}
'''
    test_file = package / "node_benchmark_export_test.go"
    with tempfile.TemporaryDirectory(prefix="spr-node-fixtures-") as directory:
        with test_file.open("x") as output:
            output.write(source)
        try:
            subprocess.run(["go", "test", "./circuits/spp_transaction/shared", "-run", "^TestNodeBenchmarkFixtures$", "-count=1", "-timeout", "20m"],
                           cwd=protocol / "prover/server", env={**os.environ, "SPR_FIXTURE_OUTPUT": directory}, check=True)
        finally:
            test_file.unlink()
        requests = {f"{i}x{o}": json.loads((Path(directory) / f"transfer-{i}x{o}.json").read_text()) for i, o in transfers}
        # Merge fixtures already exist at the pinned runtime revision.
        for key in keys:
            match = re.fullmatch(r"merge_(\d+)_1\.key", key)
            if match and int(match[1]) <= 24:
                shape = f"merge-{match[1]}x1"
                path = f"{runtime_pin['path']}/tools/spr/fixtures/{shape}.json"
                requests[shape] = json.loads(git(runtime, "show", f"{runtime_pin['revision']}:{path}"))
        bundle = {"protocolRevision": protocol_revision, "runtimeRevision": runtime_pin["revision"], "requests": requests}
        data = json.dumps(bundle, sort_keys=True, separators=(",", ":")).encode()
        target = ROOT / "fixtures/node-shapes.json.gz"
        target.write_bytes(gzip.compress(data, mtime=0))
        print(f"Wrote {len(requests)} requests to {target} ({len(data)} bytes before compression)")


if __name__ == "__main__":
    main()
