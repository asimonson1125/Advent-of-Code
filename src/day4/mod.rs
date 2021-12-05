use std::io::Read;

pub fn run() {
    let mut file_ref = std::fs::File::open("./src/day4/input.txt").unwrap();
    let mut data = String::new();

    file_ref.read_to_string(&mut data).unwrap();
    let splits = data.split("\n");
    let lvec: Vec<&str> = splits.collect();
    let nums = lvec[0].split(",");
    let mut boards: [[[i32; 5]; 5]; 100] = [[[-1; 5]; 5]; 100];
    for i in 0..100{
        for y in 0..5{
            let row = lvec[i * 6 + 2 + y];
            let rownums = row.split(" ");
            let rowvec: Vec<&str> = rownums.collect();
            let mut spaces = 0;
            for x in 0..5{
                while rowvec[x + spaces] == ""{
                    spaces += 1;
                }
                boards[i][y][x] = rowvec[x + spaces].parse().unwrap();
            }
        }
    }

    println!("Part 1: {:?}", pt1(&boards, &nums));
    println!();
    println!("Part 2: {:?}", pt2(&boards, &nums));
}

fn pt2(boards: &[[[i32; 5]; 5]; 100], nums: &std::str::Split<&str>) -> i32{
    let iter: std::str::Split<&str> = nums.clone();
    let mut called: Vec<i32> = vec![-1];
    let boards_clone: Vec<[[i32; 5]; 5]> = boards.clone().to_vec();
    let mut won: Vec<i32> = vec![];
    for num in iter{
        called.push(num.parse().unwrap());
        let cur_boards_clone = boards_clone.clone();
        for boardid in 0..cur_boards_clone.len(){
            let mut run = true;
            let bid = &(boardid as i32);
            for i in &won{
                if i == bid{
                    run = false;
                }
            }
            if run{
                let board = boards[boardid];
                for x in 0..5{
                    let mut matches = true;
                    for y in 0..5{
                        if !called.contains(&board[y][x]){
                            matches = false;
                            break;
                        }
                    }
                    if matches{
                        let mut ignore = false;
                        for i in &won{
                            if i == bid{
                                ignore = true;
                            }
                        }
                        if!ignore{
                            won.push(boardid as i32);
                        }
                    }
                }
                for y in 0..5{
                    let mut matches = true;
                    for x in 0..5{
                        if !called.contains(&board[y][x]){
                            matches = false;
                            break;
                        }
                    }
                    if matches{
                        let mut ignore = false;
                        for i in &won{
                            if i == bid{
                                ignore = true;
                            }
                        }
                        if!ignore{
                            won.push(boardid as i32);
                        }
                    }
                }
            }
        }
        if won.len() == 100 {
            return score(&boards[won[99] as usize], &called);
        }
    }
    return -1;
}

fn pt1(boards: &[[[i32; 5]; 5]; 100], nums: &std::str::Split<&str>) -> i32{
    let iter: std::str::Split<&str> = nums.clone();
    let mut called: Vec<i32> = vec![-1];
    for num in iter{
        called.push(num.parse().unwrap());
        for board in boards{
            for x in 0..5{
                let mut matches = true;
                for y in 0..5{
                    if !called.contains(&board[y][x]){
                        matches = false;
                        break;
                    }
                }
                if matches{
                    return score(board, &called);
                }
            }
            for y in 0..5{
                let mut matches = true;
                for x in 0..5{
                    if !called.contains(&board[y][x]){
                        matches = false;
                        break;
                    }
                }
                if matches{
                    return score(board, &called);
                }
            }
        }
    }
    return -1;
}

fn score(board: &[[i32; 5]; 5], nums: &Vec<i32>) -> i32{
    let mut sum: i32 = 0;
    for y in 0..board.len(){
        for x in 0..board[y].len(){
            if !nums.contains(&board[y][x]){
                sum += board[y][x];
            }
        }
    }
    return sum * nums[nums.len()-1] as i32;
}