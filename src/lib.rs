use std::io::{Write, stdout, stdin};


pub fn read_string(preceding: Option<&str>) -> std::io::Result<String> {
    if let Some(preceding) = preceding {
        stdout().write(
            format!("{}: ", preceding).as_bytes()
        )?;
        stdout().flush()?;
    }

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let input = buffer.trim();
    Ok(input.to_string())
}
