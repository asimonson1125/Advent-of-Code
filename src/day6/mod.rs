use std::io::Read;

pub fn run() {
    let mut file_ref = std::fs::File::open("./src/day6/input.txt").unwrap();
    let mut data = String::new();

    file_ref.read_to_string(&mut data).unwrap();
    let initials = data.split(",");
    let mut fish_vec: Vec<i32> = vec![];
    for i in initials{
        fish_vec.push(i.parse().unwrap());
    }
    pt1(&fish_vec);
    println!();
    pt2(&fish_vec);
}

fn pt1(data: &Vec<i32>){
    let mut all_fish = data.clone();
    for day in 0..80{
        let current_count: usize = all_fish.len();
        for fish in 0..current_count{
            if all_fish[fish] == 0{
                all_fish.push(8);
                all_fish[fish] = 6;
            }
            else{
                all_fish[fish] -= 1;
            }
        }
    }
    println!("Part 1: {}", all_fish.len());
}

fn pt2(data: &Vec<i32>){
    let mut all_fish = data.clone();
    for day in 0..256{
        println!("{}", day);
        let current_count: usize = all_fish.len();
        for fish in 0..current_count{
            if all_fish[fish] == 0{
                all_fish.push(8);
                all_fish[fish] = 6;
            }
            else{
                all_fish[fish] -= 1;
            }
        }
    }
    println!("Part 2: {}", all_fish.len());
}