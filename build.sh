rustup update
rustup default stable
cd ~/CLionProjects/ControllerRust/
rm -rf ./target
cargo build --release
rm ~/GolandProjects/ControllerGo/ControllerRust
cp -f ./target/release/ControllerRust ~/GolandProjects/ControllerGo/
