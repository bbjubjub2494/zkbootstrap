demo:
  @just blob/derived
  echo -n test | RISC0_DEV_MODE=0 cargo run blob/derived/jcat | xxd
  echo 7465 7374 0a | RISC0_DEV_MODE=0 cargo run blob/derived/jhex
