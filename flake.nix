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
      url = "github:LiGoldragon/Curriculum/487b69b337270adf8fde50ec10ec237d858012d5";
      flake = false;
    };
    # The runtime as released before skill sources were declared: the oracle
    # for the three-source parity check.
    curriculum-deploy-baseline = {
      url = "github:LiGoldragon/curriculum-deploy/a79cf02d3cba3a991d12a0b2a37f5e8c93ea6153";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-build.follows = "rust-build";
      inputs.curriculum.follows = "curriculum";
    };
  };

  outputs = { nixpkgs, flake-utils, rust-build, curriculum, curriculum-deploy-baseline, ... }:
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
        baseline = curriculum-deploy-baseline.packages.${system}.default;

        # Today's generator over Curriculum holding every skill, against this
        # runtime over three aspect sources that together hold exactly those
        # skills and a Curriculum root holding only role data. The two
        # workspaces and receipts must be identical byte for byte.
        threeSourceParity = pkgs.runCommand "curriculum-deploy-three-source-parity" { } ''
          set -eu
          cd "$TMPDIR"
          mkdir today psyche-skills mind-skills field-skills curriculum generated
          ${baseline}/bin/curriculum-deploy \
            "Generate.{ «${curriculum}» «$TMPDIR/today» }" > today.receipt

          cp ${curriculum}/roles.datom curriculum/
          index=0
          for skill in ${curriculum}/skills/*.md; do
            case $((index % 3)) in
              0) cp "$skill" psyche-skills/ ;;
              1) cp "$skill" mind-skills/ ;;
              2) cp "$skill" field-skills/ ;;
            esac
            index=$((index + 1))
          done
          authored=$(ls ${curriculum}/skills/*.md | wc -l)
          split=$(ls psyche-skills/*.md mind-skills/*.md field-skills/*.md | wc -l)
          test "$authored" -gt 0
          test "$authored" = "$split"
          test -n "$(ls psyche-skills)" && test -n "$(ls mind-skills)" && test -n "$(ls field-skills)"

          ${package}/bin/curriculum-deploy \
            "Generate.{ «$TMPDIR/curriculum» [ Psyche.«$TMPDIR/psyche-skills» Mind.«$TMPDIR/mind-skills» Field.«$TMPDIR/field-skills» ] «$TMPDIR/generated» }" \
            > generated.receipt

          diff -r today generated
          cmp today.receipt generated.receipt
          files=$(find today -type f | wc -l)
          mkdir "$out"
          {
            echo "skills: $authored (psyche $(ls psyche-skills | wc -l), mind $(ls mind-skills | wc -l), field $(ls field-skills | wc -l))"
            echo "generated files compared: $files"
            echo "receipt: $(cat today.receipt)"
            (cd today && find . -type f -print0 | sort -z | xargs -0 sha256sum) | sha256sum | sed 's/^/tree digest: /'
          } > "$out/witness"
        '';
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
          three-source-parity = threeSourceParity;
          fmt = craneLib.cargoFmt { inherit src; };
          clippy = craneLib.cargoClippy (common // {
            cargoArtifacts = artifacts;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          });
        };
        devShells.default = pkgs.mkShell { packages = [ toolchain pkgs.jujutsu ]; };
      });
}
