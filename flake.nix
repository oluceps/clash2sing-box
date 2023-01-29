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
    let b = { target, system, ... }: {

      pkg =
        let
          inherit system target;
          pkgs = nixpkgs.legacyPackages.${system};
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
          #          RUSTFLAGS = [ "-C" "link-arg=-fuse-ld=${pkgs.mold}/bin/mold"];

          nativeBuildInputs = with pkgs;[ pkg-config ];

          buildInputs = [
            (
              if target == "aarch64-unknown-linux-gnu"
              then nixpkgs.legacyPackages."aarch64-linux".openssl
              else nixpkgs.legacyPackages."x86_64-linux".openssl
            )
          ];

          postInstall = ''
            mv $out/bin/ctos $out/bin/ctos-${target}
          '';
        };
    }; in

    flake-utils.lib.eachDefaultSystem (system: {
      packages.default =
        let
          inherit system;target = "x86_64-unknown-linux-gnu";
        in
        (b { inherit target system; }).pkg;
      packages.aarch64-linux =
        let
          inherit system;target = "aarch64-unknown-linux-gnu";
        in
        (b { inherit target system; }).pkg;

      devShells.default = with nixpkgs.legacyPackages.${system}; mkShell {
        nativeBuildInputs = [
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ])
          cargo-zigbuild
          openssl.dev
          pkg-config
        ];
      };
    });


}
