// This is almost a copy of `build.rs` in `wasm-bindgen-webidl` crate.

use anyhow::{Context, Result};
use sourcefile::SourceFile;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path;
use std::process::Command;

fn main() -> Result<()> {
    #[cfg(feature = "env_logger")]
    env_logger::init();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=webidls");

    let entries = fs::read_dir("webidls").context("reading webidls directory")?;
    let mut source = SourceFile::default();
    for entry in entries {
        let entry = entry.context("getting webidls/*.webidl entry")?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("webidl")) {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path.display());
        source = source
            .add_file(&path)
            .with_context(|| format!("reading contents of file \"{}\"", path.display()))?;
    }

    let allowed = None; // Allow all definitions.
    let compile_results = wasm_bindgen_webidl::compile(&source.contents, allowed);
    let bindings = match compile_results {
        Ok(bindings) => bindings,
        Err(e) => {
            if let Some(err) = e.downcast_ref::<wasm_bindgen_webidl::WebIDLParseError>() {
                if let Some(pos) = source.resolve_offset(err.0) {
                    let ctx = format!(
                        "compiling WebIDL into wasm-bindgen bindings in file \
                         \"{}\", line {} column {}",
                        pos.filename,
                        pos.line + 1,
                        pos.col + 1
                    );
                    return Err(e.context(ctx));
                } else {
                    return Err(e.context("compiling WebIDL into wasm-bindgen bindings"));
                }
            }
            return Err(e.context("compiling WebIDL into wasm-bindgen bindings"));
        }
    };

    let out_dir = env::var("OUT_DIR").context("reading OUT_DIR environment variable")?;
    let out_file_path = path::Path::new(&out_dir).join("bindings.rs");
    fs::write(&out_file_path, bindings).context("writing bindings to output file")?;
    println!("cargo:rustc-env=BINDINGS={}", out_file_path.display());

    // run rustfmt on the generated file - really handy for debugging
    //
    // This is opportunistic though so don't assert that it succeeds.
    println!("cargo:rerun-if-env-changed=WEBIDL_RUSTFMT_BINDINGS");
    if env::var("WEBIDL_RUSTFMT_BINDINGS").ok() != Some("0".to_string()) {
        drop(Command::new("rustfmt").arg(&out_file_path).status());
    }

    Ok(())
}
