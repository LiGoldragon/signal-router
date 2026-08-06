{
  description = "signal-router - Signal contract for Persona router observations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { nixpkgs, flake-utils, fenix, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "rustc"
          "rustfmt"
          "clippy"
          "rust-src"
        ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        ethosFilter = path: _type: builtins.match ".*/ethos(/.*)?$" path != null;
        # Include `examples/` so canonical Dotos examples are present
        # at build time for `include_str!` in `tests/canonical_examples.rs`.
        examplesFilter = path: _type: builtins.match ".*/examples(/.*)?$" path != null;
        sourceFilter = path: type:
          type == "directory"
          || (craneLib.filterCargoSources path type)
          || (ethosFilter path type)
          || (examplesFilter path type);
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = sourceFilter;
          name = "source";
        };
        commonArgs = { inherit src; strictDeps = true; };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        packages.default = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        checks = {
          build = craneLib.cargoBuild (commonArgs // { inherit cargoArtifacts; });
          test = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; });
          test-round-trip = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--test round_trip";
          });
          test-interface-contract = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--test interface_contract";
          });
          # The canonical text-edge witnesses are gated behind dotos-text;
          # this check keeps the human-facing projection in the Nix proof.
          test-dotos-text = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--features dotos-text --all-targets";
          });
          fmt = craneLib.cargoFmt { inherit src; };
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          });
          clippy-dotos-text = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--features dotos-text --all-targets -- -D warnings";
          });
        };
        devShells.default = pkgs.mkShell {
          name = "signal-router";
          packages = [ pkgs.jujutsu pkgs.pkg-config toolchain ];
        };
      });
}
