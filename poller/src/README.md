# NanTuanTracker Poller
Rust implementation of a Dota2 Guild poller and publish data to corresponding destinations

# Useful commands
* `cargo build` compile the program
* `cargo run` compile and run the program
* `cargo test -- --test-threads=1` compile and run unit tests. 
  * We will need option *test-threads=1* because we have tests related to environmental variables, such that tests need to be executed in order
* `TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl` compile the program and build to AWS lambda supported platform target
