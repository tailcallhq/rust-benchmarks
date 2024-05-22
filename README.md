[![Tailcall](https://raw.githubusercontent.com/tailcallhq/tailcall/main/assets/logo_main.svg)](https://tailcall.run)

While building [Tailcall], we frequently engage in internal debates about the low-level design of our application. One recurring topic is pattern matching versus dynamic dispatch. This repository aims to explore two fundamental concepts: **pattern matching** and **dynamic dispatch**, and understand the actual differences in performance.

[Tailcall]: https://github.com/tailcallhq/tailcall

Based on the benchmark results, pattern matching in Rust is significantly faster than dynamic dispatch—no surprise there! The execution times indicate that pattern matching, at around 320 picoseconds per operation, is roughly **72,000 times faster** than dynamic dispatch, which operates at around 23 nanoseconds on the lower end of the spectrum.

```
cargo bench
   Compiling rust-dynamic-dispatch-performance v0.1.0 (/Users/tushar/Documents/Projects/rust-dynamic-dispatch-performance)
    Finished `bench` profile [optimized] target(s) in 1.20s
     Running unittests src/main.rs (target/release/deps/rust_dynamic_dispatch_performance-5c61e2e8a62037f3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/benchmarks.rs (target/release/deps/benchmarks-738a32214e4b14ea)
Dispatch vs Matching/Dynamic Dispatch
                        time:   [2.5226 ns 2.5325 ns 2.5420 ns]
                        change: [+0.3857% +0.8665% +1.3628%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Dispatch vs Matching/Pattern Matching
                        time:   [375.26 ps 377.67 ps 380.79 ps]
                        change: [-76.135% -75.895% -75.626%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
```

## Update 1

We were curious to see why pattern matching was significantly faster compared to dynamic dispatch, so we posted benchmarks on [Reddit](https://www.reddit.com/r/rust/comments/1cx7qvi/performance_pattern_matching_vs_dynamic_dispatch/). We received interesting feedback about the benchmark setup, such as the unnecessary heap allocation for dynamic dispatch. After addressing this, the performance difference was marginal.

Digging deeper, especially after this [GitHub comment](https://github.com/tailcallhq/rust-benchmarks/issues/2) and further analysis on [Rust Compiler Explorer](https://rust.godbolt.org/), we discovered that the compiler optimizes the generated machine code based on the seed value set in the variable `output`. When set to `0`, the additions and multiplications would always return `0` regardless of the contents of the passed array. This optimization resulted in the pattern matching case reducing to (an optimization that was unavailable to the `dyn` implementation):

```asm
xor eax, eax
ret
```

After correcting this by setting the seed value to `1`, the benchmarks showed a significant improvement, making pattern matching only **2.7x faster** than dynamic dispatch.

```
cargo bench
   Compiling rust-dynamic-dispatch-performance v0.1.0 (/Users/tushar/Documents/Projects/rust-dynamic-dispatch-performance)
    Finished `bench` profile [optimized] target(s) in 1.36s
     Running unittests src/main.rs (target/release/deps/rust_dynamic_dispatch_performance-5c61e2e8a62037f3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/benchmarks.rs (target/release/deps/benchmarks-738a32214e4b14ea)
Benchmarking Dispatch vs Matching/Dynamic Dispatch: Collecting 100 samples in
Dispatch vs Matching/Dynamic Dispatch
                        time:   [2.5552 ns 2.5824 ns 2.6199 ns]
                        change: [+2.0878% +8.3408% +15.931%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 20 outliers among 100 measurements (20.00%)
  1 (1.00%) high mild
  19 (19.00%) high severe
Benchmarking Dispatch vs Matching/Pattern Matching: Collecting 100 samples in
Dispatch vs Matching/Pattern Matching
                        time:   [947.52 ps 949.81 ps 952.50 ps]
                        change: [+147.37% +149.83% +151.38%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```

Happy coding! 🦀

PS: Feel free to raise a PR if you think there is something wrong with the way the benchmarks are designed.
