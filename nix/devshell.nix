{ perSystem, pkgs }:
pkgs.mkShell {
  # Add build dependencies
  packages = [ pkgs.rustup perSystem.self.rzup pkgs.mescc-tools pkgs.m2-planet pkgs.m2-mesoplanet pkgs.just ];

  # Add environment variables
  env.RISC0_DEV_MODE = "1";

  # Load custom bash code
  shellHook = ''
  '';
}
