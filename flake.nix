{
  description = "Launch Neovide from specified directories using Rofi";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustEnv = pkgs.rustPlatform.buildRustPackage rec {
          pname = "neorofi";
          version = "0.1.0";
          src = ./.;

          cargoSha256 = "k6hD6TeeIRSGzpuSE7xwn2aCTPH9g+ejOkVR294zIYA=";

          # buildInputs = with pkgs; [ rofi ];

        };
      in {
        packages.default = rustEnv;
        apps.default = {
          type = "app";
          program = "${rustEnv}/bin/neorofi";
        };
      });
}

