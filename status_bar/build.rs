fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network_manager = "./proto/network_manager.proto";
    let bluetooth_manager = "./proto/bluetooth_manager.proto";
    let battery_ctrl = "./proto/battery_ctrl.proto";

    tonic_build::configure()
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize,)]")
        .compile(
            &[network_manager, bluetooth_manager, battery_ctrl],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    Ok(())
}
