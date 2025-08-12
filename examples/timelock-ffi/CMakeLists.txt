# Copyright 2025 by Ideal Labs, LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

cmake_minimum_required(VERSION 3.10)
project(timelock_ffi_examples)

# Set C and C++ standards
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find the timelock-ffi library
# You can set TIMELOCK_FFI_DIR to point to your build directory
if(NOT DEFINED TIMELOCK_FFI_DIR)
    set(TIMELOCK_FFI_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../timelock-ffi/target/release")
endif()

# Include directories
include_directories("${CMAKE_CURRENT_SOURCE_DIR}/../../timelock-ffi")

# Find the library (try both static and dynamic)
find_library(TIMELOCK_FFI_LIB
    NAMES timelock_ffi libtimelock_ffi
    PATHS ${TIMELOCK_FFI_DIR}
    PATH_SUFFIXES lib
    NO_DEFAULT_PATH
)

if(NOT TIMELOCK_FFI_LIB)
    message(FATAL_ERROR "Could not find timelock-ffi library in ${TIMELOCK_FFI_DIR}")
endif()

message(STATUS "Found timelock-ffi library: ${TIMELOCK_FFI_LIB}")

# Platform-specific settings
if(WIN32)
    # Windows-specific libraries
    set(PLATFORM_LIBS ws2_32 userenv advapi32 kernel32 ntdll bcrypt)
elseif(APPLE)
    # macOS-specific libraries
    set(PLATFORM_LIBS "-framework Security" "-framework CoreFoundation")
else()
    # Linux/Unix-specific libraries
    set(PLATFORM_LIBS pthread dl m)
endif()

# Basic C example
add_executable(basic_example basic_example.c)
target_link_libraries(basic_example ${TIMELOCK_FFI_LIB} ${PLATFORM_LIBS})

# Basic C++ example
add_executable(basic_cpp_example basic_cpp_example.cpp)
target_link_libraries(basic_cpp_example ${TIMELOCK_FFI_LIB} ${PLATFORM_LIBS})

# Set output directory
set_target_properties(basic_example basic_cpp_example PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# Custom target to build the Rust library first
add_custom_target(build_rust_lib
    COMMAND cargo build --release --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/../../timelock-ffi/Cargo.toml
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/../..
    COMMENT "Building timelock-ffi Rust library"
)

# Make sure Rust library is built before C/C++ examples
add_dependencies(basic_example build_rust_lib)
add_dependencies(basic_cpp_example build_rust_lib)

# Install targets
install(TARGETS basic_example basic_cpp_example
    RUNTIME DESTINATION bin
)

# Install the header file
install(FILES ${TIMELOCK_FFI_DIR}/timelock.h
    DESTINATION include
)

# Copy DLL to output directory on Windows
if(WIN32)
    add_custom_command(TARGET basic_example POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "${TIMELOCK_FFI_DIR}/timelock_ffi.dll"
        $<TARGET_FILE_DIR:basic_example>
        COMMENT "Copying timelock_ffi.dll to output directory"
    )
    add_custom_command(TARGET basic_cpp_example POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "${TIMELOCK_FFI_DIR}/timelock_ffi.dll"
        $<TARGET_FILE_DIR:basic_cpp_example>
        COMMENT "Copying timelock_ffi.dll to output directory"
    )
endif()
