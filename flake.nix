{
  description = "Curriculum deployment runtime";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/2d1e72b652ee13fd1297641ce735e06416d22827";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # The external data: Curriculum's roles, and the three belief
    # repositories whose modules sit at <kind>/<subject>/<version>.md.
    curriculum = {
      url = "github:LiGoldragon/Curriculum/e3ffb318e4575a67323a52214711856d9a206ed0";
      flake = false;
    };
    psyche-belief = {
      url = "github:LiGoldragon/psyche-belief/fdd8775ac006b1f638897b5f120b1e1fb3a96410";
      flake = false;
    };
    mind-belief = {
      url = "github:LiGoldragon/mind-belief/573223da3c0cac29eb40091757802f54b08d27cc";
      flake = false;
    };
    field-belief = {
      url = "github:LiGoldragon/field-belief/a515b8017d5be6de0bb1130fcf979bb9b2e21620";
      flake = false;
    };
  };

  outputs = { nixpkgs, flake-utils, rust-build, curriculum, psyche-belief, mind-belief, field-belief, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromPkgs pkgs;
        inherit (rust) craneLib toolchain;

        # Cargo source plus the ethos file (needed by the freshness test).
        ethosFilter = path: type:
          type == "regular" && pkgs.lib.hasSuffix ".ethos" path;
        src = rust.cleanSource {
          root = ./.;
          extraFilters = [ ethosFilter ];
        };
        vendor = craneLib.vendorCargoDeps {
          inherit src;
          cargoLock = ./Cargo.lock;
        };
        common = {
          inherit src;
          cargoVendorDirectory = vendor;
          cargoLock = ./Cargo.lock;
          strictDeps = true;
        };
        artifacts = craneLib.buildDepsOnly common;
        package = craneLib.buildPackage (common // { cargoArtifacts = artifacts; });
      in
      {
        packages = {
          curriculum-deploy = package;
          default = package;
        };
        apps.default = {
          type = "app";
          program = "${package}/bin/curriculum-deploy";
        };
        checks = {
          build = craneLib.cargoBuild (common // { cargoArtifacts = artifacts; });
          test = craneLib.cargoTest (common // { cargoArtifacts = artifacts; });
          external-data = craneLib.cargoTest (common // {
            cargoArtifacts = artifacts;
            CURRICULUM_TEST_DATA_ROOT = curriculum;
            CURRICULUM_TEST_PSYCHE_BELIEF = psyche-belief;
            CURRICULUM_TEST_MIND_BELIEF = mind-belief;
            CURRICULUM_TEST_FIELD_BELIEF = field-belief;
            cargoTestExtraArgs = "--test runtime -- --include-ignored";
          });
          fmt = craneLib.cargoFmt { inherit src; };
          clippy = craneLib.cargoClippy (common // {
            cargoArtifacts = artifacts;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          });
        };
        devShells.default = pkgs.mkShell { packages = [ toolchain pkgs.jujutsu ]; };
      });
}
