package main

import (
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"strings"
	"sync/atomic"
	"testing"
	"time"
)

// TestBench proves a Zolana request with a gnark key as the bridge does
// (the .key container read onto the heap), GNARK_BENCH_ROUNDS times (5),
// each proof verified, and prints one line with the load, the memory the
// key holds, and the proofs' times and peak, as zopro's bench does (its
// tools/bench.sh). Memory is the process's own: anonymous on Linux, the
// physical footprint on macOS. bench/bench.sh runs it shape by shape.
//
//	GNARK_BENCH_KEY=<key> GNARK_BENCH_REQUEST=<request.json> go test -run TestBench -count=1 .
func TestBench(t *testing.T) {
	key := os.Getenv("GNARK_BENCH_KEY")
	if key == "" {
		t.Skip("set GNARK_BENCH_KEY and GNARK_BENCH_REQUEST")
	}
	request, err := os.ReadFile(os.Getenv("GNARK_BENCH_REQUEST"))
	if err != nil {
		t.Fatal(err)
	}
	rounds := 5
	if v := os.Getenv("GNARK_BENCH_ROUNDS"); v != "" {
		if rounds, err = strconv.Atoi(v); err != nil || rounds < 1 {
			t.Fatalf("GNARK_BENCH_ROUNDS=%q", v)
		}
	}
	// The footprint's peak, sampled every 2 ms.
	var peak atomic.Uint64
	go func() {
		for {
			if v := footprintMB(); v > peak.Load() {
				peak.Store(v)
			}
			time.Sleep(2 * time.Millisecond)
		}
	}()

	started := time.Now()
	handle, err := loadPreparedKey(key)
	if err != nil {
		t.Fatalf("load %s: %v", key, err)
	}
	loadMS := time.Since(started).Milliseconds()
	loadPeak := max(peak.Load(), footprintMB())
	held := footprintMB()
	peak.Store(held)
	var witness, prove []uint64
	for range rounds {
		result, err := provePrepared(handle, string(request))
		if err != nil {
			t.Fatalf("prove: %v", err)
		}
		witness = append(witness, result.witnessMS)
		prove = append(prove, result.proveMS)
	}
	proofPeak := max(peak.Load(), footprintMB())
	slices.Sort(witness)
	slices.Sort(prove)
	fmt.Printf("GNARK zolana %s rounds=%d load_ms=%d load_peak_mb=%d held_mb=%d witness_ms=%d prove_ms_min=%d prove_ms_median=%d peak_mb=%d\n",
		strings.TrimSuffix(filepath.Base(key), ".key"), rounds, loadMS, loadPeak, held,
		witness[len(witness)/2], prove[0], prove[len(prove)/2], proofPeak)
}
