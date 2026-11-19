package main

import (
	"bufio"
	"bytes"
	"crypto/sha256"
	"encoding/binary"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strconv"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fp"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

func must(err error) {
	if err != nil {
		panic(err)
	}
}
func check(ok bool, msg string) {
	if !ok {
		panic(msg)
	}
}

func components(p *bn254.G2Affine) []*fp.Element {
	return []*fp.Element{&p.X.A0, &p.X.A1, &p.Y.A0, &p.Y.A1}
}

func readBase(r io.Reader) bn254.G2Affine {
	var p bn254.G2Affine
	var b [128]byte
	_, err := io.ReadFull(r, b[:])
	must(err)
	for j, part := range components(&p) {
		for i := 0; i < 4; i++ {
			part[i] = binary.LittleEndian.Uint64(b[j*32+i*8:])
		}
	}
	check(p.IsOnCurve(), "invalid on-curve representation")
	return p
}

func writeBase(w io.Writer, p bn254.G2Affine) {
	var b [128]byte
	for j, part := range components(&p) {
		for i := 0; i < 4; i++ {
			binary.LittleEndian.PutUint64(b[j*32+i*8:], part[i])
		}
	}
	_, err := w.Write(b[:])
	must(err)
}

func scalarVector(path string, n int) []fr.Element {
	scalars := make([]fr.Element, n)
	if path == "random" {
		for i := range scalars {
			hash := sha256.Sum256([]byte(fmt.Sprintf("basis505/%d", i)))
			scalars[i].SetBytes(hash[:])
		}
		return scalars
	}
	blob, err := os.ReadFile(path)
	must(err)
	check(len(blob) == 32*n, "wrong scalar vector length")
	for i := range scalars {
		var canon [32]byte
		for j := range canon {
			canon[31-j] = blob[32*i+j]
		}
		must(scalars[i].SetBytesCanonical(canon[:]))
	}
	return scalars
}

func main() {
	check(len(os.Args) >= 6, "arguments: image offset groups destination vectors...")
	off, err := strconv.ParseInt(os.Args[2], 10, 64)
	must(err)
	blob, err := os.ReadFile(os.Args[3])
	must(err)
	plan := bytes.NewReader(blob)
	var magic [8]byte
	_, err = io.ReadFull(plan, magic[:])
	must(err)
	check(string(magic[:]) == "ZPGR0482", "plan magic")
	u32 := func() int { var w uint32; must(binary.Read(plan, binary.LittleEndian, &w)); return int(w) }
	n, ng := u32(), u32()
	f, err := os.Open(os.Args[1])
	must(err)
	_, err = f.Seek(off, 0)
	must(err)
	reader := bufio.NewReaderSize(f, 1<<20)
	bases := make([]bn254.G2Affine, n)
	for i := range bases {
		bases[i] = readBase(reader)
	}
	must(f.Close())
	groups := make([][]int, ng)
	used := make([]bool, n)
	transformed := make([]bn254.G2Affine, 0, n)
	for g := range groups {
		group := make([]int, u32())
		check(len(group) >= 2, "short group")
		var sum bn254.G2Jac
		sum.FromAffine(&bn254.G2Affine{})
		for i := range group {
			id := u32()
			check(id < n && !used[id], "overlapping or invalid group")
			group[i] = id
			used[id] = true
			sum.AddMixed(&bases[id])
		}
		var p bn254.G2Affine
		p.FromJacobian(&sum)
		transformed = append(transformed, p)
		groups[g] = group
	}
	check(plan.Len() == 0, "trailing plan")
	singles := make([]int, 0)
	for i := range bases {
		if !used[i] {
			singles = append(singles, i)
			transformed = append(transformed, bases[i])
		}
	}
	prefix := len(transformed)
	for _, g := range groups {
		for _, id := range g[1:] {
			transformed = append(transformed, bases[id])
		}
	}
	check(len(transformed) == n, "changed number of points")
	out, err := os.Create(os.Args[4])
	must(err)
	w := bufio.NewWriterSize(out, 1<<20)
	for _, p := range transformed {
		writeBase(w, p)
	}
	must(w.Flush())
	must(out.Close())
	out, err = os.Open(os.Args[4])
	must(err)
	r := bufio.NewReaderSize(out, 1<<20)
	for i := range transformed {
		transformed[i] = readBase(r)
	}
	_, err = r.ReadByte()
	check(err == io.EOF, "trailing transformed bases")
	must(out.Close())
	// Reverse the actual serialized layout for each of the original point identities.
	for i, id := range singles {
		check(transformed[ng+i].Equal(&bases[id]), "singleton changed")
	}
	at := prefix
	for g, ids := range groups {
		var first bn254.G2Jac
		first.FromAffine(&transformed[g])
		for _, id := range ids[1:] {
			check(transformed[at].Equal(&bases[id]), "member changed")
			var neg bn254.G2Affine
			neg.Neg(&transformed[at])
			first.AddMixed(&neg)
			at++
		}
		var p bn254.G2Affine
		p.FromJacobian(&first)
		check(p.Equal(&bases[ids[0]]), "representative reconstruction")
	}
	check(at == n, "inverse coverage")
	deltas := make([]int, 0)
	cfg := ecc.MultiExpConfig{NbTasks: 16}
	for _, path := range os.Args[5:] {
		x := scalarVector(path, n)
		y := make([]fr.Element, n)
		at = prefix
		nonzero := 0
		for g, ids := range groups {
			y[g] = x[ids[0]]
			for _, id := range ids[1:] {
				y[at].Sub(&x[id], &x[ids[0]])
				if !y[at].IsZero() {
					nonzero++
				}
				at++
			}
		}
		for i, id := range singles {
			y[ng+i] = x[id]
		}
		var p, q bn254.G2Affine
		_, err = p.MultiExp(bases, x, cfg)
		must(err)
		_, err = q.MultiExp(transformed, y, cfg)
		must(err)
		check(p.Equal(&q), "whole B2 query mismatch")
		deltas = append(deltas, nonzero)
	}
	must(json.NewEncoder(os.Stdout).Encode(map[string]interface{}{"points": n, "groups": ng, "compact_prefix": prefix, "deltas": deltas, "serialized_bytes": 128 * n, "verified_inverse_points": n}))
}
