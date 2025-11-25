extern crate cc;

use std::path::PathBuf;

fn generate_bindings(
  // The llhttp package. It is expected to be already built.
  llhttp: PathBuf,
  // A file name, ending in '.rs', which we will create and 
  // place place the generated bindings in.
  output: PathBuf,
) {
  // C files in the llhttp package that
  // we need to include in compilation.
  let llhttp_api_c = llhttp.join("src").join("native").join("api.c");
  let llhttp_http_c = llhttp.join("src").join("native").join("http.c");
  let llhttp_llhttp_c = llhttp.join("build").join("c").join("llhttp.c");

  // Directories in the llhttp package that contain c headers
  // we need to include in compilation.
  let llhttp_headers_1 = llhttp.join("src").join("native");
  let llhttp_headers_2 = llhttp.join("build");

  // The C header describing all llhttp's public api.
  let llhttp_llhttp_h = llhttp.join("build").join("llhttp.h");

  let llhttp_bindings = bindgen
    ::Builder
    ::default()
      .header(llhttp_llhttp_h.to_str().unwrap());

  #[cfg(target_os = "macos")]
  let llhttp_bindings = llhttp_bindings
    .blocklist_type("^__darwin_.*")
    .blocklist_type("^_opaque_.*");

  llhttp_bindings
    .use_core()
    .ctypes_prefix("::libc")
    .allowlist_var("^llhttp_.*")
    .allowlist_type("^llhttp_.*")
    .allowlist_function("^llhttp_.*")
    .size_t_is_usize(true)
    .rust_target(bindgen::LATEST_STABLE_RUST)
    .derive_copy(true)
    .derive_debug(true)
    .derive_default(true)
    .derive_partialeq(true)
    .newtype_enum("llhttp_errno")
    .newtype_enum("llhttp_flags")
    .newtype_enum("llhttp_lenient_flags")
    .newtype_enum("llhttp_type")
    .newtype_enum("llhttp_method")
    .generate()
    .unwrap()
    .write_to_file(output)
    .unwrap();

  cc::Build::new()
    .file(llhttp_api_c)
    .file(llhttp_http_c)
    .file(llhttp_llhttp_c)
    .include(llhttp_headers_1)
    .include(llhttp_headers_2)
    .warnings(false)
    .compile("llhttp");
}

fn main() {
  let current_dir = std::env::current_dir().unwrap();
  
  generate_bindings(
    current_dir.join("..").join("..").join("llhttp"), 
    current_dir.join("src").join("llhttp.rs"),
  );
}