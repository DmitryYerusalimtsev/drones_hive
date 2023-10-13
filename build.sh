set -e

# Set the default build type
BUILD_TYPE=RelWithDebInfo

# colcon build \
#         --symlink-install \
#         --packages-select $PACKAGE \
#         --cmake-args "-DCMAKE_BUILD_TYPE=$BUILD_TYPE" "-DCMAKE_EXPORT_COMPILE_COMMANDS=On" \
#         -Wall -Wextra -Wpedantic

colcon build \
        --packages-select "drone" \
        --cmake-args "-DCMAKE_BUILD_TYPE=RelWithDebInfo" "-DCMAKE_EXPORT_COMPILE_COMMANDS=On" \
        -Wall -Wextra -Wpedantic

# --packages-up-to $PACKAGE \