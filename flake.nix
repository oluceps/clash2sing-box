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
          (pkgs.makeRustPlatform {
            inherit (fenix.packages.${system}.minimal) cargo rustc;
          }).buildRustPackage
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
