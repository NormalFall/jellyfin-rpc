{
  final,
  prev,
}:
with final.pkgs; rec {
  jellyfin-rpc = callPackage ./default.nix {};
}