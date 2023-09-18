// mod loops;
mod structs;

// const ITEMS: [u64; 5] = [1, 2, 3, 4, 5];

fn main() {
    // loops::array_logger(ITEMS);
    // loops::loop_logger();

    for id in 1..100 {
        let user = structs::User::new(id, id % 2 == 0);
        // println!("{:?}", user);
        println!("id: {} active: {}", user.id, user.is_active);
    }
}
