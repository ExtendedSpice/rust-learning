use std::fmt::{self, Display};
use clap::Parser;

fn main() {
    println!("Hello, world!");
    std::env::args().for_each(|x| print!("{} ", x));
    println!();

    let pattern = std::env::args().nth(1).expect("expecting search pattern here");
    let path = std::env::args().nth(2).expect("expecting file path here");

    println!("{}", pattern);
    println!("{}", path);

    let _options = GrrsOptions{
        pattern,
        path: std::path::PathBuf::from(path)
    };
    //println!("{}", options);

    let options = GrrsOptions::parse();
    println!("{}", options);


    let content = std::fs::read_to_string(&options.path).expect("could not read file");
    for line in content.lines() {
        if line.contains(&options.pattern) {
            println!("{}", line);
        }
    }


}

#[derive(Parser)]
struct GrrsOptions   {
    pattern: String,
    path: std::path::PathBuf,
}

impl Display for GrrsOptions{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.pattern, self.path.display())
    }
}