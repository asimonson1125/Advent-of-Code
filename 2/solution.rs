use std::io::Read;
use try_catch::catch;

fn main() {
    let mut fileRef = std::fs::File::open("input.txt").unwrap();
    let mut data = String::new();

    fileRef.read_to_string(&mut data).unwrap();
    pt1(data);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn pt1(data : String){
    let mut forward = 0;
    let mut depth = 0;
    let mut split = data.split("\n");
    catch! {
        try{
            for i in split {
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
        }
        catch err{
            println!("Error on {}", i);
        }
    }
    println!("{}", forward)
}