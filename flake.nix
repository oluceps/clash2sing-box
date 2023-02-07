{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:

    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      rec {
        packages.default =
          let
            toolchain = with fenix.packages.${system}; combine [
              minimal.cargo
              minimal.rustc
              targets.x86_64-unknown-linux-gnu.latest.rust-std
              targets.aarch64-unknown-linux-gnu.latest.rust-std
              targets.aarch64-apple-darwin.latest.rust-std
              targets.x86_64-apple-darwin.latest.rust-std
            ];
          in
          (pkgs.makeRustPlatform
            {
              cargo = toolchain;
              rustc = toolchain;
            }
          ).buildRustPackage
            {
              name = "clash2sing-box";

              src = self;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              postInstall = ''
                mv $out/bin/ctos $out/bin/ctos-${system}
              '';
            };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ packages.default ];
          nativeBuildInputs = with pkgs;[ cargo-zigbuild rustup ];
        };
      });


}
