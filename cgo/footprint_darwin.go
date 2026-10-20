package main

/*
#include <libproc.h>
#include <unistd.h>

static unsigned long long phys_footprint(void) {
	struct rusage_info_v2 info;
	if (proc_pid_rusage(getpid(), RUSAGE_INFO_V2, (rusage_info_t *)&info) != 0) {
		return 0;
	}
	return info.ri_phys_footprint;
}
*/
import "C"

// footprintMB is the process's physical footprint in MB: what iOS's memory
// limit counts, Metal's buffers included (zopro's bench reports the same).
func footprintMB() uint64 {
	return uint64(C.phys_footprint()) >> 20
}
