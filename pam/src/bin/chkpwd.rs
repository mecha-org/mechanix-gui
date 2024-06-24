use std::process::exit;

use clap::Parser;
use pam_mechanix::{exitcode, passwords};
use sha256::digest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    password: String,
}

fn main() {
    let args = Args::parse();
    let Args { username, password } = args;

    if username.is_empty() {
        exit(exitcode::DATAERR)
    }

    let entry = passwords::get_entry_by_name(&username);

    if entry.is_none() {
        exit(exitcode::NOUSER)
    }

    let password_entry = entry.unwrap();
    if password_entry.password.is_empty() {
        exit(exitcode::NOUSER)
    }

    if password.is_empty() {
        exit(exitcode::DATAERR)
    }

    let encoded_password = digest(password);

    let pass_match = encoded_password.to_uppercase() == password_entry.password.to_uppercase();

    if !pass_match {
        exit(exitcode::DATAERR);
    }

    exit(exitcode::OK)
}
