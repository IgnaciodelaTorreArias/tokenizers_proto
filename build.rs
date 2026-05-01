#[cfg(debug_assertions)]
extern crate cbindgen;
#[cfg(debug_assertions)]
use std::env;
#[cfg(debug_assertions)]
use std::path::Path;
use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["protos/lib.proto"], &["protos/"])?;
    #[cfg(target_os = "windows")]
    {
        let mut res = tauri_winres::WindowsResource::new();
        res.compile()?;
    }
    #[cfg(debug_assertions)]
    {
        use cbindgen::{Config, Language};
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let header_path = Path::new(&crate_dir).join("tokenizers_proto.h");

        if !header_path.exists() {
            cbindgen::generate_with_config(crate_dir, Config{
                    language: Language::C,
                    usize_is_size_t: true,
                    ..Config::default()
                })
                .expect("Unable to generate bindings")
                .write_to_file("tokenizers_proto.h");
            println!("Generated tokenizers_proto.h");
        } else {
            println!("Skipping cbindgen (tokenizers_proto.h already exists)");
        }
    }
    
    Ok(())
}
