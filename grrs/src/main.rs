use std::fmt::{self, Display};
use clap::Parser;

fn main() {
    println!("Hello, world!");


    let options2 = GrrsOptions::parse();
    println!("{}", options2);


    let pattern = std::env::args().nth(1).expect("expecting search pattern here");
    let path = std::env::args().nth(2).expect("expecting file path here");

    println!("{}", pattern);
    println!("{}", path);

    let options = GrrsOptions{
        pattern,
        path: std::path::PathBuf::from(path)
    };
    println!("{}", options);


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