use std::{fs::File, io::{self, BufRead}, path::Path};
use itertools::{self, Itertools};

fn main() {

    let mut _total_prio: u32 = 0;
    if let Ok(lines) = read_lines("./input") {

        // PART 1
        for (line1, line2, line3) in lines.tuples() {
            let line1 = match line1 {
                Ok(line1) => line1,
                Err(_e) => return (),
            };
            let line2 = match line2 {
                Ok(line2) => line2,
                Err(_e) => return (),
            };
            let line3 = match line3 {
                Ok(line3) => line3,
                Err(_e) => return (),
            };
            let rucksack1: &[u8] = line1.as_bytes();
            let rucksack2: &[u8] = line2.as_bytes();
            let rucksack3: &[u8] = line3.as_bytes();

            for item in rucksack1.iter() {
                if rucksack2.contains(item) && rucksack3.contains(item) {
                    let conv_overlap =
                        match item {
                            97..=122 => item - 96,
                            65..=90 => item - 38,
                            _ => 0,
                        };
                    _total_prio += conv_overlap as u32;
                    break;
                }
            }

        }

        // PART 1
        // for line in lines {
        //     if let Ok(rucksack) = line {

        //     let rucksacklen = rucksack.len();
        //     let compartone = &rucksack[..rucksacklen/2].as_bytes();
        //     let comparttwo = &rucksack[rucksacklen/2..].as_bytes();
            
        //     for item in compartone.iter() {

        //         if comparttwo.contains(item) {
        //             let conv_overlap =
        //                 match item {
        //                     97..=122 => item - 96,
        //                     65..=90 => item - 38,
        //                     _ => 0,
        //                 };
        //             total_prio += conv_overlap as u32;
        //             break;
        //             }
        //         }
        //     }
        // }
    }
    println!("Total Priority: {}", _total_prio)

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
