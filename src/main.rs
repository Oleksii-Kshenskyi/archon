use anyhow::{anyhow, Result};
use rayon::prelude::*;
use serde_json::Value;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(anyhow!("Usage:\n\tarchon <URL-list-file>\n"));
    }
    let urls: Vec<String> = fs::read_to_string(&args[1])?
        .lines()
        .map(str::trim)
        .filter(|&s| !s.is_empty())
        .map(String::from)
        .collect();
    println!("URLs list is: {:#?}", &urls);

    urls.iter().for_each(|url| {
        println!(">>>>>>> URL '{}' START <<<<<<<<<\n", &url);
        let resp = reqwest::blocking::get(url)?;
        println!("Status code: {:?};\n", &resp.status());

        let resp_text = &resp.text()?;
        println!("===== BODY ==== \n{:#?}\n==== BODY END ===\n", &resp_text);
        println!("Response length is: {};\n", resp_text.len());

        if resp_text.len() > 0 {
            let resp_struct: Value = serde_json::from_str(resp_text)?;

            println!(
                "=== STRUCT! ===\n{:#?}\n==== END STRUCT ===\n",
                &resp_struct
            );
        }

        println!(">>>>>>> URL '{}' END <<<<<<<<<\n", &url);
    });

    Ok(())
}
