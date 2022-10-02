#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    pub fn new(id: Uuid, name: String) -> Self {
        User {
            id: id,
            name: String::from(name),
        }
    }
}
