use std::error::Error;
use walkdir::{DirEntry, WalkDir};

static OUR_MACHINERY_PATH: &str = "C:/dev/our_machinery/headers";

fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = bindgen::Builder::default();

    for header in WalkDir::new(OUR_MACHINERY_PATH)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|d| d.file_type().is_file())
        .map(DirEntry::into_path)
        .filter(|d| d.extension().map_or(false, |e| e == "h"))
        .filter_map(|p| p.into_os_string().into_string().ok())
    {
        builder = builder.header(header);
    }

    let out_path = "../src/ffi.rs";

    builder
        .clang_arg(&format!("-I{}", OUR_MACHINERY_PATH))
        .clang_arg("-Wno-microsoft-anon-tag")
        .rust_target(bindgen::RustTarget::Stable_1_40)
        .derive_debug(false)
        .layout_tests(false)
        .generate()
        .map_err(|_| "Failed to generate bindings")?
        .write_to_file(out_path)?;

    Ok(())
}
