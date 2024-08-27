# Copyright 2024 Gavin John
# SPDX-License-Identifier: GPL-3.0-or-later

{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        with pkgs;
        let
          cargoData = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        in
        {
          packages.libardriversffi =

            rustPlatform.buildRustPackage {
              pname = cargoData.package.name;
              version = cargoData.package.version;

              outputs = [ "out" "dev" ];

              src = ./.;
              cargoLock.lockFile = lib.cleanSource ./Cargo.lock;

              nativeBuildInputs = [ pkg-config ];
              buildInputs = [
                rustPlatform.bindgenHook
                udev
                opencv
                stdenv.cc.libc
              ];

              dontCargoCheck = true; # bindgen broken

              LLVM_CONFIG_PATH = "${llvm}/bin/llvm-config";

              postInstall = ''
                mkdir -p $dev/include
                cp -r target/include/. $dev/include/
              '';
            };
          packages.default = self'.packages.libardriversffi;

          devShells.default = self'.packages.default.overrideAttrs (oldAttrs: {
            nativeBuildInputs = oldAttrs.nativeBuildInputs ++ ([
              yamllint
              reuse
              clippy
            ]);
          });
        };
    };
}
