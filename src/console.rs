use std::io::{Write, stdout, stdin};


/// Reads a line of input from the stdin stream and returns it
/// The 'preceding' string is what will be printed *immediately* before reading a line
pub fn read_line(preceding: Option<&str>) -> std::io::Result<String> {
    if let Some(preceding) = preceding {
        stdout().write(
            format!("{}", preceding).as_bytes()
        )?;
        stdout().flush()?;
    }

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let input = buffer.trim();
    Ok(input.to_string())
}