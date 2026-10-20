package main

import (
	"os"
	"strconv"
	"strings"
)

// footprintMB is the process's anonymous resident memory in MB: what
// Android's low-memory killer weighs (zopro's bench reports the same).
func footprintMB() uint64 {
	status, err := os.ReadFile("/proc/self/status")
	if err != nil {
		return 0
	}
	for _, line := range strings.Split(string(status), "\n") {
		if rest, ok := strings.CutPrefix(line, "RssAnon:"); ok {
			kb, _ := strconv.ParseUint(strings.TrimSpace(strings.TrimSuffix(strings.TrimSpace(rest), "kB")), 10, 64)
			return kb >> 10
		}
	}
	return 0
}
