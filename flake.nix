{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ]; # "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cmake
            stdenv
            clang
            libclang
            pkg-config
            openssl
            rustc
            cargo
            rustfmt
            gdb
            # openblas
            # blas
            gsl
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib/
            export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
            exec fish
          '';
        };
      };
    };
}
