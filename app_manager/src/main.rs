use std::process::Command;
mod dbus;

fn main() {
    println!("Hello, world!");
    let apps_base_path = "/Users/akshayraina/Mecha/mecha-launcher/target/release";
    let status_bar_app_path = apps_base_path.to_owned() + "/mecha_status_bar";

    let mut status_bar = Command::new(status_bar_app_path)
        .spawn()
        .expect("Failed to start status bar");
}
