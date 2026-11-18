package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/big"
	"os"
	"path/filepath"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/frontend"
	"zolana/prover/prover-test/spp/protocol"
	transfer "zolana/prover/prover/transfer_eddsa_only"
)

func must[T any](value T, err error) T {
	if err != nil {
		panic(err)
	}
	return value
}

func update(p *transfer.TransferParameters) {
	inputs, nullifiers, addresses := []*big.Int{}, []*big.Int{}, []*big.Int{}
	for _, in := range p.Inputs {
		if in.IsDummy.Sign() != 0 {
			panic("only real inputs expected")
		}
		slot := p.TreeSlots[in.TreeSlot.Int64()]
		inputs = append(inputs, must(protocol.UtxoHash(protocol.Utxo(in.Utxo), slot.ID)))
		nullifiers = append(nullifiers, in.Nullifier)
		addresses = append(addresses, big.NewInt(0))
	}
	seed := must(protocol.OutputBlindingSeed(nullifiers[0], p.BlindingSeed))
	outputs := []*big.Int{}
	for i := range p.Outputs {
		o := &p.Outputs[i]
		if o.IsDummy.Sign() != 0 {
			panic("only real outputs expected")
		}
		o.Utxo.Blinding = must(protocol.OutputBlinding(nullifiers[0], seed, i))
		o.Hash = must(protocol.UtxoHash(protocol.Utxo(o.Utxo), p.OutputTreeID))
		outputs = append(outputs, o.Hash)
	}
	p.PrivateTxHash = must(protocol.PrivateTxHash(inputs, outputs, addresses, must(protocol.PrivateTxBlinding(nullifiers[0], p.BlindingSeed))))
	pub := protocol.PublicInputs{Nullifiers: nullifiers, OutputUtxoHashes: outputs, OutputTreeID: p.OutputTreeID, PrivateTxHash: p.PrivateTxHash,
		ExternalDataHash: p.ExternalDataHash, RingProgramID: p.RingProgramID, SignerPkHashes: p.SignerPkHashes, InputFlags: p.InputFlags,
		BindOutputOwnerTags: true, OutputOwnerPkHashes: p.PublishedOutputOwnerPkHashes, PreimageTail: []*big.Int{p.Cache.TreeID, p.Cache.ReadHashChain}}
	copy(pub.PublicAssets[:], p.PublicAssets)
	copy(pub.PublicAmounts[:], p.PublicAmounts)
	for _, slot := range p.TreeSlots {
		pub.TreeSlots = append(pub.TreeSlots, protocol.TreeSlot(slot))
	}
	p.PublicInputHash = must(protocol.PublicInputHash(pub))
}

func main() {
	raw := must(os.ReadFile(os.Args[1]))
	dest := os.Args[2]
	for id := 0; id <= 8; id++ {
		var p transfer.TransferParameters
		if err := json.Unmarshal(raw, &p); err != nil {
			panic(err)
		}
		initial := new(big.Int).Set(p.PublicInputHash)
		if id > 0 {
			offset := big.NewInt(int64(11 * id))
			p.BlindingSeed = new(big.Int).Add(p.BlindingSeed, big.NewInt(int64(1009*id)))
			p.Outputs[0].Utxo.Amount = new(big.Int).Sub(p.Outputs[0].Utxo.Amount, offset)
			p.Outputs[1].Utxo.Amount = new(big.Int).Add(p.Outputs[1].Utxo.Amount, offset)
		}
		update(&p)
		if (p.PublicInputHash.Cmp(initial) == 0) != (id == 0) {
			panic(fmt.Sprintf("baseline public hash mismatch on variant %d", id))
		}
		witness := must(frontend.NewWitness(must(p.CreateWitness()), ecc.BN254.ScalarField()))
		pub := must(must(witness.Public()).MarshalBinary())
		// Restore the circuit discriminator supplied by the SDK caller.
		var template, encoded map[string]json.RawMessage
		if err := json.Unmarshal(raw, &template); err != nil {
			panic(err)
		}
		if err := json.Unmarshal(must(json.Marshal(&p)), &encoded); err != nil {
			panic(err)
		}
		encoded["circuitType"] = template["circuitType"]
		pth := filepath.Join(dest, fmt.Sprintf("v%d", id))
		if err := os.MkdirAll(pth, 0755); err != nil {
			panic(err)
		}
		if err := os.WriteFile(filepath.Join(pth, "request.json"), must(json.Marshal(encoded)), 0644); err != nil {
			panic(err)
		}
		if err := os.WriteFile(filepath.Join(pth, "public.bin"), bytes.Clone(pub), 0644); err != nil {
			panic(err)
		}
	}
}
