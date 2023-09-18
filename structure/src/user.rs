#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub is_active: bool,
}

impl User {
    pub fn new(id: usize, is_active: bool) -> User {
        User { id, is_active }
    }
}
