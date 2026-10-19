package main

import (
	"bytes"
	"encoding/binary"
	"fmt"

	curve "github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	native "github.com/consensys/gnark/backend/groth16/bn254"
	"zolana/prover/prover/common"
)

// The head of a dump: what `zopro-bench pack` takes beside the point vectors
// and the solver program (the counts, the gnark key's sha256, the fixed
// points, the FFT domain and the raw verifying key). Only systems the kernel
// proves and solves alone, with no commitments, have one.
const keyImageMagic = "zolana-key-head1"

// keyImageFields is the head's fixed part, little-endian, field elements and
// points in the four-limb Montgomery memory the kernel bridge sends.
type keyImageFields struct {
	NInputs, NOutputs, RequiresP256 uint32
	NbPublic, NbSecret, Constraints uint32
	A, B, K, Z, B2                  uint32
	Digest                          [32]byte
	Alpha, Beta, Delta              curve.G1Affine
	Beta2, Delta2                   curve.G2Affine
	Generator, CosetGenerator       fr.Element
	VerifyingKeyLength              uint32
}

// keyImageHeadOf is the head of a loaded system whose proving key still has
// its point vectors.
func keyImageHeadOf(ps *common.TransferProofSystem, pk *native.ProvingKey) ([]byte, error) {
	var vk bytes.Buffer
	if _, err := ps.VerifyingKey.WriteRawTo(&vk); err != nil {
		return nil, fmt.Errorf("encoding verifying key: %w", err)
	}
	fields := keyImageFields{
		NInputs:            ps.NInputs,
		NOutputs:           ps.NOutputs,
		NbPublic:           uint32(ps.ConstraintSystem.GetNbPublicVariables()),
		NbSecret:           uint32(ps.ConstraintSystem.GetNbSecretVariables()),
		Constraints:        uint32(ps.ConstraintSystem.GetNbConstraints()),
		A:                  uint32(len(pk.G1.A)),
		B:                  uint32(len(pk.G1.B)),
		K:                  uint32(len(pk.G1.K)),
		Z:                  uint32(len(pk.G1.Z)),
		B2:                 uint32(len(pk.G2.B)),
		Digest:             ps.ProvingKeySha256,
		Alpha:              pk.G1.Alpha,
		Beta:               pk.G1.Beta,
		Delta:              pk.G1.Delta,
		Beta2:              pk.G2.Beta,
		Delta2:             pk.G2.Delta,
		Generator:          pk.Domain.Generator,
		CosetGenerator:     pk.Domain.FrMultiplicativeGen,
		VerifyingKeyLength: uint32(vk.Len()),
	}
	if ps.RequiresP256 {
		fields.RequiresP256 = 1
	}
	var out bytes.Buffer
	out.WriteString(keyImageMagic)
	if err := binary.Write(&out, binary.LittleEndian, &fields); err != nil {
		return nil, err
	}
	out.Write(vk.Bytes())
	return out.Bytes(), nil
}
