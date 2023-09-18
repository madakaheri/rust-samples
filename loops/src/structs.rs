#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub is_active: bool,
}

impl User {
    pub fn new(id: u64, is_active: bool) -> User {
        User { id, is_active }
    }
}
