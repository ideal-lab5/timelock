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

# Simple Makefile for timelock-ffi C examples

# Variables
CC = gcc
CXX = g++
CFLAGS = -Wall -Wextra -std=c11 -O2
CXXFLAGS = -Wall -Wextra -std=c++17 -O2
TIMELOCK_FFI_DIR = ../../timelock-ffi/target/release
INCLUDE_DIR = ../../timelock-ffi
LIB_DIR = $(TIMELOCK_FFI_DIR)
OUTPUT_DIR = target

# Platform detection
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    PLATFORM_LIBS = -pthread -ldl -lm
    LIB_NAME = libtimelock_ffi.a
endif
ifeq ($(UNAME_S),Darwin)
    PLATFORM_LIBS = -framework Security -framework CoreFoundation
    LIB_NAME = libtimelock_ffi.a
endif
ifeq ($(OS),Windows_NT)
    PLATFORM_LIBS = -lws2_32 -luserenv -ladvapi32 -lkernel32 -lntdll
    LIB_NAME = timelock_ffi.lib
endif

# Default to Linux if nothing else detected
ifndef PLATFORM_LIBS
    PLATFORM_LIBS = -pthread -ldl -lm
    LIB_NAME = libtimelock_ffi.a
endif

LDFLAGS = -L$(LIB_DIR) -ltimelock_ffi $(PLATFORM_LIBS)
INCLUDES = -I$(INCLUDE_DIR)

# Targets
TARGETS = $(OUTPUT_DIR)/basic_example $(OUTPUT_DIR)/basic_cpp_example
RUST_LIB = $(LIB_DIR)/$(LIB_NAME)

.PHONY: all clean rust-lib target-dir

all: target-dir rust-lib $(TARGETS)

target-dir:
	@mkdir -p $(OUTPUT_DIR)

rust-lib:
	@echo "Building timelock-ffi Rust library..."
	cd ../../timelock-ffi && cargo build --release --target-dir target

$(OUTPUT_DIR)/basic_example: basic_example.c $(RUST_LIB) | target-dir
	$(CC) $(CFLAGS) $(INCLUDES) -o $@ $< $(LDFLAGS)

$(OUTPUT_DIR)/basic_cpp_example: basic_cpp_example.cpp $(RUST_LIB) | target-dir
	$(CXX) $(CXXFLAGS) $(INCLUDES) -o $@ $< $(LDFLAGS)

clean:
	rm -rf $(OUTPUT_DIR)

# Help target
help:
	@echo "Available targets:"
	@echo "  all                    - Build all examples (default)"
	@echo "  rust-lib               - Build the Rust FFI library"
	@echo "  basic_example          - Build the basic C example"
	@echo "  basic_cpp_example      - Build the basic C++ example"
	@echo "  clean                  - Remove built examples"
	@echo "  help                   - Show this help"
	@echo ""
	@echo "Variables:"
	@echo "  CC         - C compiler (default: gcc)"
	@echo "  CXX        - C++ compiler (default: g++)"
	@echo "  CFLAGS     - C compiler flags"
	@echo "  CXXFLAGS   - C++ compiler flags"
	@echo "  TIMELOCK_FFI_DIR - Path to timelock-ffi build directory"

# Dependency tracking
basic_example.c: $(INCLUDE_DIR)/timelock.h
