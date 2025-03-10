assemble_loop:
  M1 --architecture riscv32 --little-endian -f $M2libc/riscv32/riscv32_defs.M1 -f loop.M1 -o loop.hex2
  hex2 -B 0x100000 --architecture riscv32 --little-endian -f $M2libc/riscv32/ELF-riscv32.hex2 -f loop.hex2 -o loop
