The first reused PMU-reader invocation stopped before reading records because
its comparison copy ROOT/pmu.rs was absent. The runtime source copy was already
pinned and unchanged. Restore that alias byte-for-byte from the pinned runtime
source; the reader checks equality itself. No reader code, gate, runtime source,
configuration, oracle or capture changed. Manager and CPU-reader successes are
retained; rerun only the failed PMU reader, then the not-yet-run scoped reader.
