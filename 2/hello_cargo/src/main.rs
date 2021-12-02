use std::io::Read;

fn main() {
    let mut fileRef = std::fs::File::open("input.txt").unwrap();
    let mut data = String::new();

    fileRef.read_to_string(&mut data).unwrap();
    pt2(data);
}

fn pt1(data : String){
    let mut forward = 0;
    let mut depth = 0;
    let mut split = data.split("\n");
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
    println!("{}", forward * depth)
}

fn pt2(data : String){
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut split = data.split("\n");
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
    println!("{}", forward * depth)
}