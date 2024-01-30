{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    flake-parts.url = "github:hercules-ci/flake-parts";
    pre-commit-nix.url = "github:cachix/pre-commit-hooks.nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nci.url = "github:yusdacra/nix-cargo-integration";
  };

  outputs = {
    self,
    flake-parts,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.nci.flakeModule
        inputs.pre-commit-nix.flakeModule
      ];

      systems = ["x86_64-darwin" "x86_64-linux" "aarch64-darwin" "aarch64-linux"];
      perSystem = {
        config,
        pkgs,
        ...
      }: let
        inherit (config.nci) outputs;
        inherit (pkgs) llvmPackages;
      in {
        nci = {
          toolchainConfig = ./rust-toolchain.toml;
          projects.postgres_lsp = {
            path = ./.;
            export = false;
            drvConfig = {
              deps.stdenv = llvmPackages.stdenv;
            };
            depsDrvConfig = {
              deps.stdenv = pkgs.clangStdenv;
              env.LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
              mkDerivation = {
                nativeBuildInputs = with pkgs; [cmake];
              };
            };
          };

          crates = {
            codegen = {};
            parser = {
              drvConfig = {
                mkDerivation = {
                  buildInputs = with llvmPackages; [clang];
                };
              };
              depsDrvConfig = {
                mkDerivation = {
                  buildInputs = with llvmPackages; [clang];
                };
              };
            };
            pg_query_proto_parser = {
              drvConfig = {
                mkDerivation = {
                  buildInputs = with pkgs; [protobufc];
                };
              };
            };
            postgres_lsp = {};
            xtask = {};
          };
        };

        devShells.default = outputs.postgres_lsp.devShell.overrideAttrs (oa: {
          env.LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
        });

        formatter = pkgs.alejandra;

        # FIXME: currently broken due to some error from prost-build.
        # I think this has something to do with the OUT_DIR not getting setting correctly.
        # It should point to $out (the nix created out directory).
        # packages = {
        #   default = outputs.postgres_lsp.packages.release;
        # };

        pre-commit = {
          settings.hooks = {
            alejandra.enable = true;
            rustfmt.enable = true;
            cargo-check.enable = true;
          };
        };
      };
    };
}
