{
  description = "tseeley.com";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  outputs =
    { self, nixpkgs, ... }:
    let
      forAll = nixpkgs.lib.genAttrs [
        "aarch64-darwin"
        "x86_64-linux"
      ];
    in
    {
      packages = forAll (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.stdenvNoCC.mkDerivation {
            name = "tseeley.com";
            src = self;
            # zola's load_data builds an https client even for local files,
            # which panics without CA certificates in the sandbox
            nativeBuildInputs = [
              pkgs.zola
              pkgs.cacert
            ];
            buildPhase = "zola build --output-dir $out";
            dontInstall = true;
          };
        }
      );

      devShells = forAll (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.zola
              pkgs.djlint
            ];
          };
        }
      );

      # `nix flake check` fails if the templates aren't formatted
      checks = forAll (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          templates-formatted = pkgs.runCommand "templates-formatted" {
            nativeBuildInputs = [ pkgs.djlint ];
          } "cd ${self} && djlint templates --check && touch $out";
        }
      );

      # `nix fmt` formats the templates; settings live in djlint.toml
      formatter = forAll (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        pkgs.writeShellApplication {
          name = "fmt";
          runtimeInputs = [ pkgs.djlint ];
          # ignore the paths `nix fmt` passes in; only templates/ is formatted
          text = "djlint templates --reformat";
        }
      );
    };

}
