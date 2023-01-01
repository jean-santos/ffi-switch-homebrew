with import <nixpkgs> { };
stdenv.mkDerivation {
  name = "cnx";
  buildInputs = [
    libllvm
    clang
    libclang
    llvmPackages.libclang
  ];

  shellHook = ''
    export LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib";
  '';
}