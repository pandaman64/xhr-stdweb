with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "xhr-stdweb";
  buildInputs = [
    bashInteractive
    rustup
    openssl
    pkgconfig
  ];
  shellHook = ''
    export PATH=$PWD/.cargo/bin:$PATH
  '';
}

