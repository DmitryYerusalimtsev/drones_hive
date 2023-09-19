. /opt/ros/humble/setup.sh

cd src/$PACKAGE

# colcon build \
#         --merge-install \
#         --symlink-install \
#         --packages-up-to $PACKAGE \
#         --cmake-args "-DCMAKE_BUILD_TYPE=$BUILD_TYPE" "-DCMAKE_EXPORT_COMPILE_COMMANDS=On" \
#         -Wall -Wextra -Wpedantic

# rm -r build
# rm -r install
# rm -r log

cargo ament-build --install-base /install
. install/setup.sh