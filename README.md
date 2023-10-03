# slow-test

Command: `cargo test --release -- --nocapture`

On `key.cs.utexas.edu`, the sixth iteration runs significantly slower:

```
[iteration 0] time = 81.518243ms, sum = 6146662891467571200
[iteration 1] time = 82.749995ms, sum = 6146662891467571200
[iteration 2] time = 81.583815ms, sum = 6146662891467571200
[iteration 3] time = 81.597512ms, sum = 6146662891467571200
[iteration 4] time = 81.484207ms, sum = 6146662891467571200
[iteration 5] time = 82.850284ms, sum = 6146662891467571200
[iteration 6] time = 117.819694ms, sum = 6146662891467571200
[iteration 7] time = 81.708054ms, sum = 6146662891467571200
[iteration 8] time = 81.602485ms, sum = 6146662891467571200
[iteration 9] time = 81.50396ms, sum = 6146662891467571200
[iteration 10] time = 82.508603ms, sum = 6146662891467571200
[iteration 11] time = 83.811637ms, sum = 6146662891467571200
[iteration 12] time = 83.788989ms, sum = 6146662891467571200
[iteration 13] time = 83.241887ms, sum = 6146662891467571200
[iteration 14] time = 81.725434ms, sum = 6146662891467571200
[iteration 15] time = 81.837856ms, sum = 6146662891467571200
```

Random diagnostics that may be useful:

```
$ rustc --version
rustc 1.72.1 (d5c2e9c34 2023-09-13)
$ cargo --version
cargo 1.72.1 (103a7ff2e 2023-08-15)
$ lscpu
Architecture:                       x86_64
CPU op-mode(s):                     32-bit, 64-bit
Byte Order:                         Little Endian
Address sizes:                      46 bits physical, 48 bits virtual
CPU(s):                             48
On-line CPU(s) list:                0-47
Thread(s) per core:                 2
Core(s) per socket:                 12
Socket(s):                          2
NUMA node(s):                       2
Vendor ID:                          GenuineIntel
CPU family:                         6
Model:                              85
Model name:                         Intel(R) Xeon(R) Silver 4214 CPU @ 2.20GHz
Stepping:                           7
CPU MHz:                            1000.033
CPU max MHz:                        3200.0000
CPU min MHz:                        1000.0000
BogoMIPS:                           4400.00
Virtualization:                     VT-x
L1d cache:                          768 KiB
L1i cache:                          768 KiB
L2 cache:                           24 MiB
L3 cache:                           33 MiB
NUMA node0 CPU(s):                  0,2,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32,34,36,38,40,42,44,46
NUMA node1 CPU(s):                  1,3,5,7,9,11,13,15,17,19,21,23,25,27,29,31,33,35,37,39,41,43,45,47
Vulnerability Gather data sampling: Mitigation; Microcode
Vulnerability Itlb multihit:        KVM: Mitigation: Split huge pages
Vulnerability L1tf:                 Not affected
Vulnerability Mds:                  Not affected
Vulnerability Meltdown:             Not affected
Vulnerability Mmio stale data:      Mitigation; Clear CPU buffers; SMT vulnerable
Vulnerability Retbleed:             Mitigation; Enhanced IBRS
Vulnerability Spec store bypass:    Mitigation; Speculative Store Bypass disabled via prctl and seccomp
Vulnerability Spectre v1:           Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:           Mitigation; Enhanced IBRS, IBPB conditional, RSB filling, PBRSB-eIBRS SW sequence
Vulnerability Srbds:                Not affected
Vulnerability Tsx async abort:      Mitigation; TSX disabled
Flags:                              fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm 
                                    constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3
                                     sdbg fma cx16 xtpr pdcm pcid dca sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_faul
                                    t epb cat_l3 cdp_l3 invpcid_single intel_ppin ssbd mba ibrs ibpb stibp ibrs_enhanced tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust
                                     bmi1 avx2 smep bmi2 erms invpcid cqm mpx rdt_a avx512f avx512dq rdseed adx smap clflushopt clwb intel_pt avx512cd avx512bw avx512vl xsaveopt xsavec 
                                    xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local dtherm ida arat pln pts pku ospke avx512_vnni md_clear flush_l1d arch_capabilities
```
