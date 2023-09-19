set -e

. /opt/ros/humble/setup.sh

# Set the default build type
BUILD_TYPE=RelWithDebInfo

colcon build \
        --merge-install \
        --symlink-install \
        --packages-up-to $PACKAGE \
        --cmake-args "-DCMAKE_BUILD_TYPE=$BUILD_TYPE" "-DCMAKE_EXPORT_COMPILE_COMMANDS=On" \
        -Wall -Wextra -Wpedantic