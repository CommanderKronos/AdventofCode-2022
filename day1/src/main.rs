use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut top1: u32 = 0;
        let mut top2: u32 = 0;
        let mut top3: u32 = 0;
        let mut cur_value: u32 = 0;

        for line in lines {
            if let Ok(value) = line {


                if value.trim() == "" {

                    if cur_value > top1 {
                        top3 = top2;
                        top2 = top1;
                        top1 = cur_value;
                    } else if cur_value > top2 {
                        top3 = top2;
                        top2 = cur_value;
                    } else if cur_value > top3 {
                        top3 = cur_value
                    }
                    cur_value = 0;
                    
                } else {
                    cur_value += value.parse::<u32>().unwrap();
                }
            }
        }
        println!("Top1: {}, Top2: {}, Top3: {}", top1, top2, top3);
        println!("Top3 total: {}", top1 + top2 + top3)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
