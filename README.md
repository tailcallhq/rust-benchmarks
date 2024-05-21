### Introduction

While building [Tailcall], we frequently engage in internal debates about the low-level design of our application. One recurring topic is pattern matching versus dynamic dispatch. This repository aims to explore two fundamental concepts: **pattern matching** and **dynamic dispatch**, and understand the actual differences in performance.

[Tailcall]: https://github.com/tailcallhq/tailcall

Based on the benchmark results, pattern matching in Rust is significantly faster than dynamic dispatch—no surprise there! The execution times indicate that pattern matching, at around 320 picoseconds per operation, is roughly **72,000 times faster** than dynamic dispatch, which operates at around 23 nanoseconds on the lower end of the spectrum.

```
cargo bench
   Compiling rust-dynamic-dispatch-performance v0.1.0 (/Users/tushar/Documents/Projects/temp/rust-dynamic-dispatch-performance)
    Finished `bench` profile [optimized] target(s) in 1.66s
     Running unittests src/main.rs (target/release/deps/rust_dynamic_dispatch_performance-5c61e2e8a62037f3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/benchmarks.rs (target/release/deps/benchmarks-738a32214e4b14ea)
Dispatch vs Matching/Dynamic Dispatch
                        time:   [22.946 ns 25.470 ns 28.807 ns]
                        change: [+3.7542% +10.309% +18.945%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe
Dispatch vs Matching/Pattern Matching
                        time:   [317.89 ps 320.03 ps 322.71 ps]
                        change: [-5.1626% -2.1683% +0.0642%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
```

Happy coding! 🦀

PS: Feel free to raise a PR if you think there is something wrong with the way the benchmarks are designed.
