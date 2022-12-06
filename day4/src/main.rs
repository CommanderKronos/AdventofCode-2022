use std::{io::{self, BufRead}, fs::File, path::Path};



fn main() {
    
    if let Ok(input) = read_lines("./input") {
        let mut score: u16 = 0;
        for line in input {
            if let Ok(line) = line {

                let line_vec: Vec<&str> = line.split(",").collect();
                let elf1_bounds: Vec<u16> = line_vec[0].split("-").map(|x| x.parse().unwrap()).collect();
                let elf2_bounds: Vec<u16> = line_vec[1].split("-").map(|x| x.parse().unwrap()).collect();

                let elf1_range: Vec<u16> = (elf1_bounds[0]..=elf1_bounds[1]).collect();
                let elf2_range: Vec<u16> = (elf2_bounds[0]..=elf2_bounds[1]).collect();
                
                let mut overlap: bool = false;
                if elf1_range.len() >= elf2_range.len() {
                    for section in elf2_range {
                        if elf1_range.contains(&section) {
                            overlap = true;
                            break;
                        }
                    }
                } else {
                    for section in elf1_range {
                        if elf2_range.contains(&section) {
                            overlap = true;
                            break;
                        }
                    }
                }
                
                if overlap {
                    score += 1;
                }

                // PART 1
                // let mut complete_coverage: bool = true;
                // if elf1_range.len() >= elf2_range.len() {
                //     for section in elf2_range {
                //         if !elf1_range.contains(&section) {
                //             complete_coverage = false;
                //             break;
                //         }
                //     }
                // } else {
                //     for section in elf1_range {
                //         if !elf2_range.contains(&section) {
                //             complete_coverage = false;
                //             break;
                //         }
                //     }
                // }

                // if complete_coverage {
                //     score += 1;
                // }

            }
        }

        println!("Score: {}", score)

    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
