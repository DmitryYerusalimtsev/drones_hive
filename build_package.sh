. /opt/ros/humble/setup.sh

cd src/$PACKAGE

cargo ament-build --install-base /install
. install/setup.sh