{
  description = "Nix flake for Project Halogen";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            pkg-config
            git
            direnv
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            cargo-dist
            cargo-release
            cargo-expand
            imagemagick
            llvmPackages.libclang
          ];

          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

          BINDGEN_EXTRA_CLANG_ARGS =
            "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include" +
            " -isystem ${pkgs.glibc.dev}/include" +
            " -DMAGICKCORE_HDRI_ENABLE=1" +
            " -DMAGICKCORE_QUANTUM_DEPTH=16";
        };
      }
    );
}
