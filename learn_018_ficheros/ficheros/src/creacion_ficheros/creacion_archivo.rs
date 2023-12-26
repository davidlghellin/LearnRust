use std::fs::File;

fn main() -> std::io::Result<()>{
    File::create("ejemplo.txt")?;
    Ok(())
}