{
  description = "A flake for knuckles";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    flake-utils.url = "github:numtide/flake-utils";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    self,
    flake-parts,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        pkgs,
        system,
        ...
      }: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        rustWithWasiTarget = pkgs.rust-bin.stable.latest.default.override {
          targets = ["wasm32-wasi" "x86_64-unknown-linux-musl"];
        };
        # craneLib = crane.lib.${system};
        craneLib = (crane.mkLib pkgs).overrideToolchain rustWithWasiTarget;
        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          buildInputs = with pkgs;
            [
              # Add additional build inputs here
            ]
            ++ lib.optionals stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              libiconv
            ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        knuckles-clippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        knuckles-nextest = craneLib.cargoNextest (commonArgs
          // {
            inherit cargoArtifacts;
          });

        knuckles = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
          });
      in {
        checks = {
          inherit
            knuckles
            knuckles-clippy
            knuckles-nextest
            ;
        };
        packages.default = knuckles;

        apps.default = flake-utils.lib.mkApp {
          drv = knuckles;
        };

        devShells.default = craneLib.devShell {};
        formatter = pkgs.alejandra;
      };
    };

  nixConfig = {
    extra-substituters = ["https://knuckles-rs.cachix.org"];
    extra-trusted-public-keys = ["knuckles-rs.cachix.org-1:eBlz21yOSDiD+CeBoGKbDE27FWtFNgIs1W84fQ8JpTQ="];
  };
}
