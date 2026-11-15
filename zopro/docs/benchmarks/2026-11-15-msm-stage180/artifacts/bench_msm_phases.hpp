#pragma once
// Private benchmark observer. No per-task hooks or cryptographic values.
#include <array>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cstdarg>

namespace aeglos::cpu::bench {
struct Monotonic {
  static uint64_t now() {
    return uint64_t(std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::steady_clock::now().time_since_epoch()).count());
  }
};
inline bool phases_enabled() {
  const char *s = std::getenv("AEGLOS_BENCH_MSM_PHASES");
  return s && std::strcmp(s, "1") == 0;
}
template<class Clock = Monotonic> struct PhaseTrace {
  static constexpr size_t CAPACITY = 64, JOBS = 16;
  struct Row {
    const char *name;
    uint64_t begin, end, items;
    size_t jobs;
    std::array<uint64_t, JOBS> counts;
  };
  const bool enabled;
  const size_t workers, jobs;
  std::array<Row, CAPACITY> rows; // Only initialized entries below used are read.
  size_t used = 0;
  bool open = false, overflow = false, malformed = false, complete = false;
  uint64_t origin = 0, elapsed = 0;
  PhaseTrace(bool on, size_t w, size_t j): enabled(on),workers(w),jobs(j) {
    if (enabled) { origin = Clock::now(); overflow = j > JOBS; }
  }
  PhaseTrace(const PhaseTrace&) = delete;
  PhaseTrace& operator=(const PhaseTrace&) = delete;
  void start(const char *name) {
    if (!enabled) return;
    if (open || complete) { malformed = true; return; }
    if (used == CAPACITY) { overflow = true; return; }
    Row &r = rows[used];
    r.name = name; r.items = 0; r.jobs = 0;
    r.begin = Clock::now() - origin; r.end = r.begin;
    open = true;
  }
  void stop(uint64_t items = 0, const size_t *offsets = nullptr, size_t n = 0) {
    if (!enabled) return;
    if (!open) { if (!overflow) malformed = true; return; }
    Row &r = rows[used]; r.end = Clock::now() - origin;
    if (r.end < r.begin) malformed = true;
    r.items = items;
    if (n > JOBS) overflow = true;
    else if (n) {
      if (!offsets || offsets[0] != 0 || offsets[n] != items) malformed = true;
      else {
        r.jobs = n;
        for (size_t i=0;i<n;++i) {
          if (offsets[i+1] < offsets[i]) malformed = true;
          r.counts[i] = offsets[i+1] - offsets[i];
        }
      }
    }
    ++used; open = false;
  }
  void finish() {
    if (!enabled) return;
    if (open || complete) malformed = true;
    elapsed = Clock::now() - origin; complete = true;
  }
  size_t render(char *out, size_t capacity) const {
    if (!enabled) return 0;
    size_t pos=0; bool fits=true;
    const auto put = [&](const char *format, auto... args) {
      if (!fits || pos >= capacity) { fits=false; return; }
      int n;
      if constexpr (sizeof...(args)==0) n=std::snprintf(out+pos,capacity-pos,"%s",format);
      else n=std::snprintf(out+pos,capacity-pos,format,args...);
      if (n<0 || size_t(n)>=capacity-pos) {fits=false;return;}
      pos+=size_t(n);
    };
    put("MSM_PHASES {\"version\":1,\"workers\":%zu,\"jobs\":%zu,\"capacity\":%zu,\"trace_bytes\":%zu,\"complete\":%s,\"overflow\":%s,\"malformed\":%s,\"total_ns\":%llu,\"rows\":[",workers,jobs,CAPACITY,sizeof(*this),complete?"true":"false",overflow?"true":"false",malformed?"true":"false",(unsigned long long)elapsed);
    for(size_t i=0;i<used;++i){const Row&r=rows[i];
      put("%s{\"phase\":\"%s\",\"begin_ns\":%llu,\"end_ns\":%llu,\"items\":%llu,\"job_items\":[",i?",":"",r.name,(unsigned long long)r.begin,(unsigned long long)r.end,(unsigned long long)r.items);
      for(size_t j=0;j<r.jobs;++j)put("%s%llu",j?",":"",(unsigned long long)r.counts[j]);
      put("]}");
    }
    put("]}\n");return fits?pos:0;
  }
  ~PhaseTrace() {
    if (!enabled) return;
    char line[32768];const size_t n=render(line,sizeof(line));
    if (!n) { std::fputs("MSM_PHASES {\"version\":1,\"format_overflow\":true}\n",stderr);return; }
    // Single stdio call serializes concurrent records; cost belongs to proof timer.
    std::fwrite(line,1,n,stderr);
  }
};
} // namespace aeglos::cpu::bench
