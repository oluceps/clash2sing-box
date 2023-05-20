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

    flake-utils.lib.eachSystem
      (with flake-utils.lib.system;
      [ x86_64-linux aarch64-linux ])
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
          version = pkgs.lib.substring 0 8 self.lastModifiedDate
            or self.lastModified or "19700101";
        in
        rec {
          packages.default =
            (pkgs.makeRustPlatform {
              inherit (fenix.packages.${system}.minimal) cargo rustc;
            }).buildRustPackage
              {
                name = "clash2sing-box";

                src = self;
                inherit version;

                cargoLock = {
                  lockFile = ./Cargo.lock;
                  outputHashes = {
                    "yew-0.20.0" = "sha256-x1Z85JMpeJqz1R8D4dbMLol06ZN0bVyIuA057H6Ce38=";
                  };
                };


                mainProgram = "ctos-${system}";

                nativeBuildInputs = with pkgs; [ pkg-config ];
                buildInputs = with pkgs; [ openssl ];

                # network required
                # doCheck = false;

                postInstall = ''
                  mv $out/bin/ctos $out/bin/ctos-${system}
                '';
              };
          devShells.default = pkgs.mkShell {
            inputsFrom = [ packages.default ];
          };


          apps.default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/ctos-${system}";
          };
        })
  ;

}
