fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .out_dir("src/network/messages")
        .compile_protos(&["src/network/messages/message.proto"], &["src/network/messages"])?;
    Ok(())
}