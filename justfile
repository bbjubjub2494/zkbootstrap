default:
  cargo run --bin zb import target/demo.zbb
  cargo run --bin zb deps-tree store/node/637ac3d3cc0e512f5773ce29ffeb29dd2c9c51161c7a8db245308595b9c7dc54


setup:
  rzup install r0vm 1.2.5  # required to generate and verify proofs
  rzup install rust  # required to compile guest programs
  rzup install cpp

prepare-demo:
  cargo run --bin demo target/demo.zbb
