package main

import (
	"bufio"
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	native "github.com/consensys/gnark/backend/groth16/bn254"
	"github.com/consensys/gnark/backend/witness"
	cs "github.com/consensys/gnark/constraint/bn254"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/logger"
	aeglos "github.com/helius-labs/aeglos"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"syscall"
	"time"
	"zolana/prover/prover/common"
	mergeprover "zolana/prover/prover/merge"
	transfer "zolana/prover/prover/transfer_eddsa_only"
	zoprognark "zopro/tools/gnark"
)

func must(e error) {
	if e != nil {
		panic(e)
	}
}

type parameters interface {
	ValidateShape() error
	CreateWitness() (frontend.Circuit, error)
}

func assignment(request []byte, merge bool) (witness.Witness, error) {
	var p parameters
	if merge {
		p = new(mergeprover.MergeParameters)
	} else {
		p = new(transfer.TransferParameters)
	}
	if e := json.Unmarshal(request, p); e != nil {
		return nil, e
	}
	if e := p.ValidateShape(); e != nil {
		return nil, e
	}
	a, e := p.CreateWitness()
	if e != nil {
		return nil, e
	}
	return frontend.NewWitness(a, ecc.BN254.ScalarField())
}
func main() {
	processStarted := time.Now()
	logger.Disable()
	if len(os.Args) < 5 {
		panic("usage: aeglos-bench (dump|aeglos|gnark) key fixture rounds [dump-dir]")
	}
	mode, keyPath, requestPath := os.Args[1], os.Args[2], os.Args[3]
	rounds, e := strconv.Atoi(os.Args[4])
	must(e)
	started := time.Now()
	data, e := os.ReadFile(keyPath)
	must(e)
	digest := sha256.Sum256(data)
	var ps common.TransferProofSystem
	_, e = ps.UnsafeReadFrom(bytes.NewReader(data))
	must(e)
	ps.ProvingKeySha256 = digest
	data = nil
	runtime.GC()
	loadMS := float64(time.Since(started).Nanoseconds()) / 1e6
	request, e := os.ReadFile(requestPath)
	must(e)
	if mode == "dump" {
		metadata, e := json.Marshal(map[string]any{"inputs": ps.NInputs, "outputs": ps.NOutputs, "keySha256": hex.EncodeToString(digest[:])})
		must(e)
		must(zoprognark.Dump(os.Args[5], ps.ConstraintSystem.(*cs.R1CS), ps.ProvingKey.(*native.ProvingKey), ps.VerifyingKey.(*native.VerifyingKey), metadata))
		fmt.Println("DUMP COMPLETE")
		return
	}
	threads := runtime.GOMAXPROCS(0)
	if v := os.Getenv("AEGLOS_CPU_THREADS"); v != "" {
		threads, e = strconv.Atoi(v)
		must(e)
	}
	priming := 3
	if v := os.Getenv("ZOPRO_BENCH_PRIMING"); v != "" {
		priming, e = strconv.Atoi(v)
		must(e)
		if priming < 0 {
			panic("negative priming")
		}
	}
	var timings aeglos.StageTimings
	var engine *aeglos.Engine
	if mode == "aeglos" {
		engine, e = aeglos.New(aeglos.Config{Backend: aeglos.BackendCPU, Threads: threads, CPUFamily: aeglos.CPUAuto, MemoryLimitBytes: 24 << 30, Observe: func(t aeglos.StageTimings) { timings = t }})
		must(e)
		defer func() { must(engine.Close()) }()
	} else if mode != "gnark" {
		panic(mode)
	}

	type row struct {
		Round       int     `json:"round"`
		VerifiedMS  float64 `json:"verified_ms"`
		RequestMS   float64 `json:"request_ms"`
		WitnessMS   float64 `json:"witness_ms"`
		ScalarsMS   float64 `json:"scalars_ms"`
		FFTMS       float64 `json:"fft_ms"`
		MSMMS       float64 `json:"msm_ms"`
		AssemblyMS  float64 `json:"assembly_ms"`
		EngineMS    float64 `json:"engine_ms"`
		CacheHit    bool    `json:"cache_hit"`
		NativeBytes uint64  `json:"native_bytes"`
	}
	rows := make([]row, 0, rounds)
	var proof groth16.Proof
	var lastProof, lastPublic []byte
	var ctl *os.File
	var ack *bufio.Reader
	if v := os.Getenv("CROSS_PERF_CTL"); v != "" {
		ctl, e = os.OpenFile(v, os.O_RDWR, 0)
		must(e)
		f, e := os.OpenFile(os.Getenv("CROSS_PERF_ACK"), os.O_RDWR, 0)
		must(e)
		ack = bufio.NewReader(f)
	}
	control := func(command string) {
		if ctl == nil {
			return
		}
		_, e := ctl.WriteString(command + "\n")
		must(e)
		reply, e := ack.ReadString('\n')
		must(e)
		if strings.Trim(reply, "\x00 \r\n\t") != "ack" {
			panic(reply)
		}
	}
	var before, after syscall.Rusage
	var loopBegin time.Time
	var wall time.Duration
	readyMS := float64(time.Since(processStarted).Nanoseconds()) / 1e6
	var firstMS float64
	var firstPeak int64
	var firstCacheHit bool
	var firstProof, firstPublic []byte
	exportFirst := os.Getenv("ZOPRO_BENCH_EXPORT_FIRST") == "1"
	for round := -priming; round < rounds; round++ {
		if round == 0 {
			control("enable")
			must(syscall.Getrusage(syscall.RUSAGE_SELF, &before))
			loopBegin = time.Now()
		}
		begin := time.Now()
		full, e := assignment(request, strings.HasPrefix(filepath.Base(keyPath), "merge_"))
		must(e)
		requestMS := float64(time.Since(begin).Nanoseconds()) / 1e6
		if mode == "gnark" {
			proof, e = groth16.Prove(ps.ConstraintSystem, ps.ProvingKey, full)
		} else {
			proof, e = engine.Prove(ps.ConstraintSystem, ps.ProvingKey, full)
		}
		must(e)
		public, e := full.Public()
		must(e)
		must(groth16.Verify(proof, ps.VerifyingKey, public))
		var proofBytes, publicBytes bytes.Buffer
		_, e = proof.WriteTo(&proofBytes)
		must(e)
		_, e = public.WriteTo(&publicBytes)
		must(e)
		wrapped := common.Proof{Proof: proof, ProvingKeySha256: digest}
		encoded, e := json.Marshal(&wrapped)
		must(e)
		if len(encoded) == 0 {
			panic("empty serialization")
		}
		ms := float64(time.Since(begin).Nanoseconds()) / 1e6
		if round == -priming {
			firstMS = float64(time.Since(processStarted).Nanoseconds()) / 1e6
			firstCacheHit = timings.CacheHit
			if exportFirst {
				var firstUsage syscall.Rusage
				must(syscall.Getrusage(syscall.RUSAGE_SELF, &firstUsage))
				firstPeak = firstUsage.Maxrss
				firstProof, firstPublic = proofBytes.Bytes(), publicBytes.Bytes()
			}
		}
		if proofBytes.Len() != 164 || publicBytes.Len() != 44 {
			panic("proof encoding lengths")
		}
		if round >= 0 {
			rows = append(rows, row{round, ms, requestMS, float64(timings.Witness.Nanoseconds()) / 1e6, float64(timings.Scalars.Nanoseconds()) / 1e6, float64(timings.FFT.Nanoseconds()) / 1e6, float64(timings.MSM.Nanoseconds()) / 1e6, float64(timings.Assembly.Nanoseconds()) / 1e6, float64(timings.Total.Nanoseconds()) / 1e6, timings.CacheHit, timings.DeviceBytes})
			lastProof = proofBytes.Bytes()
			lastPublic = publicBytes.Bytes()
		}
	}
	wall = time.Since(loopBegin)
	must(syscall.Getrusage(syscall.RUSAGE_SELF, &after))
	control("disable")
	if dir := os.Getenv("ZOPRO_BENCH_PROOFS"); dir != "" {
		dir = filepath.Join(dir, strings.TrimSuffix(filepath.Base(keyPath), ".key"))
		must(os.MkdirAll(dir, 0755))
		must(os.WriteFile(filepath.Join(dir, "0.proof"), lastProof, 0644))
		must(os.WriteFile(filepath.Join(dir, "0.public"), lastPublic, 0644))
		if exportFirst {
			must(os.WriteFile(filepath.Join(dir, "first.proof"), firstProof, 0644))
			must(os.WriteFile(filepath.Join(dir, "first.public"), firstPublic, 0644))
		}
	}
	for _, v := range rows {
		b, e := json.Marshal(v)
		must(e)
		fmt.Println("SAMPLE " + string(b))
	}
	values := make([]float64, 0, len(rows))
	for _, v := range rows {
		values = append(values, v.VerifiedMS)
	}
	sort.Float64s(values)
	tv := func(v syscall.Timeval) float64 { return float64(v.Sec)*1e3 + float64(v.Usec)/1e3 }
	b, e := json.Marshal(map[string]any{"mode": mode, "key": filepath.Base(keyPath), "key_sha256": hex.EncodeToString(digest[:]), "rounds": rounds, "priming": priming, "ready_from_main_ms": readyMS, "first_verified_from_main_ms": firstMS, "first_maxrss_kib": firstPeak, "first_cache_hit": firstCacheHit, "threads": threads, "load_ms": loadMS, "verified_ms_median": values[len(values)/2], "loop_wall_ms": float64(wall.Nanoseconds()) / 1e6, "user_cpu_ms": tv(after.Utime) - tv(before.Utime), "system_cpu_ms": tv(after.Stime) - tv(before.Stime), "voluntary_switches": after.Nvcsw - before.Nvcsw, "involuntary_switches": after.Nivcsw - before.Nivcsw, "minor_faults": after.Minflt - before.Minflt, "whole_process_maxrss_kib": after.Maxrss, "native_limit_bytes": uint64(24 << 30)})
	must(e)
	fmt.Println("SUMMARY " + string(b))
}
