use std::io::Read;

pub fn run() {
    let mut file_ref = std::fs::File::open("./src/day9/input.txt").unwrap();
    let mut data = String::new();

    file_ref.read_to_string(&mut data).unwrap();
    let rows = data.replace("\n", "");
    let mut grid: [[i32; 100];] = [[0; 100]; 100];
    for row in 0..100{
        for num in 0..100{
            string.cha 
        }
    }
}