use std::{fs, io, path::Path};

pub fn read_file(path: &Path) -> io::Result<String> {
    let source_contents = fs::read_to_string(path)?;

    Ok(source_contents)
}

pub fn write_file(path: &Path, contents: String) -> io::Result<()> {
    if path.is_file() {
        println!("{path:?} already exists. Overwrite? (y/N)");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input != "y" || input != "Y" {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Denied permission to overwrite.",
            ));
        }
    }

    fs::write(path, contents)?;

    Ok(())
}

pub fn validate_file(path: &Path) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!("Specified path {path:?} is not a file."));
    }

    let ext = path
        .extension()
        .ok_or(format!("Specified file {path:?} must be a .md file."))?;

    if ext != "md" {
        return Err(format!("Specified file {path:?} must be a .md file."));
    }

    Ok(())
}
