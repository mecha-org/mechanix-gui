use std::process::exit;

use clap::Parser;
use pam_mechanix::{
    exitcode,
    passwords::{get_entry_by_name, update_or_create_entry, PassswordEntry},
};
use sha256::digest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    old: String,
    #[arg(short, long)]
    new: String,
}

fn main() {
    let args = Args::parse();
    let Args { username, old, new } = args;

    if username.is_empty() {
        exit(exitcode::DATAERR)
    }

    if old.is_empty() {
        exit(exitcode::DATAERR)
    }

    let encoded_old_password = digest(old);

    let entry = get_entry_by_name(&username);

    if entry.is_none() {
        //create entry
        let password_entry = PassswordEntry::new(&username, &encoded_old_password);
        let updated_r = update_or_create_entry(&username, password_entry);
        if let Err(e) = &updated_r {
            println!("Error updating entry: {}", e);
            exit(exitcode::DATAERR)
        }
        exit(exitcode::OK)
    }

    let mut password_entry = entry.unwrap();

    if password_entry.password.is_empty() {
        //create password and update entry
        password_entry.password = encoded_old_password;
        //update other fields here if needed
        let updated_r = update_or_create_entry(&username, password_entry);
        if let Err(e) = &updated_r {
            println!("Error updating entry: {}", e);
            exit(exitcode::DATAERR)
        }
        exit(exitcode::OK)
    }

    let pass_match = encoded_old_password.to_uppercase() == password_entry.password.to_uppercase();

    if !pass_match {
        exit(exitcode::DATAERR);
    }

    if new.is_empty() {
        exit(exitcode::DATAERR)
    }

    let encoded_new_password = digest(new);

    //create entry with new password
    password_entry.password = encoded_new_password;
    //update other fields here if needed
    let updated_r = update_or_create_entry(&username, password_entry);
    if let Err(e) = &updated_r {
        println!("Error updating entry: {}", e);
        exit(exitcode::DATAERR)
    }

    exit(exitcode::OK)
}
