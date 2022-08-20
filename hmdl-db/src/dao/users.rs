use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

pub enum Roles {
    Anonymous,
    Registered,
    Admin,
}

#[derive(sqlx::FromRow)]
pub struct User {
    display_name: String,
    id: Uuid,
    keys: Vec<Passkey>,
    role: Roles,
}
