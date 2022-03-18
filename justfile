m := "7"
trace_file := "traces/gcc_trace.txt"
bimodal_file := "bimodal_1.txt"

bimodal:
    cargo run --bin bimodal {{m}} {{trace_file}}

clean:
    cargo clean