# --- Configuration ---
CC = gcc
CFLAGS = -Wall -Wextra -O2 
RUST_DIR = target/release
BIN_DIR = bin
EXAMPLE_DIR = examples

# --- Install Configuration ---
PREFIX = /usr/local
INCLUDE_DIR = $(PREFIX)/include/nextstd
LIB_DIR = $(PREFIX)/lib

# Explicitly list the compiled Rust archives to install
LIBS_TO_INSTALL = $(RUST_DIR)/libns_data.a \
                  $(RUST_DIR)/libns_io.a \
                  $(RUST_DIR)/libns_string.a \
                  $(RUST_DIR)/libns_error.a

# Flags
# Added -Iinclude so C finds your new header folder locally
INCLUDES = -I. -Iinclude
# Added -lns_strings and -lns_data to link the Rust crates
LIBS = -L$(RUST_DIR) -lns_cmd -lns_data -lns_io -lns_string -lns_error -lpthread -ldl -lm -Wl,-rpath=$(RUST_DIR)

EXAMPLES = $(patsubst $(EXAMPLE_DIR)/%.c,%,$(wildcard $(EXAMPLE_DIR)/*.c))

# --- Targets ---
.PHONY: all rust clean clean-bin directories list install uninstall

all: list

directories:
	@mkdir -p $(BIN_DIR)

rust:
	@cargo build --release -q

list:
	@echo "Available examples:"
	@for ex in $(EXAMPLES); do echo "  $$ex"; done
	@echo ""
	@echo "Usage:"
	@echo "  make <name>   : Compile & Run (e.g., 'make 01_print_integer')"

# THE MAIN RULE
%: $(EXAMPLE_DIR)/%.c include/ns.h rust directories
	@$(CC) $(CFLAGS) $< -o $(BIN_DIR)/$@ $(INCLUDES) $(LIBS)
	@./$(BIN_DIR)/$@
	@$(MAKE) -s clean-bin

# --- Install Rules ---

install:
	@if [ ! -f $(RUST_DIR)/libns_io.a ]; then \
		echo "Error: Rust binaries not found. Please run 'make rust' first."; \
		exit 1; \
	fi
	@echo "Installing NextStd headers to $(INCLUDE_DIR)..."
	@install -d $(INCLUDE_DIR)
	@cp -r include/* $(INCLUDE_DIR)/
	@echo "Installing NextStd libraries to $(LIB_DIR)..."
	@install -d $(LIB_DIR)
	@install -m 644 $(LIBS_TO_INSTALL) $(LIB_DIR)/
	@echo "Installation complete!"
	@echo "You can now include headers in any project using: #include <nextstd/ns.h>"

uninstall:
	@echo "Removing NextStd headers from $(INCLUDE_DIR)..."
	@rm -rf $(INCLUDE_DIR)
	@echo "Removing NextStd libraries from $(LIB_DIR)..."
	@rm -f $(LIB_DIR)/libns_data.a \
	       $(LIB_DIR)/libns_io.a \
	       $(LIB_DIR)/libns_string.a \
	       $(LIB_DIR)/libns_error.a
	@echo "Uninstallation complete!"

# --- Clean Rules ---

# 1. Fast Clean: Deletes only the temporary C binaries
clean-bin:
	@rm -rf $(BIN_DIR)

# 2. Deep Clean: Deletes C binaries AND purges the Rust build cache
clean: clean-bin
	@cargo clean -q
