rustup update
rustup default stable

rm -rf ./target
cargo build --release

chmod +x ./run.sh
./run.sh

