catm:
  M1 --architecture riscv32 --little-endian -f M2libc/riscv32/riscv32_defs.M1 -f catm.M1 -o catm.hex2
  hex2 -B 0x100000 --architecture riscv32 --little-endian -f M2libc/riscv32/ELF-riscv32.hex2 -f catm.hex2 -o catm
  cargo run catm

cat:
  M2-Mesoplanet -A riscv32 -f cat.c -o cat
  echo hello world! | cargo run cat

sha256sum:
  M2-Mesoplanet -A riscv32 -f sha256sum.c -o sha256sum
  cargo run sha256sum

compile_hello:
  M2-Mesoplanet -A riscv32 -f hello.c -o hello
  cargo run

assemble_loop:
  M1 --architecture riscv32 --little-endian -f M2libc/riscv32/riscv32_defs.M1 -f loop.M1 -o loop.hex2
  hex2 -B 0x100000 --architecture riscv32 --little-endian -f M2libc/riscv32/ELF-riscv32.hex2 -f loop.hex2 -o loop
