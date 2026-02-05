cmake_minimum_required(VERSION 3.10)
project(d16_hailo_shim)

set(CMAKE_CXX_STANDARD 17)

# Find HailoRT
find_package(hailort REQUIRED)

add_executable(d16_feeder d16_hailo_feeder.cpp)
add_executable(hailo_immune_system hailo_immune_system.cpp)

# Link against HailoRT
target_link_libraries(d16_feeder PRIVATE hailo_rt)
target_link_libraries(hailo_immune_system PRIVATE hailo_rt)

# Zephyr Consideration (Note for future)
# If we were building this *as* a Zephyr module (e.g. for a hybrid board),
# include($ENV{ZEPHYR_BASE}/cmake/app/boilerplate.cmake)
