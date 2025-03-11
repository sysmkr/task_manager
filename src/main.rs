use serde_json::{json, to_writer};
use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    // Create the JSON data
    let data = json!({
        "message": "Hello"
    });

    // Open or create the file `data.json` for writing
    let file = File::create("src/data.json")?;

    // Write the JSON data to the file
    to_writer(file, &data)?;

    println!("Message written to data.json");

    Ok(())
}
