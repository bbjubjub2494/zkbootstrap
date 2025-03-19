{ pkgs }:
let
arch="Linux-X64";
url="https://risc0-artifacts.s3.us-west-2.amazonaws.com/rzup/prod/${arch}/rzup";
hash = "sha256-1p1rK/XLbzcpblZKS7gCUZj7paZ2/N/HgHDIv0hOMms=";
src = pkgs.fetchurl { inherit url hash; };
in
pkgs.runCommandNoCC "rzup" {
inherit src;
} ''
install -D $src $out/bin/rzup
''
