# RUST

let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in

with nixpkgs;

mkShell {
  name = "rust";
  #buildInputs = [ rustc cargo cargo-make openssl rustup lld rust-analyzer rustfmt clippy pkgconfig ];
  buildInputs = [ cargo-make watchexec clippy rustfmt rust-analyzer openssl lld pkgconfig
  python39Packages.websockify
  latest.rustChannels.stable.rust
  latest.rustChannels.stable.cargo
  latest.rustChannels.stable.rust-src
  ];

  shellHook = ''
    export PATH=$PATH:~/.cargo/bin
  '';
}
