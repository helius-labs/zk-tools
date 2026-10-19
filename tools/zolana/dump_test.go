package zolana

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
	"time"

	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark/backend/groth16"
	native "github.com/consensys/gnark/backend/groth16/bn254"
	"github.com/consensys/gnark/backend/witness"
	cs "github.com/consensys/gnark/constraint/bn254"
	zoprognark "zopro/tools/gnark"
	"zolana/prover/prover/common"
	mergeprover "zolana/prover/prover/merge"
	transfer "zolana/prover/prover/transfer_eddsa_only"
)

// Zolana's gnark keys for zopro: ZOPRO_DUMP=dir ZOPRO_KEYS=<gnark keys>
// ZOPRO_SHAPES=2x2,merge-24x1 writes each key's dump (zoprognark.Dump) to
// dir/<shape>, with the key's Zolana metadata: its shape and the sha256 of
// the key file, which proofs report. With ZOPRO_REQUESTS, a directory of
// requests (transfer-<shape>.json, merge-<inputs>x1.json), it also writes the
// request's witness and public inputs and times gnark's own proof at the
// GOMAXPROCS the test runs with; a raw proof zopro wrote to dir/<shape>/proof
// is verified by gnark instead.
func TestZoproDump(t *testing.T) {
	dir := os.Getenv("ZOPRO_DUMP")
	if dir == "" {
		t.Skip("set ZOPRO_DUMP")
	}
	keys := os.Getenv("ZOPRO_KEYS")
	if keys == "" {
		t.Fatal("set ZOPRO_KEYS to a directory of gnark proving keys")
	}
	requests := os.Getenv("ZOPRO_REQUESTS")
	shapes := strings.Split(os.Getenv("ZOPRO_SHAPES"), ",")
	for _, shape := range shapes {
		if shape == "" {
			continue
		}
		t.Run(shape, func(t *testing.T) {
			merge := strings.HasPrefix(shape, "merge-")
			bare := strings.TrimPrefix(shape, "merge-")
			keyName := "transfer_confidential_" + strings.ReplaceAll(bare, "x", "_") + ".key"
			requestName := "transfer-" + shape + ".json"
			if merge {
				keyName = "merge_" + strings.ReplaceAll(bare, "x", "_") + ".key"
				requestName = shape + ".json"
			}
			key, err := os.ReadFile(filepath.Join(keys, keyName))
			if err != nil {
				t.Skip(err)
			}
			// Without a request only the key's parts are written, enough
			// to pack the zopro key.
			var request []byte
			if requests != "" {
				request, err = os.ReadFile(filepath.Join(requests, requestName))
				if err != nil && !os.IsNotExist(err) {
					t.Fatal(err)
				}
			}
			var ps common.TransferProofSystem
			if _, err := ps.UnsafeReadFrom(bytes.NewReader(key)); err != nil {
				t.Fatal(err)
			}
			// What the key manager records on a load: proofs report it.
			ps.ProvingKeySha256 = sha256.Sum256(key)
			var w witness.Witness
			if request != nil {
				var params parameters
				if merge {
					p := new(mergeprover.MergeParameters)
					if err := json.Unmarshal(request, p); err != nil {
						t.Fatal(err)
					}
					params = p
				} else {
					p := new(transfer.TransferParameters)
					if err := json.Unmarshal(request, p); err != nil {
						t.Fatal(err)
					}
					params = p
				}
				w, err = assignmentWitness(params)
				if err != nil {
					t.Fatal(err)
				}
			}
			pk := ps.ProvingKey.(*native.ProvingKey)
			r1cs := ps.ConstraintSystem.(*cs.R1CS)
			out := filepath.Join(dir, shape)
			if err := os.MkdirAll(out, 0o755); err != nil {
				t.Fatal(err)
			}
			if proof, err := os.ReadFile(filepath.Join(out, "proof")); err == nil && w != nil {
				verifyZoproProof(t, proof, &ps, w)
				return
			}
			metadata, err := json.Marshal(map[string]any{
				"inputs":    ps.NInputs,
				"outputs":   ps.NOutputs,
				"keySha256": hex.EncodeToString(ps.ProvingKeySha256[:]),
			})
			if err != nil {
				t.Fatal(err)
			}
			vk := ps.VerifyingKey.(*native.VerifyingKey)
			if err := zoprognark.Dump(out, r1cs, pk, vk, metadata); err != nil {
				t.Fatal(err)
			}
			if w != nil {
				public, err := w.Public()
				if err != nil {
					t.Fatal(err)
				}
				for name, data := range map[string][]byte{
					"witness": zoprognark.EncodeFields(w.Vector().(fr.Vector)),
					"public":  zoprognark.EncodeFields(public.Vector().(fr.Vector)),
				} {
					if err := os.WriteFile(filepath.Join(out, name), data, 0o644); err != nil {
						t.Fatal(err)
					}
				}
			}
			if w == nil {
				return
			}
			// gnark's own proof at this GOMAXPROCS, the comparison.
			rounds := 3
			if r1cs.GetNbConstraints() > 300000 {
				rounds = 2
			}
			var best time.Duration
			for i := 0; i < rounds; i++ {
				started := time.Now()
				if _, err := groth16.Prove(r1cs, pk, w); err != nil {
					t.Fatal(err)
				}
				if elapsed := time.Since(started); best == 0 || elapsed < best {
					best = elapsed
				}
			}
			t.Logf("ZOPRO gnark %s constraints=%d domain=%d threads=%d prove_ms=%d", shape, r1cs.GetNbConstraints(), pk.Domain.Cardinality, runtime.GOMAXPROCS(0), best.Milliseconds())
		})
	}
}

// A proof as the Rust bench writes it: Ar, Krs and Bs in gnark's raw
// big-endian encoding.
func verifyZoproProof(t *testing.T, raw []byte, ps *common.TransferProofSystem, w witness.Witness) {
	t.Helper()
	if len(raw) != 64+64+128 {
		t.Fatalf("a raw proof is 256 bytes, got %d", len(raw))
	}
	var proof native.Proof
	if _, err := proof.Ar.SetBytes(raw[:64]); err != nil {
		t.Fatalf("Ar: %v", err)
	}
	if _, err := proof.Krs.SetBytes(raw[64:128]); err != nil {
		t.Fatalf("Krs: %v", err)
	}
	if _, err := proof.Bs.SetBytes(raw[128:]); err != nil {
		t.Fatalf("Bs: %v", err)
	}
	public, err := w.Public()
	if err != nil {
		t.Fatal(err)
	}
	if err := groth16.Verify(&proof, ps.VerifyingKey, public); err != nil {
		t.Fatalf("the Rust proof does not verify: %v", err)
	}
	t.Logf("ZOPRO verified the Rust proof")
}
