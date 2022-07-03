{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };
  
  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec
    {
      packages.appendage = naersk-lib.buildPackage {
        pname = "apd";
        root = ./.;
      };
        
      packages.default = packages.appendage;
        
      apps.appendage = utils.lib.mkApp {
        drv = packages.appendage;  
      };
        
      apps.default = apps.appendage;
        
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
        ];
      };
    });
}