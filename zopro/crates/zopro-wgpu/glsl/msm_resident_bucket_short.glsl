uint bucket_length(uint count, uint level) {
  const uint scale = 1u << (args.first_log + 5u * (level - 1u));
  return (count + scale - 1u) / scale;
}

// Several empty buckets may have the same start. The last start at or
// before the shard still identifies the nonempty bucket containing it.
uint bucket_for(uint base, uint half_range, uint level, uint shard) {
  uint low = 0u;
  uint high = half_range;
  while (low + 1u < high) {
    const uint middle = (low + high) / 2u;
    if (offsets.values[base + middle][level] <= shard)
      low = middle;
    else
      high = middle;
  }
  return base + low;
}
