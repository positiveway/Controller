sudo apt-get install -y libappindicator3-dev gir1.2-appindicator3-0.1 libgtk-3-dev libgtkmm-3.0-dev clang libsdl2-dev libdrm-dev libhidapi-dev libusb-1.0-0 libusb-1.0-0-dev libevdev-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

chmod +x build.sh
./build.sh