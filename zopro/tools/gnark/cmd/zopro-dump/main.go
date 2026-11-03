// zopro-dump writes the dump of a gnark Groth16 BN254 constraint system
// and its keys, as gnark's WriteTo wrote them, for `zopro-bench pack`:
//
//	zopro-dump -r1cs circuit.r1cs -pk circuit.pk -vk circuit.vk -out dump [-metadata file]
package main

import (
	"flag"
	"fmt"
	"io"
	"os"

	native "github.com/consensys/gnark/backend/groth16/bn254"
	cs "github.com/consensys/gnark/constraint/bn254"

	zoprognark "zopro/tools/gnark"
)

func main() {
	r1csPath := flag.String("r1cs", "", "the constraint system (R1CS.WriteTo)")
	pkPath := flag.String("pk", "", "the proving key (ProvingKey.WriteTo or WriteRawTo)")
	vkPath := flag.String("vk", "", "the verifying key (VerifyingKey.WriteTo or WriteRawTo)")
	out := flag.String("out", "", "the dump directory")
	metadataPath := flag.String("metadata", "", "a file the zopro key keeps as its metadata")
	flag.Parse()
	if *r1csPath == "" || *pkPath == "" || *vkPath == "" || *out == "" {
		flag.Usage()
		os.Exit(2)
	}
	if err := run(*r1csPath, *pkPath, *vkPath, *out, *metadataPath); err != nil {
		fmt.Fprintln(os.Stderr, "zopro-dump:", err)
		os.Exit(1)
	}
}

func run(r1csPath, pkPath, vkPath, out, metadataPath string) error {
	var r1cs cs.R1CS
	if err := readFrom(r1csPath, &r1cs); err != nil {
		return err
	}
	var pk native.ProvingKey
	if err := readFrom(pkPath, &pk); err != nil {
		return err
	}
	var vk native.VerifyingKey
	if err := readFrom(vkPath, &vk); err != nil {
		return err
	}
	var metadata []byte
	if metadataPath != "" {
		var err error
		if metadata, err = os.ReadFile(metadataPath); err != nil {
			return err
		}
	}
	return zoprognark.Dump(out, &r1cs, &pk, &vk, metadata)
}

func readFrom(path string, into io.ReaderFrom) error {
	file, err := os.Open(path)
	if err != nil {
		return err
	}
	defer file.Close()
	if _, err := into.ReadFrom(file); err != nil {
		return fmt.Errorf("%s: %w", path, err)
	}
	return nil
}
