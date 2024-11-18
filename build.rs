fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/file_service.proto")?;
    tonic_build::compile_protos("proto/hdfs_service.proto")?;
    Ok(())
}
