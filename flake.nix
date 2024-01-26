{
  description = "A flake for knuckles, with Hercules CI support";

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
    hercules-ci-effects.url = "github:hercules-ci/hercules-ci-effects";
  };

  outputs = inputs@{ self, flake-parts, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.hercules-ci-effects.flakeModule
      ];
      hercules-ci.flake-update = {
        enable = true;
        when = {
          hour = [ 7 ];
          dayOfWeek = [ "Sat" ];
        };
        autoMergeMethod = "rebase";
      };
      systems = [ "x86_64-linux" ];
      perSystem = { pkgs, system, ... }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
        rustWithWasiTarget = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-wasi" "x86_64-unknown-linux-musl" ];
        };
          # craneLib = crane.lib.${system};
        craneLib = (crane.mkLib pkgs).overrideToolchain rustWithWasiTarget;
          commonArgs = {
            src = craneLib.cleanCargoSource (craneLib.path ./.);
            buildInputs = with pkgs; [
              # Add additional build inputs here
            ] ++ lib.optionals stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              libiconv
            ];

          };
          cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
            pname = "knuckles-deps";
          });

          knuckles-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          knuckles-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
          });


          knuckles = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });
        in
        {
          checks = {
            inherit
              knuckles
              knuckles-clippy
              knuckles-nextest;
          };
          packages.default = knuckles;

          apps.default = flake-utils.lib.mkApp {
            drv = knuckles;
          };

          devShells.default = craneLib.devShell {};
        };

    };

  nixConfig = {
    extra-substituters = [ "https://knuckles.cachix.org" ];
    extra-trusted-public-keys = [ "knuckles-rs.cachix.org-1:eBlz21yOSDiD+CeBoGKbDE27FWtFNgIs1W84fQ8JpTQ=" ];
  };

}
