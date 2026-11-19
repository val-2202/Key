cmake_minimum_required(VERSION 3.22.1)
project(expressive_launcher)

# Use cargo-ndk to build the Rust library
add_custom_command(
    OUTPUT libexpressive_launcher.so
    COMMAND cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi -o ./libs build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(rust_build ALL DEPENDS libexpressive_launcher.so)

# Tell CMake to install the built library so Gradle can find it
install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/libs/aarch64-linux-android/libexpressive_launcher.so" DESTINATION lib/arm64-v8a)
install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/libs/armv7-linux-androideabi/libexpressive_launcher.so" DESTINATION lib/armeabi-v7a)
