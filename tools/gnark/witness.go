package main

import (
	"fmt"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/witness"
	"github.com/consensys/gnark/frontend"
)

// A request's parameters, as zolana's provers parse them.
type parameters interface {
	ValidateShape() error
	CreateWitness() (frontend.Circuit, error)
}

func assignmentWitness(params parameters) (witness.Witness, error) {
	if err := params.ValidateShape(); err != nil {
		return nil, err
	}
	assignment, err := params.CreateWitness()
	if err != nil {
		return nil, fmt.Errorf("create assignment: %w", err)
	}
	return frontend.NewWitness(assignment, ecc.BN254.ScalarField())
}
