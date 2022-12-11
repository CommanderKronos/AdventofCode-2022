use std::{fs::File, io::{self, BufRead}, path::Path, collections::HashSet};
use std::collections::VecDeque;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let line: String = lines.flat_map(|x| x).collect();
        let input: &[u8] = line.as_bytes();

        let mut last4: VecDeque<u8> = VecDeque::new();
        let mut last14: VecDeque<u8> = VecDeque::new();
        let mut position: u32 = 0;
        let mut packet_start_pos: u32 = 0;
        let mut message_start_pos: u32 = 0;
        let mut packet_start: bool = false;
        let mut message_start: bool = false;
        for char_byte in input {
            position += 1;
            if last4.len() == 4 {
                last4.pop_front();
                last4.push_back(*char_byte);
                if !packet_start {
                    let duplicates_last4: bool = contains_duplicates(last4.clone());
                    
                    if !duplicates_last4 {
                        packet_start_pos = position;
                        packet_start = true;
                    }
                }

            } else {
                last4.push_back(*char_byte);
            }

            if last14.len() == 14 {
                last14.pop_front();
                last14.push_back(*char_byte);

                if !message_start {
                    let duplicates_last14: bool = contains_duplicates(last14.clone());

                    if !duplicates_last14 {
                        message_start_pos = position;
                        message_start = true;
                    }
                }

            } else {
                last14.push_back(*char_byte)
            }

            if packet_start && message_start {
                println!("Packet start marker at pos: {:?}, Message start marker at pos: {:?}", packet_start_pos, message_start_pos);
                break;
            }

        }

    }
}

fn contains_duplicates(vec: VecDeque<u8>) -> bool {
    let mut seen = HashSet::new();

    vec
        .iter()
        .any(|&c| !seen.insert(c))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
