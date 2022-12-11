use std::{io::{self, BufRead}, fs::File, path::Path};

// create directory and filee structs. these
struct Directory {
    name: String,
    files: Vec<Filee>,
    sub_dirs: Vec<Directory>,
}
impl Directory {
    fn new() -> self {
        Directory {}
    }
}

struct Filee {
    name: String,
    size: usize,
}

fn main() {
    if let Ok(content) = read_lines("./input") {
        let lines: Vec<String> = content.flat_map(|x| x).collect();

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
