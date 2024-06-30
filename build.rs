//! Inspired by setup in io-uring crate
#[cfg(not(feature = "bindgen"))]
fn main() {}

#[cfg(feature = "bindgen")]
fn main() {
    use std::env;
    use std::path::PathBuf;

    const INCLUDE: &str = r#"
#include <linux/nvme_ioctl.h>
    "#;

    #[cfg(not(feature = "overwrite"))]
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "overwrite")]
    let outdir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/sys");

    let mut builder = bindgen::Builder::default();

    builder = builder.header_contents("include-file.h", INCLUDE);

    // sudo dnf install kernel-devel (for nvme.h)
    builder
        .ctypes_prefix("libc")
        .prepend_enum_name(false)
        .derive_default(true)
        .generate_comments(true)
        .use_core()
        .allowlist_type("nvme_uring_cmd")
        .generate()
        .unwrap()
        .write_to_file(outdir.join("generated.rs"))
        .unwrap();
}
