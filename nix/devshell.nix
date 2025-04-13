{ perSystem, pkgs }:
pkgs.mkShell {
  # Add build dependencies
  packages = [ pkgs.rustup perSystem.self.rzup pkgs.mescc-tools pkgs.m2-planet pkgs.m2-mesoplanet pkgs.just ];

  # Add environment variables
  env.M2LIBC_PATH = "${pkgs.m2libc}/include/M2libc";

  # Load custom bash code
  shellHook = ''
  '';
}
