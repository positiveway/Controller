# exit when any command fails
set -e

cd ../target/release/
#sudo ./ControllerRust
sudo nice -n -20 ./ControllerRust
