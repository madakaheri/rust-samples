pub fn array_logger(array: [i64; 5]) {
    for item in array.iter() {
        println!("item: {}", item);
    }
}

pub fn loop_logger() {
    let mut i: isize = 0;
    loop {
        if i > 100 {
            break;
        }
        println!("i: {}", i);
        i += 1;
    }
}
