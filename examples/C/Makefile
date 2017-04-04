LIB_DIR := ../../target/debug/
HEADER_DIR := $(LIB_DIR)/include/
LIBDBGP_HEADER := $(HEADER_DIR)/libdbgp.h
LIBDBGP_ST := $(LIB_DIR)/libdbgp.a
LIBDBGP_DYN := $(LIB_DIR)/libdbgp.so

OBJ := $(shell find . -type f -name '*.o')

CC=gcc
CFLAGS = -I$(HEADER_DIR) -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lutil

.PHONY: clean

%.o: %.c $(LIBDBGP_DYN) $(LIBDBGP_HEADER)
	$(CC) -c -o $@ $< $(CFLAGS)

escape: escape/escape
escape/escape: escape/main.o
	gcc -o $@ $^ $(CFLAGS) $(LIBDBGP_ST)

clean:
	rm $(OBJ) escape/escape
