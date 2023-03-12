use anyhow::{Context, Result};
use clap::Parser;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    std::env::args().for_each(|x| print!("{} ", x));
    println!();

    let pattern = std::env::args()
        .nth(1)
        .expect("expecting search pattern here");
    let path = std::env::args().nth(2).expect("expecting file path here");

    println!("{}", pattern);
    println!("{}", path);

    let _options = GrrsOptions {
        pattern,
        path: std::path::PathBuf::from(path),
    };
    //println!("{}", options);

    let options = GrrsOptions::parse();
    println!("{}", options);

    let _file4 = File::open(&options.path)
        .with_context(|| format!("cant read file '{}'", &options.path.display()))?;

    let _file3 = File::open(&options.path).map_err(|error| {
        FileExceptionError(format!(
            "File exception `{}`: {}",
            &options.path.display(),
            error
        ))
    })?;

    let _file2 = File::open(&options.path).unwrap();

    let file_result = File::open(&options.path);
    let file = match file_result {
        Ok(stream) => stream,
        Err(error) => {
            return Err(error.into());
        }
    };

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line).expect("unable to read line") > 0 {
        if line.contains(&options.pattern) {
            println!("{}", line.trim_end());
        }
        line.clear();
    }

    Ok(())
}

#[derive(Parser)]
struct GrrsOptions {
    pattern: String,
    path: std::path::PathBuf,
}

impl Display for GrrsOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.pattern, self.path.display())
    }
}

#[derive(Debug)]
struct FileExceptionError(String);
impl Error for FileExceptionError {}

impl Display for FileExceptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
