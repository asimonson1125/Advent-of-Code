use std::io::Read;

pub fn run() {
    let mut file_ref = std::fs::File::open("./src/day8/input.txt").unwrap();
    let mut data = String::new();

    file_ref.read_to_string(&mut data).unwrap();
    let rows = data.split("\n");
    let mut row_vec: Vec<[&str; 14]> = vec![];
    for row in rows{
        let digits = row.split(" ");
        let mut rowarr: [&str; 14] = [""; 14];
        let mut index = 0;
        for i in digits{
            if index < 10 {
                rowarr[index] = i;
            }
            else if index > 10{
                rowarr[index-1] = i;
            }
            index += 1;
        }
        row_vec.push(rowarr);
    }
    pt1(&row_vec);
    println!();
    pt2(&row_vec);
}

fn pt1(data: &Vec<[&str; 14]>){
    let rows = data.clone();
    let mut ones = 0;
    let mut fours = 0;
    let mut sevens = 0;
    let mut eights = 0;
    for row in rows{
        let mut pattern: [&str; 10] = [""; 10];
        let mut unknown: Vec<&str> = vec![];
        for digit in 0..10{
            match row[digit].len() {
                2 => pattern[1] = row[digit],
                3 => pattern[7] = row[digit],
                4 => pattern[4] = row[digit],
                7 => pattern[8] = row[digit],
                _ => unknown.push(row[digit])
            }
        }
        for digit in 10..14{
            let option = getOption(&pattern, row[digit]);
            match option {
                1 => ones += 1,
                4 => fours += 1,
                7 => sevens += 1,
                8 => eights += 1,
                _ => ()
            }
            
        }
    }
    let total = ones + sevens + fours + eights;
    println!("Part 1: {} ones, {} fours, {} sevens, {} eights for a total of {}", ones, fours, sevens, eights, total);
}


fn pt2(data: &Vec<[&str; 14]>){
    let rows = data.clone();
    let mut net = 0;
    let mut totals: Vec<i32> = vec![];
    for row in rows{
        let mut sums = String::from("");
        let mut pattern: [&str; 10] = [""; 10];
        let mut unknown: Vec<&str> = vec![];
        for digit in 0..10{
            match row[digit].len() {
                2 => pattern[1] = row[digit],
                3 => pattern[7] = row[digit],
                4 => pattern[4] = row[digit],
                7 => pattern[8] = row[digit],
                _ => unknown.push(row[digit])
            }
        }
        unknown.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
        for digit in 0..3{
            if contains(pattern[4], unknown[digit]) {
                pattern[9] = unknown[digit];
                unknown.remove(digit);
                break;
            }
        }
        for digit in 0..2{
            if contains(pattern[1], unknown[digit]) {
                pattern[0] = unknown[digit];
                unknown.remove(digit);
                break;
            }
        }
        pattern[6] = unknown[0];
        unknown.remove(0);
        for digit in 0..unknown.len(){
            if contains(pattern[1], unknown[digit]){
                pattern[3] = unknown[digit];
            }
            else if contains(unknown[digit], pattern[6]){
                pattern[5] = unknown[digit];
            }
            else{
                pattern[2] = unknown[digit];
            }
        }
        for digit in 10..14{
            let option = getOption(&pattern, row[digit]);
            if option != -1 {
                sums += &option.to_string();
            }
            
        }
        totals.push(sums.parse().unwrap());
    }
    for i in totals {
        net += i;
    }
    println!("Part 2: total of {}", net);
}

fn getOption(pattern: &[&str; 10], digit: &str) -> i32{
    for index in 0..10{
        let mut correct = true;
        if pattern[index].len() == digit.len() {
            for character in pattern[index].chars() {
                match digit.chars().position(|c| c == character) {
                    Some(id) => (),
                    None => correct = false,
                }
            }
            if correct {
                return index as i32;
            }
        }
    }
    return -1;
}

fn contains(pattern: &str, digit: &str) -> bool {
    for character in pattern.chars() {
        match digit.chars().position(|c| c == character) {
            Some(id) => (),
            None => return false,
        }
    }
    return true;
}