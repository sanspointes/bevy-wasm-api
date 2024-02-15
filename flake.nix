{
  description = "Bearbroider Stack";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";
  
  inputs.turbo.url = "github:alexghr/turborepo.nix/v1.8.3";
  inputs.turbo.inputs.nixpkgs.follows = "nixpkgs";
  
  inputs.utils.url = "github:numtide/flake-utils";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, turbo, utils, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-Q9UgzzvxLi4x9aWUJTn+/5EXekC98ODRU1TwhUs9RnY=";
        };
        rustPlatform = pkgs.makeRustPlatform.override { stdenv = pkgs.clangStdenv; }  {
          inherit (rustToolchain) cargo rustc;
        };

        platformBuildInputs = {
            x86_64-linux = [ 
                # Monorepo build system
                turbo.packages.${system}.default
                pkgs.nodejs
                pkgs.nodePackages.pnpm
                pkgs.nodePackages.typescript
                pkgs.nodePackages.typescript-language-server

                rustPlatform.bindgenHook

                pkgs.openssl
                pkgs.binaryen
                pkgs.pkg-config
                pkgs.udev
                pkgs.alsa-lib
                pkgs.vulkan-loader

                pkgs.xorg.libX11
                pkgs.xorg.libXcursor
                pkgs.xorg.libXi
                pkgs.xorg.libXrandr
                # To use the x11 feature
                ];
            x86_64-darwin = [
                # Rust deps
                pkgs.clang
                pkgs.iconv
                pkgs.darwin.libobjc
                pkgs.darwin.apple_sdk.frameworks.AppKit
                rustPlatform.bindgenHook


                # Monorepo build system
                turbo.packages.${system}.default
                pkgs.nodejs
                pkgs.nodePackages.pnpm
                pkgs.nodePackages.typescript
                pkgs.nodePackages.typescript-language-server
            ];
        };
      in {
        devShells.default = with pkgs; mkShell rec {
          nativeBuildInputs = [ bashInteractive ];
          buildInputs = platformBuildInputs.${system};
          shellHook = ''
            export TURBO_BINARY_PATH="${turbo.packages.${system}.default}/bin/turbo"
          '';
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
