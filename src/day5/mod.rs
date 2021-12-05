use std::io::Read;

pub fn run() {
    let mut file_ref = std::fs::File::open("./src/day5/input.txt").unwrap();
    let mut data = String::new();

    file_ref.read_to_string(&mut data).unwrap();
    let lines = data.split("\n");
    let mut lineV: Vec<[i32; 4]> = vec![];
    for i in lines{
        let line = i.clone();
        let numbers = &line.replace(" ", "");
        let mut nums = numbers.split(",");
        let x1 = nums.next().unwrap();
        let middle = nums.next().unwrap();
        let y2 = nums.next().unwrap();
        let y1 = &middle[0..middle.find("-").unwrap()];
        let x2 = &middle[middle.find(">").unwrap()+1..];
        lineV.push([x1.parse().unwrap(), y1.parse().unwrap(), x2.parse().unwrap(), y2.parse().unwrap()]);
    }
    pt1(&lineV);
    println!();
    pt2(&lineV);
}

fn pt1(data: &Vec<[i32; 4]>){
    let lines = data.clone();
    let mut grid: [[i32; 1000];1000] = [[0; 1000]; 1000];
    for i in lines{
        let mut min = i[1];
        let mut max = i[3];
        if i[0] == i[2]{
            if i[1] > i[3]{
                min = i[3];
                max = i[1];
            }
            while max >= min{
                grid[i[0] as usize][min as usize] += 1;
                min += 1;
            }
        }
        else if i[1] == i[3]{
            let mut min = i[0];
            let mut max = i[2];
            if i[0] > i[2]{
                min = i[2];
                max = i[0];
            }
            while max >= min{
                grid[min as usize][i[1] as usize] += 1;
                min += 1;
            }
        }
    }
    let mut counter = 0;
    for y in 0..1000{
        for x in 0..1000{
            if grid[y][x] > 1{
                counter += 1;
            }
        }
    }
    println!("Part 1: {}", counter);

}


fn pt2(data: &Vec<[i32; 4]>){
    let lines = data.clone();
    let mut grid: [[i32; 1000];1000] = [[0; 1000]; 1000];
    for i in lines{
        let mut min = i[1];
        let mut max = i[3];
        if i[0] == i[2]{
            if i[1] > i[3]{
                min = i[3];
                max = i[1];
            }
            while max >= min{
                grid[i[0] as usize][min as usize] += 1;
                min += 1;
            }
        }
        else if i[1] == i[3]{
            let mut min = i[0];
            let mut max = i[2];
            if i[0] > i[2]{
                min = i[2];
                max = i[0];
            }
            while max >= min{
                grid[min as usize][i[1] as usize] += 1;
                min += 1;
            }
        }
        else{            
            let mut xmin = i[0];
            let mut xmax = i[2];
            let mut ymin = i[1];
            let mut ymax = i[3];
            let mut ymult = 1;
            let mut yv = 0;
            if i[0] > i[2]{
                xmin = i[2];
                xmax = i[0];
                ymin = i[3];
                ymax = i[1];
            } 
            if ymin > ymax{
                ymult =  -1;
            }
            let mut change = 0;
            while xmax >= xmin{
                yv = ymin + (change * ymult);
                grid[xmin as usize][yv as usize] += 1;
                change += 1;
                xmin += 1;
            }
        }
    }
    let mut counter = 0;
    for y in 0..1000{
        for x in 0..1000{
            if grid[y][x] > 1{
                counter += 1;
            }
        }
    }
    println!("Part 2: {}", counter);
}