{ pkgs }:
let
arch="Linux-X64";
url="https://risc0-artifacts.s3.us-west-2.amazonaws.com/rzup/prod/${arch}/rzup";
hash = "sha256-fnx2ei4Mq3fDkY6HnY3pKo1dKM+NutdxhyJ9LHBEVwM=";
src = pkgs.fetchurl { inherit url hash; };
in
pkgs.runCommandNoCC "rzup" {
inherit src;
} ''
install -D $src $out/bin/rzup
''
