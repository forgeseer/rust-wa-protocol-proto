fn main() -> Result<(), std::io::Error> {
    // Tell Cargo to re-run this build script if `WAProto.proto` changes.
    println!("cargo:rerun-if-changed=protos/WAProto.proto");

    // Compile the `WAProto.proto` file.
    // The `compile_protos` function will look for the file relative to the project root.
    // The second argument is the include path for any imports in your .proto files.
    prost_build::compile_protos(&["protos/WAProto.proto"], &["protos/"])?;

    Ok(())
}
