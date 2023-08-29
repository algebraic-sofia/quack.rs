use std::env;

use quake::generate_report;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("Insufficient arguments")
    } else {
        let file = std::fs::read_to_string(&args[1]).map_err(|_| "Cannot read file")?;
        let report = generate_report(&file);
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        Ok(())
    }
}
