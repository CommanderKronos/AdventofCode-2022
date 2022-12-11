use std::{fs::File, path::Path, io::{self, BufRead}};

#[derive(Debug)]
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

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn main() {
    if let Ok(content) = read_lines("./input.txt") {
        
        let lines: Vec<String> = content.flat_map(|x| x).collect();
        // let lines: Vec<String> = line_iter.flat_map(|Ok(x)| x.clone()).collect();
        // Stack parsing
        // There will be 8 lines, with at most 9 elements in it (LESS IS POSSIBLE)
        // 9th and 10th lines need to be ignored. Maybe use stacks to split lines 1..=8 and 11..
        let mut temp_stack_vec: Vec<Vec<char>> = Vec::new();
        for _i in 0..9 {
            temp_stack_vec.push(Vec::new());
        }

        let mut lines_iter = lines.iter();
        for line in lines_iter.by_ref().take(8) {
            let mut stack_nr: usize = 0; // 0-8 inclusive
            for great in line.as_bytes().chunks(4) {
                if !(great == [32, 32, 32, 32]) {
                    if !(great[1] as char == ' ') {
                        temp_stack_vec[stack_nr].push(great[1] as char);
                    }
                }
                stack_nr += 1;
            }
        }

        let mut stack_vec: Vec<Stack<char>> = Vec::new();
        for stack in temp_stack_vec {
            let mut reverse_stack: Stack<char> = Stack::new();
            for char in stack.iter().rev() {
                reverse_stack.push(*char);
            }
            println!("{:?}", reverse_stack);
            stack_vec.push(reverse_stack);
        }
        // stack_vec now contains all stacks correctly
        
        // Stack manipulation
        // Instructions start at line 11
        lines_iter.by_ref().next();
        lines_iter.by_ref().next();
        for line in lines_iter.by_ref() {

            let line_split: Vec<&str> = line.split(" ").collect();
            let mut amount: u16 = line_split[1].parse::<u16>().unwrap();
            let from: usize = line_split[3].parse::<usize>().unwrap() - 1;
            let to: usize = line_split[5].parse::<usize>().unwrap() - 1;

            // PART 1
            // while amount != 0 {
            //     let moving_crate = stack_vec[from].pop().unwrap();
            //     stack_vec[to].push(moving_crate);
            //     amount -= 1;
            // }

            // PART 2
            let mut temp_stack: Vec<char> = Vec::new();
            while amount != 0 {
                let moving_crate = stack_vec[from].pop().unwrap();
                temp_stack.push(moving_crate);

                amount -= 1;
            }

            for char in temp_stack.iter().rev() {
                stack_vec[to].push(*char);
            }
        }
        
        for stack in stack_vec {
            println!("Crate on top: {:?}", stack.peek());
        }

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
