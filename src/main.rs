mod io;

use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    match try_main(&args) {
        Ok(output_path) => println!("Successfully converted file to {output_path:?}"),
        Err(err) => eprintln!("Error! Something went wrong:\n{err}"),
    };
}

fn try_main(args: &[String]) -> Result<PathBuf, Box<dyn Error>> {
    let path = args.get(1).ok_or("No Source file path specified.")?;
    let path = Path::new(path);

    io::validate_file(path)?;

    let file_contents = io::read_file(path)?;

    let output_path = path.with_extension("html");

    io::write_file(output_path.as_path(), file_contents)?;

    Ok(output_path)
}
