# Variables
PROTO_SRC = company.proto
PROTO_OUT_DIR = src
RUST_OUT_DIR = src

# Commands
PROTOC = protoc
PROTOC_GEN_RUST = --rust_out=$(RUST_OUT_DIR)
PROTOC_GEN_GRPC = --grpc_out=$(RUST_OUT_DIR) # --plugin=protoc-gen-grpc=`which grpc_rust_plugin`

.PHONY: all clean

all: proto

build:
	cargo build

proto:
	$(PROTOC) $(PROTO_SRC) $(PROTOC_GEN_RUST)

clean:
	echo "Cleaning up..."
	rm -rf $(PROTO_OUT_DIR)/company.proto.rs# Remove generated rust file
