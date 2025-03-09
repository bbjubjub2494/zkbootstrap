{ perSystem, pkgs }:
pkgs.mkShell {
  # Add build dependencies
  packages = [ pkgs.rustup perSystem.self.rzup ];

  # Add environment variables
  env.RISC0_DEV_MODE = "1";

  # Load custom bash code
  shellHook = ''

  '';
}
