let 
    rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
    pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
    rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
            "rust-src"
        ];
    };
in {
  # https://devenv.sh/packages/
  packages = [ 
      rust
      pkgs.git
      pkgs.rustfmt
      pkgs.rust-analyzer
      pkgs.openssl
      pkgs.pkg-config
  ];

  enterShell = ''
    cargo install aoc-cli --version 0.5.0
  '';
}
