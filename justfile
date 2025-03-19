demo:
  @just blob/derived
  echo -n test | RISC0_DEV_MODE=0 cargo run blob/derived/jcat | xxd
