{
  description = "Dev shell for totp";

  inputs = {
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "aarch64-darwin";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit overlays system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          (pkgs.rust-bin.stable.latest.default.override
            {
              extensions = [ "rust-src" ];
            })
        ];
      };
    };
}
