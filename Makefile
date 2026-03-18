# --- Configuration ---
CC = gcc
CFLAGS = -Wall -Wextra -O2 
RUST_DIR = target/release
BIN_DIR = bin
EXAMPLE_DIR = examples

# Flags
# UPDATED: Added -Iinclude so C finds your new header folder
INCLUDES = -I. -Iinclude
# UPDATED: Added -lns_strings to link the new Rust crate
LIBS = -L$(RUST_DIR) -lns_data -lns_io -lns_string -lns_error -lpthread -ldl -lm -Wl,-rpath=$(RUST_DIR)

EXAMPLES = $(patsubst $(EXAMPLE_DIR)/%.c,%,$(wildcard $(EXAMPLE_DIR)/*.c))

# --- Targets ---
.PHONY: all rust clean clean-bin directories list

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
# UPDATED: Changed ns.h to include/ns.h
%: $(EXAMPLE_DIR)/%.c include/ns.h rust directories
	@$(CC) $(CFLAGS) $< -o $(BIN_DIR)/$@ $(INCLUDES) $(LIBS)
	@./$(BIN_DIR)/$@
	@$(MAKE) -s clean-bin

# --- Clean Rules ---

# 1. Fast Clean: Deletes only the temporary C binaries
clean-bin:
	@rm -rf $(BIN_DIR)

# 2. Deep Clean: Deletes C binaries AND purges the Rust build cache
clean: clean-bin
	@cargo clean -q
