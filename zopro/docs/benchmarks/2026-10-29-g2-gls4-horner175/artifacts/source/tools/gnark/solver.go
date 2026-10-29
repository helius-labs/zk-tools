package zoprognark

import (
	"bufio"
	"encoding/binary"
	"errors"
	"io"
	"math"

	"github.com/consensys/gnark/constraint"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/constraint/solver"
)

// The hints the kernel solves itself, by the kind its plan names them with.
// Only gnark's own functions qualify: the fast path runs with gnark's hint
// registry, never with hints a caller passes.
var kernelHints = map[string]uint32{
	solver.GetHintName(solver.InvZeroHint):           0,
	"github.com/consensys/gnark/std/math/bits.nBits": 1,
}

// writeKernelSolverProgram streams the kernel's program for a constraint
// system: its R1Cs and the hints of kernelHints, in gnark's dependency
// order; an error for a system the kernel cannot solve (other hints, custom
// blueprints such as GKR's, commitments, Println). The kernel validates the
// dimensions, topology and coefficient table before use.
func writeKernelSolverProgram(out io.Writer, r1cs *cs.R1CS, infinityA, infinityB []bool) error {
	commitments, ok := r1cs.CommitmentInfo.(constraint.Groth16Commitments)
	if !ok || len(commitments) != 0 || len(r1cs.Logs) != 0 || r1cs.Type != constraint.SystemR1CS {
		return errors.New("the kernel does not solve this constraint system alone")
	}
	hints := make(map[solver.HintID]uint32, len(r1cs.MHintsDependencies))
	for id, name := range r1cs.MHintsDependencies {
		kind, ok := kernelHints[name]
		if !ok {
			return errors.New("the kernel does not solve this constraint system alone")
		}
		hints[id] = kind
	}
	nInputs := len(r1cs.Public) + len(r1cs.Secret)
	nWires := nInputs + r1cs.NbInternalVariables
	if len(r1cs.Public) == 0 || len(infinityA) != nWires || len(infinityB) != nWires {
		return errors.New("the kernel does not solve this constraint system alone")
	}
	nHints := 0
	for _, instruction := range r1cs.Instructions {
		if int(instruction.BlueprintID) >= len(r1cs.Blueprints) {
			return errors.New("the kernel does not solve this constraint system alone")
		}
		switch r1cs.Blueprints[instruction.BlueprintID].(type) {
		case *constraint.BlueprintGenericR1C:
		case *constraint.BlueprintGenericHint:
			nHints++
		default:
			return errors.New("the kernel does not solve this constraint system alone")
		}
	}
	if len(r1cs.Instructions) != r1cs.GetNbConstraints()+nHints {
		return errors.New("the kernel does not solve this constraint system alone")
	}
	countIndices := func(infinity []bool) uint32 {
		var count uint32
		for _, zero := range infinity {
			if !zero {
				count++
			}
		}
		return count
	}
	buffer := bufio.NewWriterSize(out, 1<<20)
	var writeErr error
	put := func(values ...uint32) {
		if writeErr != nil {
			return
		}
		var word [4]byte
		for _, value := range values {
			binary.LittleEndian.PutUint32(word[:], value)
			if _, writeErr = buffer.Write(word[:]); writeErr != nil {
				return
			}
		}
	}
	put(2, uint32(nWires), uint32(nInputs), uint32(len(r1cs.Public)),
		uint32(r1cs.GetNbConstraints()), uint32(nHints), countIndices(infinityA), countIndices(infinityB))
	for _, infinity := range [][]bool{infinityA, infinityB} {
		for i, zero := range infinity {
			if !zero {
				put(uint32(i))
			}
			if writeErr != nil {
				return writeErr
			}
		}
	}
	var r constraint.R1C
	var h constraint.HintMapping
	count := 0
	for _, level := range r1cs.Levels {
		for _, id := range level {
			if int(id) >= len(r1cs.Instructions) {
				return errors.New("the kernel does not solve this constraint system alone")
			}
			instruction := r1cs.Instructions[id]
			if hint, ok := r1cs.Blueprints[instruction.BlueprintID].(*constraint.BlueprintGenericHint); ok {
				hint.DecompressHint(&h, instruction.Unpack(&r1cs.System))
				kind, ok := hints[h.HintID]
				if !ok || len(h.Inputs) != 1 {
					return errors.New("the kernel does not solve this constraint system alone")
				}
				put(math.MaxUint32, kind, 1, uint32(len(h.Inputs[0])))
				for _, term := range h.Inputs[0] {
					put(term.CID, term.VID)
				}
				put(h.OutputRange.Start, h.OutputRange.End)
				if writeErr != nil {
					return writeErr
				}
				continue
			}
			blueprint := r1cs.Blueprints[instruction.BlueprintID].(*constraint.BlueprintGenericR1C)
			blueprint.DecompressR1C(&r, instruction.Unpack(&r1cs.System))
			put(instruction.ConstraintOffset, uint32(len(r.L)), uint32(len(r.R)), uint32(len(r.O)))
			for _, expression := range []constraint.LinearExpression{r.L, r.R, r.O} {
				for _, term := range expression {
					put(term.CID, term.VID)
				}
			}
			if writeErr != nil {
				return writeErr
			}
			count++
		}
	}
	if count != r1cs.GetNbConstraints() {
		return errors.New("the kernel does not solve this constraint system alone")
	}
	if writeErr != nil {
		return writeErr
	}
	return buffer.Flush()
}
