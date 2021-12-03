fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn main(){
    let string = "A big fat watermelon has quite a few seeds.";
    let iter = string.split(" ");
    // println!("{:?}", type_of(&iter));
    println!("{}", get_size(&iter));
    let mut arr: [str; get_size(&iter)];
    let mut i = 0;
    while i < iter.len(){
        arr[i] = iter.next();
    }
}

fn get_size( iterable : &std::str::Split<&str>) -> i32 {
    let copyver = iterable.clone();
    let mut size = 0;
    for i in copyver{
        size += 1;
    }
    return size;
}
