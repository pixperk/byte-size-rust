# Makefile for the gRPC Chat Application

.PHONY: server client help all

# Default target
help:
	@echo "Available commands:"
	@echo "  make server  - Run the gRPC server"
	@echo "  make client  - Run the gRPC client"
	@echo "  make all     - Build the entire project"
	@echo "  make help    - Show this help message"

# Run the gRPC server
server:
	cargo run --quiet --bin grpc-server

# Run the gRPC client
client:
	cargo run --quiet --bin grpc-client

# Build everything
all:
	cargo build --quiet