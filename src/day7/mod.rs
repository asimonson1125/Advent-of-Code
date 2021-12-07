use std::io::Read;
use std::collections::HashMap;

pub fn run() {
    let mut file_ref = std::fs::File::open("./src/day7/input.txt").unwrap();
    let mut data = String::new();

    file_ref.read_to_string(&mut data).unwrap();
    let initials = data.split(",");
    let mut sub_vec: Vec<i32> = vec![];
    for i in initials{
        sub_vec.push(i.parse().unwrap());
    }
    pt1(&sub_vec);
    println!();
    pt2(&sub_vec);
}

fn pt1(data: &Vec<i32>){
    let mut i;
    let mut min = 0;
    let mut max = 2000;
    let mut current;
    loop {
        i = (max - min)/2 + min;
        current = fuel(i, &data);
        if current > fuel(i-1, &data){
            max = i;
        }
        else if current > fuel(i+1, &data){
            min = i;
        }
        else {break;}
    }
    println!("Part 1: row {} with {} fuel", i, current);
}

fn pt2(data: &Vec<i32>){
    let mut min = std::i32::MAX;
    const MAX: i32 = 2000;
    let mut row = 0;
    let mut known = HashMap::new();
    known.insert(0,0);
    for val in 0..MAX{
        let subs = data.clone();
        let mut current = 0;
        for i in subs{
            let mut run = false;
            let distance = (i-val).abs();
            match known.get(&distance) {
                Some(fuel) => current += fuel,
                None => run = true
            }
            if run {
                let mut fuel = 0;
                for x in 1..distance+1{
                    fuel += x;
                }
                current += fuel;
                known.insert(distance, fuel);
            }
        }
        if current < min {
            min = current;
            row = val;
        }
    }
    println!("Part 2: row {} with {} fuel", row, min);
}

fn fuel(val: i32, data: &Vec<i32>) -> i32{
    let subs = data.clone();
    let mut fuel = 0;
    for i in subs{
        fuel += (i - val).abs();
    }
    return fuel;
}