use protoc_rust::Customize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("protos");
    let proto_path = Path::new("./protos");
    fs::create_dir_all(&dest_path).unwrap();

    // Run protoc
    protoc_rust::Codegen::new()
        .out_dir(&dest_path.to_str().unwrap())
        .inputs(&[proto_path.join("sparkplug_b.proto").to_str().unwrap()])
        .includes(&[proto_path.to_str().unwrap()])
        .customize(Customize::default())
        .run()
        .expect("Protoc Error");

    // Create mod.rs accordingly
    let mut mod_file = File::create(dest_path.join("mod.rs")).unwrap();
    writeln!(mod_file, "pub mod sparkplug_b;").unwrap();
}
