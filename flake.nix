{
  description = "Nix Flake for openapi client generation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        packages = with pkgs; [
          jre
          openapi-generator-cli
        ];
      in
      {
        packages.backend = pkgs.rustPlatform.buildRustPackage {
          name = "backend";
          src = ./backend;
          cargoLock.lockFile = ./backend/Cargo.lock;
          # buildType = "debug";
        };

        packages.client = pkgs.rustPlatform.buildRustPackage {
          name = "client";
          src = ./client;
          cargoLock.lockFile = ./client/Cargo.lock;
          nativeBuildInputs = packages;
          # buildType = "debug";
        };

        devShells.default = pkgs.mkShell {
          shellHook = ''
            ORIGINAL_PS1="$PS1"
            NIX_PS1="(nix) \[$ORIGINAL_PS1\]"
            export PS1="$NIX_PS1"
          '';
          inherit packages;
        };
      }
    );
}
