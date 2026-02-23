use glob::glob;
use protobuf_codegen::Codegen;
use std::path::PathBuf;

fn main() {
    let pattern = "proto/foxglove/*.proto";

    let protos: Vec<PathBuf> = glob(pattern).unwrap()
        .filter_map(|e| match e {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect();

    Codegen::new()
        .pure()
        .cargo_out_dir("generated_protos")
        .include("proto/")
        .inputs(protos)
        .run_from_script();
}
