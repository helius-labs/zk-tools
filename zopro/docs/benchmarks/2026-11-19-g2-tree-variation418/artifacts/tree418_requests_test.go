package shared_test

import (
	"bytes"
	"encoding/json"
	"fmt"
	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/frontend"
	"math/big"
	"os"
	"path/filepath"
	"testing"
	"testing/cryptotest"
	defaultring "zolana/prover/circuits/spp_transaction/default"

	"zolana/prover/prover-test/spp/protocol"
	"zolana/prover/prover-test/spp/spptest"
	transfer "zolana/prover/prover/transfer_eddsa_only"
)

func TestTree418(t *testing.T) {
	cryptotest.SetGlobalRandom(t, 418)
	out := os.Getenv("ZOPRO_TREE418_OUTPUT")
	if out == "" {
		t.Fatal("output required")
	}
	for _, shape := range protocol.SupportedShapes {
		if shape.NInputs != 1 || shape.NOutputs != 2 {
			continue
		}
		for id := 1; id <= 8; id++ {
			a := buildDefaultRingEddsaOnlyAssignment(t, shape)
			a.BlindingSeed = big.NewInt(int64(11800 + id*709))
			a.Inputs[0].Utxo.Blinding = big.NewInt(int64(774332 + id*3049))
			delta := big.NewInt(int64(101 * id))
			a.Inputs[0].Utxo.Amount = new(big.Int).Add(spptest.AsBigInt(a.Inputs[0].Utxo.Amount), delta)
			a.Outputs[0].Utxo.Amount = new(big.Int).Add(spptest.AsBigInt(a.Outputs[0].Utxo.Amount), delta)
			refreshTree418(t, a, id)
			refreshDefaultRingPublicInputHash(t, a)
			c := asDefaultRingEddsaOnly(a).(*defaultring.DefaultRingEddsaOnlyCircuit)
			c.Shape = a.Shape
			params := requestParameters(c)
			raw, err := json.Marshal(params)
			if err != nil {
				t.Fatal(err)
			}
			var decoded transfer.TransferParameters
			if err = json.Unmarshal(raw, &decoded); err != nil {
				t.Fatal(err)
			}
			if err = decoded.ValidateShape(); err != nil {
				t.Fatal(err)
			}
			wit, err := decoded.CreateWitness()
			if err != nil {
				t.Fatal(err)
			}
			full, err := frontend.NewWitness(wit, ecc.BN254.ScalarField())
			if err != nil {
				t.Fatal(err)
			}
			pub, err := full.Public()
			if err != nil {
				t.Fatal(err)
			}
			var buf bytes.Buffer
			if _, err = pub.WriteTo(&buf); err != nil {
				t.Fatal(err)
			}
			var body map[string]json.RawMessage
			if err = json.Unmarshal(raw, &body); err != nil {
				t.Fatal(err)
			}
			body["circuitType"] = json.RawMessage(`"transfer-confidential"`)
			raw, err = json.Marshal(body)
			if err != nil {
				t.Fatal(err)
			}
			dir := filepath.Join(out, fmt.Sprintf("v%d", id))
			if err = os.MkdirAll(dir, 0700); err != nil {
				t.Fatal(err)
			}
			if err = os.WriteFile(filepath.Join(dir, "request.json"), raw, 0600); err != nil {
				t.Fatal(err)
			}
			if err = os.WriteFile(filepath.Join(dir, "public.bin"), buf.Bytes(), 0600); err != nil {
				t.Fatal(err)
			}
		}
	}
}
func refreshTree418(t testing.TB, assignment *testAssignment, id int) {
	t.Helper()
	inputHashes := make([]*big.Int, len(assignment.Inputs))
	stateEntries := make(map[uint64]*big.Int, len(assignment.Inputs))
	for i := range assignment.Inputs {
		inputHash := testUtxoHash(t, circuitFieldsToUtxo(assignment.Inputs[i].Utxo), assignment.inputTreeID(i))
		inputHashes[i] = inputHash
		stateEntries[(uint64(17 + i + 1024*id))] = inputHash
	}
	// A different sibling subtree at each of the low 24 levels changes the path.
	for level := 0; level < 24; level++ {
		key := uint64(17+1024*id) ^ (uint64(1) << level)
		stateEntries[key] = big.NewInt(int64(100000*id + level + 1))
	}
	stateRoot, stateProofs := spptest.MustBuildSparseStateTree(t, stateEntries)
	for k := range assignment.TreeSlots {
		assignment.TreeSlots[k].UtxoRoot = stateRoot
	}
	nullifierTree := spptest.MustNewNullifierTree(t)
	for j := 0; j < 8+id; j++ {
		v := new(big.Int).Lsh(big.NewInt(int64(id*121+j+1)), 187)
		if err := nullifierTree.Insert(v); err != nil {
			t.Fatal(err)
		}
	}
	for i := range assignment.Inputs {
		stateProof := stateProofs[(uint64(17 + i + 1024*id))]
		fillStateProofElements(assignment.Inputs[i].StatePathElements, stateProof.PathElements)
		assignment.Inputs[i].StatePathIndex = new(big.Int).SetUint64(stateProof.PathIndex)

		nullifier := spptest.MustNullifier(
			t,
			inputHashes[i],
			spptest.AsBigInt(assignment.Inputs[i].Utxo.Blinding),
			spptest.AsBigInt(assignment.Inputs[i].NullifierSecret),
		)
		assignment.Inputs[i].Nullifier = nullifier
		nfWitness := spptest.MustNonInclusion(t, nullifierTree, nullifier)
		assignment.Inputs[i].NullifierLowValue = nfWitness.LowValue
		assignment.Inputs[i].NullifierNextValue = nfWitness.NextValue
		fillStateProofElements(assignment.Inputs[i].NullifierLowPathElements, nfWitness.PathElements)
		assignment.Inputs[i].NullifierLowPathIndex = new(big.Int).SetUint64(nfWitness.LowIndex)
	}
	for k := range assignment.TreeSlots {
		assignment.TreeSlots[k].NullifierRoot = nullifierTree.Root()
	}
	refreshDerivedOutputBlindings(t, assignment)

	OutputHashes := spptest.ToBigInts(assignment.OutputHashes())
	privateTxHash := spptest.MustPrivateTxHash(
		t,
		inputHashes,
		OutputHashes,
		noAddressNullifiers(len(inputHashes)),
		assignment.privateTxBlinding(t),
	)
	assignment.PrivateTxHash = privateTxHash
	refreshPublicInputHash(t, assignment)
}
