{
  description = "Curriculum deployment runtime";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/2d1e72b652ee13fd1297641ce735e06416d22827";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    curriculum = {
      url = "github:LiGoldragon/Curriculum/5716f71aae7d0927ed01e02ad0cf2edd9f4aadac";
      flake = false;
    };
  };

  outputs = { nixpkgs, flake-utils, rust-build, curriculum, ... }:
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
