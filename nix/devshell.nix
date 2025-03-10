{ perSystem, pkgs }:
pkgs.mkShell {
  # Add build dependencies
  packages = [ pkgs.rustup perSystem.self.rzup pkgs.mescc-tools pkgs.just ];

  # Add environment variables
  env.RISC0_DEV_MODE = "1";
  env.M2libc = "${pkgs.m2libc}/include/M2libc";

  # Load custom bash code
  shellHook = ''

  '';
}
