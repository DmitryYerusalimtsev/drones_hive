cd src/$PACKAGE

cargo ament-build --install-base /install
#colcon build --packages-select $PACKAGE
. install/setup.sh