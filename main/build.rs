use std::io::Result;

fn main() -> Result<()> {
    //let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    //out_dir.join("service_descriptor.bin")
    prost_build::Config::new()
        //.out_dir("src/generated") // Optional: Output generated code to src/generated
        .type_attribute(".transport", "#[derive(serde::Serialize, serde::Deserialize)]") // Add serde derives
        .type_attribute(".transport", "#[derive(utoipa::ToSchema)]")
        .compile_protos(&["src/protos/model.proto"], &["src/protos/"])?;


    Ok(())
}

