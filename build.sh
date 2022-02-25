rustup update
rustup default stable
rm -rf ./target
cargo build --release
cd target/release
mv ControllerRust controllerRust
cd ../../
chmod +x ./run.sh
./run.sh

