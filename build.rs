use anyhow::Result;
use glob::glob;
use std::path::PathBuf;
use protobuf_codegen::Codegen;

fn main() -> Result<()> {
    let pattern = "schemas/schemas/proto/foxglove/*.proto";

    let protos: Vec<PathBuf> = glob(pattern)?
        .filter_map(|e| match e {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect();

    Codegen::new()
        .pure()
        .cargo_out_dir("generated_protos")
        .include("schemas/schemas/proto/")
        .inputs(protos)
        .run_from_script();


    Ok(())
}
