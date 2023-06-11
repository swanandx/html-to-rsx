use clap::Parser;
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

#[derive(Debug, Parser)]
struct Opt {
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::parse();

    let mut content = String::new();

    if let Some(path) = opt.input {
        let mut file = File::open(path).expect("file should open successfully");
        file.read_to_string(&mut content)
            .expect("file should be read to string");
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle
            .read_to_string(&mut content)
            .expect("file should be read to string");
    };

    let rsx = html_to_rsx::parse(&content).expect("should convert html to rsx");

    println!("{rsx}");
}
