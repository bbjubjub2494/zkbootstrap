default: setup demo

setup:
  rzup install r0vm 1.2.5

demo:
  cargo run --bin demo target/demo.zbg
