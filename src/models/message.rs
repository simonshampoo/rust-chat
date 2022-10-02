#[derive(Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub user: User,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(id: Uuid, user: User, body: String, created_at: DateTime<Utc>) -> Self {
        Message {
            id: id,
            user: user,
            body: String::from(body),
            created_at: created_at,
        }
    }
}
