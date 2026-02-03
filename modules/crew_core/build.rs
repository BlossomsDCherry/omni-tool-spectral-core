fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &["../sovereign_logic/A2A-main/specification/grpc/a2a.proto"],
            &["../sovereign_logic/A2A-main/specification/grpc"],
        )?;
    Ok(())
}
