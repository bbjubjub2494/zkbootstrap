demo:
  @just blob/derived
  cargo build --release
  echo -n | RISC0_DEV_MODE=0 target/release/zkbootstrap blob/derived/jcat
  echo 7465 7374 0a | RISC0_DEV_MODE=0 target/release/zkbootstrap blob/derived/jhex

setup:
  rzup install r0vm 1.2.5
