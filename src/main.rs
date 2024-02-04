use anyhow::{anyhow, Result};
use serde_json::Value;
use std::env;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(anyhow!("Usage:\narchon <http-URL>\n"));
    }
    let url = &args[1];

    let resp = reqwest::blocking::get(url)?;
    println!("Status code: {:?};\n", &resp.status());

    let resp_text = &resp.text()?;
    println!("===== BODY ==== \n{:#?}\n==== BODY END ===\n", &resp_text);
    println!("Response length is: {};\n", &resp_text.len());

    let resp_struct: Value = serde_json::from_str(resp_text)?;

    println!("=== STRUCT! ===\n{:?}\n==== END STRUCT ===\n", &resp_struct);
    println!(
        "Trying to print headers/Host field: [{}]",
        &resp_struct["headers"]["Host"]
    );
    Ok(())
}
