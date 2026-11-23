// Each partition owns a separate histogram and cursor for every signed bucket.
const uint PARTITION_SHIFT = 17u;

uint partition_count(uint n) {
  return ((n - 1u) >> PARTITION_SHIFT) + 1u;
}
