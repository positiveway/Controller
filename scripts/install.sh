InstallApt="sudo apt install -y"
RemoveApt="sudo apt remove -y"
AutoRemoveApt="sudo apt autoremove -y"
InstallPkg="sudo dpkg -i"
UpdateApt="sudo apt update"
DownloadStdOut="wget -O -"
AddRepo="sudo add-apt-repository -y"
RemoveFiles="sudo rm -rf"
CopyFiles="sudo cp"
SystemCtl="systemctl --user"


# exit when any command fails
set -e

$InstallApt wget clang libsdl2-dev libdrm-dev libhidapi-dev libusb-1.0-0 libusb-1.0-0-dev libevdev-dev libudev-dev

$DownloadStdOut https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

chmod +x build.sh
./build.sh