use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/mr_cache.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(false)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("mr_cache.bin"))
        .out_dir("./src/api")
        .compile(&[proto_file], &["./proto"])
        .expect("Building proto failed");

    Ok(())
}
