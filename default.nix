{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "jellyfin-rpc";
  version = "1.3.4";

  src = ./.;

  cargoHash = "sha256-dMCiguS2pPijpRUEetabiidGcHM8rHvBHYcsiY0/i1w=";

  meta = {
    description = "Displays the content you're currently watching on Discord";
    homepage = "https://github.com/NormalFall/jellyfin-rpc";
    license = lib.licenses.gpl3Only;
    maintainers = with lib.maintainers; [ ];
    mainProgram = "jellyfin-rpc";
  };
}
