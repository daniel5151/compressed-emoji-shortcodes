# checks the size of the resulting --release level binary (that's been stripped)

cargo build --release

cargo bloat --release --split-std -n 100

strip target/release/test
size -A -t target/release/test
