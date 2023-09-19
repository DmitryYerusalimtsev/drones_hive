. /opt/ros/humble/setup.sh

cargo ament-build --install-base /src/$PACKAGE/install
. install/setup.sh

ros2 run --prefix 'gdbserver localhost:3004' $PACKAGE $NODE