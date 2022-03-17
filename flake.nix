{
  description = "Profian Drawbridge";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/master;
  inputs.flake-compat.flake = false;
  inputs.flake-compat.url = github:edolstra/flake-compat;
  inputs.flake-utils.url = github:numtide/flake-utils;
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";
  inputs.fenix.url = github:nix-community/fenix;

  outputs = { self, nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        apiSpec = "api/api.yml";
        docOutput = "doc/index.html";

        pkgs = nixpkgs.legacyPackages.${system};

        nix = "${pkgs.nix}/bin/nix --extra-experimental-features flakes --extra-experimental-features nix-command";

        rust = fenix.packages."${system}".fromToolchainFile { file = ./rust-toolchain.toml; };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            rust

            pkgs.redoc-cli

            (pkgs.writeShellScriptBin "build-doc" ''
              ${nix} build '.#doc' -o '${docOutput}'
            '')
            (pkgs.writeShellScriptBin "watch-doc" ''
              ${pkgs.fd}/bin/fd | ${pkgs.ripgrep}/bin/rg 'api.yml' | ${pkgs.entr}/bin/entr -rs "${pkgs.redoc-cli}/bin/redoc-cli serve '${apiSpec}'"
            '')
          ];
        };

        packages = flake-utils.lib.flattenTree {
          doc = pkgs.stdenv.mkDerivation {
            name = "doc";
            src = self;
            buildInputs = [ pkgs.redoc-cli ];
            buildPhase = "redoc-cli bundle '${apiSpec}' -o index.html";
            installPhase = "mv index.html $out";
          };
        };
      }
    );
}
