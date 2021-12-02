use std::io::Read;
use std::env;

pub fn run() {
    println!("{:?}",env::current_dir().unwrap());
    let mut file_ref = std::fs::File::open("./src/day2/input.txt").unwrap();
    let mut data = String::new();

    file_ref.read_to_string(&mut data).unwrap();
    pt1(&data);
    println!();
    pt2(&data);
}

fn pt1(data : &String){
    let mut forward = 0;
    let mut depth = 0;
    let split = data.split("\n");
    for i in split {
        if 1 > i.len() {
            break;
        }
        match &i[..1] {
            "f" => {
                forward += &i[8..].parse::<i32>().unwrap();
            }
            "u" => {
                depth -= &i[3..].parse::<i32>().unwrap();
            }
            "d" => {
                depth += &i[5..].parse::<i32>().unwrap();
            }
            _ => {
                println!("this word starts with {}!", &i[..1]);
            }
        }
    }
    println!("Forward = {}, depth = {}", forward, depth);
    println!("forward * depth = {}", forward * depth);
}

fn pt2(data : &String){
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    let split = data.split("\n");
    for i in split {
        if 1 > i.len() {
            break;
        }
        match &i[..1] {
            "f" => {
                forward += &i[8..].parse::<i32>().unwrap();
                depth += &i[8..].parse::<i32>().unwrap() * aim;
            }
            "u" => {
                aim -= &i[3..].parse::<i32>().unwrap();
            }
            "d" => {
                aim += &i[5..].parse::<i32>().unwrap();
            }
            _ => {
                println!("this word starts with {}!", &i[..1]);
            }
        }
    }
    println!("forward = {}, depth = {}, aim = {}", forward, depth, aim);
    println!("forward * depth = {}", forward * depth);
}