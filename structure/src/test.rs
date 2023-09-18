mod user;

pub use user::User;

#[test]
pub fn logger(user: User) {
    println!("id: {} active: {}", user.id, user.is_active);
}
