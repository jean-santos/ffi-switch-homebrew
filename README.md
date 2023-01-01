### FFI on switch
This sample uses [rust nx](https://github.com/aarch64-switch-rs/nx).
On our build.rs it's compiling the C sources using devkitpro/devkita64 oficial docker image

To buid, use the [cargo nx](https://github.com/aarch64-switch-rs/cargo-nx) tool then,
```
cargo nx build
```