use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    pub interfaces: Interfaces,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Interfaces {
    pub network: Network,
    pub display: Display,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Display {
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Network {
    pub device: String,
}
