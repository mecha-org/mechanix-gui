fn main() {
    // fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identity_proto = "src/proto/identity.proto";
    let settings_proto = "src/proto/settings.proto";

    tonic_build::configure()
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(&[identity_proto, settings_proto], &[".proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    // Ok(())
}
