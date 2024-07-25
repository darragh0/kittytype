TARGET = kittytype

CC = gcc
CFLAGS = -Wall -Wextra -Werror -I./src
LIBS = -lncurses

BUILD_DIR = build
OBJ_DIR := $(BUILD_DIR)/obj
BIN_DIR := $(BUILD_DIR)/bin

SRC = src/main.c src/kb.c src/ui.c
OBJ := $(SRC:src/%.c=$(OBJ_DIR)/%.o)
BIN := $(BIN_DIR)/$(TARGET)

all: $(BIN_DIR) $(OBJ_DIR) $(BIN)

$(BIN_DIR) $(OBJ_DIR):
	@mkdir -p $(BIN_DIR) $(OBJ_DIR)

$(BIN): $(OBJ)
	@$(CC) $(OBJ) $(LIBS) -o $@

$(OBJ_DIR)/%.o: src/%.c
	@$(CC) $(CFLAGS) -c $< -o $@

clean:
	@rm -rf $(BUILD_DIR)

run:
	@./$(BIN)

.PHONY: all clean run
