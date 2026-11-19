cmake_minimum_required(VERSION 3.22.1)
project(expressive_launcher)

add_custom_command(
    OUTPUT libexpressive_launcher.so
    COMMAND cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi -o ./libs build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(rust_build ALL DEPENDS libexpressive_launcher.so)
