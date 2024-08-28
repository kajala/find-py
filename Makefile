# Define the name of the project binary
PROJECT_NAME = find-py

# Define the source directory
SRC_DIR = src

# Default target
all: build

# Build the project
build:
	cargo build --release

# Run the project
run: build
	./target/release/$(PROJECT_NAME)

# Clean the build artifacts
clean:
	cargo clean

# Define the phony targets
.PHONY: all build run clean
