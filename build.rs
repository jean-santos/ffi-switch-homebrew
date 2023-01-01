extern crate bindgen;

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let relative_path_out_dir = out_dir.replace(&cargo_manifest_dir, &"");
    println!("cargo:warning={}={}", "RELATIVE", &relative_path_out_dir);

    print_env();

    let args = format!("run --rm -t --name switchdev --user 1000:1000 -v {}:/developer devkitpro/devkita64 /opt/devkitpro/devkitA64/bin/aarch64-none-elf-gcc -c /developer/src/cool.c -g -Wall -O2 -ffunction-sections -march=armv8-a+crc+crypto -mtune=cortex-a57 -mtp=soft -fPIC -D__SWITCH__ -lnx -I/opt/devkitpro/portlibs -I/opt/devkitpro/libnx/include -L /opt/devkitpro/libnx/lib/ -o /developer{}/libcoiso.a",&cargo_manifest_dir,&relative_path_out_dir);

    let output = Command::new("docker")
        .args(args.split_whitespace())
        .current_dir(&Path::new(&cargo_manifest_dir))
        .output()
        .unwrap();

    println!("cargo:warning={}={}", "DOCKER_EXIT_CODE", &output.status);
    println!(
        "cargo:warning={}={}",
        "DOCKER_STDOUT",
        String::from_utf8(output.stdout).unwrap()
    );
    println!(
        "cargo:warning={}={}",
        "DOCKER_STDERR",
        String::from_utf8(output.stderr).unwrap()
    );

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=coiso");
    println!("cargo:rerun-if-changed=src/cool.c");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .use_core()
        .header("wrapper.h")
        .layout_tests(false)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Print relevant environment variables
fn print_env() {
    let env_keys = ["TARGET", "OUT_DIR", "HOST"];
    env::vars().for_each(|(key, val)| {
        if key.starts_with("CARGO") {
            println!("cargo:warning={}={}", key, val);
        } else if env_keys.contains(&key.as_str()) {
            println!("cargo:warning={}={}", key, val);
        } else {
        }
    });
}
