// Package zoprognark writes a gnark Groth16 BN254 proving key's parts as
// zopro's kernel takes them (a dump), for `zopro-bench pack` to make a
// zopro key of. See README.md.
package zoprognark

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	curve "github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	native "github.com/consensys/gnark/backend/groth16/bn254"
	cs "github.com/consensys/gnark/constraint/bn254"
)

// The head's magic, as zopro_core::dump reads it.
const headMagic = "zopro-dump\x00\x00\x00\x00\x00\x00"

// headFields is the head's fixed part, little endian: the counts, then the
// fixed points and the domain in the kernel's encoding (four little-endian
// Montgomery limbs per coordinate, G2's c0 before c1).
type headFields struct {
	NbPublic, NbSecret, Constraints, VerifyingKeyLength uint32
	Alpha, Beta, Delta                                  curve.G1Affine
	Beta2, Delta2                                       curve.G2Affine
	Generator, CosetGenerator                           fr.Element
}

// Dump writes dir/{head,a,b,k,z,b2,program,coefficients,public-names,
// secret-names} for a constraint system and its keys, and dir/metadata when
// metadata is not empty: the application's, kept in the zopro key as it is.
// It fails for a system the kernel cannot prove alone (commitments, or
// hints other than gnark's InvZeroHint and bits.nBits).
func Dump(dir string, r1cs *cs.R1CS, pk *native.ProvingKey, vk *native.VerifyingKey, metadata []byte) error {
	if err := DumpSolver(dir, r1cs, pk, vk, metadata); err != nil {
		return err
	}
	for name, data := range map[string][]byte{
		"a": bytesOf(pk.G1.A), "b": bytesOf(pk.G1.B), "k": bytesOf(pk.G1.K),
		"z": bytesOf(pk.G1.Z), "b2": bytesOf(pk.G2.B),
	} {
		if err := os.WriteFile(filepath.Join(dir, name), data, 0o644); err != nil {
			return err
		}
	}
	return nil
}

// DumpSolver writes the non-vector dump files. The caller supplies a matching
// set of a/b/k/z/b2 files separately; pk needs only its fixed points, domain and
// infinity masks. This lets an importer release point arrays before loading
// the constraint system. The program is written incrementally.
func DumpSolver(dir string, r1cs *cs.R1CS, pk *native.ProvingKey, vk *native.VerifyingKey, metadata []byte) error {
	if len(r1cs.Public) == 0 || r1cs.Public[0] != "1" {
		return errors.New("the first public input is not the constant one")
	}
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	program, err := os.CreateTemp(dir, ".program-*")
	if err != nil {
		return err
	}
	defer os.Remove(program.Name())
	writeErr := writeKernelSolverProgram(program, r1cs, pk.InfinityA, pk.InfinityB)
	if writeErr == nil {
		writeErr = program.Chmod(0o644)
	}
	closeErr := program.Close()
	if writeErr != nil {
		return writeErr
	}
	if closeErr != nil {
		return closeErr
	}
	if err := os.Rename(program.Name(), filepath.Join(dir, "program")); err != nil {
		return err
	}
	var raw bytes.Buffer
	if _, err := vk.WriteRawTo(&raw); err != nil {
		return fmt.Errorf("encoding the verifying key: %w", err)
	}
	fields := headFields{
		NbPublic:           uint32(r1cs.GetNbPublicVariables()),
		NbSecret:           uint32(r1cs.GetNbSecretVariables()),
		Constraints:        uint32(r1cs.GetNbConstraints()),
		VerifyingKeyLength: uint32(raw.Len()),
		Alpha:              pk.G1.Alpha,
		Beta:               pk.G1.Beta,
		Delta:              pk.G1.Delta,
		Beta2:              pk.G2.Beta,
		Delta2:             pk.G2.Delta,
		Generator:          pk.Domain.Generator,
		CosetGenerator:     pk.Domain.FrMultiplicativeGen,
	}
	var head bytes.Buffer
	head.WriteString(headMagic)
	if err := binary.Write(&head, binary.LittleEndian, &fields); err != nil {
		return err
	}
	head.Write(raw.Bytes())
	files := map[string][]byte{
		"head":         head.Bytes(),
		"coefficients": EncodeFields(r1cs.Coefficients),
		// The inputs' names in gnark's order, the constant one apart: the
		// prover takes values by name.
		"public-names": []byte(strings.Join(r1cs.Public[1:], "\n") + "\n"),
		"secret-names": []byte(strings.Join(r1cs.Secret, "\n") + "\n"),
	}
	if len(metadata) > 0 {
		files["metadata"] = metadata
	}
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	for name, data := range files {
		if err := os.WriteFile(filepath.Join(dir, name), data, 0o644); err != nil {
			return err
		}
	}
	return nil
}
