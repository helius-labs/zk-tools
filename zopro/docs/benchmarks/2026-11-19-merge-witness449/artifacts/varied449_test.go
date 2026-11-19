package merge_test

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
	"zolana/prover/prover-test/spp/spptest"
	"zolana/prover/prover/common"
	mp "zolana/prover/prover/merge"
)

func TestVariedMerge449(t *testing.T) {
	out := os.Getenv("MERGE449_INPUTS")
	if out == "" {
		t.Fatal("MERGE449_INPUTS required")
	}
	for id := int64(1); id <= 8; id++ {
		f := buildMergeFixture(t, mergeFixtureOptions{inputCount: 24, eddsa: true, variant: id, externalDataHash: big.NewInt(900 + id)})
		c := f.defaultCircuit()
		v := spptest.AsBigInt
		vs := spptest.ToBigInts
		p := &mp.MergeParameters{CircuitType: common.MergeCircuitType, Asset: f.asset, OwnerPkHash: f.ownerPkHash, UserNullifierPk: f.userNullifierPk, UserNullifierSecret: f.userNullifierSecret, RingProgramID: big.NewInt(0), OutputRingDataHash: big.NewInt(0), Output: mp.OutputParams{RingDataHash: v(c.Output.RingDataHash), Hash: v(c.OutputHash)}, OutputTreeID: v(c.OutputTreeID), ExternalDataHash: v(c.ExternalDataHash), PrivateTxHash: v(c.PrivateTxHash), AllowDummyInputs: v(c.AllowDummyInputs), PublicInputHash: v(c.PublicInputHash)}
		for _, s := range c.TreeSlots {
			p.TreeSlots = append(p.TreeSlots, common.TreeSlotParams{ID: v(s.ID), UtxoRoot: v(s.UtxoRoot), NullifierRoot: v(s.NullifierRoot)})
		}
		for i, in := range c.Inputs {
			p.Inputs = append(p.Inputs, mp.InputParams{Domain: v(in.Domain), Amount: v(in.Amount), Blinding: v(in.Blinding), RingDataHash: v(in.RingDataHash), StatePathElements: vs(in.StatePathElements), StatePathIndex: v(in.StatePathIndex), NullifierLowValue: v(in.NullifierLowValue), NullifierNextValue: v(in.NullifierNextValue), NullifierLowPathElements: vs(in.NullifierLowPathElements), NullifierLowPathIndex: v(in.NullifierLowPathIndex), TreeSlot: v(in.TreeSlot), Nullifier: v(c.Nullifiers[i])})
		}
		data, e := json.Marshal(p)
		if e != nil {
			t.Fatal(e)
		}
		var decoded mp.MergeParameters
		if e = json.Unmarshal(data, &decoded); e != nil {
			t.Fatal(e)
		}
		if e = decoded.ValidateShape(); e != nil {
			t.Fatal(e)
		}
		w, e := decoded.CreateWitness()
		if e != nil {
			t.Fatal(e)
		}
		full, e := frontend.NewWitness(w, ecc.BN254.ScalarField())
		if e != nil {
			t.Fatal(e)
		}
		pub, e := full.Public()
		if e != nil {
			t.Fatal(e)
		}
		var b bytes.Buffer
		if _, e = pub.WriteTo(&b); e != nil {
			t.Fatal(e)
		}
		var body map[string]json.RawMessage
		if e = json.Unmarshal(data, &body); e != nil {
			t.Fatal(e)
		}
		body["circuitType"] = json.RawMessage(`"merge"`)
		data, e = json.Marshal(body)
		if e != nil {
			t.Fatal(e)
		}
		for suffix, contents := range map[string][]byte{".json": data, ".public": b.Bytes()} {
			path := filepath.Join(out, fmt.Sprintf("v%d%s", id, suffix))
			if _, e = os.Stat(path); !os.IsNotExist(e) {
				t.Fatal("output exists")
			}
			if e = os.WriteFile(path, contents, 0600); e != nil {
				t.Fatal(e)
			}
		}
	}

}
