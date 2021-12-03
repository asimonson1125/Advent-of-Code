use std::io::Read;
use std::env;

pub fn run() {
    println!("{:?}",env::current_dir().unwrap());
    let mut file_ref = std::fs::File::open("./src/day3/input.txt").unwrap();
    let mut data = String::new();

    file_ref.read_to_string(&mut data).unwrap();
    pt1(&data);
    println!();
    pt2(&data);
}

fn pt1(data: &String){
    let splits = data.split("\n");
    let lvec: Vec<&str> = splits.collect();
    let len: usize = lvec[0].len();
    let mut gamma = vec![-1; len];
    let mut epsilon = vec![-1; len];
    for i in 0..len{
        let mut zeroes = 0;
        let mut ones = 0;
        for x in &lvec{
            // println!("{}", x);
            if x.chars().nth(i).unwrap() == '0' {
                zeroes += 1;
            }
            else{
                ones += 1;
            }
        }
        if zeroes > ones {
            gamma[i] = 0;
            epsilon[i] = 1;
        }
        else{
            gamma[i] = 1;
            epsilon[i] = 0;
        }
    }
    let mut gstr = "".to_string();
    let mut estr = "".to_string();
    for i in gamma{
        gstr.push_str(&i.to_string());
    }
    for i in epsilon{
        estr.push_str(&i.to_string());
    }
    println!("Gamma: {}\nEpsilon: {}", gstr, estr);
}

fn pt2(data: &String){
    let splits = data.split("\n");
    let lvec: Vec<&str> = splits.collect();
    let len: usize = lvec[0].len();
    let count: usize = lvec.len();
    let mut oxy = vec!['0'; len];
    let mut co2 = vec!['1'; len];
    let mut ostatus = vec![true; count];
    let mut cstatus = vec![true; count];
    let mut ostr = "".to_string();
    let mut cstr = "".to_string();

    for i in 0..len{
        let mut zeroes = 0;
        let mut ones = 0;
        let mut c_zeroes = 0;
        let mut c_ones = 0;
        for x in 0..count{
            if ostatus[x]{
                if lvec[x].chars().nth(i).unwrap() == '0'{
                    zeroes += 1;
                }
                else{
                    ones += 1;
                }
            }
            if cstatus[x]{
                if lvec[x].chars().nth(i).unwrap() == '0'{
                    c_zeroes += 1;
                }
                else{
                    c_ones += 1;
                }
            }
        }
        if ones >= zeroes {
            oxy[i] = '1';
        }
        if c_ones >= c_zeroes {
            co2[i] = '0';
        }

        let mut os = 0;
        let mut cs = 0;
        let mut ov = "";
        let mut cv = "";
        for x in 0..count{
            if ostatus[x]{
                os += 1;
                ov = &lvec[x];
            }
            if cstatus[x]{
                cs += 1;
                cv = &lvec[x];
            }
        }
        if os == 1{
            ostr = ov.to_string();
        }
        if cs == 1{
            cstr = cv.to_string();
        }
        for x in 0..count {
            if ostatus[x] {
                if lvec[x].chars().nth(i).unwrap() != oxy[i] {
                    ostatus[x] = false;
                }
            }
            if cstatus[x] {
                if lvec[x].chars().nth(i).unwrap() != co2[i] {
                    cstatus[x] = false;
                }
            }
        }
    }
    if ostr == "".to_string(){
        for i in oxy{
            ostr.push_str(&i.to_string());
        }
    }
    if cstr == "".to_string(){
        for i in co2{
            cstr.push_str(&i.to_string());
        }
    }
    println!("Oxygen: {}\nCO2: {}", ostr, cstr);
}