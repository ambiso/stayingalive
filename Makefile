.PHONY: all
all: test_rng
	./test_rng

test_rng: crush.c
	cargo build --release
	gcc -o test_rng crush.c target/release/libbroken_prng.so -ltestu01 -lprobdist -lmylib -lm

