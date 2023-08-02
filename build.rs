use std::{io::Result, env, path::PathBuf};

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("far_memory_descriptor.bin"))
        .type_attribute("MemoryBlockId", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(&["proto/far_memory.proto"], &["proto"])?;
    Ok(())
}