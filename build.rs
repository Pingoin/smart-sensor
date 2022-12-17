use std::env;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    match std::env::var("OPT_LEVEL") {
        Ok(level) => {
            if level != "2" && level != "3" {
                let message = format!(
                    "esp-wifi should be built with optimization level 2 or 3 - yours is {}. 
                    See https://github.com/esp-rs/esp-wifi",
                    level
                )
                .to_string();
                println!("cargo:warning={}", message);
            }
            ()
        }
        Err(_err) => (),
    }

    handle_chip();

    Ok(())
}

fn handle_chip() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");
}
