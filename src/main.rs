use std::{env, error::Error, ffi::OsStr, fs, io, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(path) = args.get(1) else {
        panic!("No source file path specified.")
    };

    match handle_file(path) {
        Ok(()) => println!("Successfully converted file to HTML."),
        Err(err) => eprintln!("Error! Could not convert '{path}':\n{err}"),
    }
}

fn handle_file(path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);

    let (source_parent, source_name) = get_file_parent_and_name(path)?;
    let source_contents = fs::read_to_string(path)?;

    let dest_name = format!("{}.html", source_name.to_str().unwrap_or("output.txt"));

    fs::write(source_parent.join(dest_name), source_contents)?;

    Ok(())
}

fn get_file_parent_and_name(path: &Path) -> io::Result<(&Path, &OsStr)> {
    let Some(ext) = path.extension() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Specified file does not have an extension",
        ));
    };

    if ext != "md" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Specified file must be a .md file.",
        ));
    }

    let Some(file_parent) = path.parent() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid path to file.",
        ));
    };

    let Some(file_name) = path.file_stem() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No valid file name found.",
        ));
    };

    Ok((file_parent, file_name))
}
