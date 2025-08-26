use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["protos/lib.proto"], &["protos/"])?;
    #[cfg(target_os = "windows")]
    {
        let mut res = tauri_winres::WindowsResource::new();
        res.compile()?;
    }
    Ok(())
}
