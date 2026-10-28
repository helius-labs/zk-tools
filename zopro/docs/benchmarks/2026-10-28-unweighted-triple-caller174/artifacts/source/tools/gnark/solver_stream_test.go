package zoprognark

import (
	"errors"
	"os"
	"path/filepath"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	native "github.com/consensys/gnark/backend/groth16/bn254"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/constraint/solver"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
)

type failingProgramWriter struct{ err error }

func (w failingProgramWriter) Write([]byte) (int, error) { return 0, w.err }

func TestStreamingProgramReportsWriteFailure(t *testing.T) {
	compiled, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &cubic{})
	if err != nil {
		t.Fatal(err)
	}
	system := compiled.(*cs.R1CS)
	n := len(system.Public) + len(system.Secret) + system.NbInternalVariables
	failure := errors.New("program output failed")
	err = writeKernelSolverProgram(failingProgramWriter{failure}, system, make([]bool, n), make([]bool, n))
	if !errors.Is(err, failure) {
		t.Fatalf("write error: %v", err)
	}
}

func TestUnsupportedProgramPreservesExistingFile(t *testing.T) {
	compiled, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &cubic{})
	if err != nil {
		t.Fatal(err)
	}
	system := compiled.(*cs.R1CS)
	system.MHintsDependencies[solver.HintID(0xffffffff)] = "unsupported-test-hint"
	dir := t.TempDir()
	path := filepath.Join(dir, "program")
	if err := os.WriteFile(path, []byte("existing program"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := DumpSolver(dir, system, &native.ProvingKey{}, &native.VerifyingKey{}, nil); err == nil {
		t.Fatal("unsupported hint accepted")
	}
	got, err := os.ReadFile(path)
	if err != nil || string(got) != "existing program" {
		t.Fatalf("existing file changed: %q, %v", got, err)
	}
	temporary, err := filepath.Glob(filepath.Join(dir, ".program-*"))
	if err != nil || len(temporary) != 0 {
		t.Fatalf("temporary files: %v, %v", temporary, err)
	}
}
