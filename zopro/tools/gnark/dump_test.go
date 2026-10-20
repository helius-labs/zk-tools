package zoprognark

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark/backend/groth16"
	native "github.com/consensys/gnark/backend/groth16/bn254"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
)

// cubic proves knowledge of X with X³ + X + 5 = Y, and that X is not zero
// (gnark's IsZero, which the kernel solves with its InvZeroHint).
type cubic struct {
	X frontend.Variable
	Y frontend.Variable `gnark:",public"`
}

func (c *cubic) Define(api frontend.API) error {
	x3 := api.Mul(c.X, c.X, c.X)
	api.AssertIsEqual(api.Add(x3, c.X, 5), c.Y)
	api.AssertIsEqual(api.IsZero(c.X), 0)
	return nil
}

// The cubic circuit's dump, with the witness of X = 3 and its public input:
// into ZOPRO_CUBIC when set (zopro-prover's test fixture), else a
// temporary directory.
func TestDumpsACircuit(t *testing.T) {
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &cubic{})
	if err != nil {
		t.Fatal(err)
	}
	pk, vk, err := groth16.Setup(ccs)
	if err != nil {
		t.Fatal(err)
	}
	dir := os.Getenv("ZOPRO_CUBIC")
	if dir == "" {
		dir = t.TempDir()
	}
	if err := Dump(dir, ccs.(*cs.R1CS), pk.(*native.ProvingKey), vk.(*native.VerifyingKey), []byte("cubic")); err != nil {
		t.Fatal(err)
	}
	w, err := frontend.NewWitness(&cubic{X: 3, Y: 35}, ecc.BN254.ScalarField())
	if err != nil {
		t.Fatal(err)
	}
	public, err := w.Public()
	if err != nil {
		t.Fatal(err)
	}
	for name, data := range map[string][]byte{
		"witness": EncodeFields(w.Vector().(fr.Vector)),
		"public":  EncodeFields(public.Vector().(fr.Vector)),
	} {
		if err := os.WriteFile(filepath.Join(dir, name), data, 0o644); err != nil {
			t.Fatal(err)
		}
	}
	names, err := os.ReadFile(filepath.Join(dir, "secret-names"))
	if err != nil || string(names) != "X\n" {
		t.Fatalf("secret names %q, %v", names, err)
	}
}
