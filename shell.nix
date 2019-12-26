with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "env";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    binutils gcc gnumake openssl pkgconfig gdb-multitarget openocd gcc-arm-embedded libusb
    nodejs
  ];
}
