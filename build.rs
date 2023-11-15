use std::{env, path::PathBuf};

use glob::glob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sources = glob("7zip/C/*.c")?.filter_map(Result::ok);
    cc::Build::new()
        .files(sources)
        .include("7zip/Headers")
        .define("_REENTRANT", None)
        .define("_FILE_OFFSET_BITS", "64")
        .define("_LARGEFILE_SOURCE", None)
        .define("EXTERNAL_CODECS", None)
        .define("_7ZIP_LARGE_PAGES", None)
        .define("UNICODE", None)
        .define("_UNICODE", None)
        .try_compile("7zip")?;
    let include_dir = env::current_dir()?.join("7zip/Headers");
    println!("cargo:include={}", include_dir.display());

    let bindings = bindgen::Builder::default()
        .header("unified.h")
        .clang_arg(format!("-I{}", include_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;

    let bindings_out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(bindings_out_path.join("bindings.rs"))?;
    Ok(())
}
