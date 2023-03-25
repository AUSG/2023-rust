{
  description = "sudosubin/2023-rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
          inherit (pkgs.stdenvNoCC.hostPlatform) isDarwin;

        in
        {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
            ];

            nativeBuildInputs = with pkgs; [
              rustfmt
              rustup
              libiconv
            ] ++ pkgs.lib.optionals isDarwin [ darwin.apple_sdk.frameworks.Security pkgconfig openssl ];
          };
        }
      );
}
