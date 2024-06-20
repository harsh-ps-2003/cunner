fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .type_attribute("message.Transaction", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("src/network/messages")
        .compile_protos(
            &["src/network/messages/message.proto"],
            &["src/network/messages"],
        )?;
    Ok(())
}
