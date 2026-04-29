.SECONDARY:

tests/%.s: tests/%.snek src/main.rs
	cargo run -- -c $< tests/$*.s

tests/%.run: tests/%.s src/runtime/start.rs
	nasm -f elf64 tests/$*.s -o src/runtime/our_code.o
	ar rcs src/runtime/libour_code.a src/runtime/our_code.o
	rustc -C link-args="-no-pie" -L src/runtime src/runtime/start.rs -o tests/$*.run
