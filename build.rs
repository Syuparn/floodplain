fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate gRPC codes for tonic
    tonic_build::compile_protos("proto/helloworld/service.proto")?;
    Ok(())
}
