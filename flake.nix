{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec {
        # `nix build`
        packages.kura_prompt = naersk-lib.buildPackage {
          pname = "kura_prompt";
          root = ./.;
        };
        defaultPackage = packages.kura_prompt;

        # `nix run`
        apps.kura_prompt =
          flake-utils.lib.mkApp { drv = packages.kura_prompt; };
        defaultApp = apps.kura_prompt;

        # `nix develop`
        devShell =
          pkgs.mkShell { nativeBuildInputs = with pkgs; [ rustc cargo ]; };
      });
}
