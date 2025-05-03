# References: 
#  - https://serokell.io/blog/practical-nix-flakes
#  - https://www.youtube.com/watch?v=yQwW8dkuHqw

{
inputs = { nixpkgs.url = "github:nixos/nixpkgs"; };
outputs = { self, nixpkgs }:
  let 
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShell.${system} = pkgs.mkShell { 
      nativeBuildInputs = with pkgs; [ 
        rustc
        cargo
        rustup
        wasm-bindgen-cli
        miniserve
        sqlite /* only required for `./web/database/create.sh` */

        /*
        Added lld because otherwise:
        error: linker `lld` not found
          |
          = note: No such file or directory (os error 2)
        */
        lld
      ];

      # https://stackabuse.com/how-to-change-the-output-color-of-echo-in-linux/
      shellHook = ''
        rustup target add wasm32-unknown-unknown
        echo -e "\033[0;32m[  OK  ]\033[0m Entered dev shell."
      '';
    };
  };
}
