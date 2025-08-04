use mxconf::cli::{get_setting, list_schemas, set_setting};
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  {} get <key>", args[0]);
        println!("  {} set <key> <value>", args[0]);
        println!("  {} list", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "get" => {
            if args.len() < 3 {
                println!("Error: 'get' requires a key");
                return Ok(());
            }
            let key = &args[2];
            let value = get_setting(key).await?;
            println!("{:?}", value);
        }
        "set" => {
            if args.len() < 4 {
                println!("Error: 'set' requires a key and value");
                return Ok(());
            }
            let key = &args[2];
            let value = &args[3];
            let result = set_setting(key, value).await?;
            println!("{}", result);
        }
        "list" => {
            list_schemas().await?;
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Valid commands: get, set, list");
        }
    }

    Ok(())
}