use std::{io::{self, BufRead}, fs::File, path::Path, collections::HashMap};

fn main() {
    // PART 1
    // let winning_combos: HashMap<&str, &str> = HashMap::from([
    //     ("X", "C"),
    //     ("Y", "A"),
    //     ("Z", "B"),
    // ]);

    // let converted_choices: HashMap<&str, &str> = HashMap::from([
    //     ("X", "A"),
    //     ("Y", "B"),
    //     ("Z", "C"),
    // ]);

    let winning_combos: HashMap<&str, &str> = HashMap::from([
        ("A", "B"),
        ("B", "C"),
        ("C", "A"),
    ]);

    let losing_combos: HashMap<&str, &str> = HashMap::from([
        ("A", "C"),
        ("B", "A"),
        ("C", "B"),
    ]);

    if let Ok(lines) = read_lines("./input") {
        let mut total_score: u32 = 0;

        for line in lines {
            if let Ok(content) = line {
                let content_vec: Vec<&str> = content.split(" ").collect();
                let column1: &str = content_vec[0];
                let column2: &str = content_vec[1];

                // PART 1
                // let result_value: u32 =
                //     if column1 == winning_combos.get(column2).unwrap().clone() {
                //         6
                //     } else if column1 == converted_choices.get(column2).unwrap().clone() {
                //         3
                //     } else {
                //         0
                //     };

                // let  choice_value: u32 = 
                // match column2 {
                //     "X" => 1,
                //     "Y" => 2,
                //     "Z" => 3,
                //     _ => 0,
                // };

                // PART 2
                let your_choice: &str =
                        if column2 == "X" {
                            losing_combos.get(column1).unwrap().clone()
                        } else if column2 == "Z" {
                            winning_combos.get(column1).unwrap().clone()
                        } else {
                            column1
                        };
                        
                let  choice_value: u32 = 
                        match your_choice {
                            "A" => 1,
                            "B" => 2,
                            "C" => 3,
                            _ => 0,
                        };
                        
                let result_value: u32 =
                        match column2 {
                            "X" => 0,
                            "Y" => 3,
                            "Z" => 6,
                            _ => 0,
                        };


                let round_result: u32 = result_value + choice_value;
                total_score += round_result
            }
        }
        println!("Total score: {}", total_score)
    }
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

