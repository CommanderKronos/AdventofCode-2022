use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {

    let mut total_prio: u32 = 0;
    if let Ok(lines) = read_lines("./input") {
        
        for line in lines {
            if let Ok(rucksack) = line {

                let rucksacklen = rucksack.len();
                let compartone = &rucksack[..rucksacklen/2].as_bytes();
                let comparttwo = &rucksack[rucksacklen/2..].as_bytes();
                
                for item in compartone.iter() {

                    if comparttwo.contains(item) {
                        let conv_overlap =
                            match item {
                                97..=122 => item - 96,
                                65..=90 => item - 38,
                                _ => 0,
                            };
                        total_prio += conv_overlap as u32;
                        break;
                    }
                }
            }
        }
    }
    println!("Total Priority: {}", total_prio)

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
