cmake_minimum_required(VERSION 3.10)

project(example CXX)

set(CMAKE_CXX_STANDARD 17)

include(ExternalProject)

set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/Rust)

ExternalProject_Add(
        libslmp
        GIT_REPOSITORY "https://github.com/sanri/slmp.git"
        CONFIGURE_COMMAND ""
        BUILD_COMMAND cargo build COMMAND cargo build --release
        BUILD_IN_SOURCE 1
        INSTALL_COMMAND ""
        LOG_BUILD ON)

include_directories("${CMAKE_BINARY_DIR}/Rust/src/libslmp/include")

if(CMAKE_BUILD_TYPE STREQUAL Debug)
    link_directories( "${CMAKE_BINARY_DIR}/Rust/src/libslmp/target/debug" )
elseif(CMAKE_BUILD_TYPE STREQUAL Release)
    link_directories( "${CMAKE_BINARY_DIR}/Rust/src/libslmp/target/release" )
else(CMAKE_BUILD_TYPE STREQUAL Debug)
    message(FATAL_ERROR "please set CMAKE_BUILD_TYPE = Debug | Release")
endif(CMAKE_BUILD_TYPE STREQUAL Debug)

add_executable(example main.cpp )
add_dependencies(example libslmp)
target_link_libraries(example slmp )

#set_target_properties(example PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)