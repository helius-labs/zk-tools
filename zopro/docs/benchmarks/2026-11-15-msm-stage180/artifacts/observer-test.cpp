#include "aeglos/cpu/bench_msm_phases.hpp"
#include <cassert>
#include <limits>
#include <thread>
#include <vector>
struct FakeClock { static uint64_t tick; static uint64_t now(){return tick+=10;} };uint64_t FakeClock::tick=0;
using Trace=aeglos::cpu::bench::PhaseTrace<FakeClock>;
int main(){
 {const auto before=FakeClock::tick;Trace t(false,96,5);t.start("disabled");t.stop();t.finish();char b[8];assert(t.render(b,sizeof(b))==0);assert(FakeClock::tick==before);}
 std::puts("TEST disabled PASS");
 {Trace t(true,32,2);size_t offsets[]={0,3,8};t.start("accumulate");t.stop(8,offsets,2);t.finish();assert(t.used==1&&t.rows[0].begin==10&&t.rows[0].end==20&&t.elapsed==30);assert(t.rows[0].counts[0]==3&&t.rows[0].counts[1]==5&&!t.malformed&&!t.overflow);char b[1024];assert(t.render(b,sizeof(b))>0);char tiny[8];assert(t.render(tiny,sizeof(tiny))==0);}
 std::puts("TEST exact_spans_and_counts PASS");
 {Trace t(true,32,0);size_t offsets[]={0};t.start("zero");t.stop(0,offsets,0);t.finish();assert(t.rows[0].jobs==0&&t.rows[0].items==0);}
 std::puts("TEST zero_work PASS");
 {Trace t(true,96,1);for(size_t i=0;i<Trace::CAPACITY+1;++i){t.start("overflow");t.stop();}t.finish();assert(t.used==Trace::CAPACITY&&t.overflow&&!t.open);}
 std::puts("TEST capacity_overflow PASS");
 {Trace t(true,96,17);t.start("job_overflow");size_t offsets[18]={};t.stop(0,offsets,17);t.finish();assert(t.overflow&&t.rows[0].jobs==0);}
 std::puts("TEST job_overflow PASS");
 {Trace t(true,96,2);size_t offsets[]={0,3,2};t.start("bad_counts");t.stop(2,offsets,2);t.finish();assert(t.malformed);}
 std::puts("TEST bad_counts PASS");
 {Trace t(true,96,1);t.start("incomplete");t.stop();assert(!t.complete);}
 std::puts("TEST incomplete PASS");
 {Trace t(true,96,1);t.start("still_open");t.finish();assert(t.malformed);}
 std::puts("TEST unfinished_span PASS");
 {Trace t(true,96,16);size_t offsets[17]={};for(size_t i=1;i<17;++i)offsets[i]=i*(std::numeric_limits<size_t>::max()/16);for(size_t i=0;i<Trace::CAPACITY;++i){t.start("max_row");t.stop(offsets[16],offsets,16);}t.finish();char b[32768];assert(t.render(b,sizeof(b))>0);}
 std::puts("TEST max_encoding PASS");
 std::vector<std::thread> threads;
 for(int i=0;i<8;++i)threads.emplace_back([]{aeglos::cpu::bench::PhaseTrace<> t(true,4,2);size_t offsets[]={0,2,4};t.start("concurrent");t.stop(4,offsets,2);t.finish();assert(!t.overflow&&!t.malformed&&t.used==1);});
 for(auto&t:threads) { t.join(); }
 std::puts("TEST concurrent_buffers PASS");
 std::printf("TRACE_BYTES %zu\n",sizeof(Trace));
}
