{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default =
        let
          pkgs = nixpkgs.legacyPackages.${system};
          target = "x86_64-unknown-linux-gnu";
          toolchain = with fenix.packages.${system}; combine [
            minimal.cargo
            minimal.rustc
            targets.${target}.latest.rust-std
          ];
        in

        with (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }); buildPackage {
          src = ./.;
          CARGO_BUILD_TARGET = target;
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
            "${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/${target}-gcc";
        };

      packages.aarch64-unknown-linux-gnu =
        let
          pkgs = nixpkgs.legacyPackages.${system};
          target = "aarch64-unknown-linux-gnu";
          toolchain = with fenix.packages.${system}; combine [
            minimal.cargo
            minimal.rustc
            targets.${target}.latest.rust-std
          ];
        in

        with (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }); buildPackage {
          src = ./.;
          CARGO_BUILD_TARGET = target;
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
            "${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/${target}-gcc";
        };
    });
}
