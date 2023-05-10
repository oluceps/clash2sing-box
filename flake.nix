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

                mainProgram = "ctos-${system}";

                nativeBuildInputs = with pkgs; [ pkg-config ];
                buildInputs = with pkgs; [ openssl ];

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
    # // {
    #   apps = nixpkgs.lib.listToAttrs ((map (n: {
    #     name = "${n}.default";
    #     value =
    #       {
    #         type = "app";
    #         program = "${self.packages.${n}.default}/bin/ctos-${n}";
    #       };
    #   })) [ "aarch64-linux" "x86_64-linux" ]);
    # }
  ;
  # {
  #   apps.x86_64-linux.default = {
  #     type = "app";
  #     program = "${self.packages.x86_64-linux.default}/bin/ctos-x86_64-linux";
  #   };
  # };


}
