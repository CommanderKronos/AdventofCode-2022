use std::{fs::File, path::Path, io::{self, BufRead}};

use itertools::Itertools;

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn main() {
    if let Ok(line_iter) = read_lines("./stack.txt") {
        
        let lines: Vec<String> = line_iter.flat_map(|x| x).collect();
        // let lines: Vec<String> = line_iter.flat_map(|Ok(x)| x.clone()).collect();
        // Stack parsing
        // There will be 8 lines, with at most 9 elements in it (LESS IS POSSIBLE)
        // 9th and 10th lines need to be ignored. Maybe use stacks to split lines 1..=8 and 11..
        let mut stack_vec: Vec<Stack<char>> = Vec::new();

        for line in lines {
            println!("{}", line);
            println!("{:?}", line.as_bytes());
            let mut temp_stack: Vec<char> = Vec::new();
            let mut great_nr: u8 = 1;
            for great in line.as_bytes().chunks(4) {
                println!("{:?}", great);
                if !(great == [32, 32, 32, 32]) {
                    println!("Crate nr: {:?}", great_nr);
                    println!("{:?}", great[1] as char);
                    temp_stack.push(great[1] as char);
                }
                great_nr += 1;
            }
        }

        // Stack manipulation
        // Instructions start at line 11

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
