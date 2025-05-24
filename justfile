default:
  cargo run --bin zb import target/demo.zbg

setup:
  rzup install r0vm 1.2.5

demo:
  cargo run --bin demo target/demo.zbg
