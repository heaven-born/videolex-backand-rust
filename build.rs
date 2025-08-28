use std::env;
use std::io::Result;
use std::path::PathBuf;
use tonic_prost_build::configure;

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    configure()
        .build_server(true)
        //.service_generator(Box::new(WebGenerator::new()))
        .file_descriptor_set_path(out_dir.join("service_descriptor.bin"))
        .compile_protos(&["src/protos/model.proto"], &["src/protos/"])?;
    Ok(())
}

