C_COMPILER = gcc
RUST_COMPILER = rustc

default:
	@echo 'Targets:'
	@echo '  lib   -- Build C library'
	@echo '  bin   -- Build Rust binary'
	@echo '  clean -- Remove build artifacts'

lib:
	$(C_COMPILER) -c simplemath.c
	ar rcs libsimplemath.a simplemath.o

bin: lib
	$(RUST_COMPILER) -l static=simplemath -L . main.rs

run: bin
	@./main

clean:
	-rm -f *.o *.so *.a main
