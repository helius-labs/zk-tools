package zoprognark

import (
	"unsafe"

	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

// EncodeFields is the kernel's encoding of field elements: four
// little-endian Montgomery limbs each, the memory of an fr.Element on a
// little-endian target. Witnesses in a dump are in it.
func EncodeFields(values []fr.Element) []byte {
	return bytesOf(values)
}

// The memory of a slice of plain values, to send as it is.
func bytesOf[T any](values []T) []byte {
	if len(values) == 0 {
		return nil
	}
	var element T
	return unsafe.Slice((*byte)(unsafe.Pointer(&values[0])), int(unsafe.Sizeof(element))*len(values))
}
