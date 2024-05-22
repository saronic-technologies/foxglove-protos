use anyhow::Result;
use glob::glob;
use std::path::PathBuf;

fn main() -> Result<()> {
    let pattern = "schemas/schemas/proto/foxglove/*.proto";

    let proto_files: Vec<PathBuf> = glob(pattern)?
        .filter_map(|e| match e {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect();

    println!("{:#?}", proto_files);

    prost_build::compile_protos(&proto_files, &["schemas/schemas/proto/"])?;

    Ok(())
}
