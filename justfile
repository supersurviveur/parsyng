bench: bench-comptime bench-runtime

bench-runtime:
    cargo bench --features proc-macro2

bench-comptime: bench-comptime-empty bench-comptime-small bench-comptime-big bench-comptime-big-proc-macro2

__bench features: (__bench_impl features features)

__bench_impl features_syn features_parsyng:
    @echo -e {{ BOLD }}{{ RED }}"\n============ Bench {{ features_parsyng }} syn ============\n"{{ NORMAL }}
    @cd benches/bench-comptime && \
        hyperfine --prepare 'cargo clean' \
        'cargo build --features parsyng,{{ features_parsyng }}'
    @echo -e {{ BOLD }}{{ RED }}"\n============ Bench {{ features_parsyng }} syn (--release) ============\n"{{ NORMAL }}
    @cd benches/bench-comptime && \
        hyperfine --prepare 'cargo clean' \
        'cargo build --release --features syn,{{ features_syn }}' \
        'cargo build --release --features unsynn,{{ features_syn }}' \
        'cargo build --release --features parsyng,{{ features_parsyng }}'

bench-comptime-empty: (__bench "empty")

bench-comptime-small: (__bench "small")

bench-comptime-big: (__bench "big")

bench-comptime-big-proc-macro2: (__bench_impl "big" "big,proc-macro2")
