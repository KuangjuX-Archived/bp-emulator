cargo run --bin bimodal 7 traces/gcc_trace.txt bimodal_1.txt && \
cargo run --bin bimodal 10 traces/gcc_trace.txt bimodal_2.txt && \
cargo run --bin bimodal 5 traces/jpeg_trace.txt bimodal_3.txt && \
cargo run --bin bimodal 6 traces/perl_trace.txt bimodal_4.txt && \
cargo run --bin gshare 10 4 traces/gcc_trace.txt gshare_1.txt && \
cargo run --bin gshare 14 9 traces/gcc_trace.txt gshare_2.txt && \
cargo run --bin gshare 11 5 traces/jpeg_trace.txt gshare_3.txt && \
cargo run --bin gshare 10 6 traces/perl_trace.txt gshare_4.txt