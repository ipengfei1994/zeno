use uuid::Uuid;

pub fn generate_uuid_id() -> String {
    Uuid::new_v4().to_string()
}
