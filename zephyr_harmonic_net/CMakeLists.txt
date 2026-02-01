cmake_minimum_required(VERSION 3.20.0)
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(harmonic_net)


# Hubble SDK Integration
list(APPEND ZEPHYR_EXTRA_MODULES "/home/pecosdwilly/omni-tool-spectral-core/modules/lib/hubblenetwork-sdk")


target_sources(app PRIVATE src/main.c)

